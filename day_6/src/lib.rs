mod error;
use error::Day6Error;
#[derive(Debug, Clone, Copy)]
pub enum OperationSymbol {
    Plus,
    Multiply,
}
#[derive(Debug)]
pub struct Colum {
    pub numbers: Vec<u64>,
    symbol: Option<OperationSymbol>,
}

impl Default for Colum {
    fn default() -> Self {
        Self {
            numbers: Vec::new(),
            symbol: None,
        }
    }
}

impl Colum {
    pub fn add_num(&mut self, num: &str) -> Result<(), Day6Error> {
        let num: u64 = num.to_string().parse()?;
        self.numbers.push(num);
        Ok(())
    }
    pub fn add_symbol(&mut self, symbol: &str) -> Result<(), Day6Error> {
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
}
