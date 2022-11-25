wai_bindgen_rust::export!("strings-and-lists.wai");

struct StringsAndLists;

impl strings_and_lists::StringsAndLists for StringsAndLists {
    fn greet(name: String) -> String {
        format!("Hello, {name}!")
    }

    fn greet_many(people: Vec<String>) -> String {
        match people.as_slice() {
            [] => "Oh, nobody's there...".to_string(),
            [person] => format!("Hello, {person}!"),
            [people @ .., last] => {
                let people = people.join(", ");
                format!("Hello, {people}, and {last}!")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strings_and_lists::StringsAndLists as _;

    #[test]
    fn greet_one_person() {
        let person_name = String::from("Michael");
        let result = StringsAndLists::greet(person_name.clone());
        assert_eq!(
            format!("Hello, {person_name}!"),
            result,
            "The strings aren't equal"
        );
    }
    #[test]
    fn greet_many_people() {
        let people = vec![
            "Rudra".to_string(),
            "Michael".to_string(),
            "Syrus".to_string(),
        ];
        let answer = "Hello, Rudra, Michael, and Syrus!".to_string();
        let result = StringsAndLists::greet_many(people);
        assert_eq!(answer, result);
    }
}
