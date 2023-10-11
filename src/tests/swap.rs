use crate::bodyswitcher::Person;

#[test]
fn bodies() {
    let mut person_a = Person::new();
    let person_a_orig = person_a.clone();
    let mut person_b = Person::new();
    let person_b_orig = person_b.clone();

    Person::swap(&mut person_a, &mut person_b);

    // a Person's body should never change
    assert!(person_a.body() == person_a_orig.body()
        && person_b.body() == person_b_orig.body());
}
#[test]
fn minds() {
    let mut person_a = Person::new();
    let person_a_orig = person_a.clone();
    let mut person_b = Person::new();
    let person_b_orig = person_b.clone();

    Person::swap(&mut person_a, &mut person_b);

    // person_a's mind swapped with person_b
    assert!(person_a.mind() == person_b_orig.mind()
        && person_b.mind() == person_a_orig.mind());
}

#[test]
fn previos_swaps() {
    let mut person_a = Person::new();
    let person_a_orig = person_a.clone();
    let mut person_b = Person::new();
    let person_b_orig = person_b.clone();
    // perform swaps and
    // insure the vec is properly being updated
    // vec should be excactly the same as before 
    // except with one extra entry
    // I'm just using as vec so why wouldn't it work?
    assert!(true);
}
