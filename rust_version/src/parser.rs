
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

struct Lexer{
    input: Vec<char>,
    position: usize,
    current_token: Token,
    len : usize
}

impl Lexer{
    fn digit(& mut self)->Token{
        let mut number_so_far = String::new();

        while self.position < self.len  && self.input[self.position].is_digit(10){
            number_so_far.push( self.input[self.position]);
            self.position+=1;
        }

        return Token::DIGIT(number_so_far.parse().unwrap());
    }

    pub fn get_next_token(&mut self) {
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

    pub fn new(input: &str) -> Result<Lexer,String> {
        if input.len()<1 {
            return Err("Must have lenght".into())
        }
        let input :Vec<char> = input.trim().chars().collect();
        Ok(Lexer {
            len: input.len(),
            input,
            position: 0,
            current_token: Token::EOF,
        })
    }
}

#[derive(Debug)]
struct ASTreeNode{
    value :  Token,
    left : Option<Box<ASTreeNode>>,
    right : Option<Box<ASTreeNode>>,
}

impl ASTreeNode{
    fn new_with_values(value: Token,left: Option<Box<ASTreeNode>>,right: Option<Box<ASTreeNode>> ) -> ASTreeNode{ // Might change to options for left and right
        ASTreeNode{
            value,
            left,
            right
        }
    }

    fn new(value : Token) ->ASTreeNode{
        ASTreeNode{
            value,
            right : None,
            left : None,
        }
    }
}

pub struct Parser {
    lexer: Lexer,
}


impl Parser {

    pub fn new(input: &str)->Result<Parser,String>{
        Ok(Parser{
            lexer: Lexer::new(input)?,
        })
    }
    

    fn atom(&mut self)-> Result<u32, String>{
        match self.lexer.current_token{
            Token::DIGIT(i)=>{
                self.lexer.get_next_token();
                Ok(i)
                },
            Token::LPAREN=> {
                let result = self.expr();
                match self.lexer.current_token{
                    Token::RPAREN=> {
                        self.lexer.get_next_token();
                        result
                    },
                    _=>{
                        println!("Current TOK ERR, {:?}", self.lexer.current_token);
                        Err("Expected ')'".into())
                    }
                }
            },
            _=>{
                println!("Current TOK ERR, {:?}", self.lexer.current_token);
                Err("Expected digit".into())
                }
        }
    }

    fn term(&mut self)-> Result<u32,String>{
        let mut result = self.atom()?;
        while let Token::MULOP(i) = self.lexer.current_token.clone(){
            self.lexer.get_next_token();
            match i{
                MulOp::MULT => result*= self.term()?,
                MulOp::DIV => result /= self.term()?,
                MulOp::MODU => result %= self.term()?
            }
        }
        Ok(result)
    }

    pub fn expr(&mut self) -> Result<u32, String> {
        self.lexer.get_next_token();
        // let mut root = ASTreeNode::new(s: Token);
        let mut result = self.term()?;

        while let Token::ADDOP(i) = self.lexer.current_token.clone(){
            self.lexer.get_next_token();
            match i{
                AddOp::PLUS => result+= self.term()?,
                AddOp::MINUS => result -= self.term()?
            }
        }

        Ok(result)
    }

}

struct Interpreter{
    parser : Parser,
}

impl Interpreter{
    pub fn new(input: &str)->Result<Interpreter,String>{
        Ok(Interpreter{
            parser: Parser::new(input)?,
        })
    }

    fn interpret_input(input: ASTreeNode)->Result<u32,String>{
         
        match input.value{
            Token::DIGIT(n)=> Ok(n),
            Token::ADDOP(n)=>{
                match n{
                    AddOp::MINUS => Ok(Interpreter::interpret_input(*input.left.unwrap())? - Interpreter::interpret_input(*input.right.unwrap())?),
                    AddOp::PLUS => Ok(Interpreter::interpret_input(*input.left.unwrap())? + Interpreter::interpret_input(*input.right.unwrap())?),
                }
            }
            Token::MULOP(n)=>{
                match n{
                    MulOp::MULT => Ok(Interpreter::interpret_input(*input.left.unwrap())? * Interpreter::interpret_input(*input.right.unwrap())?),
                    MulOp::DIV => Ok(Interpreter::interpret_input(*input.left.unwrap())? / Interpreter::interpret_input(*input.right.unwrap())?),
                    MulOp::MODU => Ok(Interpreter::interpret_input(*input.left.unwrap())? % Interpreter::interpret_input(*input.right.unwrap())?),
                }
            },
            _ => Err("Unknown Token".into())
        }

        
    }

    
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn basic_add(){
        assert_eq!(3, Parser::new("1+2").unwrap().expr().unwrap());
    }

    #[test]
    fn chain_add(){
        assert_eq!(6, Parser::new("1+2+3").unwrap().expr().unwrap());
    }

    #[test]
    fn precedence_test(){
        assert_eq!(7, Parser::new("1+2*3").unwrap().expr().unwrap());
    }

    #[test]
    fn precedence_test2(){
        assert_eq!(5, Parser::new("1*2+3").unwrap().expr().unwrap());
    }
    
    #[test]
    fn parentheses_test(){
        assert_eq!(9, Parser::new("(1+2)*3").unwrap().expr().unwrap());
    }
    
    #[test]
    fn basic_interp_plus(){
        let root = ASTreeNode::new_with_values(Token::ADDOP(AddOp::PLUS), Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))), Some(Box::new(ASTreeNode::new(Token::DIGIT(2)))));
        assert_eq!(3, Interpreter::interpret_input(root).unwrap());
    }

    #[test]
    fn basic_interp_minus(){
        let root = ASTreeNode::new_with_values(Token::ADDOP(AddOp::MINUS), Some(Box::new(ASTreeNode::new(Token::DIGIT(2)))), Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))));
        assert_eq!(1, Interpreter::interpret_input(root).unwrap());
    }

    #[test]
    fn basic_interp_times(){
        let root = ASTreeNode::new_with_values(Token::MULOP(MulOp::MULT), Some(Box::new(ASTreeNode::new(Token::DIGIT(2)))), Some(Box::new(ASTreeNode::new(Token::DIGIT(3)))));
        assert_eq!(6, Interpreter::interpret_input(root).unwrap());
    }

    #[test]
    fn basic_interp_divide(){
        let root = ASTreeNode::new_with_values(Token::MULOP(MulOp::DIV), Some(Box::new(ASTreeNode::new(Token::DIGIT(2)))), Some(Box::new(ASTreeNode::new(Token::DIGIT(3)))));
        assert_eq!(0, Interpreter::interpret_input(root).unwrap());
    }

    #[test]
    fn basic_interp_modulo(){
        let root = ASTreeNode::new_with_values(Token::MULOP(MulOp::MODU), Some(Box::new(ASTreeNode::new(Token::DIGIT(2)))), Some(Box::new(ASTreeNode::new(Token::DIGIT(3)))));
        assert_eq!(2, Interpreter::interpret_input(root).unwrap());
    }

}
