const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "a Partridge in a Pear Tree",
    "Two Turtle Doves",
    "Three French Hens",
    "Four Calling Birds",
    "Five Golden Rings",
    "Six Geese a-Laying",
    "Seven Swans a-Swimming",
    "Eight Maids a-Milking",
    "Nine Ladies Dancing",
    "Ten Lords a-Leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers Drumming",
];

fn main() {
    sing_carol();
}

fn sing_carol() {
    for (day_of_christmas, ordinal) in ORDINALS.iter().enumerate() {
        println!("On the {ordinal} day of Christmas, my true love gave to me:");
        print_gifts(day_of_christmas);
        println!("\n");
    }
}

fn print_gifts(day: usize) {
    for gift in GIFTS[..=day].iter().rev() {
        let is_last_gift = day > 0 && gift == &GIFTS[0];
        if is_last_gift {
            print!("And ");
        }
        println!("{}", gift);
    }
}
