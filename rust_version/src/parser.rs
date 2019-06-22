use std::io;
use std::io::Write;


pub fn parse_line() Result<String,String>{
    let mut buffer = String::new();
    print!("calc> ");
    io::stdout().flush().unwrap(); // Makes sense, but ugh.
    io::stdin().read_line(&mut buffer)?;

    parse_tokens(buffer)
}

fn parse_tokens(buffer : String)-> Result<Vec<String>,String>{

    Err("")
}

fn tokenizer(statement:String){
    
}