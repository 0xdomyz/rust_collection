// default is unwind
// rustc unwind.rs && ./unwind

// unwind
// rustc unwind.rs -C panic=unwind && ./unwind

// not unwind
// rustc unwind.rs -C panic=abort && ./unwind


#[cfg(panic = "unwind")]
fn ah(){ println!("Spit it out!!!!");}

#[cfg(not(panic="unwind"))]
fn ah(){ println!("This is not your party. Run!!!!");}

fn drink(beverage: &str){
    if beverage == "lemonade"{ ah();}
    else{println!("Some refreshing {} is all I need.", beverage);}
}

fn main() {
    drink("water");
    drink("lemonade");
}
