fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("x in the inner scope is {x}");
    }

    println!("x = {x}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("no. spaces: {spaces}")
}
