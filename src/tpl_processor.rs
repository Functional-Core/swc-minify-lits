use swc_core::ecma::ast::Tpl;

use crate::prelude::*;

pub trait TplProcessor
where
    Self: std::fmt::Debug,
{
    fn process_tpl(&self, tpl: &mut Tpl) -> Result<()>;
}
