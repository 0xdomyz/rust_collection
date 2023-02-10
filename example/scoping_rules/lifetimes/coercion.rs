// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Longer lifetime
    
    let first_reference = &first;

    {
        let second = 3; // Shorter lifetime
        
        println!("The product is {}", multiply(&first, &second));

        println!("{} is the first", choose_first(first_reference, &second));
    };

    println!("first_reference is still valid: {}", first_reference);
}
