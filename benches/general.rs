use criterion::{black_box, criterion_group, criterion_main, Criterion};
use thespian::{Spec, Reason};

/// Time construction of a [thespian::Spec], followed by spawning
/// from it.
pub fn create_spec_then_spawn(c: &mut Criterion) {
    c.bench_function("create_spec_and_spawn", |b| b.iter(|| {
        let mut spec = Spec::new(|_context| async {
            Reason::Normal
        });
        drop(spec);
    }));  
}

criterion_group!(benches,
    create_spec_then_spawn,
);
criterion_main!(benches);
