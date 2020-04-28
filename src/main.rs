#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

struct Human {
    name: String,
    age: u32,
    gender: Gender,
}

fn main() {
    let matias = Human {
        name: String::from("Matias"),
        age: 1231,
        gender: Gender::Female,
    };
    let rosalis = Human {
        name: String::from("Rosalis"),
        age: 11231,
        gender: Gender::Male,
    };
    for human in [&matias, &rosalis].iter() {
        println!(
            "{} has {} years old, gender:{:?}",
            human.name, human.age, human.gender
        );
    }
    for letter in matias.name.chars() {
        println!("{}", letter)
    }
}
