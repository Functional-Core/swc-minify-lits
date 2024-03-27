#![allow(unused)]

use std::os::unix::process;

use css::CssProcessor;
use serde::Deserialize;
use swc_core::{
    atoms::Atom,
    ecma::{
        ast::{Expr, Program, TaggedTpl, Tpl},
        transforms::testing::test_inline,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{errors::HANDLER, plugin_transform, proxies::TransformPluginProgramMetadata},
};

use crate::prelude::*;

mod css;
mod error;
mod prelude;
mod settings;

#[derive(Default)]
pub struct TransformVisitor {
    css_processor: CssProcessor,
    tag: Option<Atom>,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_tagged_tpl(&mut self, ttpl: &mut TaggedTpl) {
        let old_tag = self.tag.to_owned();

        if let Expr::Ident(tag) = &mut *ttpl.tag {
            self.tag = Some(tag.sym.clone());
        }

        ttpl.visit_mut_children_with(self);

        self.tag = old_tag;
    }

    fn visit_mut_tpl(&mut self, tpl: &mut Tpl) {
        tpl.visit_mut_children_with(self);

        let res = match &self.tag {
            Some(s) if s == "css" => self.css_processor.process_tpl(tpl),
            None => self.css_processor.try_process_tpl(tpl),
            _ => return,
        };

        if let Err(err) = res {
            HANDLER.with(|h| {
                h.struct_span_err(tpl.span, &err.to_string()).emit();
            });
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let opts: Settings = serde_json::from_str(
        &metadata
            .get_transform_plugin_config()
            .expect("No plugin settings found for minify-lits"),
    )
    .expect("Invalid plugin settings for minify-lits");

    let visitor = TransformVisitor {
        css_processor: CssProcessor::new(opts.css_settings),
        ..TransformVisitor::default()
    };

    program.fold_with(&mut as_folder(visitor))
}

#[cfg(test)]
mod test {
    use super::*;

    test_inline!(
        Default::default(),
        |_| as_folder(TransformVisitor::default()),
        tagged_css,
        // Input
        r#"
        var styles = css`
            :host {
                font-family: sans-serif;
            }
    
            .potato {
                padding-top: 10px;
                padding-bottom: 10px;
                padding-right: 5px;
                padding-left: 5px;
            }
        `;
        "#,
        // Output
        r#"var styles = css`:host{font-family:sans-serif}.potato{padding:10px 5px}`;"#
    );

    test_inline!(
        Default::default(),
        |_| as_folder(TransformVisitor::default()),
        untagged_css,
        // Input
        r#"
        var styles = `
            .potato {
                padding: 10px;
            }
        `;
        "#,
        // Output
        r#"var styles = `.potato{padding:10px}`;"#
    );

    test_inline!(
        Default::default(),
        |_| as_folder(TransformVisitor::default()),
        css_with_variables,
        // Input
        r#"
        var styles = css`
            :host {
                font-family: sans-serif;
            }
    
            .potato {
                padding-top: ${5 + 5}px;
                padding-bottom: 10px;
            }
        `;
        "#,
        // Output
        r#"var styles = css`:host{font-family:sans-serif}.potato{padding-top:${5 + 5}px;padding-bottom:10px}`;"#
    );
}
