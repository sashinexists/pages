use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn generate() -> String {
    let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("{}", id)
}
