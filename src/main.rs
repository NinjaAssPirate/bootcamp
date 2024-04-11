fn main() {
    println!("Welcome to the Fizz Buzz Program!");
    fizz_buzz();
}

fn fizz_buzz() {
    let mut fizz_buzz_count = 0;
    for n in 1..=300 {
        if n % 15 == 0 {
            println!("fizz buzz");
            fizz_buzz_count += 1;
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    println!("'fizz buzz' occurred {} times.", fizz_buzz_count);
}