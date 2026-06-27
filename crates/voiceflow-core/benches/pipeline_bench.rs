use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use voiceflow_core::formatting::registry::RuleRegistry;
use voiceflow_core::pipeline::models::{TransformationRequest, TransformationState};

fn benchmark_pipeline(c: &mut Criterion) {
    let registry = RuleRegistry::default();
    
    c.bench_function("pipeline_100_words", |b| {
        let text = "hello world ".repeat(50); // 100 words
        b.iter(|| {
            let mut state = TransformationState::new(text.clone());
            let request = TransformationRequest::new(text.clone());
            registry.apply_all(&mut state, &request);
        });
    });

    c.bench_function("pipeline_1000_words", |b| {
        let text = "hello world ".repeat(500); // 1000 words
        b.iter(|| {
            let mut state = TransformationState::new(text.clone());
            let request = TransformationRequest::new(text.clone());
            registry.apply_all(&mut state, &request);
        });
    });
    
    // Simulate streaming
    c.bench_function("streaming_incremental_10_tokens", |b| {
        b.iter_batched(
            || {
                let text = "This is a streaming ".to_string();
                let delta = "incremental update ".to_string();
                (text, delta)
            },
            |(mut text, delta)| {
                text.push_str(&delta);
                let mut state = TransformationState::new(text.clone());
                let request = TransformationRequest::new(text.clone());
                registry.apply_all(&mut state, &request);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, benchmark_pipeline);
criterion_main!(benches);
