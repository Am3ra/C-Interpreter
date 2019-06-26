
#[derive(Clone,Debug)]
enum Token {
    DIGIT(u32),
    ADDOP(AddOp),
    MULOP(MulOp),
    LPAREN,
    RPAREN,
    EOF,
}

#[derive(Clone,Debug)]
enum AddOp{
    PLUS,
    MINUS,
}

#[derive(Clone,Debug)]
enum MulOp{
    MULT,
    DIV,
    MODU
}

pub struct Interpreter {
    input: Vec<char>,
    position: usize,
    current_token: Token,
    len : usize
    // current_char : Option<char>
}

impl Interpreter {
    
    fn digit(& mut self)->Token{
        let mut number_so_far = String::new();

        while self.position < self.len  && self.input[self.position].is_digit(10){
            number_so_far.push( self.input[self.position]);
            self.position+=1;
        }

        return Token::DIGIT(number_so_far.parse().unwrap());
    }

    fn get_next_token(&mut self) {
        if self.position >= self.len {
            return self.current_token= Token::EOF;
        }
        let mut current_char = self.input[self.position];

        while current_char.is_whitespace() {
            self.position += 1;
            current_char = self.input[self.position];
        }

        if current_char.is_digit(10) {
            return self.current_token= self.digit();
        } 
        
        self.position += 1;

        match current_char{
            '+' => self.current_token= Token::ADDOP(AddOp::PLUS),
            '-' => self.current_token= Token::ADDOP(AddOp::MINUS),
            '*' => self.current_token = Token::MULOP(MulOp::MULT),
            '/' => self.current_token = Token::MULOP(MulOp::DIV),
            '%' => self.current_token = Token::MULOP(MulOp::MODU),
            '(' => self.current_token = Token::LPAREN,
            ')' => self.current_token = Token::RPAREN,
            _   =>  panic!("UNRECOGNIZED TOKEN: {}", current_char)
        }

    }


    pub fn new(input: &str) -> Result<Interpreter,String> {
        if input.len()<1 {
            return Err("Must have lenght".into())
        }
        let input :Vec<char> = input.trim().chars().collect();
        Ok(Interpreter {
            len: input.len(),
            input,
            position: 0,
            current_token: Token::EOF,
            // current_char : None
        })
    }

    fn atom(&mut self)-> Result<u32, String>{
        match self.current_token{
            Token::DIGIT(i)=>{
                self.get_next_token();
                Ok(i)
                },
            Token::LPAREN=> {
                let result = self.expr();
                match self.current_token{
                    Token::RPAREN=> {
                        self.get_next_token();
                        result
                    },
                    _=>{
                        println!("Current TOK ERR, {:?}", self.current_token);
                        Err("Expected ')'".into())
                    }
                }
            },
            _=>{
                println!("Current TOK ERR, {:?}", self.current_token);
                Err("Expected digit".into())
                }
        }
    }

    fn term(&mut self)-> Result<u32,String>{
        let mut result = self.atom()?;
        while let Token::MULOP(i) = self.current_token.clone(){
            self.get_next_token();
            match i{
                MulOp::MULT => result*= self.term()?,
                MulOp::DIV => result /= self.term()?,
                MulOp::MODU => result %= self.term()?
            }
        }
        Ok(result)
    }

    pub fn expr(&mut self) -> Result<u32, String> {
        self.get_next_token();
        let mut result = self.term()?;

        while let Token::ADDOP(i) = self.current_token.clone(){
            self.get_next_token();
            match i{
                AddOp::PLUS => result+= self.term()?,
                AddOp::MINUS => result -= self.term()?
            }
        }

        Ok(result)
    }

}
