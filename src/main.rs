use rand::Rng;
use std::{
    io::{self, Write},
    process::exit,
};

#[derive(PartialEq)]
enum GameState {
    InProgress,
    Complete,
}

#[derive(PartialEq)]
enum Closeness {
    Same,
    Hot,
    Hotter,
    Cold,
    Colder,
    Hit,
}

fn main() {
    let mut rng = rand::thread_rng();
    let game_answer: i32 = rng.gen_range(0..100);

    let mut game_state = GameState::InProgress;
    let mut input_guess: i32;
    let mut last_answer_diff = 0;

    while game_state == GameState::InProgress {
        print!("Guess a number between 1 and 100: ");
        Write::flush(&mut io::stdout()).unwrap();
        input_guess = get_input();
        let answer_accuracy: Closeness =
            test_answer(input_guess, &mut last_answer_diff, game_answer);
        match answer_accuracy {
            Closeness::Same => println!("C'mon, give me something new!"),
            Closeness::Hot => println!("Wow, your guess is flaming!"),
            Closeness::Hotter => println!("You're getting closer!"),
            Closeness::Cold => println!("You're way off mate!"),
            Closeness::Colder => println!("Off the mark further mate!"),
            Closeness::Hit => {
                println!("Nice. Bullseye!");
                game_state = GameState::Complete;
            }
        }
    }
}

fn test_answer(number: i32, last_answer_diff: &mut i32, answer: i32) -> Closeness {
    if number == answer {
        return Closeness::Hit;
    }
    let diff = (number - answer).abs();
    let closeness: Closeness;
    if *last_answer_diff != 0 {
        match diff.cmp(last_answer_diff) {
            std::cmp::Ordering::Less => closeness = Closeness::Hotter,
            std::cmp::Ordering::Equal => closeness = Closeness::Same,
            std::cmp::Ordering::Greater => closeness = Closeness::Colder,
        }
    } else if diff <= 30 {
        closeness = Closeness::Hot;
    } else {
        closeness = Closeness::Cold;
    }

    *last_answer_diff = if !(closeness == Closeness::Same) {
        diff
    } else {
        *last_answer_diff
    };
    closeness
}

fn get_input() -> i32 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => {
            println!("error: {}", error);
            exit(1)
        }
    }
    let number = match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Enter numbers only between 1 and 100");
            exit(1);
        }
    };
    if !(1..=100).contains(&number) {
        println!("Enter numbers only between 1 and 100");
        exit(1);
    }
    number
}
