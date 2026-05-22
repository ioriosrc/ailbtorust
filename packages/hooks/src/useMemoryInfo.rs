```rust
use std::rc::Rc;

struct MemoryInfo {
    total_js_heap_size: usize,
    js_heap_size_limit: usize,
    used_js_heap_size: usize,
}

fn use_memory_info(options: UseMemoryInfoOptions) -> Option<Rc<MemoryInfo>> {
    let refresh_interval_ms = options.refresh_interval_ms;

    let mut memory_info = Rc::new(MemoryInfo {
        total_js_heap_size: 0,
        js_heap_size_limit: 0,
        used_js_heap_size: 0,
    });

    if !window.performance.memory.is_some() {
        log::info!("No memory information available");
        return None;
    }

    let interval = std::thread::spawn(move || {
        loop {
            memory_info.with_ref(|memory_info| {
                memory_info.total_js_heap_size = window.performance.memory.as_ref().unwrap().total_js_heap_size as usize;
                memory_info.js_heap_size_limit = window.performance.memory.as_ref().unwrap().js_heap_size_limit as usize;
                memory_info.used_js_heap_size = window.performance.memory.as_ref().unwrap().used_js_heap_size as usize;
            });
            std::thread::sleep(std::time::Duration::from_millis(refresh_interval_ms));
        }
    });

    Some(memory_info)
}
```