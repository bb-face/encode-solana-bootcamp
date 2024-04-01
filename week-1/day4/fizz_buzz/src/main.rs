fn main() {
    println!("welcome");

    let result = fizz_buzz();

    println!("the result is: {}", result);
}

fn fizz_buzz() -> i32 {
    let mut counter = 0;

    for number in 0..302 {
        match (number % 3, number % 5) {
            (0, 0) => {
                counter += 1;
                println!("fizz buzz");
            }
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", number),
        }
    }

    return counter;
}
