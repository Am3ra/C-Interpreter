#[derive(Clone)]
enum Token {
    DIGIT(u32),
    PLUS,
    EOF
}


struct Interpreter{
    input : Vec<char>,
    position: usize,
    current_token : Token
}

impl Interpreter{
    fn get_next_token(&mut self)->Token{
        if self.position > self.input.len(){
            return Token::EOF;
        }
        let current_char = self.input[self.position];

        if current_char.is_digit(10){
            self.position +=1 ;
            return Token::DIGIT(current_char.to_digit(10).unwrap());
        }
        else if current_char == '+'{
            self.position+=1;
            return Token::PLUS;
        }
        Token::EOF
    }
    
    fn new(input:&str)->Interpreter{
        let input = input.chars().collect();
        Interpreter{
            input,
            position : 0,
            current_token:Token::EOF
        }
    }

    // fn eat(&self, test:Token)->Result<(),String>{
    //     if mem::discriminant(&self.current_token) == mem::discriminant(&test){
    //         return  Ok(());
    //     }else{
    //         match test{
    //             Token::DIGIT(_)=> Err("Expected DIGIT".into()),
    //             Token::PLUS=> Err("Expected PLUS".into()),
    //             _ => Err("Unknown ERR".into()),
    //         }
    //     }
    // }
    
    fn expr(&mut self)->Result<(),String>{
        self.current_token = self.get_next_token();
        let left:u32;
        let right:u32;
        if let Token::DIGIT(i) = self.current_token.clone(){
            left = i;
        }
        else{
            return Err("Expected digit".into())
        }

        if let Token::PLUS = self.get_next_token(){}
        else{
            return Err("Expected Operator (Plus)".into())
        };

        if let Token::DIGIT(i) = self.get_next_token(){
            right = i;
        }
        else{
            return Err("Expected Operator (Plus)".into())
        };

        println!("{}", left+right);


        Ok(())
    }

}
