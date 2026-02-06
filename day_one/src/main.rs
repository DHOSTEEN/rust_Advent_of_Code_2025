use std::fs;
use std::num::ParseIntError;

const MOD:u32 = 100;

fn main() {
    let file = fs::read_to_string("day_one_input.txt").expect("file doens't exist");
    
    let _ =run_day_one(&file);
    let _ = run_day_two(&file);    
}

fn run_day_one(file_info: &str) -> Result<(), ParseIntError> {

    let inputs  = file_info.split_whitespace();
    let mut safe = Safe {
        dial:50
    };
    let mut passcode = 0;
    for instruction in inputs {

        let (digit, lr):(String, String) = instruction.chars().partition(|c| c.is_digit(10));
        safe.turn(&digit, &lr)?;
        if safe.dial == 0 {
            passcode += 1;
        }
        
    }
    println!("passcode: {passcode}");
    Ok(())

}

fn run_day_two(file_info: &str) -> Result<(), ParseIntError>{

    let inputs = file_info.split_whitespace();
    let mut safe = Safe {
        dial:50
    };
    let mut passcode = 0;

    for instruction in inputs {
        let (digit, lr):(String, String) = instruction.chars().partition(|c| c.is_digit(10));
        let turns = digit.parse::<usize>()?;
        let amount = String::from("1");
        for _ in 0..turns {
          
            safe.turn(&amount, &lr)?;
            if safe.dial == 0 {
                passcode += 1;
            }
        }        
    }
    println!("passcode tast 2: {passcode}");
    Ok(())
}

struct Safe {
    dial: u32,
}


impl Safe {

   fn turn (&mut self, amount: &str, direction: &str) -> Result<(), ParseIntError> {

        let clicks = amount.parse::<u32>()?;
        let clicks = clicks % MOD;
        if direction == "L"{    
            if clicks > self.dial {
                self.dial += MOD - clicks;
            } else {
                self.dial -= clicks;
            }
        } else {
            self.dial = self.dial
                    .checked_add(clicks).expect("overflowed on right turn");
            if self.dial >= MOD {
                self.dial -= MOD;
            }
       }
       Ok(())
   }
}
