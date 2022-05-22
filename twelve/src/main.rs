// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

// On the first day of Christmas my true love sent to me
// A partridge in a pear tree.

// On the second day of Christmas my true love sent to me
// Two turtle doves,
// And a partridge in a pear tree.

// On the third day of Christmas my true love sent to me
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.

// a partridge in a pear tree
// two turtle doves
// three French hens
// four calling birds
// five gold rings
// six geese a-laying
// seven swans a-swimming
// eight maids a-milking
// nine ladies dancing
// ten lords a-leaping
// eleven pipers piping
// twelve drummers drumming


fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eighth", "nineth", "tenth", "eleventh", "twelveth"];
    let items = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for i in 1..13 {
        println!("On the {} day of Christmas my true love sent to me", days[i-1]);
        for j in 1..(i+1) {
            let k = i-j;
            if k == 0 && i != 1{
                println!("And {}",items[k]);
            } else {
                println!("{}",make_ascii_titlecase(items[k].to_string()));
            }
        }

    }
}

fn make_ascii_titlecase(s: String) -> String{
    let mut s = s;
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    s
}
