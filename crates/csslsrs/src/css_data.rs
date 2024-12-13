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
    version: 1.1,
    at_directives: None,
    pseudo_classes: None,
    pseudo_elements: None,
    properties: None,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CssCustomData {
    pub version: f64,
    pub properties: Option<Vec<PropertyEntry>>,
    pub at_directives: Option<Vec<AtDirectiveEntry>>,
    pub pseudo_classes: Option<Vec<PseudoClassEntry>>,
    pub pseudo_elements: Option<Vec<PseudoElementEntry>>,
}

#[derive(Debug, Deserialize)]
pub struct PropertyEntry {
    pub name: String,
    pub description: Option<MarkupDescriptionOrString>,
    pub status: Option<Status>,
    pub browsers: Option<Vec<String>>,
    pub references: Option<Vec<Reference>>,
    pub relevance: Option<f64>,

    // Not in the JSON schema, but found in VS Code's types and CSS data, strange.
    #[serde(rename = "atRule")]
    pub at_rule: Option<String>,
    pub syntax: Option<String>,
    pub restrictions: Option<Vec<String>>,
    pub values: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize)]
pub struct Value {
    pub name: String,
    pub description: Option<MarkupDescriptionOrString>,
    pub browsers: Option<Vec<String>>,
    pub status: Option<Status>,
    pub references: Option<Vec<Reference>>,
}

#[derive(Debug, Deserialize)]
pub struct AtDirectiveEntry {
    pub name: String,
    pub description: Option<MarkupDescriptionOrString>,
    pub status: Option<Status>,
    pub browsers: Option<Vec<String>>,
    pub references: Option<Vec<Reference>>,
}

#[derive(Debug, Deserialize)]
pub struct PseudoClassEntry {
    pub name: String,
    pub description: Option<MarkupDescriptionOrString>,
    pub status: Option<Status>,
    pub browsers: Option<Vec<String>>,
    pub references: Option<Vec<Reference>>,
}

#[derive(Debug, Deserialize)]
pub struct PseudoElementEntry {
    pub name: String,
    pub description: Option<MarkupDescriptionOrString>,
    pub status: Option<Status>,
    pub browsers: Option<Vec<String>>,
    pub references: Option<Vec<Reference>>,
}

#[derive(Debug, Deserialize)]
pub struct Reference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MarkupDescriptionOrString {
    MarkupDescription(MarkupDescription),
    String(String),
}

#[derive(Debug, Deserialize)]
pub struct MarkupDescription {
    pub kind: String, // ["plaintext", "markdown"]
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Standard,
    Experimental,
    Nonstandard,
    Obsolete,
}
