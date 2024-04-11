use std::fmt::Display;

fn main() {
    println!("I had Homework about buzzy fizzes");
    let fizz_buzz_count = fizz_buzz();
    println!("'Fizzes Buzzy' occurred {} times.", fizz_buzz_count);
}

fn fizz_buzz() -> i32 {
    (1..=301).fold(0, |acc, n| {
        let output = match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz",
            (0, _) => "Fizz",
            (_, 0) => "Buzz",
            _ => "",
        };

dbg!(n, output);

        if !output.is_empty() {
            println!("{} - {}", n, output);
        }

        acc + if output == "FizzBuzz" { 1 } else { 0 }
    })
}