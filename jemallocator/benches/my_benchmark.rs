use criterion::{Criterion, criterion_group, criterion_main};

fn bench_fib(c: &mut Criterion) {
    c.bench_function("jem allocator", |b| {
        b.iter(|| (0..100_000).map(|i| Box::new(i)).collect::<Vec<_>>())
    });
}

use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use std::env;

#[warn(unsafe_op_in_unsafe_fn)]
unsafe fn configure_jemalloc() {
    env::set_var("MALLOC_CONF", "narenas:4,background_thread:true");
    // 关键参数说明：
    //
    // • narenas: 设置内存域数量（建议为CPU核心数）
    // • dirty_decay_ms: 控制内存回收延迟
    // • background_thread: 启用后台清理线程
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);
