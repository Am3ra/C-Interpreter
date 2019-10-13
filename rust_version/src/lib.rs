use std::collections::HashMap;
use std::iter::FromIterator;

/**
 *
 * NOTE: IF IN CAPITALS, CONSUME AND ADVANCE
 *
 * Current Grammar:
 *
 * program : MAIN block
 * block  : LBRACE [statement_list] RBRACE
 * statement_list  : *(statement SEMI|block) [statement [SEMI]]
 * statement  : (expr | declaration )  
 * expr : addop *(ASSIGN expr)
 * addop : term *((PLUS/MINUS) expr)
 * mulop : atom ((MUL/DIV) expr)
 * atom : (PLUS/MINUS) atom |
 *          INTEGER |
 *          LPAREN expr RPAREN
 * declaration : type IDENTIFIER [ASSIGN expr] SEMI // might not need SEMI later
 * assignment : identifier ASSIGN expr
 * type : INT|FLOAT //TODO: IMPLEMENT FLOAT
 * identifier : alphabetic *(alphanumeric) //don't know how to write this
 * LBRACE = '{'
 * RBRACE = '}'
 * LPAREN = '('
 * RPAREN = ')'
 * ASSIGN = '='
 * COMMA  = ','
 * MAIN   = 'main'
 *
 * Proposed Grammar:
 *
 * function : TYPE IDENTIFIER argument_list block
 * argument_list : LPAREN argument  *(COMMA argument)  RPAREN
 * argument : TYPE IDENT
 *
 * ORDER OF OPERATIONS:
 *
 * 1: UNARY PLUS/MINUS, NOT (RIGHT TO LEFT ASS.)
 * 2: MULT / DIV (LEFT TO RIGHT ASS.)
 * 3: ADD/SUB (LEFT TO RIGHT ASS.)
 * 4: ASSIGNMENT, =, +=, -=, *=. /=, %=, (RIGHT TO LEFT ASS.)
 */

/**
 * TODO: Implement simple namespace
 *      Make types hold values? No... Use relevant token, like digit
 *      figure out type system...
 */

#[derive(Clone, Debug, PartialEq)]
enum Type {
    INT,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    DIGIT(i32),
    ADDOP(AddOp),
    MULOP(MulOp),
    UNOP(UnaryOp),
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMI,
    LT,
    GT,
    EQ,
    // NE,
    LE,
    GE,
    ASSIGN,
    EOF,
    Type(Type),
    IDENT(String),
    StatementList(Vec<ASTreeNode>),
    RET,
}

#[derive(Clone, Debug, PartialEq)]
enum AddOp {
    PLUS,
    MINUS,
}

#[derive(Clone, Debug, PartialEq)]
enum UnaryOp {
    PLUS,
    MINUS,
}

#[derive(Clone, Debug, PartialEq)]
enum MulOp {
    MULT,
    DIV,
    MODU,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
    current_token: Token,
    len: usize,
    restricted_words: HashMap<String, Token>,
}

impl Lexer {
    fn digit(&mut self) -> Token {
        let mut number_so_far = String::new();

        while self.position < self.len && self.input[self.position].is_digit(10) {
            number_so_far.push(self.input[self.position]);
            self.position += 1;
        }

        Token::DIGIT(number_so_far.parse().unwrap())
    }

    fn identifier(&mut self) -> Token {
        let mut string_so_far = String::new();
        while self.position < self.len && self.input[self.position].is_alphanumeric() {
            string_so_far.push(self.input[self.position]);
            self.position += 1;
        }
        if let Some(i) = self.restricted_words.get(&string_so_far) {
            // MUCH cleaner than a match.
            return i.clone();
        }
        Token::IDENT(string_so_far)
    }

    fn peek(&self) -> Option<char> {
        if self.position >= self.len {
            None
        } else {
            Some(self.input[self.position])
        }
    }

