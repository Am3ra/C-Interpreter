
#[derive(Clone,Debug,PartialEq)]
enum Token {
    DIGIT(i32),
    ADDOP(AddOp),
    MULOP(MulOp),
    UNOP(UnaryOp),
    LPAREN,
    RPAREN,
    EOF,
}

#[derive(Clone,Debug,PartialEq)]
enum AddOp{
    PLUS,
    MINUS,
}

#[derive(Clone,Debug,PartialEq)]
enum UnaryOp{
    PLUS,
    MINUS,
}

#[derive(Clone,Debug,PartialEq)]
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

#[derive(Debug, PartialEq,Clone)]
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

struct Parser {
    lexer: Lexer,
}

impl Parser {

    pub fn new(input: &str)->Result<Parser,String>{
        Ok(Parser{
            lexer: Lexer::new(input)?,
        })
    }
    

    fn atom(&mut self)-> Result<ASTreeNode, String>{
        match self.lexer.current_token{
            Token::DIGIT(i)=>{
                self.lexer.get_next_token();
                Ok(ASTreeNode::new(Token::DIGIT(i)))
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
            Token::ADDOP(AddOp::MINUS)=> {
                    self.lexer.get_next_token();
                    let mut current = ASTreeNode::new(Token::UNOP(UnaryOp::MINUS));
                    current.left = Some(Box::new(self.atom()?));
                    Ok(current)
            },
            Token::ADDOP(AddOp::PLUS)=>{
                self.lexer.get_next_token();
                let mut current = ASTreeNode::new(Token::UNOP(UnaryOp::PLUS));
                current.left = Some(Box::new(self.atom()?));
                Ok(current)
            },
            _=>{
                println!("Current TOK ERR, {:?}", self.lexer.current_token);
                Err("Expected digit, '+' , '-' , or '(' ".into())
                }
        }
    }

    fn term(&mut self)-> Result<ASTreeNode,String>{
        let left = self.atom()?;

        if let Token::MULOP(i) = self.lexer.current_token.clone(){
            match i{
                MulOp::MULT=> return Ok(ASTreeNode::new_with_values(
                        Token::MULOP(MulOp::MULT), 
                        Some(Box::new(left)), 
                        Some(Box::new(self.expr()?)
                        )
                    )),
                MulOp::DIV=> return Ok(ASTreeNode::new_with_values(
                        Token::MULOP(MulOp::DIV), 
                        Some(Box::new(left)), 
                        Some(Box::new(self.expr()?)
                        )
                    )),
                MulOp::MODU=> return Ok(ASTreeNode::new_with_values(
                        Token::MULOP(MulOp::MODU), 
                        Some(Box::new(left)), 
                        Some(Box::new(self.expr()?)
                        )
                    )),
            }
        }

        Ok(left)
    }

    pub fn expr(&mut self) -> Result<ASTreeNode, String> {
        self.lexer.get_next_token();

        let left = self.term()?;

        if let Token::ADDOP(i) = self.lexer.current_token.clone(){
            // self.lexer.get_next_token();
            // println!("Current token {:?}", self.lexer.current_token);
            match i{
                AddOp::PLUS=> return Ok(ASTreeNode::new_with_values(
                        Token::ADDOP(AddOp::PLUS), 
                        Some(Box::new(left)), 
                        Some(Box::new(self.expr()?)
                        )
                    )),
                AddOp::MINUS=> return Ok(ASTreeNode::new_with_values(
                        Token::ADDOP(AddOp::MINUS), 
                        Some(Box::new(left)), 
                        Some(Box::new(self.expr()?)
                        )
                    )),
            }
        }

        Ok(left)
    }

}

pub struct Interpreter{
    parser : Parser,
}

impl Interpreter{
    pub fn new(input: &str)->Result<Interpreter,String>{
        Ok(Interpreter{
            parser: Parser::new(input)?,
        })
    }

    fn interpret_input(input: ASTreeNode)->Result<i32,String>{
         
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
            Token::UNOP(n)=>{
                match n {
                    UnaryOp::PLUS => Ok(Interpreter::interpret_input(*input.left.unwrap())?),
                    UnaryOp::MINUS => Ok(-Interpreter::interpret_input(*input.left.unwrap())?)
                }
            },
            _ => Err("Unknown Token".into())
        }

        
    }

    pub fn interpret(&mut self)->Result<i32,String>{
        Interpreter::interpret_input(self.parser.expr()?)
    }
}

#[allow(dead_code)]
pub struct Translator{
    parser:Parser
} 

#[allow(dead_code)]
impl Translator{
    pub fn new(input : &str)->Result<Translator,String>{
        Ok(Translator{
            parser: Parser::new(input)?
        })
    }

