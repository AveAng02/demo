// Print the lyrics to the Christmas carol �The Twelve Days of Christmas,� taking advantage of the repetition in the song.

//use std::io;


fn main() {
    let chorus = "On the first day of Christmas\nMy true love sent to me";

    let lines = ["12 drummers drumming", "Eleven pipers piping",
                    "Ten lords a-leaping",
                    "Nine ladies dancing",
                    "Eight maids a-milking",
                    "Seven swans a-swimming",
                    "Six geese a-laying",
                    "Five golden rings",
                    "Four calling birds",
                    "Three French hens",
                    "Two turtle-doves",
                    "A partridge in a pear tree"];

    for i in (0..12).rev() {
        println!("\n{}", chorus);

        for j in i..12 {
            println!("{}", lines[j]);
        }
    }
}

