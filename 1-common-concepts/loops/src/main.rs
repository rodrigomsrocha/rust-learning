fn main() {
    let mut count = 0;

    'counting_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("ramaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end of count = {count}\n");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF\n");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}")
    }

    println!("\n");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF");
}
