// rustc next_birthday.rs && ./next_birthday

fn next_birthday(current_age: Option<u8>) -> Option<String> {
	// If `current_age` is `None`, this returns `None`.
	// If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}


fn main () {
    let age = Some(27);
    let next_age = next_birthday(age);
    println!("{:?}", next_age);

    let no_age = None;
    let next_age = next_birthday(no_age);
    println!("{:?}", next_age);
}