    pub fn get_next_token(&mut self) {
        if self.position >= self.len {
            return self.current_token = Token::EOF;
        }
        let mut current_char = self.input[self.position];

        while current_char.is_whitespace() {
            self.position += 1;
            current_char = self.input[self.position];
        }

        if current_char.is_digit(10) {
            return self.current_token = self.digit();
        }

        if current_char.is_alphabetic() {
            return self.current_token = self.identifier();
        }
        self.position += 1;

        match current_char {
            '+' => self.current_token = Token::ADDOP(AddOp::PLUS),
            '-' => self.current_token = Token::ADDOP(AddOp::MINUS),
            '*' => self.current_token = Token::MULOP(MulOp::MULT),
            '/' => self.current_token = Token::MULOP(MulOp::DIV),
            '%' => self.current_token = Token::MULOP(MulOp::MODU),
            '(' => self.current_token = Token::LPAREN,
            ')' => self.current_token = Token::RPAREN,
            '{' => self.current_token = Token::LBRACE,
            '}' => self.current_token = Token::RBRACE,
            ';' => self.current_token = Token::SEMI,
            '=' => {
                if let Some(n) = self.peek() {
                    match n {
                        '=' => {
                            self.current_token = Token::EQ;
                            self.position += 1;
                        }
                        _ => self.current_token = Token::ASSIGN,
                    }
                }
            }
            '<' => {
                if let Some(n) = self.peek() {
                    match n {
                        '=' => {
                            self.current_token = Token::LE;
                            self.position += 1;
                        }
                        _ => self.current_token = Token::LT,
                    }
                }
            }
            '>' => {
                if let Some(n) = self.peek() {
                    match n {
                        '=' => {
                            self.current_token = Token::GE;
                            self.position += 1;
                        }
                        _ => self.current_token = Token::GT,
                    }
                }
            }
            _ => panic!("UNRECOGNIZED TOKEN: {}", current_char),
        }
    }

