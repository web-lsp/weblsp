use criterion::criterion_main;

mod features;

criterion_main!(
    features::folding::benches,
    features::color::benches,
    features::hover::benches,
    features::document_symbols::benches
);
