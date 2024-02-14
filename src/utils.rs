use std::io::{self, Write};
use crate::error::{BoilError, BoilResult};

macro_rules! capitalize {
    ($string:expr) => {
        if $string.is_empty() {
            $string
        } else {
            let s = $string.to_owned();
            let mut b = s.chars();
            b.next().unwrap().to_uppercase().collect::<String>() + b.as_str()
        }
    };
}


pub fn user_input(msg: String) -> BoilResult<bool> {
    let mut input = String::new();
    print!("{} ", msg);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    if input.as_str().trim() == "y" {
        Ok(true)
    } else {
        Ok(false)
    }
    
}

pub(crate) use capitalize;
