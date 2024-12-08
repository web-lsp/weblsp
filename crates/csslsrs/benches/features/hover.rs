use criterion::{criterion_group, Criterion};
use csslsrs::service::LanguageService;
use lsp_types::{TextDocumentItem, Uri};
use std::{hint::black_box, str::FromStr};

fn get_hover_benchmark(c: &mut Criterion) {
    let mut ls = LanguageService::default();

    let document = TextDocumentItem {
        uri: Uri::from_str("file:///test.css").unwrap(),
        language_id: "css".to_string(),
        version: 0,
        text: "body { color: red; }".to_string(),
    };

    ls.upsert_document(document.clone());

    c.bench_function("get_hover", |b| {
        b.iter(|| ls.get_hover(black_box(document.clone()), lsp_types::Position::new(0, 8)))
    });
}

criterion_group!(benches, get_hover_benchmark);
