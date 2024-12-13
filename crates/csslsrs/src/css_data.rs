use serde::Deserialize;
use std::sync::LazyLock;

pub(crate) static BASE_CSS_DATA: LazyLock<CssCustomData> = LazyLock::new(|| {
    serde_json::from_str(include_str!(
        "../../../node_modules/@vscode/web-custom-data/data/browsers.css-data.json"
    ))
    .expect("Failed to parse css-schema.json")
});

// This is used when the user sets `include_base_css_custom_data` to false in the LanguageServiceOptions.
pub(crate) static EMPTY_CSS_DATA: CssCustomData = CssCustomData {
    at_directives: vec![],
    pseudo_classes: vec![],
    pseudo_elements: vec![],
    properties: vec![],
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CssCustomData {
    pub at_directives: Vec<AtDirectiveEntry>,
    pub pseudo_classes: Vec<PseudoClassEntry>,
    pub pseudo_elements: Vec<PseudoElementEntry>,
    pub properties: Vec<PropertyEntry>,
}

#[derive(Deserialize)]
pub struct Reference {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct AtDirectiveEntry {
    pub name: String,
    pub browsers: Option<Vec<String>>,
    pub references: Option<Vec<Reference>>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct PseudoClassEntry {
    pub name: String,
    pub browsers: Option<Vec<String>>,
    pub references: Option<Vec<Reference>>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct PseudoElementEntry {
    pub name: String,
    pub browsers: Option<Vec<String>>,
    pub references: Option<Vec<Reference>>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct PropertyEntry {
    pub name: String,
    pub browsers: Option<Vec<String>>,
    #[serde(rename = "atRule")]
    pub at_rule: Option<String>,
    pub references: Option<Vec<Reference>>,
    pub syntax: Option<String>,
    pub relevance: Option<f64>,
    pub description: Option<String>,
    pub restrictions: Option<Vec<String>>,
}
