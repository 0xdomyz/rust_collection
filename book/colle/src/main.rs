enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // vectors

    // def
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];

    // alter
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // index or get mtd
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // iterate via immutable ref
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // iterate over mut ref
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // store dif types under an enum type
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


}

