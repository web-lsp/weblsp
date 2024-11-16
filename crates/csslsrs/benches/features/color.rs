use criterion::{criterion_group, Criterion};
use csslsrs::service::LanguageService;
use lsp_types::{TextDocumentItem, Uri};
use std::{hint::black_box, str::FromStr};

fn get_colors_benchmark(c: &mut Criterion) {
    let mut ls = LanguageService::default();

    let document = TextDocumentItem {
        uri: Uri::from_str("file:///test.css").unwrap(),
        language_id: "css".to_string(),
        version: 0,
        text: "body { color: red; }".to_string(),
    };

    c.bench_function("get_document_colors", |b| {
        b.iter(|| ls.get_document_colors(black_box(document.clone())))
    });
}

fn get_color_presentations_benchmark(c: &mut Criterion) {
    let mut ls = LanguageService::default();

    let document = TextDocumentItem {
        uri: Uri::from_str("file:///test.css").unwrap(),
        language_id: "css".to_string(),
        version: 0,
        text: "body { color: red; }".to_string(),
    };

    let colors = ls.get_document_colors(document);
    let color = colors.first().unwrap();

    c.bench_function("get_color_presentations", |b| {
        b.iter(|| ls.get_color_presentations(black_box(color.clone()), black_box(color.range)))
    });
}

criterion_group!(
    benches,
    get_colors_benchmark,
    get_color_presentations_benchmark
);
