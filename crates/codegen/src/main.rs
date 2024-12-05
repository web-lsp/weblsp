use codegen_file::CodegenFile;

mod codegen_file;
use csslsrs::css_data::{
    AtDirectiveAttributes, AtDirectives, CssCustomData, Properties, PropertyAttributes,
    PseudoClassAttributes, PseudoClasses, PseudoElementAttributes, PseudoElements,
};
use proc_macro2::TokenStream;
use quote::quote;

fn main() {
    let mut file = CodegenFile::create("crates/csslsrs/src/css_data_generated.rs").unwrap();
    let json: CssCustomData = serde_json::from_str(include_str!("../res/css-schema.json")).unwrap();

    let at_atributes = build_at_attributes(json.css.at_directives);
    let pseudo_classes = build_pseudo_classes(json.css.pseudo_classes);
    let pseudo_elements = build_pseudo_elements(json.css.pseudo_elements);
    let properties = build_properties(json.css.properties);

    let start_struct = quote! {
        use crate::css_data::*;
        use std::borrow::Cow::Borrowed;
        use std::sync::LazyLock;

        #at_atributes
        #pseudo_classes
        #pseudo_elements
        #properties

        pub(crate) static CSS_DATA: LazyLock<CssCustomData> = LazyLock::new(|| CssCustomData {
            css: CssSection {
                at_directives: AtDirectives { entry: AT_DIRECTIVES.to_vec() },
                pseudo_classes: PseudoClasses { entry: PSEUDO_CLASSES.to_vec() },
                pseudo_elements: PseudoElements { entry: PSEUDO_ELEMENTS.to_vec() },
                properties: Properties { entry: PROPERTIES.to_vec() },
            }
        });

        #[allow(dead_code)]
        pub(crate) static EMPTY_CSS_DATA: LazyLock<CssCustomData> = LazyLock::new(|| CssCustomData {
            css: CssSection {
                at_directives: AtDirectives { entry: vec![] },
                pseudo_classes: PseudoClasses { entry: vec![] },
                pseudo_elements: PseudoElements { entry: vec![] },
                properties: Properties { entry: vec![] },
            }
        });
    };

    file.append(start_struct).unwrap();
}

