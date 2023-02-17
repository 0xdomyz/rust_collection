// rustc using_result_in_main.rs && ./using_result_in_main

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    // let number_str = "10";
    let number_str = "1dsf0";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
