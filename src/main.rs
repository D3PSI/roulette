use std::process::exit;

use rand::thread_rng;
use rand::Rng;

const SPINS: usize = 1000;

fn main() {
    let mut balance = 100.0f32;
    let mut bet = 4.0f32;

    let mut prev_win = true;
    let mut rng = thread_rng();
    for i in 0..SPINS {
        let spin = rng.gen_range(0..37);
        println!("bet: {}", bet);
        if spin == 0 || spin == 3 || spin == 6 || spin == 7 || spin == 10 {
            balance -= bet;
            println!("lose");
            if prev_win {
                bet = 32.0f32;
            } else {
                bet = 4.0f32;
            }
            prev_win = false;
        } else {
            prev_win = true;
            balance += bet * 9.0 / 8.0;
            println!("win");
            bet = 4.0f32;
        }
        if balance <= 0.0f32 {
            println!("no money");
            exit(0);
        }
        println!("{}", balance);
    }
}
