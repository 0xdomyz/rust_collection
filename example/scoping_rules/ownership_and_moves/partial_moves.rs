fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
        height: u8,
        gender: String,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
        height: 180,
        gender: String::from("Male"),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age, height, ref gender } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);
    println!("The person's height is {}", height);
    println!("The person's gender is {}", gender);

    // Error! borrow of partially moved value: `person` partial move occurs
    // println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);

    println!("The person's gender from person struct is {}", person.gender);
    println!("The person's height from person struct is {}", person.height);

}
