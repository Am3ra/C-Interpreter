use std::io;
use std::io::Write;
use std::error;


pub fn parse_line<'a>() -> Result<String, Box<error::Error> {
    let mut buffer = String::new();
    print!("calc> ");
    io::stdout().flush().unwrap(); // Makes sense, but ugh.
    io::stdin().read_line(&mut buffer)?;

    let token_vector : Vec<&'a str> = parse_tokens(buffer)?;
}

fn parse_tokens<'a>(buffer : String)-> Result<Vec<&'a str>,String>{

    Err("Error parsing tokensd".into())
}

fn tokenizer(statement:String){
    
}