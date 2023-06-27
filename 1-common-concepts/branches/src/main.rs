fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true")
    } else {
        println!("condition was false")
    }

    if number == 7 {
        println!("the number is seven")
    } else if number != 0 {
        println!("the number was something other than zero")
    } else {
        println!("the number is zero")
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("the value of the number is {number}")
}
