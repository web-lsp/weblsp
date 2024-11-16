use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CssCustomData {
    pub css: CssSection,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CssSection {
    pub at_directives: AtDirectives,
    pub pseudo_classes: PseudoClasses,
    pub pseudo_elements: PseudoElements,
    pub properties: Properties,
}

#[derive(Deserialize)]
pub struct AtDirectives {
    pub entry: Vec<AtDirectiveEntry>,
}

#[derive(Deserialize)]
pub struct PseudoClasses {
    pub entry: Vec<PseudoClassEntry>,
}

#[derive(Deserialize)]
pub struct PseudoElements {
    pub entry: Vec<PseudoElementEntry>,
}

#[derive(Deserialize)]
pub struct Properties {
    pub entry: Vec<PropertyEntry>,
}

#[derive(Deserialize)]
pub struct AtDirectiveEntry {
    #[serde(rename = "$")]
    pub attributes: AtDirectiveAttributes,
    pub desc: Option<String>,
}

#[derive(Deserialize)]
pub struct PseudoClassEntry {
    #[serde(rename = "$")]
    pub attributes: PseudoClassAttributes,
    pub desc: Option<String>,
}

#[derive(Deserialize)]
pub struct PseudoElementEntry {
    #[serde(rename = "$")]
    pub attributes: PseudoElementAttributes,
    pub desc: Option<String>,
}

#[derive(Deserialize)]
pub struct PropertyEntry {
    #[serde(rename = "$")]
    pub attributes: PropertyAttributes,
    pub desc: Option<String>,
}

#[derive(Deserialize)]
pub struct AtDirectiveAttributes {
    pub name: String,
    pub version: Option<String>,
    pub browsers: Option<String>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    pub syntax: Option<String>,
}

#[derive(Deserialize)]
pub struct PseudoClassAttributes {
    pub name: String,
    pub version: Option<String>,
    pub browsers: Option<String>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    pub syntax: Option<String>,
}

#[derive(Deserialize)]
pub struct PseudoElementAttributes {
    pub name: String,
    pub version: Option<String>,
    pub browsers: Option<String>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    pub syntax: Option<String>,
}

#[derive(Deserialize)]
pub struct PropertyAttributes {
    pub name: String,
    pub restriction: Option<String>,
    pub version: Option<String>,
    pub browsers: Option<String>,
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    pub syntax: Option<String>,
}