    pub fn new(input: &str) -> Result<Lexer, String> {
        if input.is_empty() {
            return Err("Must have lenght".into());
        }
        let input: Vec<char> = input.trim().chars().collect();
        // let reserved_keys : HashSet<String> = vec!["int".into()].iter().cloned().collect();
        let restricted_words: HashMap<String, Token> = HashMap::from_iter(vec![
            ("int".into(), Token::Type(Type::INT)),
            ("test".into(), Token::EOF),
            ("return".into(), Token::RET),
        ]);
        let mut lex = Lexer {
            len: input.len(),
            input,
            position: 0,
            current_token: Token::EOF,
            restricted_words,
        };
        lex.get_next_token();
        Ok(lex)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct ASTreeNode {
    value: Token,
    left: Option<Box<ASTreeNode>>,
    right: Option<Box<ASTreeNode>>,
}

impl ASTreeNode {
    fn new_with_values(
        value: Token,
        left: Option<Box<ASTreeNode>>,
        right: Option<Box<ASTreeNode>>,
    ) -> ASTreeNode {
        // Might change to options for left and right
        ASTreeNode { value, left, right }
    }

    fn new(value: Token) -> ASTreeNode {
        ASTreeNode {
            value,
            right: None,
            left: None,
        }
    }
}

impl From<ASTreeNode> for Vec<ASTreeNode> {
    fn from(item: ASTreeNode) -> Self {
        let mut vec: Vec<ASTreeNode> = Vec::new();
        vec.push(item);
        vec
    }
}

struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(input: &str) -> Result<Parser, String> {
        Ok(Parser {
            lexer: Lexer::new(input)?,
        })
    }

    fn atom(&mut self) -> Result<ASTreeNode, String> {
        match self.lexer.current_token.clone() {
            Token::DIGIT(i) => {
                self.lexer.get_next_token();
                Ok(ASTreeNode::new(Token::DIGIT(i)))
            }
            Token::LPAREN => {
                self.lexer.get_next_token();
                let result = self.expr();
                match self.lexer.current_token {
                    Token::RPAREN => {
                        self.lexer.get_next_token();
                        result
                    }
                    _ => {
                        println!("Current TOK ERR, {:?}", self.lexer.current_token);
                        Err("Expected ')'".into())
                    }
                }
            }
            Token::ADDOP(AddOp::MINUS) => {
                self.lexer.get_next_token();
                let mut current = ASTreeNode::new(Token::UNOP(UnaryOp::MINUS));
                current.left = Some(Box::new(self.atom()?));
                Ok(current)
            }
            Token::ADDOP(AddOp::PLUS) => {
                self.lexer.get_next_token();
                let mut current = ASTreeNode::new(Token::UNOP(UnaryOp::PLUS));
                current.left = Some(Box::new(self.atom()?));
                Ok(current)
            }
            Token::IDENT(i) => {
                self.lexer.get_next_token();
                Ok(ASTreeNode::new(Token::IDENT(i)))
            }
            _ => {
                println!("Current TOK ERR, {:?}", self.lexer.current_token);
                Err("Expected digit, '+' , '-' , or '(' ".into())
            }
        }
    }

    fn term(&mut self) -> Result<ASTreeNode, String> {
        let left = self.atom()?;

        if let Token::MULOP(i) = self.lexer.current_token.clone() {
            self.lexer.get_next_token();
            match i {
                MulOp::MULT => {
                    return Ok(ASTreeNode::new_with_values(
                        Token::MULOP(MulOp::MULT),
                        Some(Box::new(left)),
                        Some(Box::new(self.expr()?)),
                    ))
                }
                MulOp::DIV => {
                    return Ok(ASTreeNode::new_with_values(
                        Token::MULOP(MulOp::DIV),
                        Some(Box::new(left)),
                        Some(Box::new(self.expr()?)),
                    ))
                }
                MulOp::MODU => {
                    return Ok(ASTreeNode::new_with_values(
                        Token::MULOP(MulOp::MODU),
                        Some(Box::new(left)),
                        Some(Box::new(self.expr()?)),
                    ))
                }
            }
        }

        Ok(left)
    }

    fn addop(&mut self) -> Result<ASTreeNode, String> {
        let left = self.term()?;

        if let Token::ADDOP(i) = self.lexer.current_token.clone() {
            self.lexer.get_next_token();
            match i {
                AddOp::PLUS => {
                    return Ok(ASTreeNode::new_with_values(
                        Token::ADDOP(AddOp::PLUS),
                        Some(Box::new(left)),
                        Some(Box::new(self.expr()?)),
                    ))
                }
                AddOp::MINUS => {
                    return Ok(ASTreeNode::new_with_values(
                        Token::ADDOP(AddOp::MINUS),
                        Some(Box::new(left)),
                        Some(Box::new(self.expr()?)),
                    ))
                }
            }
        }

        Ok(left)
    }

    fn expr(&mut self) -> Result<ASTreeNode, String> {
        let left = self.addop()?;
        if Token::ASSIGN == self.lexer.current_token {
            self.lexer.get_next_token();
            return Ok(ASTreeNode::new_with_values(
                Token::ASSIGN,
                Some(Box::new(left)),
                Some(Box::new(self.expr()?)),
            ));
        }
        Ok(left)
    }

    pub fn return_value(&mut self) -> Result<ASTreeNode, String> {
        if Token::RET == self.lexer.current_token {
            self.lexer.get_next_token();
            let mut current = ASTreeNode::new(Token::RET);
            current.left = Some(Box::new(self.expr()?));
            Ok(current)
        } else {
            self.expr()
        }
    }

    fn declaration(&mut self) -> Result<ASTreeNode, String> {
        // declaration : type IDENTIFIER [ASSIGN expr] SEMI
        if let Token::Type(_i) = self.lexer.current_token.clone() {
            let mut result = ASTreeNode::new(self.lexer.current_token.clone());
            self.lexer.get_next_token();

            if let Token::IDENT(_i) = self.lexer.current_token.clone() {
                result.left = Some(Box::new(ASTreeNode::new(self.lexer.current_token.clone())));

                self.lexer.get_next_token();

                if self.lexer.current_token == Token::ASSIGN {
                    self.lexer.get_next_token();
                    result.right = Some(Box::new(self.expr()?));
                    return Ok(result);
                } else {
                    return Ok(result);
                }
            } else {
                return Err("Parsing Error: Expected identifier".into());
            }
        } else {
            Err("Parsing error: Expected type".into())
        }
    }

    fn statement(&mut self) -> Result<ASTreeNode, String> {
        /*
        statement  : (expr | declaration )
        */
        match self.lexer.current_token.clone() {
            Token::Type(_i) => self.declaration(),
            _ => self.return_value(),
        }
    }

    fn statement_list(&mut self) -> Result<ASTreeNode, String> {
        let mut statements_vec: Vec<ASTreeNode> = Vec::new();

        while self.lexer.current_token != Token::RBRACE {
            if self.lexer.current_token == Token::LBRACE {
                statements_vec.push(self.parse_block()?);
            } else {
                let curr = self.statement()?;
                if self.lexer.current_token == Token::RBRACE {
                    statements_vec.push(ASTreeNode::new_with_values(
                        Token::RET,
                        Some(Box::new(curr)),
                        None,
                    ));
                    break;
                } else {
                    if self.lexer.current_token == Token::SEMI {
                        self.lexer.get_next_token();
                        statements_vec.push(curr);
                    } else {
                        println!("{:?}", self.lexer.current_token);
                        return Err("Expected SEMI".into());
                    }
                }
            }
        }
        Ok(ASTreeNode::new(Token::StatementList(statements_vec)))
    }

    fn parse_block(&mut self) -> Result<ASTreeNode, String> {
        if self.lexer.current_token == Token::LBRACE {
            self.lexer.get_next_token();

            if self.lexer.current_token == Token::RBRACE {
                self.lexer.get_next_token();
                return Ok(ASTreeNode::new(Token::StatementList(Vec::new())));
            } else {
                let result = self.statement_list()?;

                if self.lexer.current_token == Token::RBRACE {
                    self.lexer.get_next_token();
                    Ok(result)
                } else {
                    println!("Current TOK ERR, {:?}", self.lexer.current_token);
                    Err("Expected '}'".into())
                }
            }
        } else {
            Err("Expected '{'".into())
        }
    }

    // pub fn start_block(&mut self)->Result<ASTreeNode,String>{
    //     // self.lexer.get_next_token();
    //     self.parse_block()
    // }
}

pub struct Interpreter {
    parser: Parser,
    global_vars: HashMap<String, Option<Token>>,
}

impl Interpreter {
    pub fn new(input: &str) -> Result<Interpreter, String> {
        Ok(Interpreter {
            parser: Parser::new(input)?,
            global_vars: HashMap::new(),
        })
    }

