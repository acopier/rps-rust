use rand::seq::SliceRandom;
use std::{env, io, thread, time};

fn user_input() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to capture input!");

    let input = buffer.trim().to_lowercase();

    match input.as_str() {
        "r" => "rock",
        "p" => "paper",
        "s" => "scissors",
        "y" => "y",
        "n" => "n",
        _ => &input,
    }
    .to_owned()
}

fn bot(options: [&str; 3]) -> &str {
    let mut rng = rand::thread_rng();
    options.choose(&mut rng).unwrap()
}

fn result(user: &str, bot: &str) -> String {
    let end_status =
        |message: &str| -> String { format!("[you] {} | {} | {} [bot]", user, message, bot) };

    if (bot == "rock" && user == "paper")
        || (bot == "paper" && user == "scissors")
        || (bot == "scissors" && user == "rock")
    {
        end_status(">")
    } else if (bot == "rock" && user == "rock")
        || (bot == "paper" && user == "paper")
        || (bot == "scissors" && user == "scissors")
    {
        end_status("==")
    } else {
        end_status("<")
    }
}

fn game(interactive: bool, mut user_choice: String) {
    let options = ["rock", "paper", "scissors"];
    let bot_choice = bot(options);
    match interactive {
        true => loop {
            let sleep_time = time::Duration::from_millis(500);

            println!("Rock, paper or scissors [r/p/s]: ");
            user_choice = user_input();
            println!("--------");
            println!("rock");
            thread::sleep(sleep_time);
            println!("paper");
            thread::sleep(sleep_time);
            println!("scissors");
            thread::sleep(sleep_time);
            println!("shoot!");
            thread::sleep(sleep_time);
            println!("--------");
            println!("{}\n", result(&user_choice, bot_choice));
            println!("Continue playing? [y/any other key]: ");
            let user_end = &user_input();
            match user_end.trim() {
                "y" => continue,
                _ => break,
            };
        },
        false => println!("{}", result(&user_choice, bot_choice)),
    }
}

fn main() {
    let mut interactive = true;
    let mut choice = "";
    // TODO (acopier): Refactor
    if let Some(arg) = env::args().nth(1) {
        match arg.as_str() {
            "--no-interactive" => {
                interactive = false;
                if let Some(arg) = env::args().nth(2) {
                    match arg.to_lowercase().as_str() {
                        "rock" => choice = "rock",
                        "paper" => choice = "paper",
                        "scissors" => choice = "scissors",
                        _ => {
                            panic!("Invalid choice! available choices are [rock, paper, scissors]")
                        }
                    }
                } else {
                    panic!("Invalid choice! available choices are [rock, paper, scissors]")
                }
            }
            _ => panic!("Invalid argument! available arguments are [--no-interactive]"),
        }
    };
    game(interactive, choice.to_owned())
}
