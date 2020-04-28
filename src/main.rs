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
    let humans = [&matias, &rosalis];
    println!(
        "{} has {} years old, gender:{:?}",
        matias.name, matias.age, matias.gender
    );
    println!(
        "{} has {} years old, gender:{:?}",
        rosalis.name, rosalis.age, rosalis.gender
    );
}
