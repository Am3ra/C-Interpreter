mod parser;
use std::io::{self, Write};

fn main()-> Result<(), Box<std::error::Error>> {
    loop {
        print!("calc>");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut prs =  parser::Parser::new(&input)?;
        println!("{}", prs.expr()?);

        
    }
    
}
