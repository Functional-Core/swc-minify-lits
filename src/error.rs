use lightningcss as css;

pub type CssError<T> = css::error::Error<T>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    CssParseError(String),
    #[error("{0}")]
    CssMinifyError(String),
    #[error("{0}")]
    CssPrintError(String),
}

impl<'i> From<CssError<css::error::ParserError<'i>>> for Error {
    fn from(e: CssError<css::error::ParserError<'i>>) -> Self {
        Error::CssParseError(e.to_string())
    }
}

impl<'i> From<CssError<css::error::MinifyErrorKind>> for Error {
    fn from(e: CssError<css::error::MinifyErrorKind>) -> Self {
        Error::CssMinifyError(e.to_string())
    }
}

impl<'i> From<CssError<css::error::PrinterErrorKind>> for Error {
    fn from(e: CssError<css::error::PrinterErrorKind>) -> Self {
        Error::CssPrintError(e.to_string())
    }
}
