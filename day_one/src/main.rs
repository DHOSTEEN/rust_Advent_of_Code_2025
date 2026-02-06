use std::fs;
use std::error::Error;

fn main() {
    let mut file = fs::read_to_string("day_one_input.txt").expect("AAAAAA");
    let inputs  = file.split_whitespace();
    let mut safe = Safe {
        dial:50
    };
    let mut passcode = 0;
   for instruction in inputs {
        //println!("sections: {instruction:?}");

        let (digit, lr):(String, String) = instruction.chars().partition(|c| c.is_digit(10));
        safe.turn(&digit, &lr);
        if safe.dial == 0 {
            passcode += 1;
        }
        
    }
    println!("passcode: {passcode}");
}



struct Safe {
    dial: u32,
}

impl Safe {

   fn turn (&mut self, amount: &str, direction: &str) {
    let clicks = amount.parse::<u32>().unwrap() % 100;
        if direction == "L"{
            //println!("LEFT ");    
            if clicks > self.dial {
                self.dial += 100 - clicks;
            } else {
                self.dial -= clicks;
            }
           // println!("safe pos: {}", self.dial);
        } else {
           // println!("RIGHT");
            self.dial = self.dial
                    .checked_add(clicks).expect("WHYYYY");
            if self.dial >= 100 {
                self.dial -= 100;
            }
           // println!("safe pos: {}", self.dial);
       }
   }

}
