use itertools::Itertools;
use lightningcss::{
    stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet},
    targets::Targets,
};
use swc_core::ecma::ast::Tpl;

use crate::prelude::*;

const PLACEHOLDER: &str = "@TEMPLATE_VARIABLE()";

// Same as 'process_tpl' but ignores parse errors.
pub fn try_process_tpl(tpl: &mut Tpl) -> Result<()> {
    let res = process_tpl(tpl);

    if let Err(Error::CssParseError(_)) = res {
        return Ok(());
    }

    res
}

pub fn process_tpl(tpl: &mut Tpl) -> Result<()> {
    let css_raw_iter = tpl.quasis.iter().map(|quasi| quasi.raw.as_str());
    let css_raw: String = Itertools::intersperse(css_raw_iter, PLACEHOLDER).collect();
    let mut stylesheet = parse_css(&css_raw)?;
    transform_css(&mut stylesheet)?;
    let new_css_raw = print_css(&stylesheet)?;

    let new_quasis = new_css_raw.split(PLACEHOLDER);

    tpl.quasis
        .iter_mut()
        .zip(new_quasis)
        .for_each(|(quasi, new_raw)| {
            quasi.raw = new_raw.into();
        });

    Ok(())
}

fn parse_css(raw_css: &str) -> Result<StyleSheet> {
    let parse_opts = ParserOptions::default();
    let stylesheet = StyleSheet::parse(raw_css, parse_opts)?;
    Ok(stylesheet)
}

fn transform_css<'a>(stylesheet: &mut StyleSheet) -> Result<'a, ()> {
    let targets = Targets::default();

    let minify_opts = MinifyOptions {
        targets: targets.clone(),
        ..Default::default()
    };

    stylesheet.minify(minify_opts)?;

    Ok(())
}

fn print_css<'a>(stylesheet: &'a StyleSheet) -> Result<'a, String> {
    let targets = Targets::default();

    let printer_opts = PrinterOptions {
        targets,
        minify: true,
        ..Default::default()
    };
    let css = stylesheet.to_css(printer_opts)?;

    Ok(css.code)
}
