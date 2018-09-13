# C-Interpreter
Writing and improving a C interpreter from the book "Born to Code in C"

## The language tokens being considered are:

TOKEN | EXAMPLE
------|--------
 Delimiters | Punctuation and operators (+,=,;)
 keywords | If, while, etc.
 strings | "ALAN"
 identifiers | the variable x, function names, etc. User-types not planned
 numbers | 1
 blocks | {}

## The production rules to evaluating expression:

name | definition
---|---
expression | [assignment][rvalue]
assignment | lvalue = rvalue
lvalue     | variable
rvalue     | part [rel-op part]
part       | term [+term][-term]
term       | factor[\*factor][/factor][%factor]
factor     | part[rel-op part]
part       | [+ or -] atom
atom       | variable, constannt, function, or expression
