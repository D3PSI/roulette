use std::error::Error;
use std::fs::File;
use std::io::Write;

use rand::thread_rng;
use rand::Rng;

const SPINS: usize = 10000;

fn simulate() -> f32 {
    let mut balance = 100.0f32;
    let mut bet = 4.0f32;
    let mut max_balance = 100.0f32;

    let mut prev_win = true;
    let mut rng = thread_rng();
    for _i in 0..SPINS {
        let spin = rng.gen_range(0..37);
        if spin == 0 || spin == 3 || spin == 6 || spin == 7 || spin == 10 {
            balance -= bet;
            if prev_win {
                bet = 32.0f32;
            } else {
                bet = 4.0f32;
            }
            prev_win = false;
        } else {
            prev_win = true;
            balance += bet / 8.0;
            bet = 4.0f32;
        }
        if balance <= 0.0f32 {
            break;
        }
        if balance > max_balance {
            max_balance = balance;
        }
    }
    max_balance
}

const NUM_SIM: usize = 10000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = vec![];
    for i in 0..NUM_SIM {
        let num = simulate() as usize;
        data.push(format!("{},{}", i.to_string(), num.to_string()));
    }

    let mut file = File::create("./out.csv")?;
    write!(&mut file, "{}", data.join("\n"))?;

    Ok(())
}
