use itertools::Itertools;
use swc_core::ecma::ast::Tpl;

pub fn join_quasis(tpl: &Tpl, placeholder: &str) -> String {
    let raw_iter = tpl.quasis.iter().map(|quasi| quasi.raw.as_str());
    Itertools::intersperse(raw_iter, placeholder).collect()
}

pub fn replace_quasis(tpl: &mut Tpl, new_raw: &str, placeholder: &str) {
    let new_quasis = new_raw.split(placeholder);

    tpl.quasis
        .iter_mut()
        .zip(new_quasis)
        .for_each(|(quasi, new_raw)| {
            quasi.raw = new_raw.into();
        });
}
