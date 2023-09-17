struct Person {
    body: usize,
    mind: usize,
    previous_swaps: Vec<usize>,
}

impl Person {
    fn new() -> Person {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static PERSON_ID: AtomicUsize = AtomicUsize::new(0);
        let this_person = PERSON_ID.fetch_add(1, Ordering::Relaxed);
                        // PERSON_ID++
        Person {
            body: this_person,
            mind: this_person,
            previous_swaps: Vec::new(),
        }
    }

    // tries to swap 2 people's minds returns true if succeeds else false
    fn swap(&mut self, swoop: &mut Person) -> bool {
        if !&self.can_swap(swoop) { return false; }

        // keep track of the body swaps
        &self.previous_swaps.push(swoop.body);
        &swoop.previous_swaps.push(self.body);

        let tmp = swoop.mind;
        swoop.mind = self.mind;
        self.mind = tmp;

        true
    }

    // checks if these two bodies have swapped before
    fn can_swap(&self, swoop: &Person) -> bool {
        let vec_a = &self.previous_swaps;
        let vec_b = &swoop.previous_swaps;

        // one of them has never swapped with anyone before
        if vec_a.is_empty() || vec_b.is_empty() {
            return true;
        }

        for person in vec_a {
            if person == &swoop.body {
            // these bodies have swapped before
                return false;
            }
        }

        return true;
    }
}

use std::fmt;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "body: {}, mind: {}", self.body, self.mind)
    }
}

fn main() {
    let mut people: Vec<Person> = Vec::new();
    for i in 0..=15 {
        people.push(Person::new());
    }
    for person in people {
        println!("{}", person);
    }
}