    fn interpret_statement(&mut self, input: ASTreeNode)->Result<Option<i32>, String>{
       let mut result = Ok(None);

       if input.value == Token::RET{
           result = self.interpret_input(input);
       }else{
           self.interpret_input(input)?;
       }

        result
    }

    fn find_var(&mut self, input: &str) -> Option<&Option<Token>>{
        self.global_vars.get(input)
    }

    fn declare_var(&mut self,  name: String, value:Option<Token>)->Result<(),String>{
        match self.global_vars.insert(name, value){
            None=>Ok(()),
            Some(_)=>Err("Interpreting Error: Unable to declare Var.".into())
        }
        
    }

    fn interpret_input(&mut self, input: ASTreeNode) -> Result<Option<i32>, String> {
        match input.value {
            Token::DIGIT(n) => Ok(Some(n)),
            Token::ADDOP(n) => match n {
                AddOp::MINUS => Ok(Some(
                    self.interpret_input(*input.left.unwrap())?.unwrap()
                        - self.interpret_input(*input.right.unwrap())?.unwrap(),
                )),
                AddOp::PLUS => Ok(Some(
                    self.interpret_input(*input.left.unwrap())?.unwrap()
                        + self.interpret_input(*input.right.unwrap())?.unwrap(),
                )),
            },
            Token::MULOP(n) => match n {
                MulOp::MULT => Ok(Some(
                    self.interpret_input(*input.left.unwrap())?.unwrap()
                        * self.interpret_input(*input.right.unwrap())?.unwrap(),
                )),
                MulOp::DIV => Ok(Some(
                    self.interpret_input(*input.left.unwrap())?.unwrap()
                        / self.interpret_input(*input.right.unwrap())?.unwrap(),
                )),
                MulOp::MODU => Ok(Some(
                    self.interpret_input(*input.left.unwrap())?.unwrap()
                        % self.interpret_input(*input.right.unwrap())?.unwrap(),
                )),
            },
            Token::UNOP(n) => match n {
                UnaryOp::PLUS => Ok(self.interpret_input(*input.left.unwrap())?),
                UnaryOp::MINUS => Ok(Some(-self.interpret_input(*input.left.unwrap())?.unwrap())),
            },
            Token::StatementList(list) => {
                if list.is_empty(){
                    return Ok(None);
                }
                for i in list {

                    // TODO: make interpret statement fn
                    // TODO: while intepret_statement() == None
                    if let Some(i) = self.interpret_statement(i)?{
                        return Ok(Some(i))
                    }
                }
                Err("Error parsing result".into())
            }
            Token::Type(t) => {
                if let Token::IDENT(i) = (*(input.left.unwrap())).value {
                    if let Some(_) = self.find_var(&i) {
                        return Err("Variable already declared!".into());
                    }else{
                        if let Some(j)=input.right{
                            self.declare_var(i, Some((*j).value))?;
                        }else{
                            self.declare_var(i, None)?;
                        }
                        Ok(None)
                    }
                }else{
                    Err("Interpreting Error: Expected identifier".into())
                }
            },
            Token::IDENT(i)=>{
                match self.find_var(&i){
                    Some(j)=>match j{
                        Some(Token::DIGIT(k))=>Ok(Some(k.to_owned())),
                        Some(_)=>Err("Interpreting Error: Varibale of unrecognized type, unknown reason".into()),
                        None=>Err("Interpreting Error: Variable not initialized".into())
                    }
                    None=>Err("Interpreting Error: Variable Not Declared".into())
                }
            },
            Token::RET=>{
                if let Some(i) = input.left{
                    Ok(self.interpret_input(*i)?)
                }else{
                    Err("Interpreting error: no argument to return statement".into())
                }
            },
            _ =>{
                    println!("Current Err ASTNODE: {:?}", input);
                    Err("Interpreting Error: Unknown Token".into())
                 },
        }
    }

