struct Person {
    body: usize,
    mind: usize,
    previous_swaps: Vec<usize>,
}

impl Person {
    fn new() -> Person {
        // each person is represented by a unique ID
        // starting from 0 and counting up 1 by 1
        use std::sync::atomic::{AtomicUsize, Ordering};
        static PERSON_ID: AtomicUsize = AtomicUsize::new(0);
        let this_person: usize = PERSON_ID.fetch_add(1, Ordering::Relaxed);
        Person {
            body: this_person,
            mind: this_person,
            previous_swaps: Vec::new(),
        }
    }
    fn can_swap(&self, swoop: &Person) -> bool {
        // check if the two bodies have previously swapped
        todo!();
    }
}
fn main() {
    println!("Hello, world!");
}
