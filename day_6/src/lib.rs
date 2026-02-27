pub mod error;
pub use error::Day6Error;
#[derive(Debug, Clone, Copy)]
pub enum OperationSymbol {
    Plus,
    Multiply,
}
#[derive(Debug)]
pub struct Colum {
    numbers: Vec<u64>,
    symbol: Option<OperationSymbol>,
}
pub struct ColumTaskTwo {

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
        let num= num.parse::<u64>()?;
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
    pub fn caculate_t2(&mut self) -> u64 {
        let mut string_num:Vec<String> = Vec::with_capacity(self.numbers.len());

        println!("before mutation: {:?}", self.numbers);

        for num in self.numbers.iter() {
            let num = num.to_string();
            let mut i = 0;
            let mut itr = num.chars().rev();
            for i in 0..num.len() {
                if let Some(col_num) = string_num.get_mut(i) {
                    col_num.push(itr.next().unwrap());
                } else {
                    string_num.push(itr.next().unwrap().to_string())
                }
              
            }
        }

        self.numbers = string_num.into_iter().filter_map(|s|s.parse::<u64>().ok()).collect();
        println!("after mutation: {:?}", self.numbers);
        self.caculate()
    }
}
