use std::fmt::format;

fn main() {
    christmas_song()
}

fn christmas_song() -> () {
    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves, and", 
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut index_main = 0;
    while lyrics.len() > index_main {
        let mut index_print = index_main.clone();
        let day_tracker = [
            "first",
            "second",
            "third",
            "fourth",
            "fifth",
            "sixth",
            "seventh",
            "eighth",
            "nineth",
            "teneth",
            "eleventh",
            "twelveth",
        ];
        println!("On the {} day of Christmas, my true love sent to me", day_tracker[index_main]);

        if index_main != 0 {
            while index_print > 0 {
                
                println!("{}", lyrics[index_print]);
                index_print -= 1;
            };
        } 
        println!("{}", lyrics[0]);
        

        index_main += 1;


        println!("======");

        if lyrics.len() == index_main {
            println!("It's boxing day!")
        }
    }

}