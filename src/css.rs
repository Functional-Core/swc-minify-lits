use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    targets::Targets,
};
use swc_core::ecma::ast::Tpl;

use crate::{
    prelude::*,
    tpl_processor::TplProcessor,
    utils::quasi::{join_quasis, replace_quasis},
};

const PLACEHOLDER: &str = "@TEMPLATE_VARIABLE()";

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
        let css_raw = join_quasis(tpl, PLACEHOLDER);

        event!(Level::DEBUG, css_raw, "Parsing raw string as CSS");
        let mut stylesheet = parse_css(&css_raw)?;

        self.transform_css(&mut stylesheet)?;

        let new_css_raw = self.print_css(&stylesheet)?;
        event!(
            Level::DEBUG,
            new_css_raw,
            "Transformed CSS rendered to string",
        );

        replace_quasis(tpl, &new_css_raw, PLACEHOLDER);

        Ok(())
    }
}

fn parse_css<'raw>(raw_css: &'raw str) -> Result<StyleSheet<'raw, 'raw>> {
    let parse_opts = ParserOptions::default();
    let stylesheet = StyleSheet::parse(raw_css, parse_opts)?;
    Ok(stylesheet)
}
