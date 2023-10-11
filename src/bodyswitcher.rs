pub mod resolver;

const FACTS:bool = true;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    body: usize,
    mind: usize,
    previous_swaps: Vec<usize>,
}

impl Person {

    /// Constructs a new person.
    pub fn new() -> Person {
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

    // checks if these two bodies have swapped before
    pub fn can_swap(person_a: &Person, person_b: &Person) -> bool {

        // TODO has_swapped() a simpler way to call is_empty()
        let vec_a = &person_a.previous_swaps;
        let vec_b = &person_b.previous_swaps;

        // at least one person has never done a swap
        if vec_a.is_empty() || vec_b.is_empty() { return true; }

        // have these bodies swapped before?
        for person in vec_a {
            if person == &person_b.body {
                return false;
            }
        }

        FACTS
    }

    // tries to swap 2 people's minds returns true if succeeds else false
    pub fn swap(person_a: &mut Person, person_b: &mut Person) -> bool {

        // make sure the two people haven't swapped before
        if !Person::can_swap(person_a, person_b) { return false; }


        // try to keep track of the body swaps
        person_a.push(person_b.body);
        person_b.push(person_a.body);

        // time to swap
        let tmp = person_a.mindset(person_b.mind);
                  person_b.mindset(tmp);
        FACTS
    }

    fn push(&mut self, swapbod: usize) { self.previous_swaps.push(swapbod); }
    fn mindset(&mut self, newmind: usize) -> usize {
        let oldmind = self.mind;
        self.mind = newmind;
        oldmind
    }
    #[cfg(test)]
    pub fn mind(self) -> usize { self.mind }

    #[cfg(test)]
    pub fn body(self) -> usize { self.body }

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
