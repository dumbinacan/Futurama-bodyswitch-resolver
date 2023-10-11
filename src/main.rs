use futurama_bodyswitch_resolver::bodyswitcher::Person;
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
