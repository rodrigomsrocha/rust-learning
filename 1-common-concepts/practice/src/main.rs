use std::io;

use terminal_menu::{button, label, menu, mut_menu, run};

const LYRICS: [&str; 12] = [
    "A Partridge in a Pear Tree",
    "Two Turtle Doves",
    "Three French Hens",
    "Four Calling Birds",
    "Five Gold Rings",
    "Six Geese A-Laying",
    "Seven Swans A-Swimming",
    "Eight Maids A-Milking",
    "Nine Ladies Dancing",
    "Ten Lords A-Leaping",
    "Eleven Pipers Piping",
    "Twelve Drummers Drumming",
];

fn main() {
    let menu = menu(vec![
        // tsitle
        label("-----------------"),
        label("Basic exercises"),
        label("use wasd or arrow keys"),
        label("enter to select"),
        label("'q' or esc to exit"),
        label("-----------------\n"),
        // options
        button("Convert Fahrenheit to Celsius"),
        button("Generate nth Fibonacci number"),
        button("Sing 'The Twelve Days of Christmas'"),
    ]);

    run(&menu);

    if mut_menu(&menu).selected_item_index() == 6 {
        convert_fahrenheit_to_celcius()
    } else if mut_menu(&menu).selected_item_index() == 7 {
        generate_nth_fibonacci()
    } else {
        twelve_days_of_christmas()
    }
}

fn convert_fahrenheit_to_celcius() {
    println!("Fahrenheit to Celsius conversion");
    println!("Give us the temperature to convert:");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Please type a number");

    let celsius: f64 = (((fahrenheit - 32.0) / 1.8) * 100.0).round() / 100.0;

    println!("{fahrenheit}ºF is equal to {celsius}ºC")
}

fn generate_nth_fibonacci() {
    println!("Nth fibonacci number generator");
    println!("Give us the nth fibonacci number you want:");

    let mut nth = String::new();

    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");

    let nth = nth.trim().parse().expect("Please type a number");

    println!("fibonacci = {}", fib(nth))
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn twelve_days_of_christmas() {
    for (day, line) in LYRICS.iter().enumerate() {
        let ordinal_suffix = match day + 1 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };

        println!(
            "On the {}{} day of Christmas my true love gave to me,",
            day + 1,
            ordinal_suffix
        );

        if day == 0 {
            println!("{line}.")
        } else {
            for x in (0..(day + 1)).rev() {
                if let 0 = x {
                    println!("And {}.", LYRICS[0])
                } else {
                    println!("{},", LYRICS[x])
                }
            }
        }
    }
}
