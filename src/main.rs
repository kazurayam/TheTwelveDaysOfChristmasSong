fn main() {
    println!("");
    println!("# The Twelve Days of Christmas");
    println!("");

    let items = ["A partridge in a pear tree.",
                 "Two turtle doves, and",
                 "Three French hens",
                 "Four calling birds",
                 "Five gold rings",
                 "Six geese a-laying",
                 "Seven swans a-swimming",
                 "Eight maids a-milking",
                 "Nine ladies dancing",
                 "Ten lords a-leaping",
                 "Eleven pipers piping",
                 "Twelve drummers drumming"];

    for seq in 1..(items.len()+1) {
        println!("On the {} day of Christmas,", ordinal(seq));
        println!("My true love sent to me");
        for index in (0..seq).rev() {
            println!("{}", items[index]);
        }
        println!("")
    }
}

fn ordinal(seq: usize) -> &'static str {
    let result = match seq {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "nineth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelveth",
        _ => "special",
    };
    result
}
