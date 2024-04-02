use itertools::Itertools;
use swc_core::ecma::ast::Tpl;

pub fn join_quasis(tpl: &Tpl, ph_base: &str, ph_suffix: &str) -> String {
    let raw_iter = tpl.quasis.iter().map(|quasi| quasi.raw.to_string());
    let mut ph_iter = (0..).map(|n| format!("{}{}{}", ph_base, n, ph_suffix));
    let get_ph = || ph_iter.next().unwrap();
    Itertools::intersperse_with(raw_iter, get_ph).collect()
}

pub fn split_new_quasis<'raw>(
    new_raw: &'raw str,
    ph_base: &str,
    ph_suffix: &str,
) -> (Vec<u32>, Vec<&'raw str>) {
    let mut expr_nums = Vec::new();

    let new_quasis = new_raw
        .split(ph_base)
        .map(|part| {
            if let Some((ph_num, quasi)) = part.split_once(ph_suffix) {
                expr_nums.push(ph_num.parse::<u32>().unwrap());
                quasi
            } else {
                part
            }
        })
        .collect();

    (expr_nums, new_quasis)
}

pub fn reorder_exprs(tpl: &mut Tpl, new_expr_ixs: Vec<u32>) {
    let new_exprs: Vec<Box<swc_core::ecma::ast::Expr>> = new_expr_ixs
        .into_iter()
        .map(|n| unsafe { tpl.exprs.get_unchecked(n as usize).to_owned() })
        .collect();

    tpl.exprs = new_exprs;
}

pub fn replace_quasis(tpl: &mut Tpl, new_quasis: Vec<&str>) {
    tpl.quasis
        .iter_mut()
        .zip(new_quasis)
        .for_each(|(quasi, new_raw)| {
            quasi.raw = new_raw.into();
        });
}
