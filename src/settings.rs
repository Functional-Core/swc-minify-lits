use serde::{self, Deserialize};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub minify_css: bool,
    pub minify_js: bool,
    #[serde(flatten)]
    pub css_settings: CssSettings,
    #[serde(flatten)]
    pub html_settings: HtmlSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            minify_css: true,
            minify_js: true,
            css_settings: Default::default(),
            html_settings: Default::default(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct CssSettings {}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct HtmlSettings {
    #[serde(rename = "minifyCssInHtml")]
    pub minify_css: bool,
    #[serde(rename = "minifyJsInHtml")]
    pub minify_js: bool,
    #[serde(rename = "keepHtmlComments")]
    pub keep_comments: bool,
}
