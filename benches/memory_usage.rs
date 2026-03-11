use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use soroban_security_guard::{scan_contract, ScannerConfig};
use std::fs;

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    // Test with different contract sizes
    for size in [10, 50, 100, 500].iter() {
        let mut contract_code = String::new();
        contract_code.push_str("#[contract]\npub struct MemoryTestContract {}\n\n#[contractimpl]\nimpl MemoryTestContract {\n");
        
        for i in 0..*size {
            contract_code.push_str(&format!(
                r#"
    pub fn function_{}(env: &Env, param: u64) -> u64 {{
        let result = param + {};
        env.storage().instance().set(&DataKey::Counter, &result);
        if result > 100 {{
            env.storage().instance().set(&DataKey::HighValue, &result);
        }}
        result
    }}
"#, i, i
            ));
        }
        
        contract_code.push_str("}\n");

        group.bench_with_input(
            BenchmarkId::new("scan_contract", size),
            size,
            |b, _| {
                b.iter(|| {
                    let config = ScannerConfig::default();
                    let _report = scan_contract(black_box("memory_test.rs"), black_box(config)).unwrap();
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_memory_usage);
criterion_main!(benches);
