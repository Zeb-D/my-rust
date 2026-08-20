use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let _data = vec![0u8; 1024]; // 使用jemalloc分配
    // 业务逻辑代码
}

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