    fn rpn_interp(input : ASTreeNode)->Result<String,String>{
        let mut result = String::new();
        match input.value{
            Token::DIGIT(n)=> result.push_str(&n.to_string()),
            Token::ADDOP(n)=> {
                match n{
                    AddOp::PLUS => {
                        result.push_str(&Translator::rpn_interp(*(input.left.unwrap()))?);
                        result.push(' ');
                        result.push_str(&Translator::rpn_interp(*input.right.unwrap())?);
                        result.push(' ');
                        result.push('+');
                        },
                    AddOp::MINUS => {
                        result.push_str(&Translator::rpn_interp(*(input.left.unwrap()))?);
                        result.push(' ');
                        result.push_str(&Translator::rpn_interp(*input.right.unwrap())?);
                        result.push(' ');
                        result.push('-');
                        },
                }
            },
            Token::MULOP(n)=> {
                match n{
                    MulOp::MULT=>{
                        result.push_str(&Translator::rpn_interp(*(input.left.unwrap()))?);
                        result.push(' ');
                        result.push_str(&Translator::rpn_interp(*input.right.unwrap())?);
                        result.push(' ');
                        result.push('*');
                    },
                    
                    MulOp::DIV=>{
                        result.push_str(&Translator::rpn_interp(*(input.left.unwrap()))?);
                        result.push(' ');
                        result.push_str(&Translator::rpn_interp(*input.right.unwrap())?);
                        result.push(' ');
                        result.push('/');
                    },
                    MulOp::MODU=>{
                        result.push_str(&Translator::rpn_interp(*(input.left.unwrap()))?);
                        result.push(' ');
                        result.push_str(&Translator::rpn_interp(*input.right.unwrap())?);
                        result.push(' ');
                        result.push('%');
                    },
                }
            }
            _ => return Err(format!("ERROR unexpected Token: {:?}", input.value)),
        }
        Ok(result)
    }


    pub fn to_rpn(&mut self)->Result<String,String>{
        Translator::rpn_interp(self.parser.expr()?)
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn basic_add(){
        assert_eq!(3, Interpreter::new("1+2").unwrap().interpret().unwrap());
    }

    #[test]
    fn unary_minus(){
        assert_eq!(3, Interpreter::new("--3").unwrap().interpret().unwrap());
    }

    #[test]
    fn unary_plus(){
        assert_eq!(3, Interpreter::new("++3").unwrap().interpret().unwrap());
    }

    #[test]
    fn unary_both() {
        assert_eq!(3, Interpreter::new("++3").unwrap().interpret().unwrap());
    }

    #[test]
    fn chain_add(){
        assert_eq!(6, Interpreter::new("1+2+3").unwrap().interpret().unwrap());
    }

    #[test]
    fn precedence_test(){
        assert_eq!(7, Interpreter::new("1+2*3").unwrap().interpret().unwrap());
    }

    #[test]
    fn precedence_test2(){
        assert_eq!(5, Interpreter::new("1*2+3").unwrap().interpret().unwrap());
    }
    
    #[test]
    fn parentheses_test(){
        assert_eq!(9, Interpreter::new("(1+2)*3").unwrap().interpret().unwrap());
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

    #[test]
    fn parser_test() {
        assert_eq!(
            ASTreeNode::new_with_values(
                Token::ADDOP(AddOp::PLUS), 
                Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))), 
                Some(Box::new(ASTreeNode::new(Token::DIGIT(2)))))
                , 
            Parser::new("1+2").unwrap().expr().unwrap())
    }

    #[test]
    fn lexer_test() {
        let mut tok = Lexer::new("1+2").unwrap();
        tok.get_next_token();
        assert_eq!(Token::DIGIT(1), tok.current_token);
        tok.get_next_token();
        assert_eq!(Token::ADDOP(AddOp::PLUS), tok.current_token);
        tok.get_next_token();
        assert_eq!(Token::DIGIT(2), tok.current_token);
    }

    #[test]
    fn atom_test() {
        let mut pars =  Parser::new("1+2").unwrap();
        pars.lexer.get_next_token();

        assert_eq!(Ok(ASTreeNode::new(Token::DIGIT(1))), pars.atom())
    }


    #[test]
    fn atom_test3() {
        let mut pars =  Parser::new("1+2").unwrap();
        pars.lexer.get_next_token();
        pars.lexer.get_next_token();
        pars.lexer.get_next_token();

        assert_eq!(Ok(ASTreeNode::new(Token::DIGIT(2))), pars.atom())
    }

    #[test]
    fn parser_basic() {
        assert_eq!(
            ASTreeNode::new(
                Token::DIGIT(1)
            ),
            Parser::new("1").unwrap().expr().unwrap()
        )
    }

    #[test]
    fn rpn_translate() {
        assert_eq!("1 2 +", Translator::new("1+2").unwrap().to_rpn().unwrap())
    }
}
