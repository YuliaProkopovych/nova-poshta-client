use criterion::{criterion_group, criterion_main, Criterion, black_box};
use nova_poshta::np_client::{tracking::TrackingDoc, res_template::ResponseTemplate};

pub fn deserialize_tracking_doc(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tracking document");
    let tracking_doc = include_str!("../src/tests/resources/tracking_response.json");

    group.bench_function("tracking_doc_deserializer", |b| {
        b.iter(|| {
            black_box(serde_json::from_str::<ResponseTemplate<TrackingDoc>>(black_box(tracking_doc)).unwrap());
        })
    });

    group.finish();
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(500);
    targets = deserialize_tracking_doc
}
criterion_main!(benches);