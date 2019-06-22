mod parser;

fn main()-> Result<(), Box<std::error::Error>> {
    loop {
        println!("{}", parser::parse_line()?);
    }
    
}
