use serde::{self, Deserialize};

#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    #[serde(flatten)]
    pub css_settings: CssSettings,
    #[serde(flatten)]
    pub html_settings: HtmlSettings,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CssSettings {}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HtmlSettings {
    #[serde(rename = "minifyCssInHtml")]
    pub minify_css: bool,
    #[serde(rename = "minifyJsInHtml")]
    pub minify_js: bool,
}

impl Default for HtmlSettings {
    fn default() -> Self {
        Self {
            minify_css: true,
            minify_js: true,
        }
    }
}
