use swc_core::ecma::ast::Tpl;
use tracing::instrument;

use crate::prelude::*;

pub trait TplProcessor
where
    Self: std::fmt::Debug,
{
    fn process_tpl(&self, tpl: &mut Tpl) -> Result<()>;

    // Same as 'process_tpl' but ignores parse errors.
    #[instrument(level = Level::DEBUG)]
    fn try_process_tpl(&self, tpl: &mut Tpl) -> Result<()> {
        let res = self.process_tpl(tpl);

        match res {
            Err(err) if err.is_parse_error() => {
                event!(Level::DEBUG, %err, "Ignoring parse error");
                Ok(())
            }
            _ => res,
        }
    }
}
