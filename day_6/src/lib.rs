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
        let mut string_num: Vec<String> = Vec::with_capacity(self.numbers.len());

        //println!("before mutation: {:?}", self.numbers);
        //println!("before string: {:?}", self.string_raws);

        let width = self.string_raws[0].len();

        // Preallocate all columns
        let mut columns: Vec<String> = (0..width)
            .map(|_| String::with_capacity(self.string_raws.len()))
            .collect();

        for row in &self.string_raws {
            for (col_idx, b) in row.bytes().rev().enumerate() {
                if b != b' ' {
                    columns[col_idx].push(b as char);
                }
            }
        }

        self.numbers = columns
            .into_iter()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        //println!("after mutation: {:?}", self.numbers);
        self.caculate()
    }
}
