mod parser;

fn main()-> Result<(), String> {
    loop {
        println!("{}", parser::parse_line()?);
    }
    
}
