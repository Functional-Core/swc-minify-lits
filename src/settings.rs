use serde::{self, Deserialize};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Settings {
    #[serde(flatten)]
    pub plugin_settings: PluginSettings,
    #[serde(flatten)]
    pub css_settings: CssSettings,
    #[serde(flatten)]
    pub html_settings: HtmlSettings,
}

#[derive(Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PluginSettings {
    pub minify_css: bool,
    pub minify_html: bool,
}

impl Default for PluginSettings {
    fn default() -> Self {
        PluginSettings {
            minify_css: true,
            minify_html: true,
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
