use serde::Deserialize;
use std::borrow::Cow;

#[derive(Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
#[serde(rename_all = "camelCase")]
/// Represents any CSS data provided by the user or MDN.
/// This is used to provide completions and hover information.
pub struct CssCustomData<'a> {
    pub css: CssSection<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct CssSection<'a> {
    pub at_directives: AtDirectives<'a>,
    pub pseudo_classes: PseudoClasses<'a>,
    pub pseudo_elements: PseudoElements<'a>,
    pub properties: Properties<'a>,
}

#[derive(Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct AtDirectives<'a> {
    pub entry: Vec<AtDirectiveEntry<'a>>,
}

#[derive(Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct PseudoClasses<'a> {
    pub entry: Vec<PseudoClassEntry<'a>>,
}

#[derive(Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct PseudoElements<'a> {
    pub entry: Vec<PseudoElementEntry<'a>>,
}

#[derive(Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Properties<'a> {
    pub entry: Vec<PropertyEntry<'a>>,
}

#[derive(Deserialize)]
pub struct AtDirectiveEntry<'a> {
    #[serde(rename = "$")]
    pub attributes: AtDirectiveAttributes<'a>,
    #[serde(borrow)]
    pub desc: Option<Cow<'a, str>>,
}

#[derive(Deserialize)]
pub struct PseudoClassEntry<'a> {
    #[serde(rename = "$")]
    pub attributes: PseudoClassAttributes<'a>,
    #[serde(borrow)]
    pub desc: Option<Cow<'a, str>>,
}

#[derive(Deserialize)]
pub struct PseudoElementEntry<'a> {
    #[serde(rename = "$")]
    pub attributes: PseudoElementAttributes<'a>,
    #[serde(borrow)]
    pub desc: Option<Cow<'a, str>>,
}

#[derive(Deserialize)]
pub struct PropertyEntry<'a> {
    #[serde(rename = "$")]
    pub attributes: PropertyAttributes<'a>,
    #[serde(borrow)]
    pub desc: Option<Cow<'a, str>>,
}

#[derive(Deserialize)]
pub struct AtDirectiveAttributes<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub version: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub browsers: Option<Cow<'a, str>>,
    #[serde(rename = "ref", borrow)]
    pub ref_: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub syntax: Option<Cow<'a, str>>,
}

#[derive(Deserialize)]
pub struct PseudoClassAttributes<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub version: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub browsers: Option<Cow<'a, str>>,
    #[serde(rename = "ref", borrow)]
    pub ref_: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub syntax: Option<Cow<'a, str>>,
}

#[derive(Deserialize)]
pub struct PseudoElementAttributes<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub version: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub browsers: Option<Cow<'a, str>>,
    #[serde(rename = "ref", borrow)]
    pub ref_: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub syntax: Option<Cow<'a, str>>,
}

#[derive(Deserialize)]
pub struct PropertyAttributes<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub restriction: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub version: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub browsers: Option<Cow<'a, str>>,
    #[serde(rename = "ref", borrow)]
    pub ref_: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub syntax: Option<Cow<'a, str>>,
}
