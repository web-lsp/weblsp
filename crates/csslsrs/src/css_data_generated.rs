// This file is auto-generated and any manual changes to it will be overwritten.
//
// Run `cargo run -p codegen` from the project root to regenerate it.

use crate::css_data::*;
use std::borrow::Cow::Borrowed;
use std::sync::LazyLock;
pub(crate) static CSS_DATA: LazyLock<CssCustomData> = LazyLock::new(|| {
    CssCustomData {
    css: CssSection {
        at_directives: AtDirectives {
            entry: vec![
                AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@bottom-center"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-width box within the area defined by the bottom margin, centered on the page area, and between the bottom-left and bottom-right margin boxes.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@bottom-left"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-width box within the area defined by the bottom margin and adjoining the bottom-left-corner margin box.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@bottom-left-corner"), version : Some(Borrowed("3")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A box filling the area defined by the intersection of the bottom and left margins of the page box.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@bottom-right"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-width box within the area defined by the bottom margin and adjoining the bottom-right corner margin box.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@bottom-right-corner"), version : Some(Borrowed("3")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A box filling the area defined by the intersection of the bottom and right margins of the page box.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@charset"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-syntax/#charset")), syntax :
                Some(Borrowed("@charset 'utf-8';")), }, desc :
                Some(Borrowed("Defines character set of the document.")), },
                AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@counter-style"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-counter-styles-3/#the-counter-style-rule")),
                syntax :
                Some(Borrowed("@counter-style <counter-style-name> { <declaration-list> }")),
                }, desc : Some(Borrowed("Defines a custom counter style.")), },
                AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@custom-media"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/mediaqueries-4/#at-ruledef-custom-media")),
                syntax :
                Some(Borrowed("@custom-media --narrow-viewport (max-width: 30em);")), },
                desc :
                Some(Borrowed("Defines a custom media feature that can then be used in a media feature.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@font-face"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#the-font-face-rule")),
                syntax : Some(Borrowed("@font-face { <font-description> }")), }, desc :
                Some(Borrowed("Allows for linking to fonts that are automatically activated when needed. This permits authors to work around the limitation of 'web-safe' fonts, allowing for consistent rendering independent of the fonts available in a given user's environment.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@font-feature-values"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF34")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-fonts-3/#at-font-feature-values-rule")),
                syntax : Some(Borrowed("@font-feature-values <font-family> { }")), },
                desc :
                Some(Borrowed("Defines named values for the indices used to select alternate glyphs for a given font family.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@import"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-syntax/#at-import")), syntax :
                Some(Borrowed("@import url('file.css') tv, print;")), }, desc :
                Some(Borrowed("Includes content of another file.")), }, AtDirectiveEntry
                { attributes : AtDirectiveAttributes { name : Borrowed("@keyframes"),
                version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C43,FF16,IE10,O30,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#keyframes")), syntax
                : Some(Borrowed("@keyframes animation-name")), }, desc :
                Some(Borrowed("Defines set of animation key frames.")), },
                AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@layer"), version : Some(Borrowed("5.0")), browsers :
                Some(Borrowed("E99,C99,FF97,O85,S15.4")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-cascade-5/#layering")),
                syntax : Some(Borrowed("@layer layer-name {rules}")), }, desc :
                Some(Borrowed("Declare a cascade layer and the order of precedence in case of multiple cascade layers.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@left-bottom"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-height box within the area defined by the left margin and adjacent to the top of the bottom-left-corner.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@left-middle"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-height box in the area defined by the left margin, centered on the page area, and between the left-top and left-bottom margin boxes.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@left-top"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-height box within the area defined by the left margin and adjacent to the bottom of the top-left-corner.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@media"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-mediaqueries")), syntax :
                Some(Borrowed("@media print { ... }")), }, desc :
                Some(Borrowed("Defines a stylesheet for a particular media type.")), },
                AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@-moz-document"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1.8")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/@document")), syntax
                : None, }, desc :
                Some(Borrowed("Gecko-specific at-rule that restricts the style rules contained within it based on the URL of the document.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@-moz-keyframes"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#keyframes")), syntax
                : Some(Borrowed("@-moz-keyframes animation-name")), }, desc :
                Some(Borrowed("Defines set of animation key frames.")), },
                AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@-ms-viewport"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-device-adapt/#the-viewport-rule")),
                syntax : Some(Borrowed("@$(name) { width: device-width; }")), }, desc :
                Some(Borrowed("Specifies the size, zoom factor, and orientation of the viewport.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@namespace"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1,IE9,O8,S1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-namespace/#declaration")),
                syntax : Some(Borrowed("@namespace [prefix] string|url;")), }, desc :
                Some(Borrowed("Declares a prefix and associates it with a namespace name.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@-o-keyframes"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#keyframes")), syntax
                : Some(Borrowed("@-o-keyframes animation-name { }")), }, desc :
                Some(Borrowed("Defines set of animation key frames.")), },
                AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@-o-viewport"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O11")), ref_ :
                Some(Borrowed("http://dev.w3.org/csswg/css-device-adapt/#the-viewport-rule")),
                syntax : Some(Borrowed("@$(name) { width: 320px; zoom: 0.5; }")), }, desc
                :
                Some(Borrowed("Specifies the size, zoom factor, and orientation of the viewport.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@page"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box-page-rule")),
                syntax : Some(Borrowed("@page :first { margin-top: 10cm }")), }, desc :
                Some(Borrowed("Directive defines various page parameters.")), },
                AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@property"), version : Some(Borrowed("0.1")), browsers :
                Some(Borrowed("C85,E85,O71")), ref_ :
                Some(Borrowed("https://drafts.css-houdini.org/css-properties-values-api/#at-property-rule")),
                syntax :
                Some(Borrowed("@property --property-name { syntax: '<color>'; inherits: false; initial-value: #c0ffee; }")),
                }, desc :
                Some(Borrowed("Describes the aspect of custom properties and variables.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@right-bottom"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-height box within the area defined by the right margin and adjacent to the top of the bottom-right-corner.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@right-middle"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-height box in the area defined by the right margin, centered on the page area, and between the right-top and right-bottom margin boxes.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@right-top"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-height box within the area defined by the right margin and adjacent to the bottom of the top-right-corner.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@scope"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-scoping-1/#at-ruledef-scope")),
                syntax : Some(Borrowed("@scope <selector> { <stylesheet> }")), }, desc :
                Some(Borrowed("Creates scoped style rules using CSS syntax.")), },
                AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@supports"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C28,FF22,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-conditional/#at-supports")),
                syntax :
                Some(Borrowed("@supports (display: flexbox) { div { display: flexbox; } }")),
                }, desc :
                Some(Borrowed("A conditional group rule whose condition tests whether the user agent supports CSS property:value pairs.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@top-center"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-width box within the area defined by the top margin, centered on the page area, and between the top-left and top-right margin boxes.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@top-left"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-width box within the area defined by the top margin and adjoining the top-left-corner margin box.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@top-left-corner"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A fixed-size box filling the area defined by the intersection of the top and left margins of the page box.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@top-right"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A variable-width box within the area defined by the top margin and adjoining the top-right-corner margin box.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@top-right-corner"), version : Some(Borrowed("3")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-box")), syntax :
                None, }, desc :
                Some(Borrowed("A box filling the area defined by the intersection of the top and right margins of the page box.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@viewport"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-device-adapt/#the-viewport-rule")),
                syntax : Some(Borrowed("@$(name) { width: device-width; }")), }, desc :
                Some(Borrowed("Specifies the size, zoom factor, and orientation of the viewport.")),
                }, AtDirectiveEntry { attributes : AtDirectiveAttributes { name :
                Borrowed("@-webkit-keyframes"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#keyframes")), syntax
                : Some(Borrowed("@-webkit-keyframes animation-name")), }, desc :
                Some(Borrowed("Defines set of animation key frames.")), },
            ],
        },
        pseudo_classes: PseudoClasses {
            entry: vec![
                PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":active"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#the-user-action-pseudo-classes-hover-act")),
                syntax : Some(Borrowed("a:active { color: red; }")), }, desc :
                Some(Borrowed("Applies while an element is being activated by the user. For example, between the times the user presses the mouse button and releases it.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":any-link"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#the-any-link-pseudo")),
                syntax : Some(Borrowed("a:any-link { text-decoration: none; }")), }, desc
                :
                Some(Borrowed("Represents an element that acts as the source anchor of a hyperlink. Applies to both visited and unvisited links.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":blank"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#the-blank-pseudo")),
                syntax : Some(Borrowed("p:blank { display: none; }")), }, desc :
                Some(Borrowed("The same as :empty, except that it additionally matches elements that only contain code points affected by whitespace processing.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":checked"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1,IE9,O9,S3.13")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#checked")), syntax :
                Some(Borrowed("input:checked { color: red; }")), }, desc :
                Some(Borrowed("Radio and checkbox elements can be toggled by the user. Some menu items are 'checked' when the user selects them. When such elements are toggled 'on' the :checked pseudo-class applies.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":corner-present"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Indicates whether or not a scrollbar corner is present.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":current"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#current-pseudo")), syntax
                : Some(Borrowed(":current { background-color: lightyellow; }")), }, desc
                :
                Some(Borrowed("Represents the element, or an ancestor of the element, that is currently being displayed.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":current()"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#current-pseudo")), syntax
                : Some(Borrowed(":current(div, p) { background-color: lightyellow; }")),
                }, desc :
                Some(Borrowed("Takes a list of compound selectors as its argument: it represents the :current element that matches the argument or, if that does not match, the innermost ancestor of the :current element that does.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":decrement"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Applies to buttons and track pieces. Indicates whether or not the button or track piece will decrement the view's position when used.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":default"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,FF3,O10,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#the-default-pseudo")),
                syntax : Some(Borrowed("input:default { color: red; }")), }, desc :
                Some(Borrowed("Applies to the one or more UI elements that are the default among a set of similar elements. Typically applies to context menu items, buttons, and select lists/menus.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":dir()"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#dir-pseudo")), syntax :
                Some(Borrowed("section:dir(ltr) { border-color: pink; }")), }, desc :
                Some(Borrowed("Represents an element based on its directionality as determined by the document language.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":disabled"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE9,O9,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#enableddisabled")),
                syntax : Some(Borrowed("input:disabled { background-color: silver; }")),
                }, desc :
                Some(Borrowed("Represents user interface elements that are in a disabled state; such elements have a corresponding enabled state.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":double-button"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Applies to buttons and track pieces. Applies when both buttons are displayed together at the same end of the scrollbar.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":empty"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE9,O9,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#empty-pseudo")),
                syntax : Some(Borrowed("div:empty { background-color: red; }")), }, desc
                : Some(Borrowed("Represents an element that has no children at all.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":enabled"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE9,O9,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#enableddisabled")),
                syntax : Some(Borrowed("input:enabled { background-color: green; }")), },
                desc :
                Some(Borrowed("Represents user interface elements that are in an enabled state; such elements have a corresponding disabled state.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":end"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Applies to buttons and track pieces. Indicates whether the object is placed after the thumb.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":first"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#left-right-first")),
                syntax : Some(Borrowed("@page :first { margin-left: 4cm; }")), }, desc :
                Some(Borrowed("When printing double-sided documents, the page boxes on left and right pages may be different. This can be expressed through CSS pseudo-classes defined in the  page context.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":first-child"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF3,IE7,O9.5,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#first-child-pseudo")),
                syntax : Some(Borrowed("li:first-child { font-size: 1.2em; }")), }, desc
                :
                Some(Borrowed("Same as :nth-child(1). Represents an element that is the first child of some other element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":first-of-type"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF3.5,IE9,O9.5,S3.2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#first-of-type-pseudo")),
                syntax : Some(Borrowed("dl dt:first-of-type { font-size: 200%; }")), },
                desc :
                Some(Borrowed("Same as :nth-of-type(1). Represents an element that is the first sibling of its type in the list of children of its parent element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":focus"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#the-user-action-pseudo-classes-hover-act")),
                syntax : Some(Borrowed("a:focus { color: yellow; }")), }, desc :
                Some(Borrowed("Applies while an element has the focus (accepts keyboard or mouse events, or other forms of input).")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":fullscreen"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E")), ref_ :
                Some(Borrowed("https://fullscreen.spec.whatwg.org/#:fullscreen-pseudo-class")),
                syntax : Some(Borrowed("iframe:fullscreen { border: none; }")), }, desc :
                Some(Borrowed("Matches any element that has its fullscreen flag set.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":future"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("C,O16,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#the-future-pseudo")),
                syntax : Some(Borrowed(":future { color: yellow; }")), }, desc :
                Some(Borrowed("Represents any element that is defined to occur entirely after a :current element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":horizontal"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Applies to any scrollbar pieces that have a horizontal orientation.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":host"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C35,O22")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-scoping-1/#selectordef-host0")),
                syntax : Some(Borrowed(":host { display: block; }")), }, desc :
                Some(Borrowed("When evaluated in the context of a shadow tree, matches the shadow tree's host element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":host()"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C35,O22")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-scoping-1/#selectordef-host")),
                syntax : Some(Borrowed(":host(.myclass) { color: blue; }")), }, desc :
                Some(Borrowed("When evaluated in the context of a shadow tree, it matches the shadow tree's host element if the host element, in its normal context, matches the selector argument.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":host-context()"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C35,O22")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-scoping-1/#selectordef-host-context")),
                syntax : Some(Borrowed(":host-context(.myclass) { color: blue; }")), },
                desc :
                Some(Borrowed("Tests whether there is an ancestor, outside the shadow tree, which matches a particular selector.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":hover"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#the-user-action-pseudo-classes-hover-act")),
                syntax : Some(Borrowed("a:hover { text-decoration: none; }")), }, desc :
                Some(Borrowed("Applies while the user designates an element with a pointing device, but does not necessarily activate it. For example, a visual user agent could apply this pseudo-class when the cursor (mouse pointer) hovers over a box generated by the element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":increment"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Applies to buttons and track pieces. Indicates whether or not the button or track piece will increment the view's position when used.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":indeterminate"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E,C,FF3.6,IE9,O10.6,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#indeterminate")), syntax
                : Some(Borrowed("input:indeterminate { margin: auto 2px; }")), }, desc :
                Some(Borrowed("Applies to UI elements whose value is in an indeterminate state.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":in-range"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E13,C,FF10,O9.6,S5.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#range-pseudos")), syntax
                : Some(Borrowed("input:in-range { color: green; }")), }, desc :
                Some(Borrowed("Used in conjunction with the min and max attributes, whether on a range input, a number field, or any other types that accept those attributes.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":invalid"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E,C,FF4,IE10,O10,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#validity-pseudos")),
                syntax : Some(Borrowed("input:invalid { border-color: red; }")), }, desc
                :
                Some(Borrowed("An element is :valid or :invalid when it is, respectively, valid or invalid with respect to data validity semantics defined by a different specification.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":lang()"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1,IE8,O8,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#lang-pseudo")),
                syntax : Some(Borrowed("html:lang(en) { color: blue; }")), }, desc :
                Some(Borrowed("Represents an element that is in language specified.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":last-child"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1,IE9,O9.5,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#last-child-pseudo")),
                syntax : Some(Borrowed("li:last-child { font-size: 1.2em; }")), }, desc :
                Some(Borrowed("Same as :nth-last-child(1). Represents an element that is the last child of some other element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":last-of-type"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF3.5,IE9,O9.5,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#last-of-type-pseudo")),
                syntax : Some(Borrowed("dl dt:last-of-type { font-size: 200%; }")), },
                desc :
                Some(Borrowed("Same as :nth-last-of-type(1). Represents an element that is the last sibling of its type in the list of children of its parent element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":left"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#left-right-first")),
                syntax : Some(Borrowed("@page :left { margin-left: 4cm; }")), }, desc :
                Some(Borrowed("When printing double-sided documents, the page boxes on left and right pages may be different. This can be expressed through CSS pseudo-classes defined in the  page context.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":link"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#the-link-pseudo-classes-link-and-visited")),
                syntax : Some(Borrowed("a:link { text-decoration: none; }")), }, desc :
                Some(Borrowed("Applies to links that have not yet been visited.")), },
                PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":local-link"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#local-pseudo")), syntax :
                Some(Borrowed("a:local-link { text-decoration: none; }")), }, desc :
                Some(Borrowed("Allows authors to style hyperlinks based on the users current location within a site and to differentiate site-internal versus site-external links.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":local-link()"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#local-pseudo")), syntax :
                Some(Borrowed("a:local-link(0) { text-decoration: none; }")), }, desc :
                Some(Borrowed("Accepts a non-negative integer as its sole argument, which, if the document's URL belongs to a hierarchical scheme, indicates the number of path levels to match. Use :local-link(0) to match links that target the same domain.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":matches()"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#matches")), syntax :
                Some(Borrowed("div:matches(:hover) { border-color: pink; }")), }, desc :
                Some(Borrowed("Takes a selector list as its argument. It represents an element that is represented by its argument.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-any()"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF4")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Represents an element that is represented by the selector list passed as its argument. Standardized as :matches().")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-any-link"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Represents an element that acts as the source anchor of a hyperlink. Applies to both visited and unvisited links.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-broken"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF3")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Matches elements representing broken images.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-drag-over"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Matches elements when a drag-over event applies to it.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-first-node"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Represents an element that is the first child node of some other element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-focusring"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF4")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Matches an element that has focus and focus ring drawing is enabled in the browser.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-full-screen"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF9")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Matches any element that has its fullscreen flag set. Standardized as :fullscreen.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-last-node"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Represents an element that is the last child node of some other element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-loading"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF3")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Matches elements, such as images, that haven't started loading yet.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-only-whitespace"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF1.5")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("The same as :empty, except that it additionally matches elements that only contain code points affected by whitespace processing. Standardized as :blank.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-placeholder"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF4")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Deprecated. Represents placeholder text in an input field. Use ::-moz-placeholder for Firefox 19+.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-submit-invalid"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF4")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Represents any submit button when the contents of the associated form are not valid.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-suppressed"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF3")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Matches elements representing images that have been blocked from loading.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-ui-invalid"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF4")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Represents any validated form element whose value isn't valid ")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-ui-valid"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF4")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Represents any validated form element whose value is valid ")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-user-disabled"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF3")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Matches elements representing images that have been disabled due to the user's preferences.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-moz-window-inactive"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF4")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Non-standard. Matches elements in an inactive window.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-ms-fullscreen"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE11")), ref_ :
                Some(Borrowed("https://fullscreen.spec.whatwg.org/#:fullscreen-pseudo-class")),
                syntax : Some(Borrowed("iframe:-ms-fullscreen { border: none; }")), },
                desc :
                Some(Borrowed("Matches any element that has its fullscreen flag set.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-ms-input-placeholder"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh767367.aspx")),
                syntax : Some(Borrowed("input:-ms-input-placeholder { color: red; }")),
                }, desc :
                Some(Borrowed("Represents placeholder text in an input field. Note: for Edge use the pseudo-element ::-ms-input-placeholder. Standardized as ::placeholder.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-ms-keyboard-active"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/dn336891(v=vs.85).aspx")),
                syntax :
                Some(Borrowed("input:-ms-keyboard-active { background: red; }")), }, desc
                :
                Some(Borrowed("Windows Store apps only. Applies one or more styles to an element when it has focus and the user presses the space bar.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-ms-lang()"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#lang-pseudo")), syntax :
                Some(Borrowed("html:-ms-lang(en, fr, de) { color: blue; }")), }, desc :
                Some(Borrowed("Represents an element that is in the language specified. Accepts a comma separated list of language tokens.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":no-button"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Applies to track pieces. Applies when there is no button at that end of the track.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":not()"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1,IE9,O9.5,S2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#negation")), syntax :
                Some(Borrowed("div:not(:empty) { border-color: pink; }")), }, desc :
                Some(Borrowed("The negation pseudo-class, :not(X), is a functional notation taking a simple selector (excluding the negation pseudo-class itself) as an argument. It represents an element that is not represented by its argument.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":nth-child()"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF3.5,IE9,O9.5,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#nth-child-pseudo")),
                syntax : Some(Borrowed("tr:nth-child(2n+1) { border-color: pink; }")), },
                desc :
                Some(Borrowed("Represents an element that has an+b-1 siblings before it in the document tree, for any positive integer or zero value of n, and has a parent element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":nth-column()"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#nth-column-pseudo")),
                syntax : Some(Borrowed(":nth-column(2n+1) > span { color: yellow; }")),
                }, desc :
                Some(Borrowed("Represents a cell element belonging to a column that has An+B-1 columns before it, for any positive integer or zero value of n.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":nth-last-child()"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,C,FF3.5,IE9,O9.5,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#nth-last-child-pseudo")),
                syntax :
                Some(Borrowed("tr:nth-last-child(-n+2) { border-color: pink; }")), },
                desc :
                Some(Borrowed("Represents an element that has an+b-1 siblings after it in the document tree, for any positive integer or zero value of n, and has a parent element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":nth-last-column()"), version : Some(Borrowed("4.0")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#nth-column-pseudo")),
                syntax :
                Some(Borrowed(":nth-last-column(2n+1) > span { color: yellow; }")), },
                desc :
                Some(Borrowed("Represents a cell element belonging to a column that has An+B-1 columns after it, for any positive integer or zero value of n.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":nth-last-match()"), version : Some(Borrowed("4.0")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#nth-last-match-pseudo")),
                syntax : Some(Borrowed("div:nth-last-match(p) { border-color: pink; }")),
                }, desc :
                Some(Borrowed("Represents an element that has An+B-1 siblings that match the given selector list after it in the document tree.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":nth-last-of-type()"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C,FF3.5,IE9,O9.5,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#nth-of-type-pseudo")),
                syntax :
                Some(Borrowed("tr:nth-last-of-type(n+2) { border-color: pink; }")), },
                desc :
                Some(Borrowed("Represents an element that has an+b-1 siblings with the same expanded element name after it in the document tree, for any zero or positive integer value of n, and has a parent element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":nth-match()"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#the-nth-match-pseudo")),
                syntax : Some(Borrowed("div:nth-match(p) { border-color: pink; }")), },
                desc :
                Some(Borrowed("Represents an element that has An+B-1 siblings that match the given selector list before it in the document tree.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":nth-of-type()"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF3.5,IE9,O9.5,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#nth-of-type-pseudo")),
                syntax : Some(Borrowed("tr:nth-of-type(2n) { border-color: pink; }")), },
                desc :
                Some(Borrowed("Represents an element that has an+b-1 siblings with the same expanded element name before it in the document tree, for any zero or positive integer value of n, and has a parent element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":only-child"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE9,O9.5,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#only-child-pseudo")),
                syntax : Some(Borrowed("p:only-child { color: #f00; }")), }, desc :
                Some(Borrowed("Represents an element that has a parent element and whose parent element has no other element children. Same as :first-child:last-child or :nth-child(1):nth-last-child(1), but with a lower specificity.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":only-of-type"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF3.5,IE9,O9.5,S3.2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#only-of-type-pseudo")),
                syntax : Some(Borrowed("p:only-of-type { color: #f00; }")), }, desc :
                Some(Borrowed("Matches every element that is the only child of its type, of its parent. Same as :first-of-type:last-of-type or :nth-of-type(1):nth-last-of-type(1), but with a lower specificity.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":optional"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E,C,FF4,IE10,O10,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#opt-pseudos")), syntax :
                Some(Borrowed("input:optional { color: yellow; }")), }, desc :
                Some(Borrowed("A form element is :required or :optional if a value for it is, respectively, required or optional before the form it belongs to is submitted. Elements that are not form elements are neither required nor optional.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":out-of-range"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E13,C,FF10,O9.6,S5.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#range-pseudos")), syntax
                : Some(Borrowed("input:out-of-range { color: red; }")), }, desc :
                Some(Borrowed("Used in conjunction with the min and max attributes, whether on a range input, a number field, or any other types that accept those attributes.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":past"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("C,O16,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#the-past-pseudo")),
                syntax : Some(Borrowed(":past { color: green; }")), }, desc :
                Some(Borrowed("Represents any element that is defined to occur entirely prior to a :current element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":placeholder-shown"), version : Some(Borrowed("4.0")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#placeholder")), syntax :
                Some(Borrowed("input:placeholder-shown { color: grey; }")), }, desc :
                Some(Borrowed("Matches an input element that is showing placeholder text.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":read-only"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E13,C,FF10,O9,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#rw-pseudos")), syntax :
                Some(Borrowed("input:read-only { margin: auto; }")), }, desc :
                Some(Borrowed("An element whose contents are not user-alterable is :read-only. However, elements whose contents are user-alterable (such as text input fields) are considered to be in a :read-write state. In typical documents, most elements are :read-only.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":read-write"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E13,C,FF10,O9,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#rw-pseudos")), syntax :
                Some(Borrowed("input:read-write { margin: auto 2px; }")), }, desc :
                Some(Borrowed("An element whose contents are not user-alterable is :read-only. However, elements whose contents are user-alterable (such as text input fields) are considered to be in a :read-write state. In typical documents, most elements are :read-only.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":required"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E,C,FF4,IE10,O10,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#opt-pseudos")), syntax :
                Some(Borrowed("input:optional { color: yellow; }")), }, desc :
                Some(Borrowed("A form element is :required or :optional if a value for it is, respectively, required or optional before the form it belongs to is submitted. Elements that are not form elements are neither required nor optional.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":recto"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#valdef-logical-page-selector-recto")),
                syntax : Some(Borrowed("@page :recto { margin-inline-start: 4cm; }")), },
                desc :
                Some(Borrowed("Equivalent to ':right' in left-to-right page progressions and ':left' in right-to-left page progressions.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":right"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#left-right-first")),
                syntax : Some(Borrowed("@page :right { margin-left: 4cm; }")), }, desc :
                Some(Borrowed("When printing double-sided documents, the page boxes on left and right pages may be different. This can be expressed through CSS pseudo-classes defined in the  page context.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":root"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1,IE9,O9.5,S1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#root-pseudo")),
                syntax : Some(Borrowed(":root { padding: auto 3em; }")), }, desc :
                Some(Borrowed("Represents an element that is the root of the document. In HTML 4, this is always the HTML element.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":scope"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("FF32,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#scope-pseudo")), syntax :
                Some(Borrowed(":scope { border-color: pink; }")), }, desc :
                Some(Borrowed("Represents any element that is in the contextual reference element set.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":single-button"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Applies to buttons and track pieces. Applies when both buttons are displayed separately at either end of the scrollbar.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":start"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Applies to buttons and track pieces. Indicates whether the object is placed before the thumb.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":target"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1,IE9,O9.5,S1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#root-pseudo")),
                syntax : Some(Borrowed("h2:target { background-color: yellow; }")), },
                desc :
                Some(Borrowed("Some URIs refer to a location within a resource. This kind of URI ends with a 'number sign' (#) followed by an anchor identifier (called the fragment identifier).")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":user-invalid"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#user-pseudos")), syntax :
                Some(Borrowed("input:user-invalid { outline: 2px solid red; }")), }, desc
                :
                Some(Borrowed("Represents an input element with incorrect input, but only after the user has significantly interacted with it.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":valid"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E,C,FF4,IE10,O10,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/selectors4/#validity-pseudos")),
                syntax : Some(Borrowed("input:valid { border-color: green; }")), }, desc
                :
                Some(Borrowed("An element is :valid or :invalid when it is, respectively, valid or invalid with respect to data validity semantics defined by a different specification.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":verso"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#valdef-logical-page-selector-verso")),
                syntax : Some(Borrowed("@page :verso { margin-inline-end: 4cm; }")), },
                desc :
                Some(Borrowed("Equivalent to ':left' in left-to-right page progressions and ':right' in right-to-left page progressions.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":vertical"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Applies to any scrollbar pieces that have a vertical orientation.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":visited"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-selectors/#the-link-pseudo-classes-link-and-visited")),
                syntax : Some(Borrowed("a:visited { color: purple; }")), }, desc :
                Some(Borrowed("Applies once the link has been visited by the user.")), },
                PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-webkit-any()"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S5")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Represents an element that is represented by the selector list passed as its argument. Standardized as :matches().")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":-webkit-full-screen"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S6")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Matches any element that has its fullscreen flag set. Standardized as :fullscreen.")),
                }, PseudoClassEntry { attributes : PseudoClassAttributes { name :
                Borrowed(":window-inactive"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("https://webkit.org/blog/363/styling-scrollbars/")), syntax
                : None, }, desc :
                Some(Borrowed("Non-standard. Applies to all scrollbar pieces. Indicates whether or not the window containing the scrollbar is currently active.")),
                },
            ],
        },
        pseudo_elements: PseudoElements {
            entry: vec![
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::after"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE9,O9,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-pseudo-4/#selectordef-after")),
                syntax : Some(Borrowed("div::after { content: 'abc'; }")), }, desc :
                Some(Borrowed("Represents a styleable child pseudo-element immediately after the originating element's actual content.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::backdrop"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E")), ref_ :
                Some(Borrowed("https://fullscreen.spec.whatwg.org/#::backdrop-pseudo-element")),
                syntax : Some(Borrowed("*|*:fullscreen::backdrop { position: fixed; }")),
                }, desc :
                Some(Borrowed("Used to create a backdrop that hides the underlying document for an element in a top layer (such as an element that is displayed fullscreen).")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::before"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE9,O9,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-pseudo-4/#selectordef-before")),
                syntax : Some(Borrowed("div::before { content: 'abc'; }")), }, desc :
                Some(Borrowed("Represents a styleable child pseudo-element immediately before the originating element's actual content.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::content"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C35,O22")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-scoping-1/#selectordef-content")),
                syntax : Some(Borrowed("::content > span { color: yellow; }")), }, desc :
                Some(Borrowed("Deprecated. Matches the distribution list itself, on elements that have one. Use ::slotted for forward compatibility.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::cue"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,O16,S6")), ref_ :
                Some(Borrowed("https://w3c.github.io/webvtt/#the-cue-pseudo-element")),
                syntax : Some(Borrowed("::cue { color: red; }")), }, desc :
                Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::cue()"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O16,S6")), ref_ :
                Some(Borrowed("https://w3c.github.io/webvtt/#selectordef-cue-selector")),
                syntax : Some(Borrowed("::cue(v(voice=woman)) { color: red; }")), }, desc
                : Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::cue-region"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O16,S6")), ref_ :
                Some(Borrowed("https://w3c.github.io/webvtt/#the-cue-region-pseudo-element")),
                syntax : Some(Borrowed("::cue-region { border: none; }")), }, desc :
                Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::cue-region()"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O16,S6")), ref_ :
                Some(Borrowed("https://w3c.github.io/webvtt/#the-cue-region-pseudo-element")),
                syntax : Some(Borrowed("::cue-region(v(voice=woman)) { color: red; }")),
                }, desc : Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::first-letter"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("E,C,FF1.5,IE9,O7,S1")),
                ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-pseudo-4/#first-letter-pseudo")),
                syntax : Some(Borrowed("p::first-letter { font-size: 200%; }")), }, desc
                :
                Some(Borrowed("Represents the first letter of an element, if it is not preceded by any other content (such as images or inline tables) on its line.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::first-line"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE9,O7,S1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-pseudo-4/#selectordef-first-line")),
                syntax : Some(Borrowed("p::first-line { color: green; }")), }, desc :
                Some(Borrowed("Describes the contents of the first formatted line of its originating element.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::grammar-error"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-pseudo-4/#selectordef-grammar-error")),
                syntax :
                Some(Borrowed("::grammar-error { text-decoration: dotted underline green; }")),
                }, desc :
                Some(Borrowed("Represents a portion of text that has been flagged by the user agent as misspelled.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::marker"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-lists/#selectordef-marker")),
                syntax : Some(Borrowed("li::marker { content: counter(counter); }")), },
                desc :
                Some(Borrowed("Generated by list items to represent the item's marker.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-moz-focus-inner"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF4")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::-moz-focus-outer"), version
                : Some(Borrowed("3.0")), browsers : Some(Borrowed("FF4")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-moz-list-bullet"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF1")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Used to style the bullet of a list element. Similar to the standardized ::marker.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-moz-list-number"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF1")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Used to style the numbers of a list element. Similar to the standardized ::marker.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-moz-placeholder"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF19")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Represents placeholder text in an input field")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-moz-progress-bar"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF9")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Represents the bar portion of a progress bar.")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-moz-selection"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("Represents the portion of a document that has been highlighted by the user.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-backdrop"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE11")), ref_ :
                Some(Borrowed("https://fullscreen.spec.whatwg.org/#::backdrop-pseudo-element")),
                syntax :
                Some(Borrowed("*|*:-ms-fullscreen::-ms-backdrop { position: fixed; }")),
                }, desc :
                Some(Borrowed("Used to create a backdrop that hides the underlying document for an element in a top layer (such as an element that is displayed fullscreen).")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-browse"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh779844.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the browse button of an input type=file control.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-check"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465739.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the check of a checkbox or radio button input control.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-clear"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465740.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the clear button of a text input control")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-expand"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465742.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the drop-down button of a select control.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-fill"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465757.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the bar portion of a progress bar.")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-fill-lower"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465745.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the portion of the slider track from its smallest value up to the value currently selected by the thumb. In a left-to-right layout, this is the portion of the slider track to the left of the thumb.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-fill-upper"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465748.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the portion of the slider track from the value currently selected by the thumb up to the slider's largest value. In a left-to-right layout, this is the portion of the slider track to the right of the thumb.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-reveal"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465773.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the password reveal button of an input type=password control.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-thumb"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465780.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the portion of range input control (also known as a slider control) that the user drags.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-ticks-after"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465789.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the tick marks of a slider that begin just after the thumb and continue up to the slider's largest value. In a left-to-right layout, these are the ticks to the right of the thumb.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-ticks-before"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465796.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the tick marks of a slider that represent its smallest values up to the value currently selected by the thumb. In a left-to-right layout, these are the ticks to the left of the thumb.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-tooltip"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465805.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the tooltip of a slider (input type=range).")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-track"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465813.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the track of a slider.")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-ms-value"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh465820.aspx")),
                syntax : None, }, desc :
                Some(Borrowed("Represents the content of a text or password input control, or a select control.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::placeholder"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-pseudo-4/#selectordef-placeholder")),
                syntax : Some(Borrowed("input::placeholder { color: grey; }")), }, desc :
                Some(Borrowed("Represents placeholder text in an input field")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::selection"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("E,C,IE9,O9.5,S1.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-pseudo-4/#selectordef-selection")),
                syntax : Some(Borrowed("p::selection { color: red; }")), }, desc :
                Some(Borrowed("Represents the portion of a document that has been highlighted by the user.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::region"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-scoping-1/#the-region-pseudo-element")),
                syntax : Some(Borrowed("::region p { margin-right: 5em; }")), }, desc :
                Some(Borrowed("Represents a relationship between a selector that matches a CSS Region, and a relative selector that matches some named flow content.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::shadow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C35,O22")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-scoping-1/#shadow-pseudoelement")),
                syntax : Some(Borrowed("x-foo::shadow > span { color: red; }")), }, desc
                :
                Some(Borrowed("Matches the shadow root if an element has a shadow tree.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::slotted"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-scoping-1/#selectordef-slotted")),
                syntax : Some(Borrowed("::slotted > span { color: yellow; }")), }, desc :
                Some(Borrowed("Matches the distribution list itself, on elements that have one.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::spelling-error"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-pseudo-4/#selectordef-spelling-error")),
                syntax :
                Some(Borrowed("::spelling-error { text-decoration: dotted underline red; }")),
                }, desc :
                Some(Borrowed("Represents a portion of text that has been flagged by the user agent as misspelled.")),
                }, PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-file-upload-button"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O,S6")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-inner-spin-button"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O,S6")), ref_ : None, syntax : None, }, desc
                : Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::-webkit-input-placeholder"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S4")), ref_
                : None, syntax : None, }, desc : Some(Borrowed("")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-keygen-select"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O,S6")), ref_ : None, syntax : None, }, desc
                : Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::-webkit-meter-bar"), version
                : Some(Borrowed("3.0")), browsers : Some(Borrowed("E13,C,O15,S6")), ref_
                : None, syntax : None, }, desc : Some(Borrowed("")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-meter-even-less-good-value"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("E13,C,O15,S6")), ref_ :
                None, syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry
                { attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-meter-optimum-value"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("E13,C,O15,S6")), ref_ :
                None, syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry
                { attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-meter-suboptimum-value"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("E13,C,O15,S6")), ref_ :
                None, syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry
                { attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-outer-spin-button"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O,S6")), ref_ : None, syntax : None, }, desc
                : Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::-webkit-progress-bar"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S3")), ref_
                : None, syntax : None, }, desc : Some(Borrowed("")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-progress-inner-element"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S3")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-progress-value"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::-webkit-resizer"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S5")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-scrollbar"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::-webkit-scrollbar-button"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S5")), ref_
                : None, syntax : None, }, desc : Some(Borrowed("")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-scrollbar-corner"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name : Borrowed("::-webkit-scrollbar-thumb"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S5")), ref_
                : None, syntax : None, }, desc : Some(Borrowed("")), },
                PseudoElementEntry { attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-scrollbar-track"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name :
                Borrowed("::-webkit-scrollbar-track-piece"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S5")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-search-cancel-button"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S4")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-search-decoration"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S4")), ref_ : None, syntax : None, }, desc :
                Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name :
                Borrowed("::-webkit-search-results-button"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S4")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-search-results-decoration"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S4")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-slider-runnable-track"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O,S6")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-slider-thumb"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O,S6")), ref_ : None, syntax : None, }, desc
                : Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name :
                Borrowed("::-webkit-textfield-decoration-container"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O,S6")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-validation-bubble"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O,S6")), ref_ : None, syntax : None, }, desc
                : Some(Borrowed("")), }, PseudoElementEntry { attributes :
                PseudoElementAttributes { name :
                Borrowed("::-webkit-validation-bubble-arrow"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O,S6")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-validation-bubble-arrow-clipper"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O,S6")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-validation-bubble-heading"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O,S6")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-validation-bubble-message"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O,S6")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), }, PseudoElementEntry {
                attributes : PseudoElementAttributes { name :
                Borrowed("::-webkit-validation-bubble-text-block"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O,S6")), ref_ : None,
                syntax : None, }, desc : Some(Borrowed("")), },
            ],
        },
        properties: Properties {
            entry: vec![
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("additive-symbols"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-counter-styles-3/#descdef-counter-style-additive-symbols")),
                syntax : Some(Borrowed("@counter-style { additive-symbols: 1 I; }")),
                restriction : Some(Borrowed("integer, string, image, identifier")), },
                desc :
                Some(Borrowed("@counter-style descriptor. Specifies the symbols used by the marker-construction algorithm specified by the system descriptor. Needs to be specified if the counter system is 'additive'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("align-content"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF22,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#align-content")),
                syntax : Some(Borrowed("p { $(name): flex-start; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Aligns a flex container's lines within the flex container when there is extra space in the cross-axis, similar to how 'justify-content' aligns individual items within the main-axis.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("align-items"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF22,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#align-items")), syntax
                : Some(Borrowed("p { $(name): flex-start; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Aligns flex items along the cross axis of the current line of the flex container.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("justify-items"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF45")), ref_ :
                Some(Borrowed("https://www.w3.org/TR/css-grid-1/#row-align")), syntax :
                None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines the default justify-self for all items of the box, giving them the default way of justifying each box along the appropriate axis")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("justify-self"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF45")), ref_ :
                Some(Borrowed("https://www.w3.org/TR/css-grid-1/#row-align")), syntax :
                None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines the way of justifying a box inside its container along the appropriate axis.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("alignment-baseline"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-inline/#propdef-alignment-baseline")),
                syntax : Some(Borrowed("img { $(name): middle; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies what point of an inline-level box is aligned to what point in the parent.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("align-self"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF22,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#align-items")), syntax
                : Some(Borrowed("p { $(name): flex-start; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Allows the default alignment along the cross axis to be overridden for individual flex items.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("all"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C37,FF27,O24")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-cascade-3/#all-shorthand")),
                syntax : Some(Borrowed("* { $(name): unset; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Shorthand that resets all properties except 'direction' and 'unicode-bidi'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("alt"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("S9")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-content-3/#propdef-alt")),
                syntax : Some(Borrowed("label::before { $(name): 'alt text'; }")),
                restriction : Some(Borrowed("string, enum")), }, desc :
                Some(Borrowed("Provides alternative text for assistive technology to replace the generated content of a ::before or ::after element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("animation"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C43,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation")), syntax
                : Some(Borrowed("div { $(name): movearound 4s ease 3 normal; }")),
                restriction :
                Some(Borrowed("time, timing-function, enum, identifier, number")), },
                desc :
                Some(Borrowed("Shorthand property combines six of the animation properties into a single property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("animation-composition"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-animations-2/#propdef-animation-composition")),
                syntax : Some(Borrowed("div { $(name): add; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines the composite operation used when multiple animations affect the same property simultaneously.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("animation-delay"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C43,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-delay")),
                syntax : Some(Borrowed("div { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines when the animation will start.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("animation-direction"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C43,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-direction")),
                syntax : Some(Borrowed("div { $(name): normal; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether or not the animation should play in reverse on alternate cycles.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("animation-duration"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,C43,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-duration")),
                syntax : Some(Borrowed("div { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines the length of time that an animation takes to complete one cycle.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("animation-fill-mode"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C43,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-fill-mode-property")),
                syntax : Some(Borrowed("div { $(name): forwards; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines what values are applied by the animation outside the time it is executing.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("animation-iteration-count"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C43,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-iteration-count")),
                syntax : Some(Borrowed("div { $(name): 3; }")), restriction :
                Some(Borrowed("number, enum")), }, desc :
                Some(Borrowed("Defines the number of times an animation cycle is played. The default value is one, meaning the animation will play from beginning to end once.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("animation-name"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C43,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#the-animation-name-property-")),
                syntax : Some(Borrowed("div { $(name): movearound; }")), restriction :
                Some(Borrowed("identifier, enum")), }, desc :
                Some(Borrowed("Defines a list of animations that apply. Each name is used to select the keyframe at-rule that provides the property values for the animation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("animation-play-state"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C43,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-play-state")),
                syntax : Some(Borrowed("div { $(name): running; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether the animation is running or paused.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("animation-timing-function"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C43,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-timing-function")),
                syntax : Some(Borrowed("div { $(name): ease; }")), restriction :
                Some(Borrowed("timing-function")), }, desc :
                Some(Borrowed("Describes how the animation will progress over one cycle of its duration.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("backdrop-filter"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.fxtf.org/filters-2/#propdef-backdrop-filter")),
                syntax : Some(Borrowed("div { $(name): blur(2px); }")), restriction :
                Some(Borrowed("enum, url")), }, desc :
                Some(Borrowed("Applies a filter effect where the first filter in the list takes the element's background image as the input image.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("backface-visibility"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C36,FF16,IE10,O23")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transforms/#backface-visibility-property")),
                syntax : Some(Borrowed("div { $(name): hidden; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Determines whether or not the 'back' side of a transformed element is visible when facing the viewer. With an identity transform, the front side of an element faces the viewer.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#background")),
                syntax :
                Some(Borrowed("section { $(name): url(image.png) no-repeat #999; }")),
                restriction :
                Some(Borrowed("enum, image, color, position, length, repeat, percentage, box")),
                }, desc :
                Some(Borrowed("Shorthand property for setting most background properties at the same place in the style sheet.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-attachment"), version : Some(Borrowed("1.0")),
                browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-attachment")),
                syntax : Some(Borrowed(".box { $(name): fixed; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether the background images are fixed with regard to the viewport ('fixed') or scroll along with the element ('scroll') or its contents ('local').")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-blend-mode"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C35,FF30,O22,S7.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/compositing-1/#propdef-background-blend-mode")),
                syntax : Some(Borrowed("div { $(name): saturation; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines the blending mode of each background layer.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-clip"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF4,IE9,O10.5,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-clip")),
                syntax : Some(Borrowed("header { $(name): border-box; }")), restriction :
                Some(Borrowed("box")), }, desc :
                Some(Borrowed("Determines the background painting area.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-color"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-color")),
                syntax : Some(Borrowed("body { $(name): white; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets the background color of an element.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-image"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-image")),
                syntax : Some(Borrowed("article { $(name): url(image.png); }")),
                restriction : Some(Borrowed("image, enum")), }, desc :
                Some(Borrowed("Sets the background image(s) of an element.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-image-transform"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#propdef-background-image-transform")),
                syntax : Some(Borrowed("div { $(name): logical; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether background images are transformed to match to the value of 'writing-mode' property, and whether 'background-size' widths and heights are logical or physical.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-origin"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,C,FF4,IE9,O10.5,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-origin")),
                syntax : Some(Borrowed("header { $(name): border-box; }")), restriction :
                Some(Borrowed("box")), }, desc :
                Some(Borrowed("For elements rendered as a single box, specifies the background positioning area. For elements rendered as multiple boxes (e.g., inline boxes on several lines, boxes on several pages) specifies which boxes 'box-decoration-break' operates on to determine the background positioning area(s).")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-position"), version : Some(Borrowed("1.0")),
                browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-position")),
                syntax : Some(Borrowed("div { $(name): left center}")), restriction :
                Some(Borrowed("position, length, percentage")), }, desc :
                Some(Borrowed("Specifies the initial position of the background image(s) (after any resizing) within their corresponding background positioning area.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-position-x"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("E,IE6")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-backgrounds-4/#propdef-background-position-x")),
                syntax : Some(Borrowed("body { $(name): center; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("If background images have been specified, this property specifies their initial position (after any resizing) within their corresponding background positioning area.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-position-y"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("E,IE6")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-backgrounds-4/#propdef-background-position-y")),
                syntax : Some(Borrowed("body { $(name): center; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("If background images have been specified, this property specifies their initial position (after any resizing) within their corresponding background positioning area.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-repeat"), version : Some(Borrowed("1.0")), browsers
                : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-repeat")),
                syntax : Some(Borrowed("article { $(name): no-repeat; }")), restriction :
                Some(Borrowed("repeat")), }, desc :
                Some(Borrowed("Specifies how background images are tiled after they have been sized and positioned.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("background-size"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF4,IE9,O10,S4.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-size")),
                syntax : Some(Borrowed("header { $(name): 20px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Specifies the size of the background images.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("baseline-shift"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-inline/#baseline-shift-property")),
                syntax : Some(Borrowed(".shift-up { $(name): super; }")), restriction :
                Some(Borrowed("length, percentage, enum")), }, desc :
                Some(Borrowed("Specifies by how much the box is shifted up from its alignment point.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("behavior"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE6")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/ie/gg192966.aspx")),
                syntax :
                Some(Borrowed("div { $(name): url(http://example.com/png_fix.htc); }")),
                restriction : Some(Borrowed("url")), }, desc :
                Some(Borrowed("IE only. Used to extend behaviors of the browser.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("block-size"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#propdef-block-size")),
                syntax : Some(Borrowed("header { $(name): 200px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Size of an element in the direction opposite that of the direction specified by 'writing-mode'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#borders")), syntax :
                Some(Borrowed("header { $(name): 5px solid red;}")), restriction :
                Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Shorthand property for setting border width, style, and color.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-block-end"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("header { $(name): 5px solid red;}")), restriction
                : Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Logical 'border-bottom'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-block-start"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("header { $(name): 5px solid red;}")), restriction
                : Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Logical 'border-top'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-block-end-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): red; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Logical 'border-bottom-color'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-block-start-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): red; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Logical 'border-top-color'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-block-end-style"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Logical 'border-bottom-style'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-block-start-style"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Logical 'border-top-style'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-block-end-width"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Logical 'border-bottom-width'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-block-start-width"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Logical 'border-top-width'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-bottom"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#borders")), syntax :
                Some(Borrowed("header { $(name): 5px solid red;}")), restriction :
                Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Shorthand property for setting border width, style and color.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-bottom-color"), version : Some(Borrowed("1.0")),
                browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-color")),
                syntax : Some(Borrowed("td { $(name): blue; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets the color of the bottom border.")), }, PropertyEntry
                { attributes : PropertyAttributes { name :
                Borrowed("border-bottom-left-radius"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C,FF4,IE9,O10.5,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-radius")),
                syntax : Some(Borrowed("td { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Defines the radii of the bottom left outer border edge.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-bottom-right-radius"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C,FF4,IE9,O10.5,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-radius")),
                syntax : Some(Borrowed("td { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Defines the radii of the bottom right outer border edge.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-bottom-style"), version : Some(Borrowed("1.0")),
                browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-style")),
                syntax : Some(Borrowed("td { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Sets the style of the bottom border.")), }, PropertyEntry
                { attributes : PropertyAttributes { name :
                Borrowed("border-bottom-width"), version : Some(Borrowed("1.0")),
                browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-width")),
                syntax : Some(Borrowed("td { $(name): 2px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Sets the thickness of the bottom border.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-collapse"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/CSS2/tables.html#borders")), syntax :
                Some(Borrowed("table { $(name): collapse; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Selects a table's border model.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("border-color"),
                version : Some(Borrowed("1.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-color")),
                syntax : Some(Borrowed("td { $(name): blue; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("The color of the border around all four edges of an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-image"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C16,FF15,IE11,O15,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-image")),
                syntax : Some(Borrowed("td { $(name): url(border.png) 30 30 round;}")),
                restriction : Some(Borrowed("length, percentage, number, url, enum")), },
                desc :
                Some(Borrowed("Shorthand property for setting 'border-image-source', 'border-image-slice', 'border-image-width', 'border-image-outset' and 'border-image-repeat'. Omitted values are set to their initial values.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-image-outset"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C16,FF15,IE11,O15,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-image-outset")),
                syntax : Some(Borrowed("div { $(name): 3px; }")), restriction :
                Some(Borrowed("length, number")), }, desc :
                Some(Borrowed("The values specify the amount by which the border image area extends beyond the border box on the top, right, bottom, and left sides respectively. If the fourth value is absent, it is the same as the second. If the third one is also absent, it is the same as the first. If the second one is also absent, it is the same as the first. Numbers represent multiples of the corresponding border-width.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-image-repeat"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C16,FF15,IE11,O15,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-border-image-repeat")),
                syntax : Some(Borrowed("td { $(name): stretch; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how the images for the sides and the middle part of the border image are scaled and tiled. If the second keyword is absent, it is assumed to be the same as the first.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-image-slice"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,C16,FF15,IE11,O15,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-image-slice")),
                syntax : Some(Borrowed("div { $(name): 10%; }")), restriction :
                Some(Borrowed("number, percentage")), }, desc :
                Some(Borrowed("Specifies inward offsets from the top, right, bottom, and left edges of the image, dividing it into nine regions: four corners, four edges and a middle.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-image-source"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C16,FF15,IE11,O15,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-border-image-source")),
                syntax : Some(Borrowed("aside { $(name): url(image.png); }")),
                restriction : Some(Borrowed("image")), }, desc :
                Some(Borrowed("Specifies an image to use instead of the border styles given by the 'border-style' properties and as an additional background layer for the element. If the value is 'none' or if the image cannot be displayed, the border styles will be used.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-image-transform"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#propdef-border-image-transform")),
                syntax : Some(Borrowed("div { $(name): logical; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether border images are transformed to match to the value of 'writing-mode' property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-image-width"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,C16,FF15,IE11,O15,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-image-slice")),
                syntax : Some(Borrowed(".album { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage, number")), }, desc :
                Some(Borrowed("The four values of 'border-image-width' specify offsets that are used to divide the border image area into nine parts. They represent inward distances from the top, right, bottom, and left sides of the area, respectively.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-inline-end"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("header { $(name): 5px solid red;}")), restriction
                : Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Logical 'border-right'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-inline-start"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("header { $(name): 5px solid red;}")), restriction
                : Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Logical 'border-left'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-inline-end-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): red; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Logical 'border-right-color'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-inline-start-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): red; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Logical 'border-left-color'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-inline-end-style"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Logical 'border-right-style'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-inline-start-style"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Logical 'border-left-style'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-inline-end-width"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Logical 'border-right-width'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-inline-start-width"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Logical 'border-left-width'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-left"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#borders")), syntax :
                Some(Borrowed("header { $(name): 5px solid red;}")), restriction :
                Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Shorthand property for setting border width, style and color")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-left-color"), version : Some(Borrowed("1.0")), browsers
                : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-color")),
                syntax : Some(Borrowed("td { $(name): blue; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets the color of the left border.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("border-left-style"),
                version : Some(Borrowed("1.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-style")),
                syntax : Some(Borrowed("td { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Sets the style of the left border.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("border-left-width"),
                version : Some(Borrowed("1.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-width")),
                syntax : Some(Borrowed("td { $(name): 2px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Sets the thickness of the left border.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-radius"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF4,IE9,O10.5,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-radius")),
                syntax : Some(Borrowed("td { $(name): 3px 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Defines the radii of the outer border edge.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-right"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#borders")), syntax :
                Some(Borrowed("header { $(name): 5px solid red;}")), restriction :
                Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Shorthand property for setting border width, style and color")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-right-color"), version : Some(Borrowed("1.0")), browsers
                : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-color")),
                syntax : Some(Borrowed("td { $(name): blue; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets the color of the right border.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("border-right-style"),
                version : Some(Borrowed("1.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-style")),
                syntax : Some(Borrowed("td { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Sets the style of the right border.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("border-right-width"),
                version : Some(Borrowed("1.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-width")),
                syntax : Some(Borrowed("td { $(name): 2px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Sets the thickness of the right border.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-spacing"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1,IE8,O7,S1.2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/CSS2/tables.html#borders")), syntax :
                Some(Borrowed("table { $(name): 10px 50px; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("The lengths specify the distance that separates adjoining cell borders. If one length is specified, it gives both the horizontal and vertical spacing. If two are specified, the first gives the horizontal spacing and the second the vertical spacing. Lengths may not be negative.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-style"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-style")),
                syntax : Some(Borrowed("td { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("The style of the border around edges of an element.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-top"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#borders")), syntax :
                Some(Borrowed("header { $(name): 5px solid red;}")), restriction :
                Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Shorthand property for setting border width, style and color")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-top-color"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-color")),
                syntax : Some(Borrowed("td { $(name): blue; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets the color of the top border.")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("border-top-left-radius"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C,FF4,IE9,O10.5,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-radius")),
                syntax : Some(Borrowed("td { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Defines the radii of the top left outer border edge.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-top-right-radius"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C,FF4,IE9,O10.5,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-radius")),
                syntax : Some(Borrowed("td { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Defines the radii of the top right outer border edge.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("border-top-style"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-style")),
                syntax : Some(Borrowed("td { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Sets the style of the top border.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("border-top-width"),
                version : Some(Borrowed("1.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-width")),
                syntax : Some(Borrowed("td { $(name): 2px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Sets the thickness of the top border.")), }, PropertyEntry
                { attributes : PropertyAttributes { name : Borrowed("border-width"),
                version : Some(Borrowed("1.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-width")),
                syntax : Some(Borrowed("td { $(name): 2px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Shorthand that sets the four 'border-*-width' properties. If it has four values, they set top, right, bottom and left in that order. If left is missing, it is the same as right; if bottom is missing, it is the same as top; if right is missing, it is the same as top.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("bottom"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-positioning/#propdef-bottom")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Specifies how far an absolutely positioned box's bottom margin edge is offset above the bottom edge of the box's 'containing block'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("box-decoration-break"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF32,O11")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-break/#break-decoration")),
                syntax : Some(Borrowed("div { $(name): clone; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether individual boxes are treated as broken pieces of one continuous box, or whether each box is individually wrapped with the border and padding.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("box-shadow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF4,IE9,O11.5,S5.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#box-shadow")),
                syntax :
                Some(Borrowed("div { $(name): rgba(0,0,0,0.4) 10px 10px inset; }")),
                restriction : Some(Borrowed("length, color, enum")), }, desc :
                Some(Borrowed("Attaches one or more drop-shadows to the box. The property is a comma-separated list of shadows, each specified by 2-4 length values, an optional color, and an optional 'inset' keyword. Omitted lengths are 0; omitted colors are a user agent chosen color.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("box-sizing"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C10,FF29,IE8,O8,S5.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#box-sizing")), syntax :
                Some(Borrowed("div { $(name): content-box; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the behavior of the 'width' and 'height' properties.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("box-snap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-line-grid-1/#propdef-box-snap")),
                syntax : Some(Borrowed("div { $(name): center; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how the block is snapped to the baseline grid.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("box-suppress"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-display-3/#propdef-box-suppress")),
                syntax : Some(Borrowed("div { $(name): discard; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Toggles whether or not an element appears in the formatting tree without affecting its display type when it is displayed.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("break-after"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-break/#break-between")), syntax
                : Some(Borrowed("h2 { $(name): column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column/region break behavior after the generated box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("break-before"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-break/#break-between")), syntax
                : Some(Borrowed("h2 { $(name): column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column/region break behavior before the generated box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("break-inside"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-break/#break-within")), syntax :
                Some(Borrowed("h2 { $(name): avoid-column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column/region break behavior inside the principal box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("caption-side"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF,IE8,O,S")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/CSS2/tables.html#caption-position")),
                syntax : Some(Borrowed("caption { $(name): bottom; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the position of the caption box with respect to the table box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("caret-color"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C60,FF55,O46")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#propdef-caret-color")),
                syntax : Some(Borrowed("textarea { $(name): red; }")), restriction :
                Some(Borrowed("color, enum")), }, desc :
                Some(Borrowed("Controls the color of the text insertion indicator.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("clear"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/2006/WD-CSS21-20060411/visuren.html#propdef-clear")),
                syntax : Some(Borrowed("footer { $(name): both; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Indicates which sides of an element's box(es) may not be adjacent to an earlier floating box. The 'clear' property does not consider floats inside the element itself or in other block formatting contexts.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("clip"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking/#clip-property")), syntax
                : Some(Borrowed("span { $(name): rect(0px, 60px, 200px, 0px); }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Deprecated. Use the 'clip-path' property when support allows. Defines the visible portion of an element's box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("clip-path"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF3.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking/#the-clip-path")), syntax
                : None, restriction : Some(Borrowed("url, shape, geometry-box, enum")),
                }, desc :
                Some(Borrowed("Specifies a clipping path where everything inside the path is visible and everything outside is clipped out.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("clip-rule"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C5,FF3,IE10,O9,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-clip-rule")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Indicates the algorithm which is to be used to determine what parts of the canvas are included inside the shape.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("color"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-color/#foreground")), syntax :
                Some(Borrowed("body { $(name): red; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets the color of an element's text")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("color-adjust"),
                version : Some(Borrowed("4.0")), browsers : Some(Borrowed("none")), ref_
                :
                Some(Borrowed("https://drafts.csswg.org/css-color-4/#propdef-color-adjust")),
                syntax : Some(Borrowed("image { $(name): exact; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Provides a hint to the user-agent about how it should treat color and style choices that might be expensive or generally unwise on a given device.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("color-interpolation"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#ColorInterpolationProperty")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the color space for gradient interpolations, color animations and alpha compositing.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("color-interpolation-filters"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C5,FF3,IE10,O9,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/filter-effects/#ColorInterpolationFiltersProperty")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the color space for imaging operations performed via filter effects.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("color-rendering"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#ColorRenderingProperty")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Provides a hint about how to optimize its color interpolation and compositing operations.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("column-count"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-count")),
                syntax : Some(Borrowed("div { $(name): 3; }")), restriction :
                Some(Borrowed("integer, enum")), }, desc :
                Some(Borrowed("Describes the optimal number of columns into which the content of the element will be flowed.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("column-fill"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#filling-columns")),
                syntax : Some(Borrowed("article { $(name): balance; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("In continuous media, this property will only be consulted if the length of columns has been constrained. Otherwise, columns will automatically be balanced.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("column-gap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-gap0")), syntax
                : Some(Borrowed("div { $(name): 10px; }")), restriction :
                Some(Borrowed("length, enum")), }, desc :
                Some(Borrowed("Sets the gap between columns. If there is a column rule between columns, it will appear in the middle of the gap.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("column-rule"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule0")),
                syntax : Some(Borrowed("header { $(name): 5px solid red;}")), restriction
                : Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Shorthand for setting 'column-rule-width', 'column-rule-style', and 'column-rule-color' at the same place in the style sheet. Omitted values are set to their initial values.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("column-rule-color"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,IE10,O11.6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule-color")),
                syntax : Some(Borrowed("div { $(name): #ff0; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets the color of the column rule")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("column-rule-style"),
                version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule-style")),
                syntax : Some(Borrowed("div { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Sets the style of the rule between columns of an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("column-rule-width"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,IE10,O11.5,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule-width")),
                syntax : Some(Borrowed("div { $(name): 3px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Sets the width of the rule between columns. Negative values are not allowed.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("columns"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#columns0")), syntax :
                Some(Borrowed("div { $(name): 100px 3; }")), restriction :
                Some(Borrowed("length, integer, enum")), }, desc :
                Some(Borrowed("A shorthand property which sets both 'column-width' and 'column-count'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("column-span"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-span0")),
                syntax : Some(Borrowed("article { $(name): all; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column break behavior after the generated box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("column-width"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10,O11.5,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-width")),
                syntax : Some(Borrowed("div { $(name): 100px; }")), restriction :
                Some(Borrowed("length, enum")), }, desc :
                Some(Borrowed("Describes the width of columns in multicol elements.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("contain"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C52,O40")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-containment-3/#propdef-contain")),
                syntax : Some(Borrowed("div { $(name): strict; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Indicates that an element and its contents are, as much as possible, independent of the rest of the document tree.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("content"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1,IE8,O4,S1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-content/#content")), syntax :
                Some(Borrowed("a:after { $(name): ' ( attr(href))';}")), restriction :
                Some(Borrowed("string, url")), }, desc :
                Some(Borrowed("Determines which page-based occurrence of a given element is applied to a counter or string value.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("counter-increment"), version : Some(Borrowed("2.0")), browsers
                : Some(Borrowed("E,C,FF1.5,IE8,O10.5,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-content/#counters")), syntax :
                Some(Borrowed("h1:before { $(name): section; }")), restriction :
                Some(Borrowed("identifier, integer")), }, desc :
                Some(Borrowed("Manipulate the value of existing counters.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("counter-reset"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE8,O10.5,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-content/#counters")), syntax :
                Some(Borrowed("h1 { $(name): section; }")), restriction :
                Some(Borrowed("identifier, integer")), }, desc :
                Some(Borrowed("Property accepts one or more names of counters (identifiers), each one optionally followed by an integer. The integer gives the value that the counter is set to on each occurrence of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("crop"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-content/#the-crop")), syntax :
                Some(Borrowed("<shape> | auto")), restriction : Some(Borrowed("enum")),
                }, desc :
                Some(Borrowed("Allows a replaced element to be just a rectangular area of an object, instead of the whole object.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("cue"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#cue")), syntax :
                Some(Borrowed("h1 { $(name): url(ding.wav) -3dB url(dong.wav) -2dB;}")),
                restriction : Some(Borrowed("url, volume, enum")), }, desc :
                Some(Borrowed("Shorthand for setting 'cue-before' and 'cue-after'. If two values are given the first value is 'cue-before' and the second is 'cue-after'. If only one value is given, it applies to both properties.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("cue-after"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#cue-after")), syntax :
                Some(Borrowed("h1 { $(name): url(dong.wav) -3dB;}")), restriction :
                Some(Borrowed("url, volume, enum")), }, desc :
                Some(Borrowed("Specifies an auditory icon (i.e. pre-recorded / pre-generated sound clips) to be played after the selected element within the aural box model.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("cue-before"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#cue-before")), syntax :
                Some(Borrowed("h1 { $(name): url(ding.wav) -3dB;}")), restriction :
                Some(Borrowed("url, volume, enum")), }, desc :
                Some(Borrowed("Specifies an auditory icon (i.e. pre-recorded / pre-generated sound clips) to be played before the selected element within the aural box model.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("cursor"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#cursor0")), syntax :
                Some(Borrowed("nav { $(name): pointer; }")), restriction :
                Some(Borrowed("url, number, enum")), }, desc :
                Some(Borrowed("Allows control over cursor appearance in an element")), },
                PropertyEntry { attributes : PropertyAttributes { name : Borrowed("cx"),
                version : Some(Borrowed("4.0")), browsers : Some(Borrowed("none")), ref_
                : Some(Borrowed("http://www.w3.org/TR/SVG2/geometry.html#CX")), syntax :
                Some(Borrowed("circle { $(name): 10%; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Describes the horizontal center coordinate of the position of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("cy"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/geometry.html#CY")), syntax :
                Some(Borrowed("circle { $(name): 10%; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Describes the vertical center coordinate of the position of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("direction"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-writing-modes-3/#direction")),
                syntax : Some(Borrowed("div { $(name): rtl; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the inline base direction or directionality of any bidi paragraph, embedding, isolate, or override established by the box. Note: for HTML content use the 'dir' attribute and 'bdo' element rather than this property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("display"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-display-3/#propdef-display")),
                syntax : Some(Borrowed("p { $(name): inline; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("In combination with 'float' and 'position', determines the type of box or boxes that are generated for an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("dominant-baseline"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-inline/#propdef-dominant-baseline")),
                syntax : Some(Borrowed("span { $(name): alphabetic; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the dominant baseline, which is the baseline used to align the box's text and inline-level contents.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("empty-cells"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1,IE7,O4,S1.2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/CSS2/tables.html#empty-cells")),
                syntax : Some(Borrowed("table { $(name): hide; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("In the separated borders model, this property controls the rendering of borders and backgrounds around cells that have no visible content.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("enable-background"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/filter-effects/#AccessBackgroundImage")),
                syntax : None, restriction :
                Some(Borrowed("integer, length, percentage, enum")), }, desc :
                Some(Borrowed("Deprecated. Use 'isolation' property instead when support allows. Specifies how the accumulation of the background image is managed.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("fallback"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-counter-styles-3/#descdef-counter-style-fallback")),
                syntax : Some(Borrowed("@counter-style { fallback: upper-alpha; }")),
                restriction : Some(Borrowed("identifier")), }, desc :
                Some(Borrowed("@counter-style descriptor. Specifies a fallback counter style to be used when the current counter style can't create a representation for a given counter value.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("fill"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#FillProperty")),
                syntax : None, restriction : Some(Borrowed("color, enum, url")), }, desc
                : Some(Borrowed("Paints the interior of the given graphical element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("fill-opacity"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#FillOpacity")),
                syntax : None, restriction : Some(Borrowed("number(0-1)")), }, desc :
                Some(Borrowed("Specifies the opacity of the painting operation used to paint the interior the current object.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("fill-rule"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#WindingRule")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Indicates the algorithm (or winding rule) which is to be used to determine what parts of the canvas are included inside the shape.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("filter"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E13,FF35")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/filter-effects/#propdef-filter")),
                syntax : Some(Borrowed("div { $(name): opacity(50%); }")), restriction :
                Some(Borrowed("enum, url")), }, desc :
                Some(Borrowed("Processes an element's rendering before it is displayed in the document, by applying one or more filter effects.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("flex"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF22,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex")), syntax :
                Some(Borrowed("p { $(name): 0 1 auto; }")), restriction :
                Some(Borrowed("length, number, percentage")), }, desc :
                Some(Borrowed("Specifies the components of a flexible length: the flex grow factor and flex shrink factor, and the flex basis.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("flex-basis"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF22,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-basis-propdef")),
                syntax : Some(Borrowed("p { $(name): 30%; }")), restriction :
                Some(Borrowed("length, number, percentage")), }, desc :
                Some(Borrowed("Sets the flex basis.")), }, PropertyEntry { attributes :
                PropertyAttributes { name : Borrowed("flex-direction"), version :
                Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF22,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-direction")),
                syntax : Some(Borrowed("div { $(name): column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how flex items are placed in the flex container, by setting the direction of the flex container's main axis.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("flex-flow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF28,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-flow")), syntax :
                Some(Borrowed("div { $(name): column wrap; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how flexbox items are placed in the flexbox.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("flex-grow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF22,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-grow")), syntax :
                Some(Borrowed("p { $(name): 4; }")), restriction :
                Some(Borrowed("number")), }, desc :
                Some(Borrowed("Sets the flex grow factor. Negative numbers are invalid.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("flex-shrink"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF22,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-shrink")), syntax
                : Some(Borrowed("p { $(name): 4; }")), restriction :
                Some(Borrowed("number")), }, desc :
                Some(Borrowed("Sets the flex shrink factor. Negative numbers are invalid.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("flex-wrap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF28,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-wrap")), syntax :
                Some(Borrowed("div { $(name): nowrap; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls whether the flex container is single-line or multi-line, and the direction of the cross-axis, which determines the direction new lines are stacked in.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("float"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/CSS21/visuren.html#propdef-float")),
                syntax : Some(Borrowed("img { $(name): right; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how a box should be floated. It may be set for any element, but only applies to elements that generate boxes that are not absolutely positioned.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("flood-color"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C5,FF3,IE10,O9,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/filter-effects/#FloodColorProperty")),
                syntax : None, restriction : Some(Borrowed("color")), }, desc :
                Some(Borrowed("Indicates what color to use to flood the current filter primitive subregion.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("flood-opacity"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C5,FF3,IE10,O9,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/filter-effects/#FloodOpacityProperty")),
                syntax : None, restriction : Some(Borrowed("number(0-1), percentage")),
                }, desc :
                Some(Borrowed("Indicates what opacity to use to flood the current filter primitive subregion.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("flow-from"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-regions/#flow-from")), syntax :
                Some(Borrowed("div { $(name): identifier; }")), restriction :
                Some(Borrowed("identifier")), }, desc :
                Some(Borrowed("Makes a block container a region and associates it with a named flow.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("flow-into"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-regions/#flow-into")), syntax :
                Some(Borrowed("div { $(name): identifier; }")), restriction :
                Some(Borrowed("identifier")), }, desc :
                Some(Borrowed("Places an element or its contents into a named flow.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#propdef-font")), syntax :
                Some(Borrowed("body { $(name): bold 12px arial, verdana; }")),
                restriction : Some(Borrowed("font")), }, desc :
                Some(Borrowed("Shorthand property for setting 'font-style', 'font-variant', 'font-weight', 'font-size', 'line-height', and 'font-family', at the same place in the style sheet. The syntax of this property is based on a traditional typographical shorthand notation to set multiple properties related to fonts.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-family"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-family0")), syntax :
                Some(Borrowed("body { $(name): arial, verdana; }")), restriction :
                Some(Borrowed("font")), }, desc :
                Some(Borrowed("Specifies a prioritized list of font family names or generic family names. A user agent iterates through the list of family names until it matches an available font that contains a glyph for the character to be rendered.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-feature-settings"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,FF34,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#propdef-font-feature-settings")),
                syntax : Some(Borrowed("body { $(name): 'hwid'; }")), restriction :
                Some(Borrowed("string, integer")), }, desc :
                Some(Borrowed("Provides low-level control over OpenType font features. It is intended as a way of providing access to font features that are not widely used but are needed for a particular use case.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-kerning"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C33,FF34,O20")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#propdef-font-kerning")),
                syntax : Some(Borrowed("body { $(name): normal; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Kerning is the contextual adjustment of inter-glyph spacing. This property controls metric kerning, kerning that utilizes adjustment data contained in the font.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-language-override"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF34")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-language-override-prop")),
                syntax : Some(Borrowed("body { $(name): 'SRB'; }")), restriction :
                Some(Borrowed("string")), }, desc :
                Some(Borrowed("The value of 'normal' implies that when rendering with OpenType fonts the language of the document is used to infer the OpenType language system, used to select language specific features when rendering.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-size"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-size-prop")), syntax
                : Some(Borrowed("div { $(name): 12px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Indicates the desired height of glyphs from the font. For scalable fonts, the font-size is a scale factor applied to the EM unit of the font. (Note that certain glyphs may bleed outside their EM box.) For non-scalable fonts, the font-size is converted into absolute units and matched against the declared font-size of the font, using the same absolute coordinate space for both of the matched values.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-size-adjust"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,FF3,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-size-adjust")),
                syntax : Some(Borrowed("div { $(name): 0.58; }")), restriction :
                Some(Borrowed("number")), }, desc :
                Some(Borrowed("Preserves the readability of text when font fallback occurs by adjusting the font-size so that the x-height is the same regardless of the font used.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-stretch"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,FF9,IE9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-stretch0")), syntax
                : Some(Borrowed("div { $(name): expanded; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Selects a normal, condensed, or expanded face from a font family.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-style"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-style0")), syntax :
                Some(Borrowed("body { $(name): italic; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Allows italic or oblique faces to be selected. Italic forms are generally cursive in nature while oblique faces are typically sloped versions of the regular face.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-synthesis"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF34,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#propdef-font-synthesis")),
                syntax : Some(Borrowed("html:lang(ar) { $(name): none; }")), restriction
                : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls whether user agents are allowed to synthesize bold or oblique font faces when a font family lacks bold or italic faces.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-variant"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-variant-prop")),
                syntax : Some(Borrowed("div { $(name): small-caps; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies variant representations of the font")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-variant-alternates"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF34")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#propdef-font-variant-alternates")),
                syntax : Some(Borrowed("h2 { $(name): styleset(3,5); }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("For any given character, fonts can provide a variety of alternate glyphs in addition to the default glyph for that character. This property provides control over the selection of these alternate glyphs.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-variant-caps"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF34")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-variant-caps-prop")),
                syntax : Some(Borrowed("p { $(name): titling-caps; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies control over capitalized forms.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-variant-east-asian"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF34")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-variant-east-asian-prop")),
                syntax : Some(Borrowed("mark { $(name): normal; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Allows control of glyph substitute and positioning in East Asian text.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-variant-ligatures"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C18,FF34,O15,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-variant-ligatures-prop")),
                syntax : Some(Borrowed("div { $(name): historical-ligatures; }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies control over which ligatures are enabled or disabled. A value of 'normal' implies that the defaults set by the font are used.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-variant-numeric"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF34")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-variant-numeric-prop")),
                syntax :
                Some(Borrowed(".amount { $(name): oldstyle-nums diagonal-fractions; }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies control over numerical forms.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("font-variant-position"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF34")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#propdef-font-variant-position")),
                syntax : Some(Borrowed("sub { $(name): subscript; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the vertical position")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("font-weight"), version
                : Some(Borrowed("1.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#font-weight-the-font-weight-property")),
                syntax : Some(Borrowed("th { $(name): bold; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies weight of glyphs in the font, their degree of blackness or stroke thickness.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("glyph-orientation-horizontal"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/text.html#GlyphOrientationHorizontal")),
                syntax : None, restriction : Some(Borrowed("angle, number")), }, desc :
                Some(Borrowed("Controls glyph orientation when the inline-progression-direction is horizontal.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("glyph-orientation-vertical"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/text.html#GlyphOrientationVertical")),
                syntax : None, restriction : Some(Borrowed("angle, number, enum")), },
                desc :
                Some(Borrowed("Controls glyph orientation when the inline-progression-direction is vertical.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-area"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-area")),
                syntax : Some(Borrowed("div { $(name): span 3; }")), restriction :
                Some(Borrowed("identifier, integer")), }, desc :
                Some(Borrowed("Determine a grid item's size and location within the grid by contributing a line, a span, or nothing (automatic) to its grid placement. Shorthand for 'grid-row-start', 'grid-column-start', 'grid-row-end', and 'grid-column-end'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,E16,S10.1,O44")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-grid/#propdef-grid")), syntax
                : Some(Borrowed("div { $(name): span 3; }")), restriction :
                Some(Borrowed("identifier, length, percentage, string, enum")), }, desc :
                Some(Borrowed("The grid CSS property is a shorthand property that sets all of the explicit grid properties ('grid-template-rows', 'grid-template-columns', and 'grid-template-areas'), and all the implicit grid properties ('grid-auto-rows', 'grid-auto-columns', and 'grid-auto-flow'), in a single declaration.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-auto-columns"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-auto-columns")),
                syntax : Some(Borrowed("div { $(name): 100px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Specifies the size of implicitly created columns.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-auto-flow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-auto-flow")),
                syntax : Some(Borrowed("div { $(name): column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls how the auto-placement algorithm works, specifying exactly how auto-placed items get flowed into the grid.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-auto-rows"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-auto-rows")),
                syntax : Some(Borrowed("div { $(name): 100px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Specifies the size of implicitly created rows.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-column"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-column")),
                syntax : Some(Borrowed("#item1 { $(name): span 2 / auto; }")),
                restriction : Some(Borrowed("identifier, integer, enum")), }, desc :
                Some(Borrowed("Shorthand for 'grid-column-start' and 'grid-column-end'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-column-end"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-column-end")),
                syntax : Some(Borrowed("#item1 { $(name): span 2; }")), restriction :
                Some(Borrowed("identifier, integer, enum")), }, desc :
                Some(Borrowed("Determine a grid item's size and location within the grid by contributing a line, a span, or nothing (automatic) to its grid placement.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-column-gap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-column-gap")),
                syntax : Some(Borrowed("#item1 { $(name): 2em; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Specifies the gutters between grid columns. Replaced by 'column-gap' property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-column-start"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-column-start")),
                syntax : Some(Borrowed("#item1 { $(name): span 2; }")), restriction :
                Some(Borrowed("identifier, integer, enum")), }, desc :
                Some(Borrowed("Determine a grid item's size and location within the grid by contributing a line, a span, or nothing (automatic) to its grid placement.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-gap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-gap")),
                syntax : Some(Borrowed("#item1 { $(name): 2em 1em; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Shorthand that specifies the gutters between grid columns and grid rows in one declaration. Replaced by 'gap' property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-row"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-row")),
                syntax : Some(Borrowed("#item1 { $(name): span 2 / auto; }")),
                restriction : Some(Borrowed("identifier, integer, enum")), }, desc :
                Some(Borrowed("Shorthand for 'grid-row-start' and 'grid-row-end'.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-row-end"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-row-end")),
                syntax : Some(Borrowed("#item1 { $(name): span 2; }")), restriction :
                Some(Borrowed("identifier, integer, enum")), }, desc :
                Some(Borrowed("Determine a grid item's size and location within the grid by contributing a line, a span, or nothing (automatic) to its grid placement.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-row-gap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-row-gap")),
                syntax : Some(Borrowed("#item1 { $(name): 2em; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Specifies the gutters between grid rows. Replaced by 'row-gap' property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-row-start"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-row-start")),
                syntax : Some(Borrowed("#item1 { $(name): span 2; }")), restriction :
                Some(Borrowed("identifier, integer, enum")), }, desc :
                Some(Borrowed("Determine a grid item's size and location within the grid by contributing a line, a span, or nothing (automatic) to its grid placement.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-template"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-template")),
                syntax : Some(Borrowed("#item1 { $(name): auto 1fr auto / auto 1fr; }")),
                restriction :
                Some(Borrowed("identifier, length, percentage, string, enum")), }, desc :
                Some(Borrowed("Shorthand for setting grid-template-columns, grid-template-rows, and grid-template-areas in a single declaration.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-template-areas"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-template-areas")),
                syntax :
                Some(Borrowed("#item1 { $(name): 'head head' 'nav main' 'foot foot'; }")),
                restriction : Some(Borrowed("string")), }, desc :
                Some(Borrowed("Specifies named grid areas, which are not associated with any particular grid item, but can be referenced from the grid-placement properties.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-template-columns"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-template-columns")),
                syntax :
                Some(Borrowed("#item1 { $(name): 100px 1fr max-content minmax(min-content, 1fr); }")),
                restriction : Some(Borrowed("identifier, length, percentage, enum")), },
                desc :
                Some(Borrowed("specifies, as a space-separated track list, the line names and track sizing functions of the grid.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("grid-template-rows"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF52,C57,S10.1,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-grid-1/#propdef-grid-template-rows")),
                syntax :
                Some(Borrowed("#item1 { $(name): 100px 1fr max-content minmax(min-content, 1fr); }")),
                restriction :
                Some(Borrowed("identifier, length, percentage, string, enum")), }, desc :
                Some(Borrowed("specifies, as a space-separated track list, the line names and track sizing functions of the grid.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("hanging-punctuation"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#hanging-punctuation0")),
                syntax : Some(Borrowed("p { $(name): first; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Determines whether a punctuation mark, if one is present, may be placed outside the content area at the start or at the end of a full line of text.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("height"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#height")), syntax :
                Some(Borrowed("footer { $(name): 100px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Specifies the height of the content area, padding area or border area (depending on 'box-sizing') of certain boxes.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("hyphenate-character"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-hyphenate-character")),
                syntax : Some(Borrowed("div { $(name):'~'; }")), restriction :
                Some(Borrowed("string, enum")), }, desc :
                Some(Borrowed("Specifies strings that are shown between parts of hyphenated words.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("hyphenate-limit-chars"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-hyphenate-limit-chars")),
                syntax : Some(Borrowed("div { $(name): 5 2 2; }")), restriction :
                Some(Borrowed("integer, enum")), }, desc :
                Some(Borrowed("Specifies the minimum number of characters in a hyphenated word.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("hyphenate-limit-last"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-hyphenate-limit-last")),
                syntax : Some(Borrowed("div { $(name): 2; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Indicates hyphenation behavior at the end of elements, column, pages and spreads.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("hyphenate-limit-lines"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-hyphenate-limit-lines")),
                syntax : Some(Borrowed("div { $(name): 2; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Indicates the maximum number of successive hyphenated lines in an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("hyphenate-limit-zone"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-hyphenate-limit-zone")),
                syntax : Some(Borrowed("div { $(name): 25%; }")), restriction :
                Some(Borrowed("percentage, length")), }, desc :
                Some(Borrowed("Specifies the maximum amount of unfilled space (before justification) that may be left in the line box before hyphenation is triggered to pull part of a word from the next line back up into the current line.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("hyphens"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C55,FF43,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-3/#hyphens-property")),
                syntax : Some(Borrowed("div { $(name): manual; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls whether hyphenation is allowed to create more break opportunities within a line of text.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("image-orientation"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF26")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css4-images/#image-orientation")),
                syntax : Some(Borrowed("img.ninety { $(name): 90deg; }")), restriction :
                Some(Borrowed("angle")), }, desc :
                Some(Borrowed("Specifies an orthogonal rotation to be applied to an image before it is laid out.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("image-rendering"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,FF3.6,O11.6,S")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-images-3/#the-image-rendering")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Provides a hint to the user-agent about what aspects of an image are most important to preserve when the image is scaled, to aid the user-agent in the choice of an appropriate scaling algorithm.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("image-resolution"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css4-images/#image-resolution")),
                syntax : Some(Borrowed("img.high-res { $(name): 300dpi; }")), restriction
                : Some(Borrowed("resolution")), }, desc :
                Some(Borrowed("Specifies the intrinsic resolution of all raster images used in or on the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("ime-mode"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,FF3,IE5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#ime-mode")), syntax :
                Some(Borrowed("body { $(name): active; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls the state of the input method editor for text fields.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("initial-letter"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-inline/#propdef-initial-letter")),
                syntax : Some(Borrowed("body { $(name): 3; }")), restriction :
                Some(Borrowed("number, integer, enum")), }, desc :
                Some(Borrowed("Specifies styling for dropped, raised, and sunken initial letters.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("initial-letter-align"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-inline/#propdef-initial-letter-align")),
                syntax : Some(Borrowed("body { $(name): ideographic; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the alignment points used to size and position an initial letter.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("initial-letter-wrap"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-inline/#propdef-initial-letter-wrap")),
                syntax : Some(Borrowed("body { $(name): first; }")), restriction :
                Some(Borrowed("length, percentage, enum")), }, desc :
                Some(Borrowed("Specifies whether lines impacted by an initial letter are shortened to fit the rectangular shape of the initial letter box or follow the contour of its end-edge glyph outline.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("inline-size"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#propdef-inline-size")),
                syntax : Some(Borrowed("header { $(name): 200px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Size of an element in the direction specified by 'writing-mode'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("isolation"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,FF,O,S")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/compositing-1/#isolation")), syntax :
                Some(Borrowed("div { $(name): isolate; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("In CSS setting to 'isolate' will turn the element into a stacking context. In SVG, it defines whether an element is isolated or not.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("justify-content"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF22,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#align-content")),
                syntax : Some(Borrowed("p { $(name): flex-start; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Aligns flex items along the main axis of the current line of the flex container.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("kerning"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG11/text.html#KerningProperty")),
                syntax : None, restriction : Some(Borrowed("length, enum")), }, desc :
                Some(Borrowed("Indicates whether the user agent should adjust inter-glyph spacing based on kerning tables that are included in the relevant font or instead disable auto-kerning and set inter-character spacing to a specific length.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("left"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-positioning/#propdef-left")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Specifies how far an absolutely positioned box's left margin edge is offset to the right of the left edge of the box's 'containing block'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("letter-spacing"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#letter-spacing0")), syntax
                : Some(Borrowed("h2 { $(name): 2px; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Specifies the minimum, maximum, and optimal spacing between grapheme clusters.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("lighting-color"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C5,FF3,IE10,O9,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/filter-effects/#LightingColorProperty")),
                syntax : None, restriction : Some(Borrowed("color")), }, desc :
                Some(Borrowed("Defines the color of the light source for filter primitives 'feDiffuseLighting' and 'feSpecularLighting'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("line-break"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE5.5,C58,O45,S")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#line-break0")), syntax :
                Some(Borrowed("p { $(name): normal; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies what set of line breaking restrictions are in effect within the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("line-grid"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-line-grid-1/#propdef-line-grid")),
                syntax : Some(Borrowed("div { $(name): create; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether this box creates a new baseline grid for its descendants or uses the same baseline grid as its parent.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("line-height"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-linebox/#line-height")), syntax
                : Some(Borrowed("#menu { $(name): 22px; }")), restriction :
                Some(Borrowed("number, length, percentage")), }, desc :
                Some(Borrowed("Determines the block-progression dimension of the text content area of an inline box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("line-snap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-line-grid-1/#propdef-line-snap")),
                syntax : Some(Borrowed("div { $(name): baseline; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Snaps line boxes contained by the element to the line grid specified by the 'line-grid' property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("list-style"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-lists/#list-style")), syntax :
                Some(Borrowed("ul { $(name): square url('square.png');}")), restriction :
                Some(Borrowed("image, enum, url")), }, desc :
                Some(Borrowed("Shorthand for setting 'list-style-type', 'list-style-position' and 'list-style-image'")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("list-style-image"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-lists/#list-style-image")),
                syntax : Some(Borrowed("<uri> | none")), restriction :
                Some(Borrowed("image")), }, desc :
                Some(Borrowed("Sets the image that will be used as the list item marker. When the image is available, it will replace the marker set with the 'list-style-type' marker.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("list-style-position"), version : Some(Borrowed("1.0")),
                browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-lists/#list-style-position")),
                syntax : Some(Borrowed("ul { $(name): inside; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the position of the '::marker' pseudo-element's box in the list item.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("list-style-type"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-lists/#list-style-type")),
                syntax :
                Some(Borrowed("<glyph> | <algorithmic> | <numeric> | <alphabetic> | <symbolic> | <non-repeating> | normal | none")),
                restriction : Some(Borrowed("enum, string")), }, desc :
                Some(Borrowed("Used to construct the default contents of a list item's marker")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("margin"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#margin1")), syntax :
                Some(Borrowed("div { $(name): 4px 7px 2px 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Shorthand property to set values for the thickness of the margin area. If left is omitted, it is the same as right. If bottom is omitted it is the same as top, if right is omitted it is the same as top. Negative values for margin properties are allowed, but there may be implementation-specific limits.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("margin-block-end"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#logical-prop")),
                syntax : Some(Borrowed("div { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'margin-bottom'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("margin-block-start"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#logical-prop")),
                syntax : Some(Borrowed("div { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'margin-top'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("margin-bottom"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#margin1")), syntax :
                Some(Borrowed("div { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Shorthand property to set values for the thickness of the margin area. If left is omitted, it is the same as right. If bottom is omitted it is the same as top, if right is omitted it is the same as top. Negative values for margin properties are allowed, but there may be implementation-specific limits..")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("margin-inline-end"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#logical-prop")),
                syntax : Some(Borrowed("div { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'margin-right'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("margin-inline-start"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#logical-prop")),
                syntax : Some(Borrowed("div { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'margin-left'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("margin-left"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#margin1")), syntax :
                Some(Borrowed("div { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Shorthand property to set values for the thickness of the margin area. If left is omitted, it is the same as right. If bottom is omitted it is the same as top, if right is omitted it is the same as top. Negative values for margin properties are allowed, but there may be implementation-specific limits..")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("margin-right"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#margin1")), syntax :
                Some(Borrowed("div { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Shorthand property to set values for the thickness of the margin area. If left is omitted, it is the same as right. If bottom is omitted it is the same as top, if right is omitted it is the same as top. Negative values for margin properties are allowed, but there may be implementation-specific limits..")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("margin-top"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#margin1")), syntax :
                Some(Borrowed("div { $(name): 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Shorthand property to set values for the thickness of the margin area. If left is omitted, it is the same as right. If bottom is omitted it is the same as top, if right is omitted it is the same as top. Negative values for margin properties are allowed, but there may be implementation-specific limits..")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("marker"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#MarkerProperty")),
                syntax : None, restriction : Some(Borrowed("url")), }, desc :
                Some(Borrowed("Specifies the marker symbol that shall be used for all points on the sets the value for all vertices on the given 'path' element or basic shape.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("marker-end"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#VertexMarkerProperties")),
                syntax : None, restriction : Some(Borrowed("url")), }, desc :
                Some(Borrowed("Specifies the marker that will be drawn at the last vertices of the given markable element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("marker-mid"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#VertexMarkerProperties")),
                syntax : None, restriction : Some(Borrowed("url")), }, desc :
                Some(Borrowed("Specifies the marker that will be drawn at all vertices except the first and last.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("marker-side"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-lists/#propdef-marker-side")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies if list item markers position themselves relative to the list item or the list container's directionality.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("marker-start"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#VertexMarkerProperties")),
                syntax : None, restriction : Some(Borrowed("url")), }, desc :
                Some(Borrowed("Specifies the marker that will be drawn at the first vertices of the given markable element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask")), syntax :
                None, restriction :
                Some(Borrowed("url, image, length, percentage, position, repeat, geometry-box, enum")),
                }, desc : Some(Borrowed("The mask shorthand.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("mask-border"), version
                : Some(Borrowed("3.0")), browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-border")),
                syntax : None, restriction :
                Some(Borrowed("image, length, number, percentage, enum")), }, desc :
                Some(Borrowed("Shorthand property for setting 'mask-border-source', 'mask-border-slice', 'mask-border-width', 'mask-border-outset', 'mask-border-repeat', and 'mask-border-mode'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-border-mode"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-border-mode")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Indicates whether the image value for 'mask-border-source' is treated as luminance mask or alpha mask.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-border-outset"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-border-outset")),
                syntax : None, restriction : Some(Borrowed("length, number")), }, desc :
                Some(Borrowed("Specifies the amount by which the mask border image area extends beyond the border box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-border-repeat"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-border-repeat")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how the images for the sides and the middle part of the mask border image are scaled and tiled.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-border-slice"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-border-slice")),
                syntax : None, restriction : Some(Borrowed("number, percentage, enum")),
                }, desc :
                Some(Borrowed("Specifies inward offsets from the top, right, bottom, and left edges of the mask border image, dividing it into nine regions: four corners, four edges and a middle.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-border-source"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-border-source")),
                syntax : None, restriction : Some(Borrowed("image, enum")), }, desc :
                Some(Borrowed("Specifies an image to be used as mask border image.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-border-width"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-border-width")),
                syntax : None, restriction : Some(Borrowed("length, percentage, enum")),
                }, desc :
                Some(Borrowed("Specifies offsets that are used to divide the mask border image area into nine parts.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-clip"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-clip")),
                syntax : None, restriction : Some(Borrowed("geometry-box, enum")), },
                desc :
                Some(Borrowed("Determines the mask painting area, which determines the area that is affected by the mask.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-composite"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-composite")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines the compositing operation used on the current mask layer with the mask layers below it.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-image"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,FF53")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-image")),
                syntax : None, restriction : Some(Borrowed("url, image, enum")), }, desc
                : Some(Borrowed("Sets the mask layer image of an element.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-mode"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF53")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-mode")),
                syntax : None, restriction : Some(Borrowed("url, image, enum")), }, desc
                :
                Some(Borrowed("Indicates whether the mask layer image is treated as luminance mask or alpha mask.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-origin"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF53")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-origin")),
                syntax : None, restriction : Some(Borrowed("geometry-box, enum")), },
                desc : Some(Borrowed("Specifies the mask positioning area.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-position"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF53")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-position")),
                syntax : None, restriction :
                Some(Borrowed("position, length, percentage")), }, desc :
                Some(Borrowed("Specifies how mask layer images are positioned.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-repeat"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF53")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-repeat")),
                syntax : None, restriction : Some(Borrowed("repeat")), }, desc :
                Some(Borrowed("Specifies how mask layer images are tiled after they have been sized and positioned.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-size"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("F53")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-size")),
                syntax : None, restriction : Some(Borrowed("length, percentage, enum")),
                }, desc : Some(Borrowed("Specifies the size of the mask layer images.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mask-type"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C24,FF35,O15,S7")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-type")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether the content of the <mask> element is treated as as luminance mask or alpha mask.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("max-block-size"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#propdef-min-block-size")),
                syntax : Some(Borrowed("header { $(name): 200px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Maximum size of an element in the direction opposite that of the direction specified by 'writing-mode'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("max-height"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1,IE7,O7,S1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#max-height")), syntax :
                Some(Borrowed("footer { $(name): 300px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Allows authors to constrain content height to a certain range.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("max-inline-size"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#propdef-min-block-size")),
                syntax : Some(Borrowed("header { $(name): 200px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Maximum size of an element in the direction specified by 'writing-mode'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("max-width"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1,IE7,O7,S1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#max-width")), syntax :
                Some(Borrowed("footer { $(name): 300px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Allows authors to constrain content width to a certain range.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("max-lines"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-overflow-3/#max-lines0")), syntax
                : Some(Borrowed("div::nth-fragment(1) { $(name): 3; }")), restriction :
                Some(Borrowed("integer, enum")), }, desc :
                Some(Borrowed("Forces a fragment to break after the specified number of lines.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("max-zoom"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-device-adapt/#max-zoom-desc")),
                syntax : Some(Borrowed("@viewport { $(name): 400%; }")), restriction :
                Some(Borrowed("number, percentage, enum")), }, desc :
                Some(Borrowed("@viewport descriptor. Specifies the largest allowed zoom factor.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("min-block-size"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#propdef-min-block-size")),
                syntax : Some(Borrowed("header { $(name): 200px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Minimal size of an element in the direction opposite that of the direction specified by 'writing-mode'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("min-height"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1,IE7,O7,S1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#min-height")), syntax :
                Some(Borrowed("footer { $(name): 300px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Allows authors to constrain content height to a certain range.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("min-inline-size"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#propdef-min-block-size")),
                syntax : Some(Borrowed("header { $(name): 200px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Minimal size of an element in the direction specified by 'writing-mode'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("min-width"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1,IE7,O7,S1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#min-width")), syntax :
                Some(Borrowed("footer { $(name): 300px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Allows authors to constrain content width to a certain range.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("mix-blend-mode"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C41,FF32,O29,S7.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/compositing-1/#propdef-mix-blend-mode")),
                syntax : Some(Borrowed("div { $(name): saturation; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines the formula that must be used to mix the colors with the backdrop.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("min-zoom"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-device-adapt/#min-zoom-desc")),
                syntax : Some(Borrowed("@viewport { $(name): 100%; }")), restriction :
                Some(Borrowed("number, percentage, enum")), }, desc :
                Some(Borrowed("@viewport descriptor. Specifies the smallest allowed zoom factor.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("motion"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C46,O33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/motion-1/#propdef-motion")), syntax :
                None, restriction :
                Some(Borrowed("url, length, percentage, angle, shape, geometry-box, enum")),
                }, desc :
                Some(Borrowed("Shorthand property for setting 'motion-path', 'motion-offset' and 'motion-rotation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("motion-offset"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C46,O33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/motion-1/#propdef-motion-offset")),
                syntax : Some(Borrowed("div { $(name): 10%; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("A distance that describes the position along the specified motion path.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("motion-path"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C46,O33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/motion-1/#propdef-motion-path")),
                syntax : None, restriction :
                Some(Borrowed("url, shape, geometry-box, enum")), }, desc :
                Some(Borrowed("Specifies the motion path the element gets positioned at.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("motion-rotation"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C46,O33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/motion-1/#propdef-motion-rotation")),
                syntax : Some(Borrowed("div { $(name): 90%; }")), restriction :
                Some(Borrowed("angle")), }, desc :
                Some(Borrowed("Defines the direction of the element while positioning along the motion path.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("move-to"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-content/#moving")), syntax :
                Some(Borrowed("normal | here | <identifier>")), restriction :
                Some(Borrowed("identifier")), }, desc :
                Some(Borrowed("Property causes the element or pseudo-element to be removed from the flow and reinserted at a later point in the document.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-animation"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation")), syntax
                : Some(Borrowed("div { $(name): movearound 4s ease 3 normal; }")),
                restriction :
                Some(Borrowed("time, enum, timing-function, identifier, number")), },
                desc :
                Some(Borrowed("Shorthand property combines six of the animation properties into a single property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-animation-delay"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-delay")),
                syntax : Some(Borrowed("div { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines when the animation will start.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-animation-direction"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-direction")),
                syntax : Some(Borrowed("div { $(name): normal; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether or not the animation should play in reverse on alternate cycles.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-animation-duration"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-duration")),
                syntax : Some(Borrowed("div { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines the length of time that an animation takes to complete one cycle.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-animation-iteration-count"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("FF9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-iteration-count")),
                syntax : Some(Borrowed("div { $(name): 3; }")), restriction :
                Some(Borrowed("number, enum")), }, desc :
                Some(Borrowed("Defines the number of times an animation cycle is played. The default value is one, meaning the animation will play from beginning to end once.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-animation-name"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#the-animation-name-property-")),
                syntax : Some(Borrowed("div { $(name): movearound; }")), restriction :
                Some(Borrowed("identifier, enum")), }, desc :
                Some(Borrowed("Defines a list of animations that apply. Each name is used to select the keyframe at-rule that provides the property values for the animation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-animation-play-state"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-play-state")),
                syntax : Some(Borrowed("div { $(name): running; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether the animation is running or paused.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-animation-timing-function"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("FF9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-timing-function")),
                syntax : Some(Borrowed("div { $(name): ease; }")), restriction :
                Some(Borrowed("timing-function")), }, desc :
                Some(Borrowed("Describes how the animation will progress over one cycle of its duration. See the 'transition-timing-function'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-appearance"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-appearance")),
                syntax : Some(Borrowed(".example { $(name): toolbarbutton; }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Used in Gecko (Firefox) to display an element using a platform-native styling based on the operating system's theme.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-backface-visibility"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#backface-visibility")),
                syntax : Some(Borrowed("div { $(name): hidden; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Determines whether or not the 'back' side of a transformed element is visible when facing the viewer. With an identity transform, the front side of an element faces the viewer.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-background-clip"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF1-3.6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-clip")),
                syntax : Some(Borrowed("header { $(name): border-box; }")), restriction :
                Some(Borrowed("box, enum")), }, desc :
                Some(Borrowed("Determines the background painting area.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-background-inline-policy"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-background-inline-policy")),
                syntax : Some(Borrowed("div { $(name): bounding-box; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("In Gecko-based applications like Firefox, the -moz-background-inline-policy CSS property specifies how the background image of an inline element is determined when the content of the inline element wraps onto multiple lines. The choice of position has significant effects on repetition.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-background-origin"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-origin")),
                syntax : Some(Borrowed("header { $(name): border-box; }")), restriction :
                Some(Borrowed("box")), }, desc :
                Some(Borrowed("For elements rendered as a single box, specifies the background positioning area. For elements rendered as multiple boxes (e.g., inline boxes on several lines, boxes on several pages) specifies which boxes 'box-decoration-break' operates on to determine the background positioning area(s).")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-border-bottom-colors"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-border-left-colors")),
                syntax : Some(Borrowed("td { $(name):  #00ff33 #33ff66 #66ff99; }")),
                restriction : Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets a list of colors for the bottom border.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-border-image"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF3.6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-image")),
                syntax : Some(Borrowed("td { $(name): url(border.png) 30 30 round;}")),
                restriction : Some(Borrowed("length, percentage, number, url, enum")), },
                desc :
                Some(Borrowed("Shorthand property for setting 'border-image-source', 'border-image-slice', 'border-image-width', 'border-image-outset' and 'border-image-repeat'. Omitted values are set to their initial values.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-border-left-colors"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-border-left-colors")),
                syntax : Some(Borrowed("td { $(name):  #00ff33 #33ff66 #66ff99; }")),
                restriction : Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets a list of colors for the bottom border.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-border-right-colors"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-border-left-colors")),
                syntax : Some(Borrowed("td { $(name):  #00ff33 #33ff66 #66ff99; }")),
                restriction : Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets a list of colors for the bottom border.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-border-top-colors"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-border-left-colors")),
                syntax : Some(Borrowed("td { $(name):  #00ff33 #33ff66 #66ff99; }")),
                restriction : Some(Borrowed("color")), }, desc :
                Some(Borrowed("Ske Firefox, -moz-border-bottom-colors sets a list of colors for the bottom border.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-box-align"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-box-align")),
                syntax : Some(Borrowed("div { $(name): end; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how a XUL box aligns its contents across (perpendicular to) the direction of its layout. The effect of this is only visible if there is extra space in the box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-box-direction"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-box-direction")),
                syntax : Some(Borrowed("div { $(name): reverse; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether a box lays out its contents normally (from the top or left edge), or in reverse (from the bottom or right edge).")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-box-flex"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-box-flex")),
                syntax : Some(Borrowed("div { $(name): 1; }")), restriction :
                Some(Borrowed("number")), }, desc :
                Some(Borrowed("Specifies how a box grows to fill the box that contains it, in the direction of the containing box's layout.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-box-flexgroup"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-box-flexgroup")),
                syntax : Some(Borrowed("div { $(name): 3; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Flexible elements can be assigned to flex groups using the 'box-flex-group' property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-box-ordinal-group"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-box-ordinal-group")),
                syntax : Some(Borrowed("div { $(name): 5; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Indicates the ordinal group the element belongs to. Elements with a lower ordinal group are displayed before those with a higher ordinal group.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-box-orient"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-box-orient")),
                syntax : Some(Borrowed("div { $(name): vertical; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("In Mozilla applications, -moz-box-orient specifies whether a box lays out its contents horizontally or vertically.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-box-pack"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-box-pack")),
                syntax : Some(Borrowed("div { $(name): end; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how a box packs its contents in the direction of its layout. The effect of this is only visible if there is extra space in the box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-box-sizing"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#box-sizing")), syntax :
                Some(Borrowed("div { $(name): content-box; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Box Model addition in CSS3.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("-moz-column-count"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("FF3.5")), ref_
                : Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-count")),
                syntax : Some(Borrowed("div { $(name): 3; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Describes the optimal number of columns into which the content of the element will be flowed.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-column-gap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF3.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-gap0")), syntax
                : Some(Borrowed("div { $(name): 10px; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Sets the gap between columns. If there is a column rule between columns, it will appear in the middle of the gap.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-column-rule"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF3.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule0")),
                syntax : Some(Borrowed("header { $(name): 5px solid red;}")), restriction
                : Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("Shorthand for setting 'column-rule-width', 'column-rule-style', and 'column-rule-color' at the same place in the style sheet. Omitted values are set to their initial values.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-column-rule-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF3.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule-color")),
                syntax : Some(Borrowed("div { $(name): #ff0; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets the color of the column rule")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-moz-column-rule-style"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF3.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule-style")),
                syntax : Some(Borrowed("div { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Sets the style of the rule between columns of an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-column-rule-width"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF3.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule-width")),
                syntax : Some(Borrowed("div { $(name): 3px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Sets the width of the rule between columns. Negative values are not allowed.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-columns"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#columns0")), syntax :
                Some(Borrowed("div { $(name): 100px 3; }")), restriction :
                Some(Borrowed("length, integer")), }, desc :
                Some(Borrowed("A shorthand property which sets both 'column-width' and 'column-count'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-column-width"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF3.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-width")),
                syntax : Some(Borrowed("div { $(name): 100px; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("This property describes the width of columns in multicol elements.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-font-feature-settings"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#propdef-font-feature-settings")),
                syntax : Some(Borrowed("body { $(name): 'hwid'; }")), restriction :
                Some(Borrowed("string, integer")), }, desc :
                Some(Borrowed("Provides low-level control over OpenType font features. It is intended as a way of providing access to font features that are not widely used but are needed for a particular use case.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-hyphens"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#hyphens0")), syntax :
                Some(Borrowed("div { $(name): manual; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls whether hyphenation is allowed to create more break opportunities within a line of text.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-perspective"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#perspective")),
                syntax : Some(Borrowed("div { $(name): none; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Applies the same transform as the perspective(<number>) transform function, except that it applies only to the positioned or transformed children of the element, not to the transform on the element itself.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-perspective-origin"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#perspective-origin")),
                syntax : Some(Borrowed("div { $(name): 10px; }")), restriction :
                Some(Borrowed("position, percentage, length")), }, desc :
                Some(Borrowed("Establishes the origin for the perspective property. It effectively sets the X and Y position at which the viewer appears to be looking at the children of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-text-align-last"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-align-last0")),
                syntax : Some(Borrowed("div { $(name): right; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes how the last line of a block or a line right before a forced line break is aligned when 'text-align' is set to 'justify'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-text-decoration-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-decoration-color")),
                syntax : Some(Borrowed("div { $(name): #ff0; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Specifies the color of text decoration (underlines overlines, and line-throughs) set on the element with text-decoration-line.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-text-decoration-line"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-decoration-line")),
                syntax : Some(Borrowed("div { $(name): underline; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies what line decorations, if any, are added to the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-text-decoration-style"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-decoration-style")),
                syntax : Some(Borrowed("div { $(name): solid; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the line style for underline, line-through and overline text decoration.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-text-size-adjust"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF")), ref_ :
                Some(Borrowed("http://dev.w3.org/csswg/css-size-adjust/")), syntax :
                Some(Borrowed("body { $(name): 150%; }")), restriction :
                Some(Borrowed("enum, percentage")), }, desc :
                Some(Borrowed("Specifies a size adjustment for displaying text content in mobile browsers.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-transform"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF3.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-2d-transforms/#transform-property")),
                syntax : Some(Borrowed("div { $(name): rotate(-90deg); }")), restriction
                : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("A two-dimensional transformation is applied to an element through the 'transform' property. This property contains a list of transform functions similar to those allowed by SVG.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-transform-origin"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF3.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-2d-transforms/#transform-origin")),
                syntax : Some(Borrowed(".album { $(name): 20% 40%; }")), restriction :
                Some(Borrowed("position, length, percentage")), }, desc :
                Some(Borrowed("Establishes the origin of transformation for an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-transition"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition")),
                syntax : Some(Borrowed("div { $(name): background-color linear 1s; }")),
                restriction : Some(Borrowed("time, property, timing-function, enum")), },
                desc :
                Some(Borrowed("Shorthand property combines four of the transition properties into a single property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-transition-delay"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-delay")),
                syntax : Some(Borrowed("div { $(name): 1s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines when the transition will start. It allows a transition to begin execution some period of time from when it is applied.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-transition-duration"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-duration")),
                syntax : Some(Borrowed("div { $(name): 1s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Specifies how long the transition from the old value to the new value should take.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-transition-property"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-property")),
                syntax : Some(Borrowed("div { $(name): background-color; }")),
                restriction : Some(Borrowed("property")), }, desc :
                Some(Borrowed("Specifies the name of the CSS property to which the transition is applied.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-transition-timing-function"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("FF4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-timing-function")),
                syntax : Some(Borrowed("div { $(name): linear; }")), restriction :
                Some(Borrowed("timing-function")), }, desc :
                Some(Borrowed("Describes how the intermediate values used during a transition will be calculated.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-user-focus"), version : Some(Borrowed("")), browsers :
                Some(Borrowed("FF1.5")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en-US/docs/CSS/-moz-user-focus")),
                syntax : Some(Borrowed("div { $(name): ignore; }")), restriction :
                Some(Borrowed("")), }, desc :
                Some(Borrowed("Used to indicate whether the element can have focus.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-moz-user-select"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF1.5")), ref_ :
                Some(Borrowed("https://developer.mozilla.org/en/CSS/-moz-user-select")),
                syntax : Some(Borrowed("div { $(name): text; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls the appearance of selection.")), }, PropertyEntry
                { attributes : PropertyAttributes { name : Borrowed("-ms-accelerator"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("E,IE10")),
                ref_ : Some(Borrowed("http://www.css3.com/css-accelerator/")), syntax :
                Some(Borrowed("u { $(name): true; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("IE only. Has the ability to turn off its system underlines for accelerator keys until the ALT key is pressed")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-behavior"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/ie/gg192966.aspx")),
                syntax :
                Some(Borrowed("div { $(name): url(http://example.com/png_fix.htc); }")),
                restriction : Some(Borrowed("url")), }, desc :
                Some(Borrowed("IE only. Used to extend behaviors of the browser")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-block-progression"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/2003/CR-css3-text-20030514/#Progression")),
                syntax : Some(Borrowed("div { $(name): bt; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Sets the block-progression value and the flow orientation")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-content-zoom-chaining"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh441243.aspx")),
                syntax : Some(Borrowed("div { $(name): chained; }")), restriction :
                Some(Borrowed("")), }, desc :
                Some(Borrowed("Specifies the zoom behavior that occurs when a user hits the zoom limit during a manipulation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-content-zooming"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh441251.aspx")),
                syntax : Some(Borrowed("div { $(name): zoom; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether zooming is enabled.")), }, PropertyEntry
                { attributes : PropertyAttributes { name :
                Borrowed("-ms-content-zoom-limit"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh996912.aspx")),
                syntax : Some(Borrowed("div { $(name): 10%; }")), restriction :
                Some(Borrowed("percentage")), }, desc :
                Some(Borrowed("Shorthand property for the -ms-content-zoom-limit-min and -ms-content-zoom-limit-max properties.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-content-zoom-limit-max"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh996913.aspx")),
                syntax : Some(Borrowed("div { $(name): 10%; }")), restriction :
                Some(Borrowed("percentage")), }, desc :
                Some(Borrowed("Specifies the maximum zoom factor.")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-ms-content-zoom-limit-min"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh996914.aspx")),
                syntax : Some(Borrowed("div { $(name): 10%; }")), restriction :
                Some(Borrowed("percentage")), }, desc :
                Some(Borrowed("Specifies the minimum zoom factor.")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-ms-content-zoom-snap"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh441255.aspx")),
                syntax : Some(Borrowed("header { $(name): proximity; }")), restriction :
                Some(Borrowed("")), }, desc :
                Some(Borrowed("Shorthand property for the -ms-content-zoom-snap-type and -ms-content-zoom-snap-points properties.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-content-zoom-snap-points"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh441259.aspx")),
                syntax : None, restriction : Some(Borrowed("")), }, desc :
                Some(Borrowed("Defines where zoom snap-points are located.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-content-zoom-snap-type"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh441264.aspx")),
                syntax : Some(Borrowed("header { $(name): proximity; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how zooming is affected by defined snap-points.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-filter"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE8-9")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/ie/gg192966.aspx")),
                syntax : Some(Borrowed("div { $(name): 'alpha(opacity = 50)'; }")),
                restriction : Some(Borrowed("string")), }, desc :
                Some(Borrowed("IE only. Used to produce visual effects.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flex"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex")), syntax :
                Some(Borrowed("p { $(name): 1 auto; }")), restriction :
                Some(Borrowed("length, number, percentage")), }, desc :
                Some(Borrowed("specifies the parameters of a flexible length: the positive and negative flexibility, and the preferred size.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flex-align"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-align0")), syntax
                : Some(Borrowed("div { $(name): center; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Aligns flex items along the cross axis of the current line of the flex container.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flex-direction"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-direction0")),
                syntax : Some(Borrowed("div { $(name): column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how flex items are placed in the flex container, by setting the direction of the flex container's main axis.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flex-flow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-flow")), syntax :
                Some(Borrowed("div { $(name): column wrap; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how flexbox items are placed in the flexbox.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flex-item-align"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-item-align")),
                syntax : Some(Borrowed("div { $(name): center; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Allows the default alignment along the cross axis to be overridden for individual flex items.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flex-line-pack"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-line-pack0")),
                syntax : Some(Borrowed("div { $(name): justify; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Aligns a flex container's lines within the flex container when there is extra space in the cross-axis, similar to how 'justify-content' aligns individual items within the main-axis.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flex-order"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-order0")), syntax
                : Some(Borrowed("p { $(name): 1; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Controls the order in which children of a flex container appear within the flex container, by assigning them to ordinal groups.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flex-pack"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-pack0")), syntax :
                Some(Borrowed("div { $(name): justify; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Aligns flex items along the main axis of the current line of the flex container.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flex-wrap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#flex-wrap")), syntax :
                Some(Borrowed("div { $(name): nowrap; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls whether the flex container is single-line or multi-line, and the direction of the cross-axis, which determines the direction new lines are stacked in.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flow-from"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-regions/#flow-from")), syntax :
                Some(Borrowed("div { $(name): identifier; }")), restriction :
                Some(Borrowed("identifier")), }, desc :
                Some(Borrowed("Makes a block container a region and associates it with a named flow.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-flow-into"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-regions/#flow-into")), syntax :
                Some(Borrowed("div { $(name): identifier; }")), restriction :
                Some(Borrowed("identifier")), }, desc :
                Some(Borrowed("Places an element or its contents into a named flow.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-grid-column"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-grid-layout/#grid-column")),
                syntax : Some(Borrowed("#item1 { $(name): start end; }")), restriction :
                Some(Borrowed("integer, string, enum")), }, desc :
                Some(Borrowed("Used to place grid items and explicitly defined grid cells in the Grid.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-grid-column-align"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-grid-layout/#grid-column-align")),
                syntax : Some(Borrowed("article { $(name): center; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Aligns the columns in a grid.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("-ms-grid-columns"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("E,IE10")),
                ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-grid-layout/#grid-columns")),
                syntax : Some(Borrowed("div { $(name): 150px 1fr; }")), restriction :
                Some(Borrowed("")), }, desc :
                Some(Borrowed("Lays out the columns of the grid.")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-ms-grid-column-span"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-grid-layout/#grid-row-span-and-grid-column-span")),
                syntax : Some(Borrowed("#item { $(name): 2; }.")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Specifies the number of columns to span.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-grid-layer"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/2011/WD-css3-grid-layout-20110407/#grid-layer")),
                syntax : Some(Borrowed("div { $(name): 2; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Grid-layer is similar in concept to z-index, but avoids overloading the meaning of the z-index property, which is applicable only to positioned elements.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-grid-row"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-grid-layout/#grid-row")), syntax
                : Some(Borrowed("#item1 { $(name): start end; }")), restriction :
                Some(Borrowed("integer, string, enum")), }, desc :
                Some(Borrowed("grid-row is used to place grid items and explicitly defined grid cells in the Grid.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-grid-row-align"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-grid-layout/#grid-row-align")),
                syntax : Some(Borrowed("div { $(name): stretch; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Aligns the rows in a grid.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("-ms-grid-rows"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("E,IE10")),
                ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-grid-layout/#grid-rows")),
                syntax : Some(Borrowed("div { $(name): 50px 1fr 50px; }")), restriction :
                Some(Borrowed("")), }, desc :
                Some(Borrowed("Lays out the columns of the grid.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("-ms-grid-row-span"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("E,IE10")),
                ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-grid-layout/#grid-row-span-and-grid-column-span")),
                syntax : Some(Borrowed("#item { $(name): 2; }.")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Specifies the number of rows to span.")), }, PropertyEntry
                { attributes : PropertyAttributes { name :
                Borrowed("-ms-high-contrast-adjust"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh441137.aspx")),
                syntax : Some(Borrowed("section { $(name): auto; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies if properties should be adjusted in high contrast mode.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-hyphenate-limit-chars"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-hyphenate-limit-chars")),
                syntax : Some(Borrowed("div { $(name): 5 2 2; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Specifies the minimum number of characters in a hyphenated word.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-hyphenate-limit-lines"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-hyphenate-limit-lines")),
                syntax : Some(Borrowed("div { $(name): 2; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Indicates the maximum number of successive hyphenated lines in an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-hyphenate-limit-zone"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-hyphenate-limit-zone")),
                syntax : Some(Borrowed("div { $(name): 25%; }")), restriction :
                Some(Borrowed("percentage, length")), }, desc :
                Some(Borrowed("Specifies the maximum amount of unfilled space (before justification) that may be left in the line box before hyphenation is triggered to pull part of a word from the next line back up into the current line.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-hyphens"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#hyphens0")), syntax :
                Some(Borrowed("div { $(name): manual; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls whether hyphenation is allowed to create more break opportunities within a line of text.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-ime-mode"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#ime-mode")), syntax :
                Some(Borrowed("body { $(name): active; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls the state of the input method editor for text fields.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-interpolation-mode"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE7")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ie/ms530822(v=vs.85).aspx")),
                syntax : Some(Borrowed("img.highqual { $(name): bicubic; }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets the interpolation (resampling) method used to stretch images.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-layout-grid"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ie/ms530771(v=vs.85).aspx")),
                syntax : Some(Borrowed("div { $(name): both fixed 12px 12px}")),
                restriction : Some(Borrowed("")), }, desc :
                Some(Borrowed("Sets or retrieves the composite document grid properties that specify the layout of text characters.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-layout-grid-char"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ie/ms530772(v=vs.85).aspx")),
                syntax : Some(Borrowed("div { $(name): auto; }")), restriction :
                Some(Borrowed("enum, length, percentage")), }, desc :
                Some(Borrowed("Sets or retrieves the size of the character grid used for rendering the text content of an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-layout-grid-line"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ie/ms530773(v=vs.85).aspx")),
                syntax : Some(Borrowed("div { $(name): auto; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Sets or retrieves the gridline value used for rendering the text content of an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-layout-grid-mode"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ie/ms530774(v=vs.85).aspx")),
                syntax : Some(Borrowed("div { $(name): line; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets whether the text layout grid uses two dimensions.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-layout-grid-type"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ie/ms530775(v=vs.85).aspx")),
                syntax : Some(Borrowed("div { $(name): strict; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Sets or retrieves the type of grid used for rendering the text content of an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-line-break"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#line-break0")), syntax :
                Some(Borrowed("p { $(name): strict; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies what set of line breaking restrictions are in effect within the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-overflow-style"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh441298.aspx")),
                syntax : Some(Borrowed("p { $(name): scrollbar; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specify whether content is clipped when it overflows the element's content area.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-perspective"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#perspective")),
                syntax : Some(Borrowed("div { $(name): none; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Applies the same transform as the perspective(<number>) transform function, except that it applies only to the positioned or transformed children of the element, not to the transform on the element itself.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-perspective-origin"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#perspective-origin")),
                syntax : Some(Borrowed("div { $(name): 10px; }")), restriction :
                Some(Borrowed("position, percentage, length")), }, desc :
                Some(Borrowed("Establishes the origin for the perspective property. It effectively sets the X and Y position at which the viewer appears to be looking at the children of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-perspective-origin-x"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#perspective-origin")),
                syntax : None, restriction :
                Some(Borrowed("position, percentage, length")), }, desc :
                Some(Borrowed("Establishes the origin for the perspective property. It effectively sets the X  position at which the viewer appears to be looking at the children of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-perspective-origin-y"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#perspective-origin")),
                syntax : None, restriction :
                Some(Borrowed("position, percentage, length")), }, desc :
                Some(Borrowed("Establishes the origin for the perspective property. It effectively sets the Y position at which the viewer appears to be looking at the children of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-progress-appearance"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/Hh779845.aspx")),
                syntax : Some(Borrowed("progress { $(name): bar; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets a value that specifies whether a progress control displays as a bar or a ring.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scrollbar-3dlight-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531153(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the top and left edges of the scroll box and scroll arrows of a scroll bar.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scrollbar-arrow-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531154(VS.85).aspx")),
                syntax : Some(Borrowed("body { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the arrow elements of a scroll arrow.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scrollbar-base-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531155(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the main elements of a scroll bar, which include the scroll box, track, and scroll arrows.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scrollbar-darkshadow-color"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531156(v=VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the gutter of a scroll bar.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scrollbar-face-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531157(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the scroll box and scroll arrows of a scroll bar.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scrollbar-highlight-color"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531158(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the top and left edges of the scroll box and scroll arrows of a scroll bar.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scrollbar-shadow-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531159(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the bottom and right edges of the scroll box and scroll arrows of a scroll bar.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scrollbar-track-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531160(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the track element of a scroll bar.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-chaining"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh466007.aspx")),
                syntax : Some(Borrowed("div { $(name): chained; }")), restriction :
                Some(Borrowed("enum, length")), }, desc :
                Some(Borrowed("Gets or sets a value that indicates the scrolling behavior that occurs when a user hits the content boundary during a manipulation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-limit"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh996918.aspx")),
                syntax : None, restriction : Some(Borrowed("length")), }, desc :
                Some(Borrowed("Gets or sets a shorthand value that sets values for the -ms-scroll-limit-x-min, -ms-scroll-limit-y-min, -ms-scroll-limit-x-max, and -ms-scroll-limit-y-max properties.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-limit-x-max"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh996919.aspx")),
                syntax : Some(Borrowed("div { $(name): auto; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Gets or sets a value that specifies the maximum value for the scrollLeft property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-limit-x-min"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh996920.aspx")),
                syntax : Some(Borrowed("div { $(name): 5px; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Gets or sets a value that specifies the minimum value for the scrollLeft property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-limit-y-max"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh996921.aspx")),
                syntax : Some(Borrowed("div { $(name): auto; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Gets or sets a value that specifies the maximum value for the scrollTop property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-limit-y-min"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh996922.aspx")),
                syntax : Some(Borrowed("div { $(name): 5px; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Gets or sets a value that specifies the minimum value for the scrollTop property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-rails"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh466018.aspx")),
                syntax : Some(Borrowed("div { $(name): railed; }")), restriction :
                Some(Borrowed("enum, length")), }, desc :
                Some(Borrowed("Gets or sets a value that indicates whether or not small motions perpendicular to the primary axis of motion will result in either changes to both the scrollTop and scrollLeft properties or a change to the primary axis (for instance, either the scrollTop or scrollLeft properties will change, but not both).")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-snap-points-x"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh466031.aspx")),
                syntax : Some(Borrowed("div { $(name): snapInterval(100%, 100%); }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets a value that defines where snap-points will be located along the x-axis.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-snap-points-y"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh466042.aspx")),
                syntax : Some(Borrowed("div { $(name): snapInterval(100%, 100%); }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets a value that defines where snap-points will be located along the y-axis.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-snap-type"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh466057.aspx")),
                syntax : Some(Borrowed("div { $(name): proximity; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets a value that defines what type of snap-point should be used for the current element. There are two type of snap-points, with the primary difference being whether or not the user is guaranteed to always stop on a snap-point.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-snap-x"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh466066.aspx")),
                syntax :
                Some(Borrowed("div { $(name): proximity snapInterval(100%, 100%); }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets a shorthand value that sets values for the -ms-scroll-snap-type and -ms-scroll-snap-points-x properties.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-snap-y"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh466078.aspx")),
                syntax :
                Some(Borrowed("div { $(name): proximity snapInterval(100%, 100%); }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets a shorthand value that sets values for the -ms-scroll-snap-type and -ms-scroll-snap-points-y properties.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-scroll-translation"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh996917.aspx")),
                syntax : Some(Borrowed("div { $(name): vertical-to-horizontal; }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets a value that specifies whether vertical-to-horizontal scroll wheel translation occurs on the specified element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-text-align-last"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE8")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-align-last0")),
                syntax : Some(Borrowed("div { $(name): right; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes how the last line of a block or a line right before a forced line break is aligned when 'text-align' is set to 'justify'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-text-autospace"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,IE8")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-autospace")), syntax
                : Some(Borrowed("div { $(name): ideograph-numeric; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Determines whether or not a full-width punctuation mark character should be trimmed if it appears at the beginning of a line, so that its 'ink' lines up with the first glyph in the line above and below.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-text-combine-horizontal"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE11")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-writing-modes-3/#text-combine-upright")),
                syntax : Some(Borrowed("span { $(name): all; }")), restriction :
                Some(Borrowed("enum, integer")), }, desc :
                Some(Borrowed("This property specifies the combination of multiple characters into the space of a single character.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-text-justify"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE8")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-justify0")), syntax :
                Some(Borrowed("div { $(name): inter-word; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Selects the justification algorithm used when 'text-align' is set to 'justify'. The property applies to block containers, but the UA may (but is not required to) also support it on inline elements.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-text-kashida-space"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh453798.aspx")),
                syntax : Some(Borrowed("article { $(name): 10%; }")), restriction :
                Some(Borrowed("percentage")), }, desc :
                Some(Borrowed("Sets or retrieves the ratio of kashida expansion to white space expansion when justifying lines of text in the object.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-text-overflow"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#text-overflow0")), syntax :
                Some(Borrowed("span { $(name): ellipsis; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Text can overflow for example when it is prevented from wrapping")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-text-size-adjust"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://dev.w3.org/csswg/css-size-adjust/")), syntax :
                Some(Borrowed("body { $(name): 150%; }")), restriction :
                Some(Borrowed("enum, percentage")), }, desc :
                Some(Borrowed("Specifies a size adjustment for displaying text content in mobile browsers.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-text-underline-position"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-underline-position0")),
                syntax : Some(Borrowed("div { $(name): auto; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Sets the position of an underline specified on the same element: it does not affect underlines specified by ancestor elements.This property is typically used in vertical writing contexts such as in Japanese documents where it often desired to have the underline appear 'over' (to the right of) the affected run of text")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-touch-action"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/Hh767313.aspx")),
                syntax :
                Some(Borrowed("div { $(name): manipulation double-tap-zoom; }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets a value that indicates whether and how a given region can be manipulated by the user.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-touch-select"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ie/hh975292(v=vs.85).aspx")),
                syntax : Some(Borrowed("div::selection { $(name): grippers; }")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets a value that toggles the 'gripper' visual elements that enable touch text selection.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-transform"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE9-9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-2d-transforms/#transform-property")),
                syntax : Some(Borrowed("div { $(name): rotate(-90deg); }")), restriction
                : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("A two-dimensional transformation is applied to an element through the 'transform' property. This property contains a list of transform functions similar to those allowed by SVG.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-transform-origin"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE9-9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-2d-transforms/#transform-origin")),
                syntax : Some(Borrowed(".album { $(name): 20% 40%; }")), restriction :
                Some(Borrowed("position, length, percentage")), }, desc :
                Some(Borrowed("Establishes the origin of transformation for an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-transform-origin-x"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#transform-origin")),
                syntax : None, restriction : Some(Borrowed("length, percentage")), },
                desc :
                Some(Borrowed("The x coordinate of the origin for transforms applied to an element with respect to its border box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-transform-origin-y"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#transform-origin")),
                syntax : None, restriction : Some(Borrowed("length, percentage")), },
                desc :
                Some(Borrowed("The y coordinate of the origin for transforms applied to an element with respect to its border box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-transform-origin-z"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#transform-origin")),
                syntax : None, restriction : Some(Borrowed("length, percentage")), },
                desc :
                Some(Borrowed("The z coordinate of the origin for transforms applied to an element with respect to its border box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-user-select"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://ie.microsoft.com/testdrive/HTML5/msUserSelect/")),
                syntax : Some(Borrowed("div { $(name): none; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls the appearance of selection.")), }, PropertyEntry
                { attributes : PropertyAttributes { name : Borrowed("-ms-word-break"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#word-break0")), syntax :
                Some(Borrowed("p.album { $(name): break-all; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies line break opportunities for non-CJK scripts.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-word-wrap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#word-wrap0")), syntax :
                Some(Borrowed("p { $(name): break-word; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether the UA may break within a word to prevent overflow when an otherwise-unbreakable string is too long to fit.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-wrap-flow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-exclusions/#wrap-flow-property")),
                syntax : Some(Borrowed("div { $(name): maximum; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("An element becomes an exclusion when its 'wrap-flow' property has a computed value other than 'auto'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-wrap-margin"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh466103.aspx")),
                syntax : Some(Borrowed("div { $(name): 20px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Gets or sets a value that is used to offset the inner wrap shape from other shapes.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-wrap-through"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-exclusions/#propdef-wrap-through")),
                syntax : Some(Borrowed("div { $(name): wrap; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies if an element inherits its parent wrapping context. In other words if it is subject to the exclusions defined outside the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-writing-mode"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/2003/CR-css3-text-20030514/#writing-mode")),
                syntax : Some(Borrowed("span { $(name): lr-tb; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Shorthand property for both 'direction' and 'block-progression'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-zoom"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("IE8")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/ie/gg192966.aspx")),
                syntax : Some(Borrowed(".example { $(name): 1; }")), restriction :
                Some(Borrowed("enum, integer, number, percentage")), }, desc :
                Some(Borrowed("Sets or retrieves the magnification scale of the object.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-ms-zoom-animation"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("IE10")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/windows/apps/hh466117.aspx")),
                syntax : Some(Borrowed("div { $(name): none; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Gets or sets a value that indicates whether an animation is used when zooming.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("nav-down"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O9.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#nav-dir")), syntax :
                Some(Borrowed("auto | 'id' [ current | root | 'target-name' ]?")),
                restriction : Some(Borrowed("enum, identifier, string")), }, desc :
                Some(Borrowed("Provides an way to control directional focus navigation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("nav-index"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O9.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#nav-index0")), syntax :
                Some(Borrowed("auto | 'number'")), restriction :
                Some(Borrowed("number")), }, desc :
                Some(Borrowed("Provides an input-method-neutral way of specifying the sequential navigation order (also known as 'tabbing order').")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("nav-left"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O9.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#nav-dir")), syntax :
                Some(Borrowed("auto | 'id' [ current | root | 'target-name' ]?")),
                restriction : Some(Borrowed("enum, identifier, string")), }, desc :
                Some(Borrowed("Provides an way to control directional focus navigation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("nav-right"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O9.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#nav-dir")), syntax :
                Some(Borrowed("auto | 'id' [ current | root | 'target-name' ]?")),
                restriction : Some(Borrowed("enum, identifier, string")), }, desc :
                Some(Borrowed("Provides an way to control directional focus navigation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("nav-up"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O9.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#nav-dir")), syntax :
                Some(Borrowed("auto | 'id' [ current | root | 'target-name' ]?")),
                restriction : Some(Borrowed("enum, identifier, string")), }, desc :
                Some(Borrowed("Provides an way to control directional focus navigation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("negative"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-counter-styles-3/#descdef-counter-style-negative")),
                syntax : Some(Borrowed("@counter-style { negative: '(' ')'; }")),
                restriction : Some(Borrowed("image, identifier, string")), }, desc :
                Some(Borrowed("@counter-style descriptor. Defines how to alter the representation when the counter value is negative.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-animation"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation")), syntax
                : Some(Borrowed("div { $(name): movearound 4s ease 3 normal; }")),
                restriction :
                Some(Borrowed("time, enum, timing-function, identifier, number")), },
                desc :
                Some(Borrowed("Shorthand property combines six of the animation properties into a single property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-animation-delay"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("O12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-delay")),
                syntax : Some(Borrowed("div { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines when the animation will start.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-animation-direction"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("O12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-direction")),
                syntax : Some(Borrowed("div { $(name): normal; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether or not the animation should play in reverse on alternate cycles.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-animation-duration"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("O12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-duration")),
                syntax : Some(Borrowed("div { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines the length of time that an animation takes to complete one cycle.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-animation-fill-mode"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("O12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-fill-mode-property")),
                syntax : Some(Borrowed("div { $(name): forwards; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines what values are applied by the animation outside the time it is executing.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-animation-iteration-count"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("O12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-iteration-count")),
                syntax : Some(Borrowed("div { $(name): 3; }")), restriction :
                Some(Borrowed("number, enum")), }, desc :
                Some(Borrowed("Defines the number of times an animation cycle is played. The default value is one, meaning the animation will play from beginning to end once.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-animation-name"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("O12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#the-animation-name-property-")),
                syntax : Some(Borrowed("div { $(name): movearound; }")), restriction :
                Some(Borrowed("identifier, enum")), }, desc :
                Some(Borrowed("Defines a list of animations that apply. Each name is used to select the keyframe at-rule that provides the property values for the animation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-animation-play-state"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("O12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-play-state")),
                syntax : Some(Borrowed("div { $(name): running; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether the animation is running or paused.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-animation-timing-function"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("O12")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-timing-function")),
                syntax : Some(Borrowed("div { $(name): ease; }")), restriction :
                Some(Borrowed("timing-function")), }, desc :
                Some(Borrowed("Describes how the animation will progress over one cycle of its duration. See the 'transition-timing-function'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("object-fit"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C32,FF36,O19,S7.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-images/#object-fit")), syntax :
                Some(Borrowed("p { $(name): cover; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how the contents of a replaced element should be scaled relative to the box established by its used height and width.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("object-position"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C32,FF36,O19")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-images/#object-position")),
                syntax : Some(Borrowed("img { $(name): left top; }")), restriction :
                Some(Borrowed("position, length, percentage")), }, desc :
                Some(Borrowed("Determines the alignment of the replaced element inside its box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-border-image"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O11.6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-image")),
                syntax : Some(Borrowed("div { $(name): url(border.png) 0 5 0 5;}")),
                restriction : Some(Borrowed("length, percentage, number, image, enum")),
                }, desc :
                Some(Borrowed("Shorthand property for setting 'border-image-source', 'border-image-slice', 'border-image-width', 'border-image-outset' and 'border-image-repeat'. Omitted values are set to their initial values.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-object-fit"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O10.6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css4-images/#object-fit")), syntax :
                Some(Borrowed("p { $(name): cover; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how the contents of a replaced element should be scaled relative to the box established by its used height and width.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-object-position"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("O10.6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css4-images/#object-position")),
                syntax : Some(Borrowed("img { $(name): left top; }")), restriction :
                Some(Borrowed("position, length, percentage")), }, desc :
                Some(Borrowed("Determines the alignment of the replaced element inside its box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("opacity"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,FF3.6,IE9,O9,S1.2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-color/#opacity")), syntax :
                Some(Borrowed("article { $(name): opacity: 0.4; }")), restriction :
                Some(Borrowed("number(0-1)")), }, desc :
                Some(Borrowed("Opacity of an element's text, where 1 is opaque and 0 is entirely transparent.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("order"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C29,FF22,IE11,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-flexbox/#order")), syntax :
                Some(Borrowed("p { $(name): -1; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Controls the order in which children of a flex container appear within the flex container, by assigning them to ordinal groups.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("orphans"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("C,IE8,O7,S1.3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-break/#widows-orphans")), syntax
                : Some(Borrowed("<integer>")), restriction : Some(Borrowed("integer")),
                }, desc :
                Some(Borrowed("Specifies the minimum number of line boxes in a block container that must be left in a fragment before a fragmentation break.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-table-baseline"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("O9.6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/mathml-for-css/")), syntax :
                Some(Borrowed("td { $(name): 2; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Determines which row of a inline-table should be used as baseline of inline-table.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-tab-size"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O10.6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#tab-size")), syntax :
                Some(Borrowed("div { $(name): 4; }")), restriction :
                Some(Borrowed("integer, length")), }, desc :
                Some(Borrowed("This property determines the width of the tab character (U+0009), in space characters (U+0020), when rendered.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-text-overflow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#text-overflow0")), syntax :
                Some(Borrowed("span { $(name): ellipsis; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Text can overflow for example when it is prevented from wrapping")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-transform"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O10.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-2d-transforms/#transform-property")),
                syntax : Some(Borrowed("div { $(name): rotate(-90deg); }")), restriction
                : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("A two-dimensional transformation is applied to an element through the 'transform' property. This property contains a list of transform functions similar to those allowed by SVG.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-transform-origin"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("O10.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-2d-transforms/#transform-origin")),
                syntax : Some(Borrowed("div { $(name): 20% 40%; }")), restriction :
                Some(Borrowed("positon, length, percentage")), }, desc :
                Some(Borrowed("Establishes the origin of transformation for an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-transition"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("O11.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition")),
                syntax : Some(Borrowed("div { $(name): background-color linear 1s; }")),
                restriction : Some(Borrowed("time, property, timing-function, enum")), },
                desc :
                Some(Borrowed("Shorthand property combines four of the transition properties into a single property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-transition-delay"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("O11.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-delay")),
                syntax : Some(Borrowed("div { $(name): 1s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines when the transition will start. It allows a transition to begin execution some period of time from when it is applied.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-transition-duration"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("O11.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-duration")),
                syntax : Some(Borrowed("div { $(name): 1s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Specifies how long the transition from the old value to the new value should take.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-transition-property"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("O11.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-property")),
                syntax : Some(Borrowed("div { $(name): background-color; }")),
                restriction : Some(Borrowed("property")), }, desc :
                Some(Borrowed("Specifies the name of the CSS property to which the transition is applied.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-o-transition-timing-function"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("O11.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-timing-function")),
                syntax : Some(Borrowed("div { $(name): linear; }")), restriction :
                Some(Borrowed("timing-function")), }, desc :
                Some(Borrowed("Describes how the intermediate values used during a transition will be calculated.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("offset-block-end"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#logical-prop")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'bottom'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("offset-block-start"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#logical-prop")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'top'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("offset-inline-end"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#logical-prop")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'right'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("offset-inline-start"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#logical-prop")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'left'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("orientation"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-device-adapt/#orientation-desc")),
                syntax : Some(Borrowed("@viewport { $(name): landscape; }")), restriction
                : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("@viewport descriptor. Used to request that a document is displayed in portrait or landscape mode.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("outline"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE8,O8,S1.2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#outline0")), syntax :
                Some(Borrowed("header { $(name): 5px solid red;}")), restriction :
                Some(Borrowed("length, line-width, line-style, color, enum")), }, desc :
                Some(Borrowed("Shorthand property for 'outline-style', 'outline-width', and 'outline-color'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("outline-color"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE8,O8,S1.2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#outline-color")), syntax :
                Some(Borrowed("body { $(name): red; }")), restriction :
                Some(Borrowed("enum, color")), }, desc :
                Some(Borrowed("The color of the outline.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("outline-offset"),
                version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,FF1.5,O9.5,S1.2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#outline-offset0")), syntax :
                Some(Borrowed("article { $(name): 15px; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Offset the outline and draw it beyond the border edge.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("outline-style"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE8,O8,S1.2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#outline-style0")), syntax :
                Some(Borrowed("td { $(name): solid; }")), restriction :
                Some(Borrowed("line-style, enum")), }, desc :
                Some(Borrowed("Style of the outline.")), }, PropertyEntry { attributes :
                PropertyAttributes { name : Borrowed("outline-width"), version :
                Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE8,O8,S1.2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#outline-width0")), syntax :
                Some(Borrowed("td { $(name): 2px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Width of the outline.")), }, PropertyEntry { attributes :
                PropertyAttributes { name : Borrowed("overflow"), version :
                Some(Borrowed("2.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-overflow-3/#overflow")), syntax :
                Some(Borrowed("div { overflow: hidden auto; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Shorthand for setting 'overflow-x' and 'overflow-y'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("overflow-wrap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C23,O12.1,S6.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#overflow-wrap0")), syntax
                : Some(Borrowed("div { $(name): break-word; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether the UA may break within a word to prevent overflow when an otherwise-unbreakable string is too long to fit within the line box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("overflow-x"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE5,O9.5,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#overflow-x")), syntax :
                Some(Borrowed("div { $(name): hidden; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the handling of overflow in the horizontal direction.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("overflow-y"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE5,O9.5,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#overflow-x")), syntax :
                Some(Borrowed("div { $(name): hidden; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the handling of overflow in the vertical direction.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("pad"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-counter-styles-3/#descdef-counter-style-pad")),
                syntax : Some(Borrowed("@counter-style { pad: 3 '0'; }")), restriction :
                Some(Borrowed("integer, image, string, identifier")), }, desc :
                Some(Borrowed("@counter-style descriptor. Specifies a \"fixed-width\" counter style, where representations shorter than the pad value are padded with a particular <symbol>")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("padding"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#padding1")), syntax :
                Some(Borrowed("div { $(name): 4px 7px 2px 4px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Shorthand property to set values for the thickness of the padding area. If left is omitted, it is the same as right. If bottom is omitted it is the same as top, if right is omitted it is the same as top. The value may not be negative.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("padding-bottom"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#padding1")), syntax :
                Some(Borrowed("ul { $(name): 2em; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Shorthand property to set values for the thickness of the padding area. If left is omitted, it is the same as right. If bottom is omitted it is the same as top, if right is omitted it is the same as top. The value may not be negative.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("padding-block-end"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'padding-bottom'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("padding-block-start"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'padding-top'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("padding-inline-end"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'padding-right'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("padding-inline-start"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF41")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-logical-props/#border-padding")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Logical 'padding-left'. Mapping depends on the parent element's 'writing-mode', 'direction', and 'text-orientation'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("padding-left"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#padding1")), syntax :
                Some(Borrowed("ul { $(name): 2em; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Shorthand property to set values for the thickness of the padding area. If left is omitted, it is the same as right. If bottom is omitted it is the same as top, if right is omitted it is the same as top. The value may not be negative.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("padding-right"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#padding1")), syntax :
                Some(Borrowed("ul { $(name): 2em; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Shorthand property to set values for the thickness of the padding area. If left is omitted, it is the same as right. If bottom is omitted it is the same as top, if right is omitted it is the same as top. The value may not be negative.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("padding-top"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#padding1")), syntax :
                Some(Borrowed("ul { $(name): 2em; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Shorthand property to set values for the thickness of the padding area. If left is omitted, it is the same as right. If bottom is omitted it is the same as top, if right is omitted it is the same as top. The value may not be negative.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("page"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page")), syntax :
                Some(Borrowed("auto | <identifier>")), restriction :
                Some(Borrowed("identifier")), }, desc :
                Some(Borrowed("Specifies a particular type of page (called a named page) on which an element must be displayed.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("page-break-after"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-break/#page-break-properties")),
                syntax : Some(Borrowed("table { $(name): always; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines rules for page breaks after an element.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("page-break-before"), version : Some(Borrowed("2.0")), browsers
                : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-break/#page-break-properties")),
                syntax : Some(Borrowed("table { $(name): always; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines rules for page breaks before an element.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("page-break-inside"), version : Some(Borrowed("2.0")), browsers
                : Some(Borrowed("C,IE8,O7,S1.3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-break/#page-break-properties")),
                syntax : Some(Borrowed("table { $(name): avoid; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines rules for page breaks inside an element.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("page-policy"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-content/#page-policy")), syntax
                : Some(Borrowed("@string chapter { $(name): last; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Determines which page-based occurrence of a given element is applied to a counter or string value.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("paint-order"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("C35,FF31,O22,S7.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#PaintOrderProperty")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls the order that the three paint operations that shapes and text are rendered with: their fill, their stroke and any markers they might have.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("pause"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#pause")), syntax :
                Some(Borrowed("h2 { $(name): 30ms 40ms; }")), restriction :
                Some(Borrowed("time, enum")), }, desc :
                Some(Borrowed("Shorthand for setting 'pause-before' and 'pause-after'. If two values are given, the first value is 'pause-before' and the second is 'pause-after'. If only one value is given, it applies to both properties.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("pause-after"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#pause-after")), syntax :
                Some(Borrowed("h3 { $(name): 30ms; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Specifies a pause or prosodic boundary to be observed after an element or, if any 'cue-after' is specified, after these. Values indicate the prosodic strength of the break in speech output.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("pause-before"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#pause-before")), syntax
                : Some(Borrowed("h3 { $(name): 30ms; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Specifies a pause or prosodic boundary to be observed before an element or, if any 'cue-before' is specified, before these. Values indicate the prosodic strength of the break in speech output.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("perspective"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C36,FF16,IE10,O23,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#perspective")),
                syntax : Some(Borrowed("div { $(name): none; }")), restriction :
                Some(Borrowed("length, enum")), }, desc :
                Some(Borrowed("Applies the same transform as the perspective(<number>) transform function, except that it applies only to the positioned or transformed children of the element, not to the transform on the element itself.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("perspective-origin"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("E,C36,FF16,IE10,O23,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#perspective-origin")),
                syntax : Some(Borrowed("div { $(name): 10px; }")), restriction :
                Some(Borrowed("position, percentage, length")), }, desc :
                Some(Borrowed("Establishes the origin for the perspective property. It effectively sets the X and Y position at which the viewer appears to be looking at the children of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("pointer-events"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/interact.html#PointerEventsProperty")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies under what circumstances a given element can be the target element for a pointer event.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("position"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-positioning/#propdef-position")),
                syntax : Some(Borrowed("div { $(name): absolute; }")), restriction :
                Some(Borrowed("enum")), }, desc : Some(Borrowed("")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("prefix"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("FF33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-counter-styles-3/#descdef-counter-style-prefix")),
                syntax : Some(Borrowed("@counter-style { prefix: '#'; }")), restriction :
                Some(Borrowed("image, string, identifier")), }, desc :
                Some(Borrowed("@counter-style descriptor. Specifies a <symbol> that is prepended to the marker representation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("quotes"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("E,C,FF1.5,IE8,O8,S5.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-content/#quotes")), syntax :
                Some(Borrowed("none | [ <string> <string> ]+")), restriction :
                Some(Borrowed("string")), }, desc :
                Some(Borrowed("Specifies quotation marks for any number of embedded quotations.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("r"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/geometry.html#R")), syntax :
                Some(Borrowed("circle { $(name): 100px;")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Describes the radius of the 'circle' element.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("range"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-counter-styles-3/#descdef-counter-style-range")),
                syntax :
                Some(Borrowed("@counter-style { range: 2 infinite, 8 834048; }")),
                restriction : Some(Borrowed("integer, enum")), }, desc :
                Some(Borrowed("@counter-style descriptor. Defines the ranges over which the counter style is defined.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("region-fragment"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://dev.w3.org/csswg/css-regions/#region-fragment")),
                syntax : Some(Borrowed("article { $(name): break; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls the behavior of the last region associated with a named flow.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("resize"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,FF4,O15,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#resize0")), syntax :
                Some(Borrowed("div { $(name): both; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether or not an element is resizable by the user, and if so, along which axis/axes.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("rest"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#rest")), syntax :
                Some(Borrowed("h3 { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Shorthand for setting 'rest-before' and 'rest-after'. If two values are given, the first value is 'rest-before' and the second is 'rest-after'. If only one value is given, it applies to both properties.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("rest-after"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#rest-after")), syntax :
                Some(Borrowed("h3 { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Specifies a rest or prosodic boundary to be observed after an element's content.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("rest-before"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#rest-before")), syntax :
                Some(Borrowed("h3 { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Specifies a rest or prosodic boundary to be observed before an element's content.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("right"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-positioning/#propdef-right")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Specifies how far an absolutely positioned box's right margin edge is offset to the left of the right edge of the box's 'containing block'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("rotation"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#rotation")), syntax :
                Some(Borrowed("img { $(name): 90deg; }")), restriction :
                Some(Borrowed("angle")), }, desc :
                Some(Borrowed("Rotates a block-level element counterclockwise around the point given by 'rotation-point'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("rotation-point"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#rotation-point")), syntax :
                Some(Borrowed("img { $(name): 10% 25%; }")), restriction :
                Some(Borrowed("position, percentage, length")), }, desc :
                Some(Borrowed("Pair of values that defines a point as an offset from the top left border edge. Initial value is 50% 50%.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("ruby-align"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF10,IE5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ruby/#rubyalign")), syntax :
                Some(Borrowed("auto | start | left | center | end | right | distribute-letter | distribute-space | line-edge")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies how text is distributed within the various ruby boxes when their contents do not exactly fill their respective boxes.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("ruby-overhang"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF10,IE5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ruby/#rubyover")), syntax :
                Some(Borrowed("auto | start | end | none")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Determines whether, and on which side, ruby text is allowed to partially overhang any adjacent text in addition to its own base, when the ruby text is wider than the ruby base.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("ruby-position"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF10,IE5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ruby/#ruby-position")), syntax :
                Some(Borrowed("before | after | right")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Used by the parent of elements with display: ruby-text to control the position of the ruby text with respect to its base.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("ruby-span"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ruby/#rubyspan")), syntax :
                Some(Borrowed("attr(x) | none")), restriction : Some(Borrowed("enum")),
                }, desc :
                Some(Borrowed("Determines whether, and on which side, ruby text is allowed to partially overhang any adjacent text in addition to its own base, when the ruby text is wider than the ruby base.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("rx"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/geometry.html#RX")), syntax :
                Some(Borrowed("circle { $(name): 100px;")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Describes the horizontal radius of the 'ellipse' element, and the curve radius of the 'rect' element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("ry"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/geometry.html#RY")), syntax :
                Some(Borrowed("circle { $(name): 100px;")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Describes the vertical radius of the 'ellipse' element, and the curve radius of the 'rect' element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scrollbar-3dlight-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE6")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531153(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the top and left edges of the scroll box and scroll arrows of a scroll bar.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scrollbar-arrow-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE6")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531154(VS.85).aspx")),
                syntax : Some(Borrowed("body { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the arrow elements of a scroll arrow.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scrollbar-base-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE6")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531155(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the main elements of a scroll bar, which include the scroll box, track, and scroll arrows.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scrollbar-darkshadow-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE6")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531156(v=VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the gutter of a scroll bar.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scrollbar-face-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE6")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531157(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the scroll box and scroll arrows of a scroll bar.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scrollbar-highlight-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE6")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531158(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the top and left edges of the scroll box and scroll arrows of a scroll bar.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scrollbar-shadow-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE6")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531159(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the bottom and right edges of the scroll box and scroll arrows of a scroll bar.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scrollbar-track-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("IE6")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531160(VS.85).aspx")),
                syntax : Some(Borrowed("textarea { $(name): #00ffff; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Determines the color of the track element of a scroll bar.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scroll-behavior"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF36")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/cssom-view/#scroll-behavior")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the scrolling behavior for a scrolling box, when scrolling happens due to navigation or CSSOM scrolling APIs.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scroll-snap-coordinate"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF39")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-snappoints-1/#propdef-scroll-snap-coordinate")),
                syntax : None, restriction :
                Some(Borrowed("position, length, percentage, enum")), }, desc :
                Some(Borrowed("Defines the x and y coordinate within the element which will align with the nearest ancestor scroll container's snap-destination for the respective axis.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scroll-snap-destination"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF39")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-snappoints-1/#propdef-scroll-snap-destination")),
                syntax : None, restriction :
                Some(Borrowed("position, length, percentage")), }, desc :
                Some(Borrowed("Define the x and y coordinate within the scroll container's visual viewport which element snap points will align with.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scroll-snap-points-x"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF39")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-snappoints-1/#propdef-scroll-snap-points-x")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines the positioning of snap points along the x axis of the scroll container it is applied to.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scroll-snap-points-y"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF39")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-snappoints-1/#propdef-scroll-snap-points-y")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines the positioning of snap points along the y axis of the scroll container it is applied to.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("scroll-snap-type"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF39")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-snappoints-1/#propdef-scroll-snap-type")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines how strictly snap points are enforced on the scroll container.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("shape-image-threshold"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C37,O24")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-shapes-1/#propdef-shape-image-threshold")),
                syntax : Some(Borrowed("div { $(name): 0.5; }")), restriction :
                Some(Borrowed("number")), }, desc :
                Some(Borrowed("Defines the alpha channel threshold used to extract the shape using an image. A value of 0.5 means that the shape will enclose all the pixels that are more than 50% opaque.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("shape-inside"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-shapes-2/#propdef-shape-inside")),
                syntax : Some(Borrowed("div { $(name): outside-shape; }")), restriction :
                Some(Borrowed("image, shape, box, enum")), }, desc :
                Some(Borrowed("Adds one or more exclusion areas to the element's wrapping context.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("shape-margin"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C37,O24")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-shapes-1/#propdef-shape-margin")),
                syntax : Some(Borrowed("div { $(name): 10px; }")), restriction :
                Some(Borrowed("url, length, percentage")), }, desc :
                Some(Borrowed("Adds a margin to a 'shape-outside'. This defines a new shape that is the smallest contour that includes all the points that are the 'shape-margin' distance outward in the perpendicular direction from a point on the underlying shape.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("shape-outside"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C37,O24")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-shapes-1/#shape-outside-property")),
                syntax : Some(Borrowed("div { $(name): margin-box; }")), restriction :
                Some(Borrowed("image, box, shape, enum")), }, desc :
                Some(Borrowed("Specifies an orthogonal rotation to be applied to an image before it is laid out.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("shape-padding"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-shapes-2/#propdef-shape-padding")),
                syntax : Some(Borrowed("div { $(name): 10px; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Adds padding to a 'shape-inside'.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("shape-rendering"),
                version : Some(Borrowed("3.0")), browsers : None, ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#ShapeRenderingProperty")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Provides hints about what tradeoffs to make as it renders vector graphics elements such as <path> elements and basic shapes such as circles and rectangles.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("size"), version : Some(Borrowed("2.1")), browsers :
                Some(Borrowed("C,O8")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-page/#page-size-prop")), syntax
                :
                Some(Borrowed("<length>{1,2} | auto | [ <page-size> || [ portrait | landscape] ]")),
                restriction : Some(Borrowed("length")), }, desc : Some(Borrowed("")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("speak"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#speak")), syntax :
                Some(Borrowed("div { $(name): normal; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether text will be rendered aurally and if so, in what manner.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("speak-as"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#speak-as")), syntax :
                Some(Borrowed("div { $(name): spell-out; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Determines in what manner text gets rendered aurally, based upon a basic predefined list of possibilities.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("src"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#src-desc")), syntax :
                Some(Borrowed("src: url(font.woff) format('woff');")), restriction :
                Some(Borrowed("enum, url, identifier")), }, desc :
                Some(Borrowed("@font-face descriptor. Specifies the resource containing font data. It is required, whether the font is downloadable or locally installed.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("stop-color"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/pservers.html#StopColorProperty")),
                syntax : None, restriction : Some(Borrowed("color")), }, desc :
                Some(Borrowed("Indicates what color to use at that gradient stop.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("stop-opacity"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/pservers.html#StopOpacityProperty")),
                syntax : None, restriction : Some(Borrowed("number(0-1)")), }, desc :
                Some(Borrowed("Defines the opacity of a given gradient stop.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("stroke"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#StrokeProperty")),
                syntax : None, restriction : Some(Borrowed("color, enum, url")), }, desc
                :
                Some(Borrowed("Paints along the outline of the given graphical element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("stroke-dasharray"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#StrokeDasharrayProperty")),
                syntax : None, restriction :
                Some(Borrowed("length, percentage, number, enum")), }, desc :
                Some(Borrowed("Controls the pattern of dashes and gaps used to stroke paths.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("stroke-dashoffset"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#StrokeDashoffsetProperty")),
                syntax : None, restriction : Some(Borrowed("percentage, length")), },
                desc :
                Some(Borrowed("Specifies the distance into the dash pattern to start the dash.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("stroke-linecap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#StrokeLinecapProperty")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the shape to be used at the end of open subpaths when they are stroked.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("stroke-linejoin"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#StrokeLinejoinProperty")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the shape to be used at the corners of paths or basic shapes when they are stroked.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("stroke-miterlimit"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#StrokeMiterlimitProperty")),
                syntax : Some(Borrowed("path { $(name): 4; }")), restriction :
                Some(Borrowed("number")), }, desc :
                Some(Borrowed("When two line segments meet at a sharp angle and miter joins have been specified for 'stroke-linejoin', it is possible for the miter to extend far beyond the thickness of the line stroking the path.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("stroke-opacity"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#StrokeOpacityProperty")),
                syntax : None, restriction : Some(Borrowed("number(0-1)")), }, desc :
                Some(Borrowed("Specifies the opacity of the painting operation used to stroke the current object.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("stroke-width"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#StrokeWidth")),
                syntax : None, restriction : Some(Borrowed("percentage, length")), },
                desc :
                Some(Borrowed("Specifies the width of the stroke on the current object.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("suffix"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-counter-styles-3/#descdef-counter-style-suffix")),
                syntax : Some(Borrowed("@counter-style { suffix: '\\2E\\20'; }")),
                restriction : Some(Borrowed("image, string, identifier")), }, desc :
                Some(Borrowed("@counter-style descriptor. Specifies a <symbol> that is appended to the marker representation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("system"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-counter-styles-3/#descdef-counter-style-system")),
                syntax : Some(Borrowed("@counter-style triangle { system: cyclic; }")),
                restriction : Some(Borrowed("enum, integer")), }, desc :
                Some(Borrowed("@counter-style descriptor. Specifies which algorithm will be used to construct the counter's representation based on the counter value.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("symbols"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("FF33")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-counter-styles-3/#descdef-counter-style-symbols")),
                syntax : Some(Borrowed("@counter-style { symbols: '*' ⁑ † ‡; }")),
                restriction : Some(Borrowed("image, string, identifier")), }, desc :
                Some(Borrowed("@counter-style descriptor. Specifies the symbols used by the marker-construction algorithm specified by the system descriptor.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("table-layout"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/CSS2/tables.html#width-layout")),
                syntax : Some(Borrowed("table { $(name): fixed; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls the algorithm used to lay out the table cells, rows, and columns.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("tab-size"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C21,O15,S6.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#tab-size")), syntax :
                Some(Borrowed("div { $(name): 4; }")), restriction :
                Some(Borrowed("integer, length")), }, desc :
                Some(Borrowed("Determines the width of the tab character (U+0009), in space characters (U+0020), when rendered.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-align"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-align0")), syntax :
                Some(Borrowed("h2 { $(name): center; }")), restriction :
                Some(Borrowed("string")), }, desc :
                Some(Borrowed("Describes how inline contents of a block are horizontally aligned if the contents do not completely fill the line box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-align-last"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,FF12,IE5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-align-last0")),
                syntax : Some(Borrowed("div { $(name): right; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes how the last line of a block or a line right before a forced line break is aligned when 'text-align' is set to 'justify'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-anchor"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/text.html#TextAnchorProperty")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Used to align (start-, middle- or end-alignment) a string of text relative to a given point.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-combine-upright"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-writing-modes-3/#text-combine-upright")),
                syntax : Some(Borrowed("span { $(name): all; }")), restriction :
                Some(Borrowed("enum, integer")), }, desc :
                Some(Borrowed("This property specifies the combination of multiple characters into the space of a single character.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-decoration"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-decoration-style")),
                syntax : Some(Borrowed("a:visited { $(name): line-through; }")),
                restriction : Some(Borrowed("enum, color")), }, desc :
                Some(Borrowed("Decorations applied to font used for an element's text.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-decoration-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF36,C57,O44")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-decoration-color")),
                syntax : Some(Borrowed("div { $(name): #ff0; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Specifies the color of text decoration (underlines overlines, and line-throughs) set on the element with text-decoration-line.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-decoration-line"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF36")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-decoration-line")),
                syntax : Some(Borrowed("div { $(name): underline; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies what line decorations, if any, are added to the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-decoration-skip"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-decoration-skip")),
                syntax : Some(Borrowed("none | [ images || spaces || ink || all ]")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies what parts of the element's content any text decoration affecting the element must skip over. It controls all text decoration lines drawn by the element and also any text decoration lines drawn by its ancestors.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-decoration-style"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("FF36")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-decoration-style")),
                syntax : Some(Borrowed("div { $(name): solid; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the line style for underline, line-through and overline text decoration.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-emphasis"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-emphasis")),
                syntax :
                Some(Borrowed("<color> | <string> | none | [ [ filled | open ] || [ dot | circle | double-circle | triangle | sesame ] ]")),
                restriction : Some(Borrowed("color, string")), }, desc :
                Some(Borrowed("Shorthand for setting text-emphasis-style and text-emphasis-color in one declaration.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-emphasis-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-emphasis-color")),
                syntax : Some(Borrowed("div { $(name): #ff0; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Describes the foreground color of the emphasis marks.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-emphasis-position"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-emphasis-position")),
                syntax : Some(Borrowed("[ above | below ] && [ right | left ]")),
                restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes where emphasis marks are drawn at.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-emphasis-style"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-text-decor-3/#text-emphasis-style")),
                syntax :
                Some(Borrowed("none | [[ filled | open ] || [ dot | circle | double-circle | triangle | sesame ]] | <string>")),
                restriction : Some(Borrowed("string")), }, desc :
                Some(Borrowed("Applies emphasis marks to the element's text.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-indent"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-indent0")), syntax :
                Some(Borrowed("li { $(name): 5px; }")), restriction :
                Some(Borrowed("percentage, length")), }, desc :
                Some(Borrowed("Specifies the indentation applied to lines of inline content in a block. The indentation only affects the first line of inline content in the block unless the 'hanging' keyword is specified, in which case it affects all lines except the first.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-justify"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,IE5.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-justify0")), syntax :
                Some(Borrowed("div { $(name): inter-word; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Selects the justification algorithm used when 'text-align' is set to 'justify'. The property applies to block containers, but the UA may (but is not required to) also support it on inline elements.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-orientation"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,O15,S5.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-writing-modes-3/#text-orientation")),
                syntax : Some(Borrowed("span { $(name): mixed; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the orientation of text within a line.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-overflow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF9,IE5.5,O11.6,S2")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#text-overflow0")), syntax :
                Some(Borrowed("span { $(name): ellipsis; }")), restriction :
                Some(Borrowed("enum, string")), }, desc :
                Some(Borrowed("Text can overflow for example when it is prevented from wrapping.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-rendering"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,FF3,O9,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#TextRenderingProperty")),
                syntax : None, restriction : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("The creator of SVG content might want to provide a hint to the implementation about what tradeoffs to make as it renders text. The 'text-rendering' property provides these hints.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-shadow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF3.6,IE10,O9.5,S1.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-shadow0")), syntax :
                Some(Borrowed("h1 { $(name): 20px 12px 2px #333;}")), restriction :
                Some(Borrowed("length, color")), }, desc :
                Some(Borrowed("Enables shadow effects to be applied to the text of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-size-adjust"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://dev.w3.org/csswg/css-size-adjust/")), syntax :
                Some(Borrowed("body { $(name): 150%; }")), restriction :
                Some(Borrowed("enum, percentage")), }, desc :
                Some(Borrowed("Specifies a size adjustment for displaying text content in mobile browsers.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-space-collapse"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#white-space-collapsing")),
                syntax : Some(Borrowed("div { $(name): collapse; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Declares whether and how white space inside the element is collapsed. Values have the following meanings, which must be interpreted according to the white space processing rules.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-space-trim"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-text-space-trim")),
                syntax : Some(Borrowed("div { $(name): trim-inner; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies trimming behavior at the beginning and end of a box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-spacing"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-text-spacing")),
                syntax : Some(Borrowed("div { $(name): none; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls spacing between adjacent characters on the same line within the same inline formatting context using a set of character-class-based rules.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-transform"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-transform0")), syntax
                : Some(Borrowed("h1 { $(name): capitalize; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls capitalization effects of an element's text.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-underline-position"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,IE10")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#text-underline-position0")),
                syntax : Some(Borrowed("article { $(name): auto; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Sets the position of an underline specified on the same element: it does not affect underlines specified by ancestor elements. This property is typically used in vertical writing contexts such as in Japanese documents where it often desired to have the underline appear 'over' (to the right of) the affected run of text")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("text-wrap"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#propdef-text-wrap")),
                syntax : Some(Borrowed("p.test { $(name): none; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies mode for text wrapping.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("top"), version :
                Some(Borrowed("2.0")), browsers : Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-positioning/#propdef-top")),
                syntax : Some(Borrowed("article { $(name): 50px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Specifies how far an absolutely positioned box's top margin edge is offset below the top edge of the box's 'containing block'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("touch-action"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C36,IE11,O23")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/pointerevents/#the-touch-action-css-property")),
                syntax : Some(Borrowed("div { $(name): pan-x; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Determines whether touch input may trigger default behavior supplied by user agent.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("transform"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C36,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transforms/#transform-property")),
                syntax : Some(Borrowed("div { $(name): rotate(-90deg); }")), restriction
                : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("A two-dimensional transformation is applied to an element through the 'transform' property. This property contains a list of transform functions similar to those allowed by SVG.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("transform-box"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-transforms-1/#propdef-transform-box")),
                syntax : Some(Borrowed("div { $(name): border-box; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("All transformations defined by the 'transform' and 'transform-origin' property are relative to the position and dimension of the specified reference box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("transform-origin"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C36,FF16,IE10,O12.1,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transforms/#propdef-transform-origin")),
                syntax : Some(Borrowed(".album { $(name): 20% 40%; }")), restriction :
                Some(Borrowed("position, length, percentage")), }, desc :
                Some(Borrowed("Establishes the origin of transformation for an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("transform-style"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C36,FF16,IE10,O23,S9")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transforms/#propdef-transform-style")),
                syntax : Some(Borrowed("div { $(name): flat; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines how nested elements are rendered in 3D space.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("transition"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,FF16,IE10,O12.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition")),
                syntax : Some(Borrowed("div { $(name): background-color linear 1s; }")),
                restriction : Some(Borrowed("time, property, timing-function, enum")), },
                desc :
                Some(Borrowed("Shorthand property combines four of the transition properties into a single property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("transition-delay"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,FF16,IE10,O12.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-delay")),
                syntax : Some(Borrowed("div { $(name): 1s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines when the transition will start. It allows a transition to begin execution some period of time from when it is applied.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("transition-duration"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,FF16,IE10,O12.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-duration")),
                syntax : Some(Borrowed("div { $(name): 1s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Specifies how long the transition from the old value to the new value should take.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("transition-property"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,FF16,IE10,O12.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-property")),
                syntax : Some(Borrowed("div { $(name): background-color; }")),
                restriction : Some(Borrowed("property")), }, desc :
                Some(Borrowed("Specifies the name of the CSS property to which the transition is applied.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("transition-timing-function"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,FF16,IE10,O12.5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-timing-function")),
                syntax : Some(Borrowed("div { $(name): linear; }")), restriction :
                Some(Borrowed("timing-function")), }, desc :
                Some(Borrowed("Describes how the intermediate values used during a transition will be calculated.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("unicode-bidi"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-writing-modes-3/#unicode-bidi")),
                syntax : Some(Borrowed("p { $(name): embed; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("The level of embedding with respect to the bidirectional algorithm.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("unicode-range"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#unicode-range-desc")),
                syntax : Some(Borrowed("@font-face { $(name): U+26; }")), restriction :
                Some(Borrowed("unicode-range")), }, desc :
                Some(Borrowed("@font-face descriptor. Defines the set of Unicode codepoints that may be supported by the font face for which it is declared.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("user-select"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-ui-4/#propdef-user-select")),
                syntax : Some(Borrowed("div { $(name): text; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls the appearance of selection.")), }, PropertyEntry
                { attributes : PropertyAttributes { name : Borrowed("user-zoom"), version
                : Some(Borrowed("3.0")), browsers : Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-device-adapt/#user-zoom-desc")),
                syntax : Some(Borrowed("@viewport { $(name): zoom; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("@viewport descriptor. Specifies if the zoom factor can be changed by user interaction or not.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("vector-effect"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/painting.html#VectorEffectProperty")),
                syntax : Some(Borrowed("circle { $(name): non-rotation; }")), restriction
                : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Affects the vertical positioning of the inline boxes generated by an inline-level element inside a line box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("vertical-align"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-linebox/#vertical-align")),
                syntax : Some(Borrowed("div { $(name): middle; }")), restriction :
                Some(Borrowed("percentage, length")), }, desc :
                Some(Borrowed("Affects the vertical positioning of the inline boxes generated by an inline-level element inside a line box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("visibility"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#visibility")), syntax :
                Some(Borrowed("img { $(name): hidden; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether the boxes generated by an element are rendered. Invisible boxes still affect layout (set the 'display' property to 'none' to suppress box generation altogether).")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("voice-balance"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#voice-balance")), syntax
                :
                Some(Borrowed("<number> | left | center | right | leftwards | rightwards")),
                restriction : Some(Borrowed("number(-100-100)")), }, desc :
                Some(Borrowed("Controls the spatial distribution of audio output across a lateral sound stage.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("voice-duration"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#voice-duration")),
                syntax : Some(Borrowed("<time>")), restriction : Some(Borrowed("time")),
                }, desc :
                Some(Borrowed("Specifies how long it should take to render the selected element's content (not including audio cues, pauses and rest).")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("voice-family"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#voice-family")), syntax
                :
                Some(Borrowed("[[<specific-voice> | <generic-voice> ],]* [<specific-voice> | <generic-voice> ]")),
                restriction : Some(Borrowed("number, string, identifier")), }, desc :
                Some(Borrowed("Comma-separated, prioritized list of voice family names (compare with 'font-family').")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("voice-pitch"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#voice-pitch")), syntax :
                Some(Borrowed("h2 { $(name): absolute 30Hz; }")), restriction :
                Some(Borrowed("percentage, number, frequency, semitones")), }, desc :
                Some(Borrowed("Specifies the average pitch (a frequency) of the speaking voice. The average pitch of a voice depends on the voice family. For example, the average pitch for a standard male voice is around 120Hz, but for a female voice, it's around 210Hz.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("voice-range"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#voice-props-voice-range")),
                syntax : Some(Borrowed("h2 { $(name): +10Hz; }")), restriction :
                Some(Borrowed("percentage, number, frequency, semitones")), }, desc :
                Some(Borrowed("Specifies the variability in the \"baseline\" pitch, i.e. how much the fundamental frequency may deviate from the average pitch of the speech output.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("voice-rate"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#voice-rate")), syntax :
                Some(Borrowed("<percentage> | x-slow | slow | medium | fast | x-fast")),
                restriction : Some(Borrowed("percentage")), }, desc :
                Some(Borrowed("Controls the speaking rate. The default rate for a voice depends on the language and dialect and on the personality of the voice.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("voice-stress"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#voice-stress")), syntax
                : Some(Borrowed("h2 { $(name): moderate; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Indicates the strength of emphasis to be applied. Emphasis is indicated using a combination of pitch change, timing changes, loudness and other acoustic differences) that varies from one language to the next.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("voice-volume"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-speech/#voice-volume")), syntax
                :
                Some(Borrowed("<decibel> | silent | x-soft | soft | medium | loud | x-loud")),
                restriction : Some(Borrowed("volume, enum")), }, desc :
                Some(Borrowed("Controls the amplitude of the audio waveform generated by the speech synthesiser.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-animation"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation")), syntax
                : Some(Borrowed("div { $(name): movearound 4s ease 3 normal; }")),
                restriction :
                Some(Borrowed("time, enum, timing-function, identifier, number")), },
                desc :
                Some(Borrowed("Shorthand property combines six of the animation properties into a single property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-animation-delay"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-delay")),
                syntax : Some(Borrowed("div { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines when the animation will start.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-animation-direction"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-direction")),
                syntax : Some(Borrowed("div { $(name): normal; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether or not the animation should play in reverse on alternate cycles.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-animation-duration"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-duration")),
                syntax : Some(Borrowed("div { $(name): 4s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines the length of time that an animation takes to complete one cycle.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-animation-fill-mode"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-fill-mode-property")),
                syntax : Some(Borrowed("div { $(name): forwards; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines what values are applied by the animation outside the time it is executing.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-animation-iteration-count"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-iteration-count")),
                syntax : Some(Borrowed("div { $(name): 3; }")), restriction :
                Some(Borrowed("number, enum")), }, desc :
                Some(Borrowed("Defines the number of times an animation cycle is played. The default value is one, meaning the animation will play from beginning to end once.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-animation-name"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#the-animation-name-property-")),
                syntax : Some(Borrowed("div { $(name): movearound; }")), restriction :
                Some(Borrowed("identifier, enum")), }, desc :
                Some(Borrowed("Defines a list of animations that apply. Each name is used to select the keyframe at-rule that provides the property values for the animation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-animation-play-state"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-play-state")),
                syntax : Some(Borrowed("div { $(name): running; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines whether the animation is running or paused.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-animation-timing-function"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-animations/#animation-timing-function")),
                syntax : Some(Borrowed("div { $(name): ease; }")), restriction :
                Some(Borrowed("timing-function")), }, desc :
                Some(Borrowed("Describes how the animation will progress over one cycle of its duration. See the 'transition-timing-function'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-appearance"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-appearance")),
                syntax : Some(Borrowed("h3 { $(name): button; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Changes the appearance of buttons and other controls to resemble native controls.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-backdrop-filter"), version : Some(Borrowed("4.0")),
                browsers : Some(Borrowed("S9")), ref_ :
                Some(Borrowed("https://drafts.fxtf.org/filters-2/#propdef-backdrop-filter")),
                syntax : Some(Borrowed("div { $(name): blur(2px); }")), restriction :
                Some(Borrowed("enum, url")), }, desc :
                Some(Borrowed("Applies a filter effect where the first filter in the list takes the element's background image as the input image.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-backface-visibility"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#backface-visibility")),
                syntax : Some(Borrowed("div { $(name): hidden; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Determines whether or not the 'back' side of a transformed element is visible when facing the viewer. With an identity transform, the front side of an element faces the viewer.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-background-clip"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-clip")),
                syntax : Some(Borrowed("header { $(name): border-box; }")), restriction :
                Some(Borrowed("box")), }, desc :
                Some(Borrowed("Determines the background painting area.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-background-composite"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S3")), ref_ : None,
                syntax : Some(Borrowed("div { $(name): padding; }")), restriction :
                Some(Borrowed("enum")), }, desc : Some(Borrowed("")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-webkit-background-origin"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#the-background-origin")),
                syntax : Some(Borrowed("header { $(name): border-box; }")), restriction :
                Some(Borrowed("box")), }, desc :
                Some(Borrowed("For elements rendered as a single box, specifies the background positioning area. For elements rendered as multiple boxes (e.g., inline boxes on several lines, boxes on several pages) specifies which boxes 'box-decoration-break' operates on to determine the background positioning area(s).")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-border-image"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-background/#border-image")),
                syntax : Some(Borrowed("td { $(name): url(border.png) 30 30 round;}")),
                restriction : Some(Borrowed("length, percentage, number, url, enum")), },
                desc :
                Some(Borrowed("Shorthand property for setting 'border-image-source', 'border-image-slice', 'border-image-width', 'border-image-outset' and 'border-image-repeat'. Omitted values are set to their initial values.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-box-align"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-box-align")), syntax
                : Some(Borrowed("div { $(name): end; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies the alignment of nested elements within an outer flexible box element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-box-direction"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-box-direction")),
                syntax : Some(Borrowed("div { $(name): reverse; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("In webkit applications, -webkit-box-direction specifies whether a box lays out its contents normally (from the top or left edge), or in reverse (from the bottom or right edge).")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-box-flex"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-box-flex")), syntax
                : Some(Borrowed("div { $(name): 1; }")), restriction :
                Some(Borrowed("number")), }, desc :
                Some(Borrowed("Specifies an element's flexibility.")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-webkit-box-flex-group"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-box-flex-group")),
                syntax : Some(Borrowed("div { $(name): 4; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Flexible elements can be assigned to flex groups using the 'box-flex-group' property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-box-ordinal-group"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-box-ordinal-group")),
                syntax : Some(Borrowed("div { $(name): 3; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Indicates the ordinal group the element belongs to. Elements with a lower ordinal group are displayed before those with a higher ordinal group.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-box-orient"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-box-orient")),
                syntax : Some(Borrowed("div { $(name): vertical; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("In webkit applications, -webkit-box-orient specifies whether a box lays out its contents horizontally or vertically.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-box-pack"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-box-pack")), syntax
                : Some(Borrowed("div { $(name): end; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies alignment of child elements within the current element in the direction of orientation.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-box-reflect"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S4")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-box-reflect")),
                syntax : Some(Borrowed("div { $(name): below 5px; }")), restriction :
                Some(Borrowed("")), }, desc :
                Some(Borrowed("Defines a reflection of a border box.")), }, PropertyEntry
                { attributes : PropertyAttributes { name :
                Borrowed("-webkit-box-sizing"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-ui/#box-sizing")), syntax :
                Some(Borrowed("div { $(name): content-box; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Box Model addition in CSS3.")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("-webkit-break-after"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("S7")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-breaks")),
                syntax : Some(Borrowed("h2 { $(name): column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column break behavior before the generated box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-break-before"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("S7")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-breaks")),
                syntax : Some(Borrowed("h2 { $(name): column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column break behavior before the generated box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-break-inside"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("S7")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-breaks")),
                syntax : Some(Borrowed("h2 { $(name): avoid-column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column break behavior inside the generated box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-break-after"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-breaks")),
                syntax : Some(Borrowed("h2 { $(name): column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column break behavior before the generated box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-break-before"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-breaks")),
                syntax : Some(Borrowed("h2 { $(name): column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column break behavior before the generated box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-break-inside"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-breaks")),
                syntax : Some(Borrowed("h2 { $(name): avoid-column; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column break behavior inside the generated box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-count"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-count")),
                syntax : Some(Borrowed("div { $(name): 3; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("Describes the optimal number of columns into which the content of the element will be flowed.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-gap"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-gap0")), syntax
                : Some(Borrowed("div { $(name): 10px; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Sets the gap between columns. If there is a column rule between columns, it will appear in the middle of the gap.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-rule"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule0")),
                syntax : Some(Borrowed("header { $(name): 5px solid red;}")), restriction
                : Some(Borrowed("length, line-width, line-style, color")), }, desc :
                Some(Borrowed("This property is a shorthand for setting 'column-rule-width', 'column-rule-style', and 'column-rule-color' at the same place in the style sheet. Omitted values are set to their initial values.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-rule-color"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule-color")),
                syntax : Some(Borrowed("div { $(name): #ff0; }")), restriction :
                Some(Borrowed("color")), }, desc :
                Some(Borrowed("Sets the color of the column rule")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-rule-style"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule-style")),
                syntax : Some(Borrowed("div { $(name): solid; }")), restriction :
                Some(Borrowed("line-style")), }, desc :
                Some(Borrowed("Sets the style of the rule between columns of an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-rule-width"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-rule-width")),
                syntax : Some(Borrowed("div { $(name): 3px; }")), restriction :
                Some(Borrowed("length, line-width")), }, desc :
                Some(Borrowed("Sets the width of the rule between columns. Negative values are not allowed.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-columns"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#columns0")), syntax :
                Some(Borrowed("div { $(name): 100px 3; }")), restriction :
                Some(Borrowed("length, integer")), }, desc :
                Some(Borrowed("A shorthand property which sets both 'column-width' and 'column-count'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-span"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-span0")),
                syntax : Some(Borrowed("article { $(name): all; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Describes the page/column break behavior after the generated box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-column-width"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-multicol/#column-width")),
                syntax : Some(Borrowed("div { $(name): 100px; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("This property describes the width of columns in multicol elements.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-filter"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C18,O15,S6")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/filter-effects/#propdef-filter")),
                syntax : Some(Borrowed("img { $(name): blur(3px); }")), restriction :
                Some(Borrowed("enum, url")), }, desc :
                Some(Borrowed("Processes an element's rendering before it is displayed in the document, by applying one or more filter effects.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-flow-from"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("S6.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-regions/#flow-from")), syntax :
                Some(Borrowed("div { $(name): identifier; }")), restriction :
                Some(Borrowed("identifier")), }, desc :
                Some(Borrowed("Makes a block container a region and associates it with a named flow.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-flow-into"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("S6.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-regions/#flow-into")), syntax :
                Some(Borrowed("div { $(name): identifier; }")), restriction :
                Some(Borrowed("identifier")), }, desc :
                Some(Borrowed("Places an element or its contents into a named flow.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-font-feature-settings"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C16")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-fonts/#propdef-font-feature-settings")),
                syntax : Some(Borrowed("body { $(name): 'hwid'; }")), restriction :
                Some(Borrowed("string, integer")), }, desc :
                Some(Borrowed("This property provides low-level control over OpenType font features. It is intended as a way of providing access to font features that are not widely used but are needed for a particular use case.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-hyphens"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("S5.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#hyphens0")), syntax :
                Some(Borrowed("div { $(name): manual; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls whether hyphenation is allowed to create more break opportunities within a line of text.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-line-break"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-line-break")),
                syntax : Some(Borrowed("p { $(name): normal; }")), restriction :
                Some(Borrowed("")), }, desc :
                Some(Borrowed("Specifies line-breaking rules for CJK (Chinese, Japanese, and Korean) text.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-margin-bottom-collapse"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S3")), ref_ : None,
                syntax : Some(Borrowed("div { $(name): collapse; }")), restriction :
                Some(Borrowed("enum")), }, desc : Some(Borrowed("")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-webkit-margin-collapse"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ : None, syntax :
                Some(Borrowed("div { $(name): collapse; }")), restriction :
                Some(Borrowed("enum")), }, desc : Some(Borrowed("")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-webkit-margin-start"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ : None, syntax :
                Some(Borrowed("div { $(name): 5px; }")), restriction :
                Some(Borrowed("percentage, length")), }, desc : Some(Borrowed("")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-margin-top-collapse"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ : None, syntax :
                Some(Borrowed("div { $(name): collapse; }")), restriction :
                Some(Borrowed("enum")), }, desc : Some(Borrowed("")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("-webkit-mask-clip"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O15,S4")),
                ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-clip")),
                syntax : None, restriction : Some(Borrowed("box")), }, desc :
                Some(Borrowed("Determines the mask painting area, which determines the area that is affected by the mask.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-mask-image"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,O15,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-image")),
                syntax : None, restriction : Some(Borrowed("url, image, enum")), }, desc
                : Some(Borrowed("Sets the mask layer image of an element.")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-mask-origin"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O15,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-origin")),
                syntax : None, restriction : Some(Borrowed("box")), }, desc :
                Some(Borrowed("Specifies the mask positioning area.")), }, PropertyEntry
                { attributes : PropertyAttributes { name :
                Borrowed("-webkit-mask-repeat"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O15,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-repeat")),
                syntax : None, restriction : Some(Borrowed("repeat")), }, desc :
                Some(Borrowed("Specifies how mask layer images are tiled after they have been sized and positioned.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-mask-size"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,O15,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-masking-1/#the-mask-size")),
                syntax : None, restriction : Some(Borrowed("length, percentage, enum")),
                }, desc : Some(Borrowed("Specifies the size of the mask layer images.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-nbsp-mode"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-nbsp-mode")), syntax
                : Some(Borrowed("p { $(name): space; }")), restriction :
                Some(Borrowed("")), }, desc :
                Some(Borrowed("Defines the behavior of nonbreaking spaces within text.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-overflow-scrolling"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S5")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-nbsp-mode")), syntax
                : Some(Borrowed("div { $(name): touch; }")), restriction :
                Some(Borrowed("")), }, desc :
                Some(Borrowed("Specifies whether to use native-style scrolling in an overflow:scroll element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-padding-start"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ : None, syntax :
                Some(Borrowed("div { $(name): 5px; }")), restriction :
                Some(Borrowed("percentage, length")), }, desc : Some(Borrowed("")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-perspective"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#perspective")),
                syntax : Some(Borrowed("div { $(name): none; }")), restriction :
                Some(Borrowed("length")), }, desc :
                Some(Borrowed("Applies the same transform as the perspective(<number>) transform function, except that it applies only to the positioned or transformed children of the element, not to the transform on the element itself.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-perspective-origin"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#perspective-origin")),
                syntax : Some(Borrowed("div { $(name): 10px; }")), restriction :
                Some(Borrowed("position, percentage, length")), }, desc :
                Some(Borrowed("Establishes the origin for the perspective property. It effectively sets the X and Y position at which the viewer appears to be looking at the children of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-region-fragment"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("S7")), ref_ :
                Some(Borrowed("http://dev.w3.org/csswg/css-regions/#region-fragment")),
                syntax : Some(Borrowed("article { $(name): break; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("The 'region-fragment' property controls the behavior of the last region associated with a named flow.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-tap-highlight-color"), version : Some(Borrowed("3")),
                browsers : Some(Borrowed("E,C,S3.1")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-tap-highlight-color")),
                syntax : None, restriction : Some(Borrowed("color")), }, desc :
                Some(Borrowed("")), }, PropertyEntry { attributes : PropertyAttributes {
                name : Borrowed("-webkit-text-fill-color"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("E,C,S3")), ref_ : None,
                syntax : Some(Borrowed("div { $(name): red; }")), restriction :
                Some(Borrowed("color")), }, desc : Some(Borrowed("")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-webkit-text-size-adjust"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("E,C,S3")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-size-adjust/#text-size-adjust")),
                syntax : Some(Borrowed("div { $(name): 60%; }")), restriction :
                Some(Borrowed("percentage")), }, desc :
                Some(Borrowed("Specifies a size adjustment for displaying text content in mobile browsers.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-text-stroke"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("S3")), ref_ : None, syntax :
                Some(Borrowed("div { $(name): red 2x; }")), restriction :
                Some(Borrowed("length, line-width, color, percentage")), }, desc :
                Some(Borrowed("")), }, PropertyEntry { attributes : PropertyAttributes {
                name : Borrowed("-webkit-text-stroke-color"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("S3")), ref_ : None,
                syntax : Some(Borrowed("div { $(name): red; }")), restriction :
                Some(Borrowed("color")), }, desc : Some(Borrowed("")), }, PropertyEntry {
                attributes : PropertyAttributes { name :
                Borrowed("-webkit-text-stroke-width"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("S3")), ref_ : None, syntax :
                Some(Borrowed("div { $(name): 2px; }")), restriction :
                Some(Borrowed("length, line-width, percentage")), }, desc :
                Some(Borrowed("")), }, PropertyEntry { attributes : PropertyAttributes {
                name : Borrowed("-webkit-touch-callout"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("S3")), ref_ : None,
                syntax : Some(Borrowed("a { $(name): none; }")), restriction :
                Some(Borrowed("enum")), }, desc : Some(Borrowed("")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("-webkit-transform"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O12,S3.1")),
                ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-2d-transforms/#transform-property")),
                syntax : Some(Borrowed("div { $(name): rotate(-90deg); }")), restriction
                : Some(Borrowed("enum")), }, desc :
                Some(Borrowed("A two-dimensional transformation is applied to an element through the 'transform' property. This property contains a list of transform functions similar to those allowed by SVG.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-transform-origin"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O15,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-2d-transforms/#transform-origin")),
                syntax : Some(Borrowed(".album { $(name): 20% 40%; }")), restriction :
                Some(Borrowed("position, length, percentage")), }, desc :
                Some(Borrowed("Establishes the origin of transformation for an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-transform-origin-x"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#transform-origin")),
                syntax : Some(Borrowed("img { $(name): 5px}")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("The x coordinate of the origin for transforms applied to an element with respect to its border box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-transform-origin-y"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3.1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#transform-origin")),
                syntax : Some(Borrowed("img { $(name): 5px}")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("The y coordinate of the origin for transforms applied to an element with respect to its border box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-transform-origin-z"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#transform-origin")),
                syntax : Some(Borrowed("img { $(name): 5px}")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("The z coordinate of the origin for transforms applied to an element with respect to its border box.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-transform-style"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S4")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-3d-transforms/#transform-origin")),
                syntax : Some(Borrowed("div { $(name): flat; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Defines how nested elements are rendered in 3D space.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-transition"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("C,O12,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition")),
                syntax : Some(Borrowed("div { $(name): background-color linear 1s; }")),
                restriction : Some(Borrowed("time, property, timing-function, enum")), },
                desc :
                Some(Borrowed("Shorthand property combines four of the transition properties into a single property.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-transition-delay"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O12,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-delay")),
                syntax : Some(Borrowed("div { $(name): 1s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Defines when the transition will start. It allows a transition to begin execution some period of time from when it is applied.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-transition-duration"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O12,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-duration")),
                syntax : Some(Borrowed("div { $(name): 1s; }")), restriction :
                Some(Borrowed("time")), }, desc :
                Some(Borrowed("Specifies how long the transition from the old value to the new value should take.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-transition-property"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,O12,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-property")),
                syntax : Some(Borrowed("div { $(name): background-color; }")),
                restriction : Some(Borrowed("property")), }, desc :
                Some(Borrowed("Specifies the name of the CSS property to which the transition is applied.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-transition-timing-function"), version :
                Some(Borrowed("3.0")), browsers : Some(Borrowed("C,O12,S5")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-transitions/#transition-timing-function")),
                syntax : Some(Borrowed("div { $(name): linear; }")), restriction :
                Some(Borrowed("timing-function")), }, desc :
                Some(Borrowed("Describes how the intermediate values used during a transition will be calculated.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-user-drag"), version : Some(Borrowed("3.0")), browsers
                : Some(Borrowed("S3")), ref_ : None, syntax :
                Some(Borrowed("div { $(name): element; }")), restriction :
                Some(Borrowed("enum")), }, desc : Some(Borrowed("")), }, PropertyEntry {
                attributes : PropertyAttributes { name : Borrowed("-webkit-user-modify"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("C,S3")), ref_
                : Some(Borrowed("http://css-infos.net/property/-webkit-user-modify")),
                syntax : Some(Borrowed("div { $(name): read-only; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Determines whether a user can edit the content of an element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("-webkit-user-select"), version : Some(Borrowed("3.0")),
                browsers : Some(Borrowed("C,S3")), ref_ :
                Some(Borrowed("http://css-infos.net/property/-webkit-user-select")),
                syntax : Some(Borrowed("div { $(name): text; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Controls the appearance of selection.")), }, PropertyEntry
                { attributes : PropertyAttributes { name : Borrowed("widows"), version :
                Some(Borrowed("2.0")), browsers : Some(Borrowed("C,IE8,O9.5,S1")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-break/#widows-orphans")), syntax
                : Some(Borrowed("<integer>")), restriction : Some(Borrowed("integer")),
                }, desc :
                Some(Borrowed("Specifies the minimum number of line boxes of a block container that must be left in a fragment after a break.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("width"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-box/#width")), syntax :
                Some(Borrowed("header { $(name): 200px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Specifies the width of the content area, padding area or border area (depending on 'box-sizing') of certain boxes.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("will-change"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("C36,FF36,O24")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-will-change/")), syntax :
                Some(Borrowed("body { $(name): scroll-position; }")), restriction :
                Some(Borrowed("enum, identifier")), }, desc :
                Some(Borrowed("Provides a rendering hint to the user agent, stating what kinds of changes the author expects to perform on the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("word-break"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,FF15,IE5,S3")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#word-break0")), syntax :
                Some(Borrowed("p.album { $(name): break-all; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies line break opportunities for non-CJK scripts.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("word-spacing"), version : Some(Borrowed("1.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#word-spacing0")), syntax :
                Some(Borrowed("article { $(name): 3px; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Specifies additional spacing between \"words\".")), },
                PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("word-wrap"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-text/#word-wrap0")), syntax :
                Some(Borrowed("p { $(name): break-word; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies whether the UA may break within a word to prevent overflow when an otherwise-unbreakable string is too long to fit.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("wrap-after"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-wrap-after")),
                syntax : Some(Borrowed("span { $(name): avoid-line; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies modifications to break opportunities in line breaking (and flex line breaking.)")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("wrap-before"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-wrap-after")),
                syntax : Some(Borrowed("span { $(name): avoid-line; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies modifications to break opportunities in line breaking (and flex line breaking.)")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("wrap-flow"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-exclusions/#wrap-flow-property")),
                syntax : Some(Borrowed("div { $(name): maximum; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("An element becomes an exclusion when its 'wrap-flow' property has a computed value other than 'auto'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("wrap-inside"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("https://drafts.csswg.org/css-text-4/#propdef-wrap-inside")),
                syntax : Some(Borrowed("span { $(name): avoid; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies line breaking within boxes.")), }, PropertyEntry
                { attributes : PropertyAttributes { name : Borrowed("wrap-through"),
                version : Some(Borrowed("3.0")), browsers : Some(Borrowed("none")), ref_
                :
                Some(Borrowed("http://www.w3.org/TR/css3-exclusions/#propdef-wrap-through")),
                syntax : Some(Borrowed("div { $(name): wrap; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("Specifies if an element inherits its parent wrapping context. In other words if it is subject to the exclusions defined outside the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("writing-mode"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,FF41")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css-writing-modes-3/#writing-mode")),
                syntax : Some(Borrowed("span { $(name): lr-tb; }")), restriction :
                Some(Borrowed("enum")), }, desc :
                Some(Borrowed("This is a shorthand property for both 'direction' and 'block-progression'.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("x"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/geometry.html#X")), syntax :
                Some(Borrowed("rect { $(name): 42em; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Describes the horizontal coordinate of the position of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("y"), version : Some(Borrowed("4.0")), browsers :
                Some(Borrowed("none")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/SVG2/geometry.html#Y")), syntax :
                Some(Borrowed("rect { $(name): 42em; }")), restriction :
                Some(Borrowed("length, percentage")), }, desc :
                Some(Borrowed("Describes the vertical coordinate of the position of the element.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("z-index"), version : Some(Borrowed("2.0")), browsers :
                Some(Borrowed("all")), ref_ :
                Some(Borrowed("http://www.w3.org/TR/css3-positioning/#propdef-z-index")),
                syntax : Some(Borrowed("img { $(name): 3; }")), restriction :
                Some(Borrowed("integer")), }, desc :
                Some(Borrowed("For a positioned box, the 'z-index' property specifies the stack level of the box in the current stacking context and whether the box establishes a local stacking context.")),
                }, PropertyEntry { attributes : PropertyAttributes { name :
                Borrowed("zoom"), version : Some(Borrowed("3.0")), browsers :
                Some(Borrowed("E,C,IE6,O15,S4")), ref_ :
                Some(Borrowed("https://msdn.microsoft.com/en-us/library/ms531189(v=vs.85).aspx")),
                syntax : Some(Borrowed(".example { $(name): 1; }")), restriction :
                Some(Borrowed("enum, integer, number, percentage")), }, desc :
                Some(Borrowed("Non-standard. Specifies the magnification scale of the object. See 'transform: scale()' for a standards-based alternative.")),
                },
            ],
        },
    },
}
});

#[allow(dead_code)]
pub(crate) static EMPTY_CSS_DATA: LazyLock<CssCustomData> = LazyLock::new(|| CssCustomData {
    css: CssSection {
        at_directives: AtDirectives { entry: vec![] },
        pseudo_classes: PseudoClasses { entry: vec![] },
        pseudo_elements: PseudoElements { entry: vec![] },
        properties: Properties { entry: vec![] },
    },
});