fn build_at_attributes(at_directives: AtDirectives) -> TokenStream {
    let constants = at_directives.entry.iter().map(|value| {
        let AtDirectiveAttributes {
            name,
            version,
            browsers,
            ref_,
            syntax,
        } = &value.attributes;

        let name = quote! {name: Borrowed(#name)};
        let version = version.as_ref().map_or_else(
            || quote! {version: None},
            |v| quote! {version: Some(Borrowed(#v))},
        );
        let browsers = browsers.as_ref().map_or_else(
            || quote! {browsers: None},
            |b| quote! {browsers: Some(Borrowed(#b))},
        );
        let ref_ = ref_.as_ref().map_or_else(
            || quote! {ref_: None},
            |r| quote! {ref_: Some(Borrowed(#r))},
        );
        let syntax = syntax.as_ref().map_or_else(
            || quote! {syntax: None},
            |s| quote! {syntax: Some(Borrowed(#s))},
        );

        let desc = value.desc.clone().unwrap_or(std::borrow::Cow::Borrowed(""));

        quote! {
            AtDirectiveEntry {
                attributes: AtDirectiveAttributes {
                    #name,
                    #version,
                    #browsers,
                    #ref_,
                    #syntax,
                },
                desc: Some(Borrowed(#desc)),
            },
        }
    });

    let length = at_directives.entry.len();
    quote! {
        #[derive(Clone)]
        const AT_DIRECTIVES: [AtDirectiveEntry; #length] = [#(#constants)*];
    }
}

fn build_pseudo_classes(pseudo_classes: PseudoClasses) -> TokenStream {
    let constants = pseudo_classes.entry.iter().map(|value| {
        let PseudoClassAttributes {
            name,
            version,
            browsers,
            ref_,
            syntax,
        } = &value.attributes;

        let name = quote! {name: Borrowed(#name)};
        let version = version.as_ref().map_or_else(
            || quote! {version: None},
            |v| quote! {version: Some(Borrowed(#v))},
        );
        let browsers = browsers.as_ref().map_or_else(
            || quote! {browsers: None},
            |b| quote! {browsers: Some(Borrowed(#b))},
        );
        let ref_ = ref_.as_ref().map_or_else(
            || quote! {ref_: None},
            |r| quote! {ref_: Some(Borrowed(#r))},
        );
        let syntax = syntax.as_ref().map_or_else(
            || quote! {syntax: None},
            |s| quote! {syntax: Some(Borrowed(#s))},
        );

        let desc = value.desc.clone().unwrap_or(std::borrow::Cow::Borrowed(""));

        quote! {
            PseudoClassEntry {
                attributes: PseudoClassAttributes {
                    #name,
                    #version,
                    #browsers,
                    #ref_,
                    #syntax,
                },
                desc: Some(Borrowed(#desc)),
            },
        }
    });

    let length = pseudo_classes.entry.len();
    quote! {
        #[derive(Clone)]
        const PSEUDO_CLASSES: [PseudoClassEntry; #length] = [#(#constants)*];
    }
}

fn build_pseudo_elements(pseudo_elements: PseudoElements) -> TokenStream {
    let constants = pseudo_elements.entry.iter().map(|value| {
        let PseudoElementAttributes {
            name,
            version,
            browsers,
            ref_,
            syntax,
        } = &value.attributes;

        let name = quote! {name: Borrowed(#name)};
        let version = version.as_ref().map_or_else(
            || quote! {version: None},
            |v| quote! {version: Some(Borrowed(#v))},
        );
        let browsers = browsers.as_ref().map_or_else(
            || quote! {browsers: None},
            |b| quote! {browsers: Some(Borrowed(#b))},
        );
        let ref_ = ref_.as_ref().map_or_else(
            || quote! {ref_: None},
            |r| quote! {ref_: Some(Borrowed(#r))},
        );
        let syntax = syntax.as_ref().map_or_else(
            || quote! {syntax: None},
            |s| quote! {syntax: Some(Borrowed(#s))},
        );

        let desc = value.desc.clone().unwrap_or(std::borrow::Cow::Borrowed(""));

        quote! {
            PseudoElementEntry {
                attributes: PseudoElementAttributes {
                    #name,
                    #version,
                    #browsers,
                    #ref_,
                    #syntax,
                },
                desc: Some(Borrowed(#desc)),
            },
        }
    });

    let length = pseudo_elements.entry.len();
    quote! {
        #[derive(Clone)]
        const PSEUDO_ELEMENTS: [PseudoElementEntry; #length] = [#(#constants)*];
    }
}

fn build_properties(properties: Properties) -> TokenStream {
    let constants = properties.entry.iter().map(|value| {
        let PropertyAttributes {
            name,
            version,
            browsers,
            ref_,
            syntax,
            restriction,
        } = &value.attributes;

        let name = quote! {name: Borrowed(#name)};
        let version = version.as_ref().map_or_else(
            || quote! {version: None},
            |v| quote! {version: Some(Borrowed(#v))},
        );
        let browsers = browsers.as_ref().map_or_else(
            || quote! {browsers: None},
            |b| quote! {browsers: Some(Borrowed(#b))},
        );
        let ref_ = ref_.as_ref().map_or_else(
            || quote! {ref_: None},
            |r| quote! {ref_: Some(Borrowed(#r))},
        );
        let syntax = syntax.as_ref().map_or_else(
            || quote! {syntax: None},
            |s| quote! {syntax: Some(Borrowed(#s))},
        );
        let restriction = restriction.as_ref().map_or_else(
            || quote! {restriction: None},
            |r| quote! {restriction: Some(Borrowed(#r))},
        );

        let desc = value.desc.clone().unwrap_or(std::borrow::Cow::Borrowed(""));

        quote! {
            PropertyEntry {
                attributes: PropertyAttributes {
                    #name,
                    #version,
                    #browsers,
                    #ref_,
                    #syntax,
                    #restriction,
                },
                desc: Some(Borrowed(#desc)),
            },
        }
    });

    let length = properties.entry.len();
    quote! {
        #[derive(Clone)]
        const PROPERTIES: [PropertyEntry; #length] = [#(#constants)*];
    }
}
