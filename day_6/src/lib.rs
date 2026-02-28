pub mod error;
pub use error::Day6Error;
#[derive(Debug, Clone, Copy)]
pub enum OperationSymbol {
    Plus,
    Multiply,
}
#[derive(Debug, Default)]
pub struct Colum {
    numbers: Vec<u64>,
    string_raws: Vec<String>,
    symbol: Option<OperationSymbol>,
}

impl Colum {
    pub fn add_num(&mut self, num: &str) -> Result<(), Day6Error> {
        //println!("|{num}|");
        let num = num.trim().parse::<u64>()?;
        self.numbers.push(num);
        Ok(())
    }
    pub fn add_string(&mut self, raw_num: &str) {
        self.string_raws.push(raw_num.to_owned());
    }
    pub fn add_symbol(&mut self, symbol: &str) -> Result<(), Day6Error> {
        let mut symbol = symbol.trim();
        if symbol.len() > 1 {
            if symbol.contains("*") {
                symbol = "*";
            } else {
                symbol = "+";
            }
        }
        println!("{symbol}");
        use OperationSymbol::*;
        match symbol {
            "+" => {
                self.symbol = Some(Plus);
                Ok(())
            }
            "*" => {
                self.symbol = Some(Multiply);
                Ok(())
            }
            _ => Err(Day6Error::NoSymbol),
        }
    }

    pub fn caculate(&self) -> u64 {
        match self.symbol {
            Some(symbol) => {
                use OperationSymbol::*;
                match symbol {
                    Plus => self.numbers.iter().sum(),
                    Multiply => self.numbers.iter().product(),
                }
            }
            None => 0,
        }
    }
    pub fn caculate_t2(&self) -> Result<(Option<u64>, Option<OperationSymbol>), Day6Error> {
        let mut string_num: Vec<String> = Vec::with_capacity(self.numbers.len());

        //println!("before mutation: {:?}", self.numbers);
        println!("before string: {:?}", self.string_raws);
        println!("symbol: {:?}", self.symbol );
        
        let num= self.string_raws.concat();
        let num = num.trim();
        println!("collected: {num}");
        if num.is_empty() {
           return Ok((None, self.symbol))
        }
        let num:u64 = num.parse()?;
        Ok((Some(num), self.symbol))
    }
}