    pub fn interpret_block(&mut self) -> Result<Option<i32>, String> {
        let curr = self.parser.statement()?;
        self.interpret_input(curr)
    }
    pub fn interpret_program(&mut self) -> Result<Option<i32>, String> {
        let curr = self.parser.parse_block()?;
        self.interpret_input(curr)
    }
}

#[allow(dead_code)]
pub struct Translator {
    parser: Parser,
}

#[allow(dead_code)]
impl Translator {
    pub fn new(input: &str) -> Result<Translator, String> {
        Ok(Translator {
            parser: Parser::new(input)?,
        })
    }

    fn rpn_interp(input: ASTreeNode) -> Result<String, String> {
        let mut result = String::new();
        match input.value {
            Token::DIGIT(n) => result.push_str(&n.to_string()),
            Token::ADDOP(n) => match n {
                AddOp::PLUS => {
                    result.push_str(&Translator::rpn_interp(*(input.left.unwrap()))?);
                    result.push(' ');
                    result.push_str(&Translator::rpn_interp(*input.right.unwrap())?);
                    result.push(' ');
                    result.push('+');
                }
                AddOp::MINUS => {
                    result.push_str(&Translator::rpn_interp(*(input.left.unwrap()))?);
                    result.push(' ');
                    result.push_str(&Translator::rpn_interp(*input.right.unwrap())?);
                    result.push(' ');
                    result.push('-');
                }
            },
            Token::MULOP(n) => match n {
                MulOp::MULT => {
                    result.push_str(&Translator::rpn_interp(*(input.left.unwrap()))?);
                    result.push(' ');
                    result.push_str(&Translator::rpn_interp(*input.right.unwrap())?);
                    result.push(' ');
                    result.push('*');
                }

                MulOp::DIV => {
                    result.push_str(&Translator::rpn_interp(*(input.left.unwrap()))?);
                    result.push(' ');
                    result.push_str(&Translator::rpn_interp(*input.right.unwrap())?);
                    result.push(' ');
                    result.push('/');
                }
                MulOp::MODU => {
                    result.push_str(&Translator::rpn_interp(*(input.left.unwrap()))?);
                    result.push(' ');
                    result.push_str(&Translator::rpn_interp(*input.right.unwrap())?);
                    result.push(' ');
                    result.push('%');
                }
            },
            _ => return Err(format!("ERROR unexpected Token: {:?}", input.value)),
        }
        Ok(result)
    }

