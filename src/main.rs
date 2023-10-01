const FACTS:bool = true;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Person {
    body: usize,
    mind: usize,
    previous_swaps: Vec<usize>,
}

impl Person {

    /// Constructs a new person.
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

    fn push(&mut self, swapbod: usize) { self.previous_swaps.push(swapbod); }
    fn mindset(&mut self, newmind: usize) -> usize {
        let oldmind = self.mind;
        self.mind = newmind;
        oldmind
    }

    // tries to swap 2 people's minds returns true if succeeds else false
    fn swap(sweep: &mut Person, swoop: &mut Person) -> bool {

        // make sure the two people haven't swapped before
        if !Person::can_swap(sweep, swoop) { return false; }


        // try to keep track of the body swaps
        sweep.push(swoop.body);
        swoop.push(sweep.body);

        // time to swap
        let tmp = sweep.mindset(swoop.mind);
                  swoop.mindset(tmp);
        FACTS
    }

    // checks if these two bodies have swapped before
    fn can_swap(sweep: &Person, swoop: &Person) -> bool {

        // TODO has_swapped() a simpler way to call is_empty()
        let vec_a = &sweep.previous_swaps;
        let vec_b = &swoop.previous_swaps;

        // at least one person has never done a swap
        if vec_a.is_empty() || vec_b.is_empty() { return true; }

        // have these bodies swapped before?
        for person in vec_a {
            if person == &swoop.body {
                return false;
            }
        }

        FACTS
    }
}

use std::fmt;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

      /*
        /* include previous swaps */
        let mut past_swaps = format!("[ ");
        for person in self.previous_swaps.iter() {
            past_swaps = format!("{} {}", past_swaps, person);
        }
        past_swaps = format!("{} {}", past_swaps, "]");

        write!(f, "body: {}, mind: {}, previous_swaps: {}", self.body, self.mind, past_swaps)
      */

        write!(f, "body: {}, mind: {}", self.body, self.mind)
    }
}


fn main() {
    let mut people: Vec<Person> = Vec::new();
    let mut p2: Vec<Person> = Vec::new();

    for _ in 0..=15 {
        people.push(Person::new());
        p2.push(Person::new());
    }

    for (person_a, person_b) in people.iter_mut().zip(p2.iter_mut()) {
        if Person::swap(person_a, person_b) {
            println!("{}, {}", person_a, person_b);
        }
    }

    for person_a in people.iter_mut() {
        for person_b in p2.iter_mut() {
            if Person::swap(person_a, person_b) {
                println!("{}, {}", person_a, person_b);
            }
        }
    }
}
