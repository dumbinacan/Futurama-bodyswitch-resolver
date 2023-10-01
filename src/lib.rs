#[cfg(test)]
mod tests {
    #[test]
    fn can_swap() {
        // two people who have never swapped /* maybe also when either one never swapped */
        let case1_a = Person::new();
        let case1_b = Person::new();

        // two people that have swapped but not with each other
        // two people who have swapped with each other
        // let can_swap = Person::can_swap(
        assert!( Person::can_swap( case1_a, case1_b ) );
    }
}
