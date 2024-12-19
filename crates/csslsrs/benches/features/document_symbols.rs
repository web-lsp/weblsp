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

h1.foo {
  color: rgba(0, 0, 0, 0.5);
}

h1 > span {
  color: linear-gradient(to right, red, #fff);
}
"#;

fn get_document_symbols_benchmark(c: &mut Criterion) {
    let mut ls = LanguageService::default();

    let document = TextDocumentItem {
        uri: Uri::from_str("file:///test.css").unwrap(),
        language_id: "css".to_string(),
        version: 0,
        text: TEST_CASE.to_string(),
    };

    ls.upsert_document(document.clone());

    c.bench_function("get_document_symbols", |b| {
        b.iter(|| ls.get_document_symbols(black_box(document.clone())))
    });
}

criterion_group!(benches, get_document_symbols_benchmark);
