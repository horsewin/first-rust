// use ferris_says::say;
// use std::io::{stdout, BufWriter};
//
// fn main() {
//     let stdout = stdout();
//     let out = b"hogehoge";
//     let width = 12;
//
//     let mut writer = BufWriter::new(stdout.lock());
//     say(out, width, &mut writer).unwrap();
//     println!("Hello, world!");
// }

// this is equivalent to use rand;
// extern crate rand;
use rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess number!");

    let secret_number = rand::thread_rng().gen_range(1, 50);

    loop {
        println!("Input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}