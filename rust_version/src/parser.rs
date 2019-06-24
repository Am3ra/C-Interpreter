#[derive(Clone)]
enum Token {
    DIGIT(u32),
    OPERATOR(Operator),
    EOF,
}

#[derive(Clone)]
enum Operator{
    PLUS,
    MINUS,
    MULT,
    DIV
}

pub struct Interpreter {
    input: Vec<char>,
    position: usize,
    current_token: Token,
    // current_char : Option<char>
}

impl Interpreter {
    

    fn get_next_token(&mut self) -> Token {
        let len = self.input.len();
        if self.position > len {
            return Token::EOF;
        }
        let mut current_char = self.input[self.position];

        while current_char.is_whitespace() {
            self.position += 1;
            current_char = self.input[self.position];
        }

        if current_char.is_digit(10) {
            let mut number_so_far = String::new();

            while self.position < len  && self.input[self.position].is_digit(10){
                number_so_far.push( self.input[self.position]);
                self.position+=1;
            }
            return Token::DIGIT(number_so_far.parse().unwrap());
        } 
        else if current_char == '+' {
            self.position += 1;
            return Token::OPERATOR(Operator::PLUS);
        } 
        else if current_char == '-' {
            self.position += 1;
            return Token::OPERATOR(Operator::MINUS);
        }
        else{
            Token::EOF
        }
    }


    pub fn new(input: &str) -> Result<Interpreter,String> {
        if input.len()<1 {
            return Err("Must have lenght".into())
        }
        let input = input.trim().chars().collect();
        Ok(Interpreter {
            input,
            position: 0,
            current_token: Token::EOF,
            // current_char : None
        })
    }

    pub fn expr(&mut self) -> Result<(), String> {
        self.current_token = self.get_next_token();
        let left: u32;
        let right: u32;
        let op : Operator;
        if let Token::DIGIT(i) = self.current_token.clone() {
            left = i;
        } else {
            return Err("Expected digit".into());
        }

        if let Token::OPERATOR(i) = self.get_next_token() {
            op = i;
        } else {
            return Err("Expected Operator".into());
        };

        if let Token::DIGIT(i) = self.get_next_token() {
            right = i;
        } else {
            return Err("Expected Digit".into());
        };

        match op{
            Operator::MINUS => println!("{}", left - right),
            Operator::PLUS => println!("{}", left + right),
            Operator::MULT => println!("{}", left * right),
            Operator::DIV => println!("{}", left / right)
        }



        Ok(())
    }

}
