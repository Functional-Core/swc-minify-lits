use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    targets::Targets,
};
use swc_core::ecma::ast::Tpl;

use crate::{
    prelude::*,
    tpl_processor::TplProcessor,
    utils::tpl::{join_quasis, reorder_exprs, replace_quasis, split_new_quasis},
};

const PLACEHOLDER_BASE: &str = "@TEMPLATE_VARIABLE_";
const PLACEHOLDER_SUFFIX: &str = "_()";

#[derive(Debug, Default)]
pub struct CssProcessor {
    pub settings: CssSettings,
}

impl CssProcessor {
    pub fn new(settings: CssSettings) -> Self {
        Self { settings }
    }

    fn transform_css(&self, stylesheet: &mut StyleSheet) -> Result<()> {
        let targets = Targets::default();

        let minify_opts = MinifyOptions {
            targets: targets.clone(),
            ..Default::default()
        };

        stylesheet.minify(minify_opts)?;

        Ok(())
    }

    fn print_css(&self, stylesheet: &StyleSheet) -> Result<String> {
        let targets = Targets::default();

        let printer_opts = PrinterOptions {
            targets,
            minify: true,
            ..Default::default()
        };
        let css = stylesheet.to_css(printer_opts)?;

        Ok(css.code)
    }
}

impl TplProcessor for CssProcessor {
    #[instrument(level = Level::DEBUG)]
    fn process_tpl(&self, tpl: &mut Tpl) -> Result<()> {
        let css_raw = join_quasis(tpl, PLACEHOLDER_BASE, PLACEHOLDER_SUFFIX);

        debug!(css_raw, "Parsing raw string as CSS");
        let mut stylesheet = parse_css(&css_raw)?;

        self.transform_css(&mut stylesheet)?;

        let new_css_raw = self.print_css(&stylesheet)?;
        debug!(new_css_raw, "Transformed CSS rendered to string",);

        let (new_expr_ixs, new_quasis) =
            split_new_quasis(&new_css_raw, PLACEHOLDER_BASE, PLACEHOLDER_SUFFIX);
        let debug_new_ixs = format!("{:?}", new_expr_ixs);
        let debug_new_quasis = format!("{:?}", new_quasis);
        debug!(
            debug_new_ixs,
            debug_new_quasis, "New expression indexes for CSS template",
        );

        replace_quasis(tpl, new_quasis);
        reorder_exprs(tpl, new_expr_ixs);

        Ok(())
    }
}

fn parse_css<'raw>(raw_css: &'raw str) -> Result<StyleSheet<'raw, 'raw>> {
    let parse_opts = ParserOptions::default();
    let stylesheet = StyleSheet::parse(raw_css, parse_opts)?;
    Ok(stylesheet)
}

pub fn is_css(tpl: &Tpl) -> bool {
    let raw = join_quasis(tpl, PLACEHOLDER_BASE, PLACEHOLDER_SUFFIX);
    let res = parse_css(&raw);
    res.is_ok()
}
