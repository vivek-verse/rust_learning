struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

struct PersonIterator {
    values: Vec<String>,
}

impl Iterator for PersonIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        } else {
            Some(self.values.remove(0))
        }
    }
}

impl IntoIterator for Person {
    type Item = String;
    type IntoIter = PersonIterator;

    fn into_iter(self) -> Self::IntoIter {
        PersonIterator {
            values: vec![self.first_name, self.last_name, self.occupation],
        }
    }
}

fn main() {
    let p = Person {
        first_name: "Vivek".to_owned(),
        last_name: "Yadav".to_owned(),
        occupation: "Software Engineer".to_owned(),
    };

    for item in p {
        println!("{}", item);
    }
}
