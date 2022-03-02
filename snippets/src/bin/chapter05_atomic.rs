use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let my_atomic = AtomicUsize::new(42);

    my_atomic.fetch_add(1, Ordering::SeqCst);

    assert!(my_atomic.load(Ordering::SeqCst) == 43);

    my_atomic.fetch_sub(1, Ordering::SeqCst);

    my_atomic.store(10, Ordering::SeqCst);
    assert!(my_atomic.load(Ordering::SeqCst) == 10);

    let my_arc_atomic = Arc::new(AtomicUsize::new(4));

    let second_ref_atomic = my_arc_atomic.clone();
    thread::spawn(move || {
        second_ref_atomic.store(42, Ordering::SeqCst);
    });
}
