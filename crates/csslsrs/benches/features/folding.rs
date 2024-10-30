use criterion::{criterion_group, Criterion};
use csslsrs::service::LanguageService;
use lsp_types::{TextDocumentItem, Uri};
use std::{hint::black_box, str::FromStr};

static TEST_CASE: &str = r#"
body {
	background-color: #fff;
}

a {
	color: red;
}

h1 {
	color: rgba(0, 0, 0, 0.5);
}

h2 {
	color: linear-gradient(to right, red, #fff);
}

//#region Outer Region
h3 {
	color: hsl(120, 100%, 50%);
}
//#region Inner Region
h4 {
	color: hwb(120, 0%, 0%);
}
//#endregion
//#endregion

@media (max-width: 600px) {
	body {
		background-color: #000;
	}
}
"#;

fn get_folding_ranges_benchmark(c: &mut Criterion) {
    let mut ls = LanguageService::default();

    let document = TextDocumentItem {
        uri: Uri::from_str("file:///test.css").unwrap(),
        language_id: "css".to_string(),
        version: 0,
        text: TEST_CASE.to_string(),
    };

    c.bench_function("get_folding_ranges", |b| {
        b.iter(|| ls.get_folding_ranges(black_box(document.clone())))
    });
}

criterion_group!(benches, get_folding_ranges_benchmark);
