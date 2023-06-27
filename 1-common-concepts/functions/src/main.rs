fn main() {
    println!("Hello, world!");
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 5;
        x + 1
    };

    println!("y = {y}");

    let x = plus_one(5);
    println!("x = {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
