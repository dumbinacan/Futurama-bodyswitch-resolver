use crate::bodyswitcher::Person;
// two people who have never swapped
#[test]
fn two_new_persons() {
    let person_a = Person::new();
    let person_b = Person::new();
    assert!( Person::can_swap(&person_a, &person_b) );
}

// one person who has never swapped
#[test]
fn one_new_person() {
    let person_a = Person::new();
    let mut person_b = Person::new();
    let mut person_c = Person::new();
    Person::swap(&mut person_b, &mut person_c);
    assert!( Person::can_swap(&person_a, &person_b) );
}

// two people that have swapped but not with each other
#[test]
fn two_swappable_persons() {
    let mut person_a = Person::new();
    let mut person_b = Person::new();
    let mut person_c = Person::new();
    let mut person_d = Person::new();
    Person::swap(&mut person_b, &mut person_c);
    Person::swap(&mut person_a, &mut person_d);
    assert!( Person::can_swap(&person_a, &person_b) );
}

// two people who have swapped with each other
#[test]
fn two_unswappable_persons() {
    let mut person_a = Person::new();
    let mut person_b = Person::new();
    Person::swap(&mut person_a, &mut person_b);
    assert!( !Person::can_swap(&person_a, &person_b) );
}
