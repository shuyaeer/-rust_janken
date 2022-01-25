use std::io;
use rand::seq::SliceRandom;

fn main() {
    println!("input user hand.");
    loop {
        let result = janken();
        if result == true {break;}
    }
}

fn janken() -> bool {
    let mut user_hand = String::new();
    io::stdin().read_line(&mut user_hand).expect("Failed to read line.");
    let hands = ["g", "c", "p"];
    let hands: Vec<_> = hands
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();
    let user_hand: &str = &user_hand.trim();
    let machine_hand = hands[0];
    println!("Your hand is {}.", user_hand);
    println!("Machine hand is {}.", machine_hand);
    if &user_hand == machine_hand {
        println!("Please try again.");
        return false
    } else if (&user_hand == &"g" && machine_hand == &"c") || (&user_hand == &"c" && machine_hand == &"p") || (&user_hand == &"p" && machine_hand == &"g") {
        println!("You win!");
        return true
    } else {
        println!("You lose!");
        return true
    }
}
