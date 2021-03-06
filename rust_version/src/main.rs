#![deny(missing_docs)]
//! This crate is made as a test of skills of some sort.
//! It Takes code inputs and returns numeric outputs for the most part.
use std::collections::HashMap;
use std::fs;
use std::io::stdin;
use std::iter::FromIterator;
use std::path::PathBuf;
use structopt::StructOpt;
// use std::mem::discriminant;

#[derive(StructOpt, Debug)]
struct CLI {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn input() -> String {
    let mut ret = String::new();
    stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret
}

fn main() {
    let opt = CLI::from_args();

    if opt.debug {
        println!("{:#?}", opt);
    }

    match opt.output {
        None => loop {
            println!(
                "{:#?}",
                Interpreter::new(&input())
                    .unwrap()
                    .interpret_block()
                    .unwrap()
            )
        },
        Some(i) => println!(
            "{:#?}",
            Interpreter::new(
                &fs::read_to_string(i).expect("Something went wrong reading the file")
            )
            .unwrap()
            .interpret_program()
            .unwrap()
        ),
    }
}

/**
 *
 * NOTE: IF IN CAPITALS, CONSUME AND ADVANCE
 *
 * Current Grammar:
 *
 * program          : MAIN block
 * block            : LBRACE [statement_list] RBRACE
 * statement_list   : [(statement [SEMI]|block|function) [statement_list]]
 * statement        : (expr | declaration | if)  
 * expr             : addop *(ASSIGN expr)
 * if               :
 * addop            : term *((PLUS/MINUS) expr)
 * mulop            : atom ((MUL/DIV) expr)
 * atom             : (PLUS/MINUS) atom |
 *                       INTEGER |
 *                       LPAREN expr RPAREN
 * declaration      : type IDENTIFIER [ASSIGN expr]
 * assignment       : identifier ASSIGN expr
 * type             : INT|FLOAT //TODO: IMPLEMENT FLOAT
 * identifier       : alphabetic *(alphanumeric) //don't know how to write this
 * LBRACE = '{'
 * RBRACE = '}'
 * LPAREN = '('
 * RPAREN = ')'
 * ASSIGN = '='
 * COMMA  = ','
 * MAIN   = 'main'
 *
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
 *      14/2/2020: Still haven't figured it out
 */

// enum Category{
//     FUNCTION(Type, Box<ASTreeNode>),
//     VAR(Type)
// }

#[derive(Clone, Debug, PartialEq, Copy)]
enum Type {
    INT,
    FLOAT,
    _STRING,
    FUNC,
    NONE,
    _TYPE,
    // BOOL
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Compare{
    LT,
    GT,
    EQ,
    NE,
    LE,
    GE,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    FLOAT(f32),
    DIGIT(i32),
    ADDOP(AddOp),
    MULOP(MulOp),
    UNOP(UnaryOp),
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMI,
    COMPARE(Compare),
    ASSIGN,
    EOF,
    COMMA,
    IDENT(String),
    StatementList(Vec<ASTreeNode>),
    FuncData(String, Type, Vec<(Type, String)>, Box<ASTreeNode>),
    ArgList(Vec<ASTreeNode>),
    RET,
    ARROW,
    Type(Type),
    If,
    Else,
    IfData(Box<ASTreeNode>),
    BOOL(Bool)
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

#[derive(Clone, Debug, PartialEq)]
enum Bool{
    True,
    False
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

        if self.position < self.len && self.input[self.position] == '.' {
            number_so_far.push(self.input[self.position]);
            self.position += 1;
            while self.position < self.len && self.input[self.position].is_digit(10) {
                number_so_far.push(self.input[self.position]);
                self.position += 1;
            }
            Token::FLOAT(number_so_far.parse().unwrap())
        } else {
            Token::DIGIT(number_so_far.parse().unwrap())
        }
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

    fn skip_comment(&mut self) {
        while self.input[self.position] != '\n' {
            self.position += 1;
        }
    }

    pub fn get_next_token(&mut self) {
        if self.position >= self.len {
            self.current_token = Token::EOF;
            return;
        }
        let mut current_char = self.input[self.position];

        while current_char.is_whitespace() {
            self.position += 1;
            current_char = self.input[self.position];
        }

        if current_char.is_digit(10) {
            self.current_token = self.digit();
            return;
        }

        if current_char.is_alphabetic() {
            self.current_token = self.identifier();
            return;
        }
        self.position += 1;

        match current_char {
            '+' => self.current_token = Token::ADDOP(AddOp::PLUS),
            '-' => {
                if let Some(n) = self.peek() {
                    match n {
                        '>' => {
                            self.current_token = Token::ARROW;
                            self.position += 1;
                        }
                        _ => self.current_token = Token::ADDOP(AddOp::MINUS),
                    }
                }
            }
            '*' => self.current_token = Token::MULOP(MulOp::MULT),
            '/' => {
                if let Some(n) = self.peek() {
                    match n {
                        '/' => {
                            self.skip_comment();
                            self.get_next_token();
                        }
                        _ => self.current_token = Token::MULOP(MulOp::DIV),
                    }
                }
            }
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
                            self.current_token = Token::COMPARE(Compare::EQ);
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
                            self.current_token = Token::COMPARE(Compare::LE);
                            self.position += 1;
                        }
                        _ => self.current_token = Token::COMPARE(Compare::LT),
                    }
                }
            }
            '>' => {
                if let Some(n) = self.peek() {
                    match n {
                        '=' => {
                            self.current_token = Token::COMPARE(Compare::GE);
                            self.position += 1;
                        }
                        _ => self.current_token = Token::COMPARE(Compare::GT),
                    }
                }
            }
            ',' => self.current_token = Token::COMMA,
            '!' => {
                if let Some(n) = self.peek() {
                    match n {
                        '=' => {
                            self.current_token = Token::COMPARE(Compare::NE);
                            self.position += 1;
                        }
                        _ => panic!("UNRECOGNIZED TOKEN: !{}", current_char),
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
            ("fn".into(), Token::Type(Type::FUNC)),
            ("if".into(), Token::If),
            ("else".into(), Token::Else),
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

    fn func_call(&mut self) -> Result<Vec<ASTreeNode>, String> {
        // let result = ASTreeNode::new(Token::ArgList);
        let mut args: Vec<ASTreeNode> = Vec::new();

        self.lexer.get_next_token(); // ASSUMING already an LPAREN
        while self.lexer.current_token != Token::RPAREN || self.lexer.current_token == Token::COMMA
        {
            args.push(self.expr()?);//Error here
        }

        self.lexer.get_next_token();
        Ok(args)
    }

    fn atom(&mut self) -> Result<ASTreeNode, String> {
        match self.lexer.current_token.clone() {
            Token::DIGIT(i) => {
                self.lexer.get_next_token();
                Ok(ASTreeNode::new(Token::DIGIT(i)))
            }
            Token::FLOAT(i) => {
                self.lexer.get_next_token();
                Ok(ASTreeNode::new(Token::FLOAT(i)))
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
                if Token::LPAREN == self.lexer.current_token {
                    return Ok(ASTreeNode::new_with_values(
                        Token::IDENT(i),
                        Some(Box::new(ASTreeNode::new(Token::ArgList(self.func_call()?)))),
                        None,
                    ));
                }
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
        let curr = self.lexer.current_token.clone();
        if let Token::MULOP(_) = self.lexer.current_token {
            self.lexer.get_next_token();
            Ok(ASTreeNode::new_with_values(
                curr,
                Some(Box::new(left)),
                Some(Box::new(self.expr()?)),
            ))
        }else{
            Ok(left)
        }
    }

    fn addop(&mut self) -> Result<ASTreeNode, String> {
        let left = self.term()?;
        let curr = self.lexer.current_token.clone();
        if let Token::ADDOP(_) = self.lexer.current_token {
            self.lexer.get_next_token();
            Ok(ASTreeNode::new_with_values(
                curr,
                Some(Box::new(left)),
                Some(Box::new(self.expr()?)),
            ))
        }else{
            Ok(left)
        }
    }

    fn compare(&mut self) -> Result<ASTreeNode, String> {
        let left = self.addop()?;

        let curr = self.lexer.current_token.clone();

        if let Token::COMPARE(_) = self.lexer.current_token {
            self.lexer.get_next_token();
            Ok(ASTreeNode::new_with_values(
                curr,
                Some(Box::new(left)),
                Some(Box::new(self.expr()?)),
            ))
        }else{
            Ok(left)
        }
    }

    fn expr(&mut self) -> Result<ASTreeNode, String> {
        let left = self.compare ()?;
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

    fn get_arg_list(&mut self) -> Result<Vec<(Type, String)>, String> {
        if Token::LPAREN != self.lexer.current_token {
            return Err("expected '('".into());
        }
        self.lexer.get_next_token();
        let mut result: Vec<(Type, String)> = Vec::new();
        while self.lexer.current_token != Token::RPAREN {
            let t;
            if let Token::Type(i) = self.lexer.current_token.clone() {
                t = i;
                self.lexer.get_next_token();
            } else {
                return Err(format!(
                    "Expected type, current token: {:#?}",
                    self.lexer.current_token
                ));
            }

            if let Token::IDENT(i) = self.lexer.current_token.clone() {
                result.push((t, i));
                self.lexer.get_next_token();
            } else {
                return Err(format!(
                    "Expected Identifier, current token: {:#?}",
                    self.lexer.current_token
                ));
            }
        }
        // self.lexer.get_next_token();

        if Token::RPAREN != self.lexer.current_token {
            return Err(format!(
                "Expected Identifier, current token: {:#?}",
                self.lexer.current_token
            ));
        }
        Ok(result)
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

        match self.lexer.current_token {
            Token::Type(Type::FUNC) => {
                let mut result = ASTreeNode::new(Token::Type(Type::FUNC));
                // expect IDENT
                self.lexer.get_next_token();
                if let Token::IDENT(name) = self.lexer.current_token.clone() {
                    //Expect PARENS/ARGS LIST
                    self.lexer.get_next_token();
                    let args = self.get_arg_list()?;
                    let mut func_type = Type::NONE;

                    self.lexer.get_next_token();
                    if Token::ARROW == self.lexer.current_token {
                        self.lexer.get_next_token();
                        if let Token::Type(i) = self.lexer.current_token.clone() {
                            func_type = i;
                            self.lexer.get_next_token();
                        } else {
                            return Err("Expected Type!".into());
                        }
                    }
                    if Token::LBRACE == self.lexer.current_token {
                        result.left = Some(Box::new(ASTreeNode::new(Token::FuncData(
                            name,
                            func_type,
                            args,
                            Box::new(self.parse_block()?),
                        ))));
                        // !WARNING, test line
                        // self.lexer.get_next_token();
                        Ok(result)
                    } else {
                        Err("Expected '->' or {".into())
                    }
                } else {
                    Err("Expected Function Name".into())
                }
            }
            Token::Type(_) => {
                let mut result = ASTreeNode::new(self.lexer.current_token.clone());
                self.lexer.get_next_token();

                if let Token::IDENT(_i) = &self.lexer.current_token {
                    result.left = Some(Box::new(ASTreeNode::new(self.lexer.current_token.clone())));

                    self.lexer.get_next_token();

                    if self.lexer.current_token == Token::ASSIGN {
                        self.lexer.get_next_token();
                        result.right = Some(Box::new(self.expr()?));
                        Ok(result)
                    } else {
                        Ok(result)
                    }
                } else {
                    Err("Parsing Error: Expected identifier".into())
                }
            }
            _ => Err("Parsing error: Expected type".into()),
        }
    }

    fn get_if_body(&mut self) -> Result<ASTreeNode, String> {

        if Token::LBRACE != self.lexer.current_token {
            return Err(format!(
                "expected '{{' after condition expression.\n Current token: {:#?}",
                self.lexer.current_token
            ));
        }
        self.lexer.get_next_token();
        let block = self.statement_list()?;

        if Token::RBRACE != self.lexer.current_token {
            return Err(format!(
                "expected '}}' after condition expression.\n Current token: {:#?}",
                self.lexer.current_token
            ));
        }

        Ok(block)
    }

    fn if_statement(&mut self) -> Result<ASTreeNode, String> {
        //current token is if
        //grammar: IF expr LBRACE StatementList RBRACE [else_statement]
        //            ^
        //Do i need parens? Don't think so.
        self.lexer.get_next_token();

        let condition = self.expr()?;
        //grammar: IF expr LBRACE StatementList RBRACE [else_statement]
        //                  ^

        let block = self.get_if_body()?;

        self.lexer.get_next_token();

        if self.lexer.current_token == Token::Else{
            self.lexer.get_next_token();
            let right : ASTreeNode;
            if self.lexer.current_token == Token::If{
                right = self.if_statement()?;
            }else{
                right = self.get_if_body()?;
            }
            self.lexer.get_next_token();

            Ok(ASTreeNode::new_with_values(
                Token::IfData(Box::new(condition)),
                Some(Box::new(block)),
                Some(Box::new(right)),
            ))
        }else{
            Ok(ASTreeNode::new_with_values(
                Token::IfData(Box::new(condition)),
                Some(Box::new(block)),
                None,
            ))
        }
    }

    fn statement(&mut self) -> Result<ASTreeNode, String> {
        /*
        statement  : (expr | declaration )
        */
        match self.lexer.current_token.clone() {
            Token::Type(_) => self.declaration(),
            Token::If => self.if_statement(),
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
                
                if curr.value == Token::Type(Type::FUNC) {
                    statements_vec.push(curr);
                } else if let Token::IfData(_) = curr.value.clone(){
                    statements_vec.push(curr);
                } else if self.lexer.current_token == Token::RBRACE {
                    statements_vec.push(ASTreeNode::new_with_values(
                        Token::RET,
                        Some(Box::new(curr)),
                        None,
                    ));
                } else if self.lexer.current_token == Token::SEMI {
                    self.lexer.get_next_token();
                    statements_vec.push(curr);
                } else {
                println!("CURR: {:#?}", curr);
                println!("Current Token: {:?}", self.lexer.current_token);
                return Err("Expected SEMI".into());
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
                Ok(ASTreeNode::new(Token::StatementList(Vec::new())))
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
type Scope = Vec<Vec<HashMap<String, (Type, Option<Token>)>>>;
struct Interpreter {
    parser: Parser,
    global_vars: HashMap<String, (Type, Option<Token>)>,
    scope: Scope,
}

impl Interpreter {
    pub fn new(input: &str) -> Result<Interpreter, String> {
        Ok(Interpreter {
            parser: Parser::new(input)?,
            global_vars: HashMap::new(),
            scope: Vec::new(),
        })
    }

    fn interpret_statement(&mut self, input: ASTreeNode) -> Result<Token, String> {
        if let Token::IfData(_) = input.value.clone() {
            self.interpret_input(input)
        } else if input.value == Token::RET {
            self.interpret_input(input)
        } else {
            self.interpret_input(input)?;
            Ok(Token::Type(Type::NONE))
        }
    }

    fn update_var(&mut self, name: &str, value: Token) -> Result<Token, String> {
        for i in self.scope.last_mut().unwrap().iter_mut().rev() {
            if let Some(j) = i.get_mut(name) {
                *j = ((j.0), Some(value.clone()));
                return Ok(value);
            }
        }
        match self.global_vars.get_mut(name) {
            Some(j) => {
                *j = ((j.0), Some(value.clone()));
                Ok(value)
            }
            None => Err("Variable not found/declared".into()),
        }
    }
    /**
     * Search for var in lexical scopes, then global scope.
     */
    fn find_var(&mut self, input: &str) -> Option<(Type, Option<Token>)> {
        for i in self.scope.last_mut().unwrap().iter_mut().rev() {
            if let Some(j) = i.get(input) {
                return Some((*j).clone());
            }
        }
        Some((*self.global_vars.get(input)?).clone())
    }
    fn var_declared(&mut self, input: &str) -> bool {
        if let Some(i) = self.scope.last().unwrap().last() {
            (*i).get(input).is_some()
        } else {
            self.global_vars.get(input).is_some()
        }
    }

    fn declare_var(
        &mut self,
        name: String,
        var_type: Type,
        value: Option<Token>,
    ) -> Result<(), String> {
        if self.scope.last().unwrap().is_empty() {
            match self.global_vars.insert(name, (var_type, value)) {
                None => Ok(()),
                Some(_) => Err("Interpreting Error: Unable to declare Var.".into()),
            }
        } else if let Some(i) = self.scope.last_mut().unwrap().last_mut() {
            match i.insert(name, (var_type, value)) {
                None => Ok(()),
                Some(_) => Err("Interpreting Error: Unable to declare Var.".into()),
            }
        } else {
            Err("Unknown Interpreting error, unable to declare var".into())
        }
    }

    // fn add(&mut self, input: ASTreeNode) -> Result<Option<Token>, String>

    fn add(&mut self, input: ASTreeNode) -> Result<Token, String> {
        if let Some(j) = input.left {
            if let Some(k) = input.right {
                if let Token::DIGIT(m) = self.interpret_input(*(j.clone()))? {
                    if let Token::DIGIT(n) = self.interpret_input(*k)? {
                        match input.value {
                            Token::ADDOP(AddOp::PLUS) => Ok(Token::DIGIT(m + n)),
                            Token::ADDOP(AddOp::MINUS) => Ok(Token::DIGIT(m - n)),
                            Token::MULOP(MulOp::MULT) => Ok(Token::DIGIT(m * n)),
                            Token::MULOP(MulOp::DIV) => Ok(Token::DIGIT(m / n)),
                            Token::MULOP(MulOp::MODU) => Ok(Token::DIGIT(m % n)),
                            _ => Err("Unkown interpreting error - unexpected operations".into()),
                        }
                    } else {
                        Err("R-value cannot be non-digit item".into())
                    }
                } else if let Token::FLOAT(m) = self.interpret_input(*j)? {
                    if let Token::FLOAT(n) = self.interpret_input(*k)? {
                        match input.value {
                            Token::ADDOP(AddOp::PLUS) => Ok(Token::FLOAT(m + n)),
                            Token::ADDOP(AddOp::MINUS) => Ok(Token::FLOAT(m - n)),
                            Token::MULOP(MulOp::MULT) => Ok(Token::FLOAT(m * n)),
                            Token::MULOP(MulOp::DIV) => Ok(Token::FLOAT(m / n)),
                            Token::MULOP(MulOp::MODU) => Ok(Token::FLOAT(m % n)),
                            _ => Err("Unkown interpreting error - unexpected operations".into()),
                        }
                    } else {
                        Err("R-value cannot be non-float item".into())
                    }
                } else {
                    Err("L-value must be float or digit item".into())
                }
            } else {
                Err("interpreting error, need r - value in operation.".into())
            }
        } else {
            Err("Need at least two values to add".into())
        }
    }
    // purely lexical checking of types... or is it?
    fn check_vars(&mut self, args: Option<Token>, input: ASTreeNode) -> Result<(), String> {
        match args {
            Some(i) => {
                if let Token::ArgList(j) = (*(input.left.unwrap())).value {
                    if let Token::FuncData(g, _, n, _) = i.clone() {
                        for it in n.iter().zip(j.iter()) {
                            let (ai, bi) = it;
                            let bi = self.interpret_input((*bi).clone())?;
                            match bi {
                                Token::DIGIT(_) => {
                                    if ai.0 != Type::INT {
                                        return Err(format!(
                                            "{} is of incorrect type: Should be {:#?}, is INT",
                                            ai.1, ai.0
                                        ));
                                    }
                                }
                                Token::FLOAT(_) => {
                                    if ai.0 != Type::FLOAT {
                                        return Err(format!(
                                            "{} is of incorrect type: Should be {:#?}, is FLOAT",
                                            ai.1, ai.0
                                        ));
                                    }
                                }
                                Token::Type(i) => {
                                    if ai.0 != i {
                                        return Err(format!(
                                            "{} is of incorrect type: Should be {:#?}, is {:#?}",
                                            ai.1, ai.0, i
                                        ));
                                    }
                                }
                                _ => return Err(format!("Unable to check syntax of argument. Token found: {:#?}",bi)),
                            }
                            self.declare_var(ai.1.clone(), ai.0, Some(bi))?;
                        }
                        self.declare_var(g, Type::FUNC, Some(i))?;
                        return Ok(());
                    }
                }
                Err("Error checking types of arguments".into())
            }
            None => Err("Error checking types of arguments".into()),
        }
    }

    fn update_args(&mut self, mut input: ASTreeNode)->Result<ASTreeNode,String>{
        let mut new_vec : Vec<ASTreeNode> = Vec::new();
        if let Token::ArgList(j) = (*(input.left.unwrap())).value {
            for arg in j{
                new_vec.push(ASTreeNode::new(self.interpret_input(arg)?));
            }
        } 

        input.left = Some(Box::new(ASTreeNode::new(Token::ArgList(new_vec))));

        Ok(input)
    }

    fn interpret_input(&mut self, input: ASTreeNode) -> Result<Token, String> {
        match input.clone().value.clone() {
            Token::DIGIT(_) => Ok(input.value),
            Token::FLOAT(_) => Ok(input.value),
            Token::IDENT(i) => {
                match self.find_var(&i) {
                    //de-structure result - tuple
                    Some(j) => {
                        if j.0 == Type::FUNC {
                            if let Token::FuncData(_, _, _, m) = j.1.clone().unwrap() {
                                // push new scope of scopes
                                let input_clone = self.update_args(input.clone())?;

                                self.scope.push(Vec::new());
                                // push new scope to scope of scopes
                                self.scope.last_mut().unwrap().push(HashMap::new());
                                // check arg types
                                self.check_vars(j.1.clone(), input_clone)?;
                                // add variables from arglist
                                // self. (input.left.unwrap().value)?;
                                // return AST
                                Ok(self.interpret_input(*m)?)
                            } else {
                                Err("Wrong Token value in Map".into())
                            }
                        } else {
                            match j.1 {
                                // match found variable value
                                Some(k) => {
                                    if input.left.is_some() {
                                    } else {
                                        // Err("Interpreting error, ".into())
                                    }

                                    Ok(k)
                                }
                                None => Err("Interpreting Error: Variable not initialized".into()),
                            }
                        }
                    }
                    None => {
                        println!("Error var: {:#?}", i);
                        println!("Error SCOPE: {:#?}", self.scope.last());
                        Err("Interpreting Error: Variable Not Declared".into())
                        }
                }
            }
            Token::ADDOP(_) | Token::MULOP(_) => self.add(input),

            Token::UNOP(i) => {
                if let Some(j) = input.clone().left {
                    if let Token::DIGIT(m) = self.interpret_input(*j)? {
                        match i {
                            UnaryOp::PLUS => Ok(Token::DIGIT(m)),
                            UnaryOp::MINUS => Ok(Token::DIGIT(-m)),
                        }
                    } else {
                        Err("L-value cannot be non-digit item".into())
                    }
                } else {
                    Err("Need at least two values to add".into())
                }
            }
            Token::StatementList(list) => {
                self.scope.last_mut().unwrap().push(HashMap::new());
                if list.is_empty() {
                    self.scope.pop();
                    return Ok(Token::Type(Type::NONE));
                }
                for i in list {
                    let mid_result = self.interpret_statement(i)?;
                    
                    if mid_result != Token::Type(Type::NONE) {
                        self.scope.pop();
                        return Ok(mid_result);
                    }
                }
                self.scope.last_mut().unwrap().pop();
                Ok(Token::Type(Type::NONE))
            }
            Token::Type(var_type) => {
                // match *(input.left?)
                if let Token::IDENT(i) = (*(input.left.clone().expect("No L-Value"))).value {
                    if self.var_declared(&i) {
                        Err("Variable already declared!".into())
                    } else {
                        if let Some(j) = input.right {
                            self.declare_var(i, var_type, Some((*j).value))?;
                        } else {
                            self.declare_var(i, var_type, None)?;
                        }
                        Ok(Token::Type(Type::NONE))
                    }
                } else if let Token::FuncData(i, j, k, m) =
                    (*(input.left.expect("No L-Value"))).value
                {
                    self.declare_var(i.clone(), var_type, Some(Token::FuncData(i, j, k, m)))?;
                    Ok(Token::Type(Type::NONE))
                } else {
                    Err("Interpreting Error: Expected identifier".into())
                }
            }

            Token::ASSIGN => {
                if let Some(i) = input.left.clone() {
                    if let Token::IDENT(j) = (*i).value {
                        if let Some(k) = input.right.clone() {
                            let inter_value = self.interpret_input(*k)?;
                            if inter_value != Token::Type(Type::NONE) {
                                Ok(self.update_var(&j, inter_value)?)
                            } else {
                                Err("Unable to resolve r-value".into())
                            }
                        } else {
                            Err("No rvalue to assign.".into())
                        }
                    } else {
                        println!("current tree:{:?}", input.clone());
                        Err("Interpreting error: can't assign value to non-variable".into())
                    }
                } else {
                    println!("current tree:{:?}", input.clone());
                    Err("Interpreting error: Nothing to left of assignment".into())
                }
            }
            Token::RET => {
                if let Some(i) = input.left {
                    Ok(self.interpret_input(*i)?)
                } else {
                    Err("Interpreting error: no argument to return statement".into())
                }
            }
            // Token::Type(F) => {
            //     //Func declaration
            //     Err("Unknown error in function declaration".into())
            // }
            Token::ArgList(_i) => Err("Unknown error in function call".into()),
            Token::IfData(i) => {
                let condition = self.interpret_input(*i)?;
                //Only implementing ifs, not elses. Unless...?
                if condition != Token::DIGIT(0) {
                    if let Token::StatementList(list) = input.left.unwrap().value {
                        for i in list {
                            let mid_result = self.interpret_statement(i)?;
                            if mid_result != Token::Type(Type::NONE) {
                                return Ok(mid_result);
                            }
                        }
                        Ok(Token::Type(Type::NONE))
                    } else {
                        Err("Interpreting error: No body to if statement".into())
                    }
                } else {
                    if let Some(i) = input.right {
                        Ok(self.interpret_input(*i)?)
                    } else {
                        Ok(Token::Type(Type::NONE))
                    }
                }
            }
            Token::COMPARE(i) => {
                match i{
                    Compare::EQ => {
                        if let Some(j) = input.left{
                            if let Some(k) = input.right {
                                if (*j).value == (*k).value{
                                    Ok(Token::BOOL(Bool::True))
                                }else{
                                    Ok(Token::BOOL(Bool::False))
                                }
                            }else{
                                Err(format!("Error, no right value in comparison:"))
                            }
                        }else{
                            Err(format!("Error, no left value in comparison:"))
                        }
                    }
                    Compare::GE => {
                        Err("UNIMPLEMENTED".into())
                    }
                    Compare::GT => {
                        Err("UNIMPLEMENTED".into())
                    }
                    Compare::LE => {
                        Err("UNIMPLEMENTED".into())
                    }
                    Compare::LT => {
                        Err("UNIMPLEMENTED".into())
                    }
                    Compare::NE => {
                        if let Some(j) = input.left{
                            if let Some(k) = input.right {
                                if (*j).value != (*k).value{
                                    Ok(Token::BOOL(Bool::True))
                                }else{
                                    Ok(Token::BOOL(Bool::False))
                                }
                            }else{
                                Err(format!("Error, no right value in comparison: "))
                            }
                        }else{
                            Err(format!("Error, no left value in comparison: "))
                        }
                    }
                }

            }
            _ => {
                println!("Current Err ASTNODE: {:?}", input);
                Err("Interpreting Error: Unknown Token".into())
            }
        }
    }

    fn interpret_block(&mut self) -> Result<Token, String> {
        let curr = self.parser.statement()?;
        self.interpret_input(curr)
    }
    pub fn interpret_program(&mut self) -> Result<Token, String> {
        self.scope.push(Vec::new());
        let curr = self.parser.parse_block()?;
        let res = self.interpret_input(curr);
        self.scope.pop();
        res
    }
}

#[allow(dead_code)]
struct Translator {
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
mod lexer_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn lexer_if() {
        let tok = Lexer::new("if").unwrap();
        assert_eq!(Token::If, tok.current_token);
    }

    #[test]
    fn lexer_else() {
        let tok = Lexer::new("else").unwrap();
        assert_eq!(Token::Else, tok.current_token);
    }

    #[test]
    fn lexer_test_float() {
        let mut tok = Lexer::new("1.2+2.3").unwrap();
        assert_eq!(Token::FLOAT(1.2), tok.current_token);
        tok.get_next_token();
        assert_eq!(Token::ADDOP(AddOp::PLUS), tok.current_token);
        tok.get_next_token();
        assert_eq!(Token::FLOAT(2.3), tok.current_token);
    }

    #[test]
    fn lexer_peek() {
        let lex = Lexer::new("1+2").unwrap();
        assert_eq!(lex.current_token, Token::DIGIT(1));
        assert_eq!(lex.peek(), Some('+'))
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
    fn lexer_test_fn() {
        let tok = Lexer::new("fn").unwrap();
        assert_eq!(Token::Type(Type::FUNC), tok.current_token);
    }
}
#[cfg(test)]
mod parser_tests {
    use super::*;
    #[test]
    fn parser_atom_test_float() {
        let mut pars = Parser::new("1.2+2").unwrap();
        assert_eq!(Ok(ASTreeNode::new(Token::FLOAT(1.2))), pars.atom())
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
    fn parser_atom_test() {
        let mut pars = Parser::new("1+2").unwrap();
        assert_eq!(Ok(ASTreeNode::new(Token::DIGIT(1))), pars.atom())
    }

    #[test]
    fn parser_empty_block() {
        let root = Parser::new("{}");
        assert_eq!(
            Ok(ASTreeNode::new(Token::StatementList(Vec::new()))),
            root.unwrap().parse_block()
        )
    }

    #[test]
    fn parser_block_basic() {
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
    fn parser_block2() {
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
    fn parser_assignment() {
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
    fn parser_block_with_assign() {
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
    fn parser_block_nosemi() {
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
    fn parser_atom_test3() {
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
    fn parser_statement() {
        assert_eq!(
            Parser::new("1+2").unwrap().expr().unwrap(),
            Parser::new("1+2;").unwrap().statement().unwrap()
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
    fn parser_basic_declaration() {
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
    fn parser_declarations() {
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
    fn parser_double_declaration() {
        assert_eq!(
            Interpreter::new("{int a; int a;}")
                .unwrap()
                .interpret_program(),
            Err("Variable already declared!".into())
        )
    }

    #[test]
    fn parser_basic_function() {
        assert_eq!(
            ASTreeNode::new_with_values(
                Token::Type(Type::FUNC),
                Some(Box::new(ASTreeNode::new(Token::FuncData(
                    "func".into(),
                    Type::NONE,
                    Vec::new(),
                    Box::new(ASTreeNode::new(Token::StatementList(Vec::new())))
                )))),
                None
            ),
            Parser::new("fn func(){}").unwrap().statement().unwrap()
        );
    }

    #[test]
    fn basic_function_call() {
        assert_eq! {
            ASTreeNode::new(Token::StatementList(vec![
                ASTreeNode::new_with_values(Token::Type(Type::FUNC),
                    Some(Box::new(ASTreeNode::new(Token::FuncData("returnThree".into(),Type::INT,Vec::new(),Box::new(ASTreeNode::new(Token::StatementList(vec![
                        ASTreeNode::new_with_values(
                            Token::RET,
                            Some(Box::new(ASTreeNode::new(Token::DIGIT(3)))),
                            None,
                        ),

                    ]))))))),
                    None)
                ,
                ASTreeNode::new_with_values(
                    Token::RET,
                    Some(Box::new(ASTreeNode::new_with_values(
                        Token::IDENT("returnThree".into()),
                        Some(Box::new(ASTreeNode::new(Token::ArgList(Vec::new())))),
                        None))),
                    None,
                )
            ])),
            Parser::new("{
                fn returnThree()->int{
                    3
                }
                returnThree()
            }").unwrap().parse_block().unwrap()
        }
    }

    #[test]
    fn parse_if() {
        assert_eq!(
            ASTreeNode::new_with_values(
                Token::StatementList(vec![
                    ASTreeNode::new_with_values(
                        Token::Type(Type::INT),
                        Some(Box::new(ASTreeNode::new(Token::IDENT("a".into())))),
                        None)
                    ,
                    ASTreeNode::new_with_values(
                        Token::IfData(Box::new(ASTreeNode::new(Token::DIGIT(1)))),
                        Some(Box::new(ASTreeNode::new(Token::StatementList(vec![
                            ASTreeNode::new_with_values(
                                Token::ASSIGN,
                                Some(Box::new(ASTreeNode::new(Token::IDENT("a".into())))),
                                Some(Box::new(ASTreeNode::new(Token::DIGIT(3))))
                            )
                        ])))),
                        None
                    ),
                    ASTreeNode::new_with_values(
                        Token::RET, 
                        Some(Box::new(ASTreeNode::new(Token::IDENT("a".into())))), 
                        None)
                    
                    ]),
                None,
                None
            ),
            Parser::new(
                "
            {
                int a;
                if(1){
                    a = 3;
                }
                return a;
            }
            "
            )
            .unwrap()
            .parse_block()
            .unwrap()
        )
    }

    #[test]
    fn parse_else() {
        let a = ASTreeNode::new_with_values(
            Token::StatementList(vec![
                ASTreeNode::new_with_values(
                    Token::Type(Type::INT),
                    Some(Box::new(ASTreeNode::new(Token::IDENT("a".into())))),
                    None)
                ,
                ASTreeNode::new_with_values(
                    Token::IfData(Box::new(ASTreeNode::new(Token::DIGIT(0)))),
                    Some(Box::new(ASTreeNode::new(Token::StatementList(vec![
                        ASTreeNode::new_with_values(
                            Token::ASSIGN,
                            Some(Box::new(ASTreeNode::new(Token::IDENT("a".into())))),
                            Some(Box::new(ASTreeNode::new(Token::DIGIT(3))))
                        )
                    ])))),
                    Some(Box::new(ASTreeNode::new(Token::StatementList(vec![
                        ASTreeNode::new_with_values(
                            Token::ASSIGN,
                            Some(Box::new(ASTreeNode::new(Token::IDENT("a".into())))),
                            Some(Box::new(ASTreeNode::new(Token::DIGIT(5))))
                        )
                    ]))))
                ),
                ASTreeNode::new_with_values(
                    Token::RET, 
                    Some(Box::new(ASTreeNode::new(Token::IDENT("a".into())))), 
                    None)
                
                ]),
            None,
            None
        );
        let b = Parser::new(
            "
        {
            int a;
            if(0){
                a = 3;
            }else{
                a = 5;
            }
            return a;
        }
        "
        )
        .unwrap()
        .parse_block()
        .unwrap();

        println!("LEFT: {:#?} \n RIGHT: {:#?}", a,b );
        assert_eq!(
            a
            ,
            b
        )
    }
}

// INTERPRETER TESTS
#[cfg(test)]
mod interp_test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn interp_basic_add_float() {
        assert_eq!(
            Token::FLOAT(1.4 + 2.3),
            Interpreter::new("1.4+2.3")
                .unwrap()
                .interpret_block()
                .unwrap()
        );
    }

    #[test]
    fn interp_basic_add() {
        assert_eq!(
            Token::DIGIT(3),
            Interpreter::new("1+2").unwrap().interpret_block().unwrap()
        );
    }

    #[test]
    fn interp_unary_minus() {
        assert_eq!(
            Token::DIGIT(3),
            Interpreter::new("--3").unwrap().interpret_block().unwrap()
        );
    }

    #[test]
    fn interp_unary_plus() {
        assert_eq!(
            Token::DIGIT(3),
            Interpreter::new("++3").unwrap().interpret_block().unwrap()
        );
    }

    #[test]
    fn interp_unary_both() {
        assert_eq!(
            Token::DIGIT(3),
            Interpreter::new("++3").unwrap().interpret_block().unwrap()
        );
    }

    #[test]
    fn interp_chain_add() {
        assert_eq!(
            Token::DIGIT(6),
            Interpreter::new("1+2+3")
                .unwrap()
                .interpret_block()
                .unwrap()
        );
    }

    #[test]
    fn interp_precedence_test() {
        assert_eq!(
            Token::DIGIT(7),
            Interpreter::new("1+2*3")
                .unwrap()
                .interpret_block()
                .unwrap()
        );
    }

    #[test]
    fn interp_precedence_test2() {
        assert_eq!(
            Token::DIGIT(5),
            Interpreter::new("1*2+3")
                .unwrap()
                .interpret_block()
                .unwrap()
        );
    }
    #[test]
    fn interp_parentheses_test() {
        assert_eq!(
            Token::DIGIT(9),
            Interpreter::new("(1+2)*3")
                .unwrap()
                .interpret_block()
                .unwrap()
        );
    }
    #[test]
    fn interp_basic_interp_plus() {
        assert_eq!(
            Token::DIGIT(3),
            Interpreter::new("1+2").unwrap().interpret_block().unwrap()
        );
    }

    #[test]
    fn interp_basic_interp_minus() {
        assert_eq!(
            Token::DIGIT(1),
            Interpreter::new("2-1").unwrap().interpret_block().unwrap()
        );
    }

    #[test]
    fn interp_basic_interp_times() {
        assert_eq!(
            Token::DIGIT(6),
            Interpreter::new("2*3").unwrap().interpret_block().unwrap()
        );
    }

    #[test]
    fn interp_basic_interp_divide() {
        assert_eq!(
            Token::DIGIT(0),
            Interpreter::new("2/3").unwrap().interpret_block().unwrap()
        );
    }

    #[test]
    fn interp_basic_interp_modulo() {
        assert_eq!(
            Token::DIGIT(2),
            Interpreter::new("2%3").unwrap().interpret_block().unwrap()
        );
    }
    #[test]
    fn interp_test_vars() {
        assert_eq!(
            Token::DIGIT(3),
            Interpreter::new("{int a = 3; a}")
                .unwrap()
                .interpret_program()
                .unwrap()
        )
    }

    #[test]
    fn interp_rpn_translate() {
        assert_eq!(
            "1 2 +",
            Translator::new("1+2").unwrap().rpn_translate().unwrap()
        )
    }

    #[test]
    fn interp_empty_block() {
        assert_eq!(
            Token::Type(Type::NONE),
            Interpreter::new("{}").unwrap().interpret_program().unwrap()
        )
    }

    #[test]
    fn interp_different_return_varibale() {
        assert_eq!(
            Token::DIGIT(6),
            Interpreter::new(
                "
            {
                int b = 3; 
                b+3
            }
                "
            )
            .unwrap()
            .interpret_program()
            .unwrap()
        )
    }

    #[test]
    fn interp_variable_test() {
        assert_eq!(
            Token::DIGIT(8),
            Interpreter::new(
                "
            {
                int b = 3; 
                b = 5;
                b+3
            }
                "
            )
            .unwrap()
            .interpret_program()
            .unwrap()
        )
    }
    #[test]
    fn interp_final_variable_test2() {
        assert_eq!(
            Token::DIGIT(14),
            Interpreter::new(
                "
            {
                int b = 3; 
                int a;
                a = b+3;
                b=5;
                b+3+a
            }
                "
            )
            .unwrap()
            .interpret_program()
            .unwrap()
        )
    }
    #[test]
    fn interp_scope_test1() {
        assert_eq!(
            Token::DIGIT(3),
            Interpreter::new(
                "
            {
                int b = 3; 
                {
                    //this is in a different scope
                    int b = 2
                }
                b
            }
                "
            )
            .unwrap()
            .interpret_program()
            .unwrap()
        )
    }

    #[test]
    fn interp_basic_function_dec() {
        assert_eq! {
            Token::Type(Type::NONE),
            Interpreter::new("
            {
                fn returnThree()->int{
                    3
                }
            }").unwrap().interpret_program().unwrap()
        }
    }
    #[test]
    fn interp_basic_function_call() {
        assert_eq! {
            Token::DIGIT(3),
            Interpreter::new("
            {
                fn returnThree()->int{
                    3
                }
                returnThree()
            }").unwrap().interpret_program().unwrap()
        }
    }

    #[test]
    fn interp_function_vars() {
        assert_eq!(
            Token::DIGIT(8),
            Interpreter::new(
                "
            {
                fn returnThree()->int{
                    int b = 3; 
                    b = 5;
                    b+3
                }
                returnThree()
            }"
            )
            .unwrap()
            .interpret_program()
            .unwrap()
        )
    }

    #[test]
    fn interp_function_args() {
        let b = Interpreter::new(
            "
        {
            fn returnArg(int a)->int{
               a
            }
            returnArg(3)
        }",
        )
        .unwrap()
        .interpret_program()
        .unwrap();
        assert_eq!(Token::DIGIT(3), b)
    }

    #[test]
    #[should_panic]
    fn interp_function_args_type_error() {
        Interpreter::new(
            "
        {
            fn returnArg(int a)->int{
               a
            }
            returnArg(3.5)
        }",
        )
        .unwrap()
        .interpret_program()
        .unwrap();
    }

    #[test]
    fn interp_recursion() {
        let b = Interpreter::new(
            "
        {
            fn returnArg(int a)->int{
               if(a == 1){
                   return 1
               }else{
                   return returnArg(a-1)
               }
            }
            returnArg(3)
        }",
        )
        .unwrap()
        .interpret_program()
        .unwrap();
        assert_eq!(Token::DIGIT(1), b)
    }

    #[test]
    fn interp_if() {
        assert_eq! {
            Token::DIGIT(5),
            Interpreter::new(
                "
            {
                int a = 6;
                if(1){
                    return 5;
                }
                return 3;
            }",
            )
            .unwrap()
            .interpret_program()
            .unwrap()
        }
    }
    #[test]
    fn interp_else() {
        assert_eq! {
            Token::DIGIT(3),
            Interpreter::new(
                "
            {
                int a = 6;
                if(0){
                    return 5;
                }else{
                    return 3;
                }
            }",
            )
            .unwrap()
            .interpret_program()
            .unwrap()
        }
    }
}
