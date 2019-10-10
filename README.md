# Cyclone
Writing and Improvising a C-Clone interpreter 

## The language tokens being considered are:

TOKEN | EXAMPLE
------|--------
LBRACE | ' { '
RBRACE | ' } '
LPAREN | ' ( '
RPAREN | ' ) '
ASSIGN | ' = '
COMMA  | ' , '
DIGIT  | 123
FLOAT  | 3.14
ADDOP  | ' + ' , ' - '
MULOP  | ' * ' , ' / ', ' % '
LT     | ' < '
GT     | ' > '
EQ     | ' =='
LE     | ' <='
GE     | ' >= '
TYPE   | ' int '
IDENT  | ' i ' , ' tree '



## The production rules to evaluating code:

#### Note, the terms in capitals imply the token should be consumed when parsing

name | definition
---|---
program  | MAIN block
block  | LBRACE statement_list RBRACE
statement_list  | [statement *(SEMI statement) [SEMI]]
statement  | (expr SEMI \| declaration \| block) 
expr  | addop *(ASSIGN expr)
addop  | term *((PLUS/MINUS) expr)
mulop  | atom ((MUL/DIV) expr)
atom  | (PLUS/MINUS) atom \|  INTEGER \|   LPAREN expr RPAREN \| IDENTIFIER
declaration  | type IDENTIFIER [ASSIGN expr] SEMI
type  | INT,FLOAT //TODO: IMPLEMENT FLOAT

Pay attention to the definition of statement list, here I'm saying that it is possible to have an empty statement list, as well as that the last statement doesn't necesarily need a closing SEMI. This is because I'm using rust syntax, in which if the last statement doesn't use a closing semi, it is considered a return value.

## Proposed Grammar for future versions:
name | definition
---|---
function | TYPE IDENTIFIER argument_list block
argument_list | LPAREN argument  *(COMMA argument)  RPAREN
argument | TYPE IDENT    
