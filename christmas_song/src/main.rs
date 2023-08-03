fn main() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eight",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    let lines = [
        "and a Partridge in a Pear Tree",
        "2 Turtle Doves",
        "3 French Hens",
        "4 Calling Birds",
        "5 Golden Rings",
        "6 Geese a Laying",
        "7 Swans a Swimming",
        "8 Maids a Milking",
        "9 Ladies Dancing",
        "10 Lords a Leaping",
        "11 Pipers Piping",
        "12 Drummers Drumming"
    ];

    for (index, day) in days.iter().enumerate() {
        println!("On the {day} day of Christmas\n\
            my true love sent to me:");

        if index == 0 {
            println!("A Partridge in a Pear Tree");
        } else {
            for song_lines in (0..index + 1).rev() {
                println!("{}", lines[song_lines]);
            }
        }

        println!();
    }
}