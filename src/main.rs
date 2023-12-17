use std::io::stdin;

// Actual Rust source code (mostly™)
// enum Option<T> {
//     None,
//     Some(T),
// }
//
// enum Result<T,E> {
//     Ok(T),
//     Err(E),
// }

/// Possible actions a player can take.
enum PlayerMove {
    /// Attack the opponent.
    Attack,
    /// Drink a healing potion, if (and only if) one is available.
    Potion,
    /// An unknown command was entered, and here it is.
    Unknown(String),
}

/// Get a line of input from the player. Returns `None` if there was no input
/// and never will be again.
fn get_player_move() -> Option<PlayerMove> {
    let input_line = match stdin().lines().next() {
        Some(woot) => woot,
        None => {
            println!("Well I guess you're done with the game. Bye!");
            return None;
        }
    };
    let input_line = match input_line {
        Ok(yay) => yay,
        Err(uhoh) => {
            println!("There was an error! {uhoh}");
            return None;
        }
    };
    let player_move = match input_line.as_str() {
        "attack" | "a" => PlayerMove::Attack,
        "potion" | "p" | "drink" | "d" => PlayerMove::Potion,
        _ => PlayerMove::Unknown(input_line),
    };
    Some(player_move)
}

fn main() {
    let mut player_hp = 50;
    let mut opponent_hp = 100;
    let mut potion_count = 3;

    println!("Welcome!  You're in a boss fight!  Isn't that exciting...\n");
    while player_hp > 0 && opponent_hp > 0 {
        println!(
            "Your current hp is {player_hp} and your opponent's hp is {}",
            opponent_hp
        );
        println!("Would you like to attack or use a potion?");
        let Some(input_line) = get_player_move() else {
            return;
        };
        let mut player_move = false;

        // match thing {
        //     pattern => code,
        //     pattern => code,
        // }

        match input_line {
            PlayerMove::Attack => {
                opponent_hp -= 10;
                println!("You have dealt 10 hp damage to your opponent.");
                player_move = true;
            }
            PlayerMove::Potion => {
                if potion_count > 0 {
                    player_hp += 30;
                    println!("You drank a potion. You healed 30 hp.");
                    if player_hp > 50 {
                        player_hp = 50;
                        println!("You are fully healed, but you wasted some of your potion.");
                    }
                    potion_count -= 1;
                    player_move = true;
                } else if potion_count == 0 {
                    println!("Sorry, you're out of potions.");
                } else {
                    println!("Something went wackadoodle.");
                }
            }
            PlayerMove::Unknown(wat) => {
                println!("Please choose either 'attack' or 'potion'. '{wat}' is not an option, you fool.");
            }
        }

        if player_move {
            player_hp -= 10;
            println!("Your opponent dealt 10 hp damage.\n");
        }
    }
    if player_hp == 0 {
        if opponent_hp == 0 {
            println!(
                "While you defeated your opponent, you are also dead, so you can't celebrate."
            );
        }
        println!("Game over, you lost.");
    } else if opponent_hp == 0 {
        println!("Congratulations, you won!");
    } else {
        println!("I don't know what happened.");
    }
}

/*
std::io::stdin() (gives us a Stdin)
  .lines() (gives us a Lines???)
  .next() (gives us Option<Result<String, _>>)
  .unwrap() (gives us Result<String, _>)
  .unwrap() (gives us String)
  yay a String! <---

Option<T> = there might be a T, there might be no T
Result<T, E> = we might succeed and give you a T, we might fail and give you an E
*/
