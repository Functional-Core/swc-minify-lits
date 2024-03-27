use itertools::Itertools;
use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    targets::Targets,
};
use swc_core::ecma::ast::Tpl;
use tracing::{event, instrument, Level};

use crate::prelude::*;

const PLACEHOLDER: &str = "@TEMPLATE_VARIABLE()";

#[derive(Debug, Default)]
pub struct CssProcessor {
    pub settings: CssSettings,
}

impl CssProcessor {
    pub fn new(settings: CssSettings) -> Self {
        Self { settings }
    }

    // Same as 'process_tpl' but ignores parse errors.
    #[instrument(level = Level::DEBUG)]
    pub fn try_process_tpl(&self, tpl: &mut Tpl) -> Result<()> {
        let res = self.process_tpl(tpl);

        if let Err(Error::CssParseError(error)) = res {
            event!(Level::DEBUG, error, "Ignoring CSS parse error",);
            return Ok(());
        }

        res
    }

    #[instrument(level = Level::DEBUG)]
    pub fn process_tpl(&self, tpl: &mut Tpl) -> Result<()> {
        let css_raw_iter = tpl.quasis.iter().map(|quasi| quasi.raw.as_str());
        let css_raw: String = Itertools::intersperse(css_raw_iter, PLACEHOLDER).collect();

        event!(Level::DEBUG, css_raw, "Parsing raw string as CSS");
        let mut stylesheet = parse_css(&css_raw)?;

        self.transform_css(&mut stylesheet)?;

        let new_css_raw = self.print_css(&stylesheet)?;
        event!(
            Level::DEBUG,
            new_css_raw,
            "Transformed CSS rendered to string",
        );

        let new_quasis = new_css_raw.split(PLACEHOLDER);

        tpl.quasis
            .iter_mut()
            .zip(new_quasis)
            .for_each(|(quasi, new_raw)| {
                quasi.raw = new_raw.into();
            });

        Ok(())
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

fn parse_css<'raw>(raw_css: &'raw str) -> Result<StyleSheet<'raw, 'raw>> {
    let parse_opts = ParserOptions::default();
    let stylesheet = StyleSheet::parse(raw_css, parse_opts)?;
    Ok(stylesheet)
}
