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

struct PotionlessFool {
    hp: i32,
}

struct SomeoneWithPotions {
    hp: i32,
    potion_count: i32,
}

impl SomeoneWithPotions {
    fn is_alive(&self) -> bool {
        self.hp > 0
    }
    /// Try to attack the opponent. Return true if the move was successful.
    fn attack_opponent(&self, opponent: &mut PotionlessFool) -> bool {
        opponent.hp -= 10;
        println!("You have dealt 10 hp damage to your opponent.");
        true
    }

    /// Try to drink a potion. Return true if the move was successful.
    fn drink_potion(&mut self) -> bool {
        if self.potion_count > 0 {
            self.hp += 30;
            println!("You drank a potion. You healed 30 hp.");
            if self.hp > 50 {
                self.hp = 50;
                println!("You are fully healed, but you wasted some of your potion.");
            }
            self.potion_count -= 1;
            return true;
        } else if self.potion_count == 0 {
            println!("Sorry, you're out of potions.");
            return false;
        } else {
            println!("Something went wackadoodle.");
            return false;
        }
    }
}

impl PotionlessFool {
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn attack_player(&self, player: &mut SomeoneWithPotions) -> bool {
        player.hp -= 10;
        println!("Your opponent dealt 10 hp damage.\n");
        true
    }
}

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
    let mut player = SomeoneWithPotions {
        hp: 50,
        potion_count: 3,
    };

    // fn attack_opponent(&self, opponent: &mut Opponent) -> bool {

    let mut opponent = PotionlessFool { hp: 100 };
    // let mut opponent = Player {
    //     hp: 50,
    //     potion_count: 3,
    // };

    println!("Welcome!  You're in a boss fight!  Isn't that exciting...\n");
    while player.is_alive() && opponent.is_alive() {
        println!(
            "Your current hp is {} and your opponent's hp is {}",
            player.hp, opponent.hp
        );
        println!("Would you like to attack or use a potion?");
        let Some(input_line) = get_player_move() else {
            return;
        };
        // Same as:
        // let input_line = match get_player_move() {
        //     Some(x) => x,
        //     _ => return,
        // };

        // match thing {
        //     pattern => code,
        //     pattern => code,
        // }

        let player_move;
        match input_line {
            PlayerMove::Attack => {
                player_move = player.attack_opponent(&mut opponent);
            }

            PlayerMove::Potion => {
                player_move = player.drink_potion();
            }

            PlayerMove::Unknown(wat) => {
                println!("Please choose either 'attack' or 'potion'. '{wat}' is not an option, you fool.");
                player_move = false;
            }
        }

        if player_move {
            opponent.attack_player(&mut player);
        }
    }
    if !player.is_alive() {
        if !opponent.is_alive() {
            println!(
                "While you defeated your opponent, you are also dead, so you can't celebrate."
            );
        }
        println!("Game over, you lost.");
    } else if !opponent.is_alive() {
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
