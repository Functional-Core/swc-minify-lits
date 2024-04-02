use minify_html::Cfg;
use swc_core::ecma::ast::Tpl;

use crate::{prelude::*, tpl_processor::*, utils::tpl::*};

const PLACEHOLDER_BASE: &str = "TEMPLATE_VARIABLE_";
const PLACEHOLDER_SUFFIX: &str = "_END";

#[derive(Debug, Default)]
pub struct HtmlProcessor {
    pub settings: HtmlSettings,
}

impl HtmlProcessor {
    pub fn new(settings: HtmlSettings) -> Self {
        Self { settings }
    }

    fn transform_html(&self, raw_html: &str) -> Result<String> {
        let cfg: Cfg = Cfg {
            do_not_minify_doctype: true,
            ensure_spec_compliant_unquoted_attribute_values: true,
            keep_closing_tags: true,
            keep_html_and_head_opening_tags: true,
            keep_spaces_between_attributes: true,
            keep_comments: self.settings.keep_comments,
            keep_input_type_text_attr: true,
            keep_ssi_comments: true,
            preserve_brace_template_syntax: true,
            preserve_chevron_percent_template_syntax: true,
            minify_css: self.settings.minify_css,
            minify_js: self.settings.minify_js,
            remove_bangs: false,
            remove_processing_instructions: false,
        };

        let minified = minify_html::minify(raw_html.as_bytes(), &cfg);

        unsafe { Ok(String::from_utf8_unchecked(minified)) }
    }
}

impl TplProcessor for HtmlProcessor {
    #[instrument(level = Level::DEBUG)]
    fn process_tpl(&self, tpl: &mut Tpl) -> Result<()> {
        let raw_html = join_quasis(tpl, PLACEHOLDER_BASE, PLACEHOLDER_SUFFIX);

        debug!(raw_html, "Transforming raw string as HTML");
        let new_raw = self.transform_html(&raw_html)?;
        debug!(new_raw, "Transformed HTML");

        let (new_expr_ixs, new_quasis) =
            split_new_quasis(&new_raw, PLACEHOLDER_BASE, PLACEHOLDER_SUFFIX);
        let debug_new_ixs = format!("{:?}", new_expr_ixs);
        let debug_new_quasis = format!("{:?}", new_quasis);
        debug!(
            debug_new_ixs,
            debug_new_quasis, "New quasis and expr indexes"
        );

        replace_quasis(tpl, new_quasis);
        reorder_exprs(tpl, new_expr_ixs);

        Ok(())
    }
}