    pub fn rpn_translate(&mut self) -> Result<String, String> {
        Translator::rpn_interp(self.parser.expr()?)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn basic_add() {
        assert_eq!(
            3,
            Interpreter::new("1+2")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn unary_minus() {
        assert_eq!(
            3,
            Interpreter::new("--3")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn unary_plus() {
        assert_eq!(
            3,
            Interpreter::new("++3")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn unary_both() {
        assert_eq!(
            3,
            Interpreter::new("++3")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn chain_add() {
        assert_eq!(
            6,
            Interpreter::new("1+2+3")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn precedence_test() {
        assert_eq!(
            7,
            Interpreter::new("1+2*3")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn precedence_test2() {
        assert_eq!(
            5,
            Interpreter::new("1*2+3")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }
    #[test]
    fn parentheses_test() {
        assert_eq!(
            9,
            Interpreter::new("(1+2)*3")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }
    #[test]
    fn basic_interp_plus() {
        assert_eq!(
            3,
            Interpreter::new("1+2")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn basic_interp_minus() {
        assert_eq!(
            1,
            Interpreter::new("2-1")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn basic_interp_times() {
        assert_eq!(
            6,
            Interpreter::new("2*3")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn basic_interp_divide() {
        assert_eq!(
            0,
            Interpreter::new("2/3")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn basic_interp_modulo() {
        assert_eq!(
            2,
            Interpreter::new("2%3")
                .unwrap()
                .interpret_block()
                .unwrap()
                .unwrap()
        );
    }

    #[test]
    fn parse_empty_block() {
        let root = Parser::new("{}");
        assert_eq!(
            Ok(ASTreeNode::new(Token::StatementList(Vec::new()))),
            root.unwrap().parse_block()
        )
    }
    #[test]
    fn interp_empty_block() {
        assert_eq!(
            None,
            Interpreter::new("{}").unwrap().interpret_program().unwrap()
        )
    }
    #[test]
    fn parse_block_basic() {
        let root = Parser::new("{1+2;}");
        assert_eq!(
            ASTreeNode::new(Token::StatementList(vec![ASTreeNode::new_with_values(
                Token::ADDOP(AddOp::PLUS),
                Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))),
                Some(Box::new(ASTreeNode::new(Token::DIGIT(2))))
            )])),
            root.unwrap().parse_block().unwrap()
        )
    }
    #[test]
    fn parse_block2() {
        let root = Parser::new("{1+2;3+2;}");
        assert_eq!(
            Ok(ASTreeNode::new(Token::StatementList(vec![
                ASTreeNode::new_with_values(
                    Token::ADDOP(AddOp::PLUS),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(2))))
                ),
                ASTreeNode::new_with_values(
                    Token::ADDOP(AddOp::PLUS),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(3)))),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(2))))
                )
            ]))),
            root.unwrap().parse_block()
        )
    }

    #[test]
    fn parse_block_with_assign() {
        let root = Parser::new("{1+2;3+2; int a = 3;}");
        assert_eq!(
            Ok(ASTreeNode::new(Token::StatementList(vec![
                ASTreeNode::new_with_values(
                    Token::ADDOP(AddOp::PLUS),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(2))))
                ),
                ASTreeNode::new_with_values(
                    Token::ADDOP(AddOp::PLUS),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(3)))),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(2))))
                ),
                ASTreeNode::new_with_values(
                    Token::Type(Type::INT),
                    Some(Box::new(ASTreeNode::new(Token::IDENT("a".into())))),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(3))))
                )
            ]))),
            root.unwrap().parse_block()
        )
    }
    #[test]
    fn parse_block_nosemi() {
        let root = Parser::new("{1+2;3+2}");
        assert_eq!(
            Ok(ASTreeNode::new(Token::StatementList(vec![
                ASTreeNode::new_with_values(
                    Token::ADDOP(AddOp::PLUS),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(2))))
                ),
                ASTreeNode::new_with_values(
                    Token::RET,
                    Some(Box::new(ASTreeNode::new_with_values(
                        Token::ADDOP(AddOp::PLUS),
                        Some(Box::new(ASTreeNode::new(Token::DIGIT(3)))),
                        Some(Box::new(ASTreeNode::new(Token::DIGIT(2))))
                    ))),
                    None
                )
            ]))),
            root.unwrap().parse_block()
        )
    }

    #[test]
    fn parser_precedence() {
        assert_eq!(
            ASTreeNode::new_with_values(
                Token::ADDOP(AddOp::PLUS),
                Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))),
                Some(Box::new(ASTreeNode::new_with_values(
                    Token::MULOP(MulOp::MULT),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(2)))),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(3))))
                )))
            ),
            Parser::new("1+2*3").unwrap().expr().unwrap()
        )
    }

    #[test]
    fn lexer_peek() {
        let lex = Lexer::new("1+2").unwrap();
        assert_eq!(lex.current_token, Token::DIGIT(1));
        assert_eq!(lex.peek(), Some('+'))
    }

    #[test]
    fn assignment() {
        assert_eq!(
            ASTreeNode::new_with_values(
                Token::ASSIGN,
                Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))),
                Some(Box::new(ASTreeNode::new(Token::DIGIT(1))))
            ),
            Parser::new("1=1").unwrap().expr().unwrap()
        )
    }

    #[test]
    fn parser_test() {
        assert_eq!(
            ASTreeNode::new_with_values(
                Token::ADDOP(AddOp::PLUS),
                Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))),
                Some(Box::new(ASTreeNode::new(Token::DIGIT(2))))
            ),
            Parser::new("1+2").unwrap().expr().unwrap()
        )
    }
    #[test]
    fn parser_test_mult() {
        assert_eq!(
            ASTreeNode::new_with_values(
                Token::MULOP(MulOp::MULT),
                Some(Box::new(ASTreeNode::new(Token::DIGIT(1)))),
                Some(Box::new(ASTreeNode::new(Token::DIGIT(2))))
            ),
            Parser::new("1*2").unwrap().expr().unwrap()
        )
    }

    #[test]
    fn lexer_test() {
        let mut tok = Lexer::new("1+2").unwrap();
        assert_eq!(Token::DIGIT(1), tok.current_token);
        tok.get_next_token();
        assert_eq!(Token::ADDOP(AddOp::PLUS), tok.current_token);
        tok.get_next_token();
        assert_eq!(Token::DIGIT(2), tok.current_token);
    }

    #[test]
    fn lexer_return() {
        let mut tok = Lexer::new("return a").unwrap();
        assert_eq!(Token::RET, tok.current_token);
        tok.get_next_token();
        assert_eq!(Token::IDENT("a".into()), tok.current_token);
    }
    #[test]
    fn lexer_test_assign() {
        let mut tok = Lexer::new("1=2").unwrap();
        assert_eq!(Token::DIGIT(1), tok.current_token);
        tok.get_next_token();
        assert_eq!(Token::ASSIGN, tok.current_token);
        tok.get_next_token();
        assert_eq!(Token::DIGIT(2), tok.current_token);
    }
    #[test]
    fn lexer_test_ident_vs_key() {
        let mut tok = Lexer::new("int a").unwrap();
        assert_eq!(Token::Type(Type::INT), tok.current_token);
        tok.get_next_token();
        assert_eq!(Token::IDENT("a".into()), tok.current_token);
    }

    #[test]
    fn atom_test() {
        let mut pars = Parser::new("1+2").unwrap();
        assert_eq!(Ok(ASTreeNode::new(Token::DIGIT(1))), pars.atom())
    }

    #[test]
    fn atom_test3() {
        let mut pars = Parser::new("1+2").unwrap();
        pars.lexer.get_next_token();
        pars.lexer.get_next_token();

        assert_eq!(Ok(ASTreeNode::new(Token::DIGIT(2))), pars.atom())
    }

    #[test]
    fn parser_basic() {
        assert_eq!(
            ASTreeNode::new(Token::DIGIT(1)),
            Parser::new("1").unwrap().expr().unwrap()
        )
    }

    #[test]
    fn parser_statement() {
        assert_eq!(
            Parser::new("1+2").unwrap().expr().unwrap(),
            Parser::new("1+2;").unwrap().statement().unwrap()
        )
    }
    #[test]
    fn parser_return() {
        assert_eq!(
            ASTreeNode::new_with_values(
                Token::RET,
                Some(Box::new(ASTreeNode::new(Token::DIGIT(3)))),
                None
            ),
            Parser::new("return 3").unwrap().statement().unwrap()
        )
    }

    #[test]
    fn rpn_translate() {
        assert_eq!(
            "1 2 +",
            Translator::new("1+2").unwrap().rpn_translate().unwrap()
        )
    }

    #[test]
    fn basic_declaration() {
        assert_eq!(
            ASTreeNode::new_with_values(
                Token::Type(Type::INT),
                Some(Box::new(ASTreeNode::new(Token::IDENT("a".into())))),
                Some(Box::new(ASTreeNode::new(Token::DIGIT(3)))),
            ),
            Parser::new("int a = 3").unwrap().statement().unwrap()
        )
    }

    #[test]
    fn test_vars() {
        assert_eq!(
            3,
            Interpreter::new("{int a = 3; a}")
                .unwrap()
                .interpret_program()
                .unwrap()
                .unwrap()
        )
    }
    #[test]
    fn parse_declarations() {
        assert_eq!(
            ASTreeNode::new_with_values(
                Token::StatementList(vec![ASTreeNode::new_with_values(
                    Token::Type(Type::INT),
                    Some(Box::new(ASTreeNode::new(Token::IDENT("a".into())))),
                    Some(Box::new(ASTreeNode::new(Token::DIGIT(3))))
                )]),
                None,
                None
            ),
            Parser::new("{int a = 3;}").unwrap().parse_block().unwrap()
        )
    }

    #[test]
    fn double_declaration() {
        assert_eq!(
            Interpreter::new("{int a; int a;}")
                .unwrap()
                .interpret_program(),
            Err("Variable already declared!".into())
        )
    }

}
