use criterion::{criterion_group, criterion_main, Criterion};

use bencher_adapter::Adapter;
use bencher_json::project::report::JsonAdapter;

const JSON_RESULT: &str = r#"{
    "tests::benchmark_1": {
      "latency": {
        "value": 1.0,
        "lower_bound": 1.0,
        "upper_bound": 1.0
      }
    },
    "tests::benchmark_2": {
      "latency": {
        "value": 22.0,
        "lower_bound": 22.0,
        "upper_bound": 22.0
      }
    },
    "tests::benchmark_3": {
      "latency": {
        "value": 333.0,
        "lower_bound": 333.0,
        "upper_bound": 333.0
      }
    },
    "tests::benchmark_4": {
      "latency": {
        "value": 4444.0,
        "lower_bound": 4444.0,
        "upper_bound": 4444.0
      }
    }
  }"#;

fn adapter_magic_json(c: &mut Criterion) {
    c.bench_function("JsonAdapter::Magic (JSON)", |b| {
        b.iter(|| JsonAdapter::Magic.convert(JSON_RESULT))
    });
}

fn adapter_json(c: &mut Criterion) {
    c.bench_function("JsonAdapter::Json", |b| {
        b.iter(|| JsonAdapter::Json.convert(JSON_RESULT))
    });
}

const RUST_RESULT: &str = "
running 5 tests
test tests::ignored ... ignored
test tests::benchmark_1 ... bench:              1  ns/iter (+/- 1)
test tests::benchmark_2 ... bench:              22 ns/iter (+/- 22)
test tests::benchmark_3 ... bench:             333 ns/iter (+/- 333)
test tests::benchmark_4 ... bench:           4,444 μs/iter (+/- 4,444)

test result: ok. 0 passed; 0 failed; 1 ignored; 4 measured; 0 filtered out; finished in 0.11s

";

fn adapter_magic_rust(c: &mut Criterion) {
    c.bench_function("JsonAdapter::Magic (Rust)", |b| {
        b.iter(|| JsonAdapter::Magic.convert(RUST_RESULT))
    });
}

fn adapter_rust(c: &mut Criterion) {
    c.bench_function("JsonAdapter::Rust", |b| {
        b.iter(|| JsonAdapter::Rust.convert(RUST_RESULT))
    });
}

criterion_group!(
    benches,
    adapter_magic_json,
    adapter_json,
    adapter_magic_rust,
    adapter_rust
);
criterion_main!(benches);
