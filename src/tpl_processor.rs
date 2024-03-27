use swc_core::ecma::ast::Tpl;
use tracing::instrument;

use crate::prelude::*;

pub trait TplProcessor
where
    Self: std::fmt::Debug,
{
    fn process_tpl(&self, tpl: &mut Tpl) -> Result<()>;

    // Same as 'process_tpl' but ignores parse errors.
    fn try_process_tpl(&self, tpl: &mut Tpl) -> Result<()>;
}
