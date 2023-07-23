use std::{
    io,
    path::PathBuf,
    process::{Command, Output as StdOutput},
};

use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

fn nargo_cmd() -> std::process::Command {
    Command::new("nargo")
}

fn nargo_compile(test_program_dir: &PathBuf) -> io::Result<StdOutput> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("compile")
        .arg("my_test_circuit")
        .spawn()
        .unwrap()
        .wait_with_output()
}

fn nargo_compile_new(test_program_dir: &PathBuf) -> io::Result<StdOutput> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("compile")
        .arg("my_test_circuit")
        .arg("--experimental-ssa")
        .spawn()
        .unwrap()
        .wait_with_output()
}

fn nargo_prove(test_program_dir: &PathBuf) -> io::Result<StdOutput> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("prove")
        .arg("my_test_proof")
        // .arg("my_test_circuit")
        .spawn()
        .unwrap()
        .wait_with_output()
}

fn nargo_prove_new(test_program_dir: &PathBuf) -> io::Result<StdOutput> {
    nargo_cmd()
        .current_dir(test_program_dir)
        .arg("prove")
        .arg("my_test_proof")
        // .arg("my_test_circuit")
        .arg("--experimental-ssa")
        .spawn()
        .unwrap()
        .wait_with_output()
}

pub fn benchmark_compile_old_ssa(c: &mut Criterion) {
    c.bench_function("nargo compile eddsa old", |b| {
        b.iter(|| nargo_compile(&PathBuf::from("./tests/test_data/eddsa")))
    });
}

pub fn benchmark_compile_new_ssa(c: &mut Criterion) {
    c.bench_function("nargo compile eddsa new", |b| {
        b.iter(|| nargo_compile_new(&PathBuf::from("./tests/test_data/eddsa")))
    });
}

pub fn benchmark_prove_old_ssa(c: &mut Criterion) {
    c.bench_function("nargo prove eddsa old", |b| {
        b.iter(|| nargo_prove(&PathBuf::from("./tests/test_data/eddsa")))
    });
}

pub fn benchmark_prove_new_ssa(c: &mut Criterion) {
    c.bench_function("nargo prove eddsa new", |b| {
        b.iter(|| nargo_prove_new(&PathBuf::from("./tests/test_data/eddsa")))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = benchmark_compile_old_ssa, benchmark_compile_new_ssa, benchmark_prove_old_ssa, benchmark_prove_new_ssa
}
criterion_main!(benches);
