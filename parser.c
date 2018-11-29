#include <setjmp.h>
#include <math.h>
#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>  

#define NUM_FUNC        100
#define NUM_GLOBAL_VARS 100
#define NUM_LOCAL_VARS  200
#define ID_LEN          31
#define FUNC_CALLS      31
#define PROG_SIZE       10000
#define FOR_NEST        31



extern char *prog; //current location in source
extern char *p_buff; // start of program buffer
extern jmp_buf e_buf; // Hold environment for longjump()

char token[80]; //string of token
char token_type; 
char tok; // holds internal representation of token if keyword

enum token_types
{
    DELIMITER, IDENTIFIER, NUMBER, KEYWORD, TEMP, STRING, BLOCK
};

enum double_ops
{
    LT = 1, LE, GT, GE, EQ, NE
};

enum tokens
{
    ARG, CHAR, INT, IF, ELSE, FOR, DO, WHILE, SWITCH, RETURN, EOL, FINISHED, END
};

// Constants to call syntax_error(). SYNTAX is default.
enum error_message{
    SYNTAX, UNBAL_PARENS, NO_EXP, EQUALS_EXPECTED, NOT_VAR, 
    PARAM_ERR, SEMI_EXPECTED, UNBAL_BRACES, FUNC_UNDEFINED,
    TYPE_EXPECTED, NEST_FUNC, RET_NOCALL, PAREN_EXPECTED,
    WHILE_EXPECTED, QUOTE_EXPECTED, NOT_TEMP, TOO_MANY_LVARS
};

extern struct var_type{
    char var_name[32];
    // enum variable_type var_type; //CHECK FOR ERROR
    int value;
} global_vars[NUM_GLOBAL_VARS];


extern struct func_type{
    char func_name[32]; //Try to remove magic numbers
    char *loc; //location of function entry point in file

}func_stack[NUM_FUNC];

extern struct commands{
    char command[20];
    char tok;
}table[];

/**
 * standard library function declaration, requires interanal function table
 */


int call_getchar(void), call_putchar(void);
int call_puts(void), print(void), getnum(void);

struct intern_func_type
{
    char *f_name; //function name
    int (* p)(); // pointer to function

} intern_func[] = {
    "getche", call_getchar,
    "putch", call_putchar,
    "puts", call_puts,
    "print", print,
    "getnum", getnum,
    "", 0 // null terminate list
}; //THIS EXPLICITLY DECLARES STRUCT ARRAY!

extern char token[80]; //string of token
extern char token_type; // type of token
extern char tok; // internal representation of token

extern int ret_value;

void eval_exp(int *value);
void eval_exp0(int *value);
void eval_exp1(int *value);
void eval_exp2(int *value);
void eval_exp3(int *value);
void eval_exp4(int *value);
void eval_exp5(int *value);
void atom(int *value);
void syntx_error(int error), putback(void);
void assign_var(char *var_name, int value);
int isdelim(char c), look_up(char *s), iswhite(char c);
int find_var(char *s ), get_token(void);
int internal_func(char *s);
int is_var(char *s);
char *find_func(char *name);
void call(void);

// Entry point for parser

void eval_exp(int *value){
    get_token();
    if (!*token)
    {
        syntx_error(NO_EXP);
        return;
    }
    if(*token == ';'){
        *value = 0; //Empty expression
        return;
    }
    eval_exp0(value);
    putback();
}

void eval_exp0(int *value){
    char temp[ID_LEN]; // hold name of var.

    register int temp_tok;

    if(token_type == IDENTIFIER){
        if(is_var(token)){
            strcpy(temp, token);
            temp_tok = token_type;
            get_token();
            if(*token=='='){ //Is assignment
                get_token();
                eval_exp0(value); // get value to assign.
                assign_var(temp, *value);//Assign value
                return;
            }
            else{
                putback();
                strcpy(token, temp);
                token_type = temp_tok;
            }
        }
    }
    eval_exp1(value);
}


void eval_exp1(int *value){
    char relops[7] = {LT, LE, GT, GE, EQ, NE, 0};
    int partial_value;
    register char op;

    eval_exp2(value);
    op = *token;

    if(strchr(relops,op)){ //find op in relops
        get_token();
        eval_exp2(&partial_value);
        switch(op){
            case LT:
                *value = *value < partial_value;
                break;
            case LE:
                *value = *value <= partial_value;
                break;
            case GT:
                *value = *value > partial_value;
                break;
            case GE:
                *value = *value >= partial_value;
                break;
            case EQ:
                *value = *value == partial_value;
                break;
            case NE:
                *value = *value != partial_value;
                break;
        }
    }
}

//Add or subtract values

void eval_exp2(int * value){
    register char op;
    int partial_value;

    eval_exp3(value);
    
    while((op = *token)=='+'|| op == '-'){
        get_token();
        eval_exp3(&partial_value);
        switch(op){
            case '-':
                *value -= partial_value;
                break;
            case '+':
                *value += partial_value;
                break;
        }
    }
}

//multiply or divide two terms

void eval_exp3(int *value){
    register char op;
    int partial_value;

    eval_exp4(value);
    
    while((op = *token) == '*' || op == '/' || op == '%'){
        get_token();
        eval_exp4(&partial_value);
        switch(op){
            case '*':
                *value *= partial_value;
                break;
            case '/':
                *value /= partial_value;
                break;
            case '%':
                *value %= partial_value;
                break;
        }
    }
}

// IS unary + or -;

void eval_exp4(int *value){
    register char op;

    op = '\0';
    if(*token == '+' || *token == '-'){
        op = *token;
        get_token();
    }
    eval_exp5(value);
    if(op)
        if(op == '-')
            *value = -(*value);
}

// process parenthisez expression

void eval_exp5(int *value){
    if (*token == '(')
    {
        get_token();
        eval_exp0(value);
        if (*token != ')')
            get_token();
    }
    else atom(value);
}

void atom(int *value){
    int i;

    
    switch (token_type){
        case IDENTIFIER:
            i = internal_func(token);
            if (i!= -1) // call standard functions
            {
                *value = (*intern_func[i].p)();
            }
            else if (find_func(token)){
                call();
                *value = ret_value;
            } else *value = find_var(token);
            get_token();
            return;
        case NUMBER: // is a numeric constant
            *value = atoi(token);
            get_token();
            return;
        case DELIMITER:
            if (*token == '\''){
                *value = *prog;
                prog++;
                if(*prog != '\'') syntx_error(QUOTE_EXPECTED);
                prog++;
                get_token();
            }
            return;
        default:
            if (*token == ')') return; // process empty expression
            else syntx_error(SYNTAX); //No idea, lmao
    }
}

void syntx_error(int error){
    char *p, *temp;
    int linecount = 0;
    register int i;

    static char *e[] = {
        "Syntax error",
        "Unbalanced parentheses",
        "No expression present",
        "Equals sign expected",
        "Not a variable",
        "Parameter error",
        "Semicolon (;) expected",
        "Unbalanced Braces",
        "Function undefined",
        "Type specifier expected",
        "Too many nested function calls",
        "Return without call",
        "Parentheses expected",
        "While expected",
        "closing quote epected",
        "not a string",
        "too many local variables"
    };
    printf("%s",e[error]);
    p = p_buff;
    
    while(p != prog){
        p++;
        if(*p == '\r'){
            linecount++;
        }
    }
    printf(" in line %d\n", linecount);

    temp = p;
    for(i = 0;i < 10 && p>p_buff && *p != '\n';i++,p--);
    for(i = 0;i < 30 && p<=temp;i++,p++)
        printf("%c",*p);
    longjmp(e_buf,1); // Return to sve point
}

int get_token(void){
    char *temp;
    token_type = 0;
    tok = 0;

    temp = token;
    *temp = '\0';
    
    while(iswhite(*prog) && *prog) prog++;
    
    if(*prog == '\r'){
        ++prog;
        ++prog;
        while(iswhite(*prog) && *prog) prog++;
    }
    if(*prog == '\0'){
        *token = '\0';
        tok = FINISHED;
        while(iswhite(*prog) && *prog) prog++;
    }
    
    if (strchr("{}",*prog)) { // Block delimiter
        *temp = *prog;
        temp++;
        *temp = '\0';
        prog++;
        return (token_type = BLOCK);
    }

    // look for comments

    
    if (*prog == '/') {
        if (*(prog+1) == '*') { //THIS IS A COMMENT
            prog += 2;
            do{//Find end of comment
                while(*prog != '*'){
                    prog +=2;
                }
            } while(*prog!= '/');
            prog++;
        }
    }
    if (strchr("!<>=",*prog)) { //Possibly relation operator
        
        switch (*prog)
        {
            case '=':
                if(*(prog+1) == '='){
                    prog += 2;
                    *temp = EQ; //Equals
                    temp ++; 
                    *temp = EQ;
                    temp++;
                    *temp = '\0';
                }
                break;
            case '!':
                if (*(prog + 1) == '=')
                {
                    prog += 2;
                    *temp = NE; //Equals
                    temp++;
                    *temp = NE;
                    temp++;
                    *temp = '\0';
                }
                break;
            case '<':
                if (*(prog + 1) == '=')
                {
                    prog += 2;
                    *temp = LE; //Equals
                    temp++;
                    *temp = LE;
                }
                else{
                    prog++;
                    *temp = LT;
                }
                temp++;
                *temp = '\0';
                break;
            case '>':
                if (*(prog + 1) == '=')
                {
                    prog += 2;
                    *temp = GE; //Equals
                    temp++;
                    *temp = GE;
                }
                else
                {
                    prog++;
                    *temp = GT;
                }
                temp++;
                *temp = '\0';
                break;
            default:
                break;
        }
        if(*token) return (token_type= DELIMITER);
    }

    if (strchr("+-*^/%", *prog)){
        *temp = *prog;
        prog ++;
        temp++;
        *temp = '\0';
        return(token_type=DELIMITER);
    }
    
    if (*prog == '"') {
        prog++;
        while(*prog!='"' && *prog!='\r') *temp ++ = *prog++;
        if(*prog == '\r') syntx_error(SYNTAX);//NO CLUE why this would happen
        prog++;
        *temp='\0';
        return(token_type = STRING);
    }

    if (isdigit(*prog)){

        while (isdelim(*prog))
            *temp++ = *prog++;
        *temp = '\0';
        return(token_type = NUMBER);
    }
    
    if (isalpha(*prog)){
        while (isdelim(*prog))
            *temp++ = *prog++;
        token_type=TEMP;
    }

    *temp = '\0';

    
    if (token_type == TEMP) {
        tok = look_up(token);//convert to internal representation
        if(tok) token_type = KEYWORD;
        else token_type = IDENTIFIER;
    }
    return token_type;
}

void putback(void){
    char *t ;

    t = token;
    for(;*t;t++) prog--;
}

int look_up(char *s){
    register int i;
    char *p;
    p = s;
    while(*p){*p= tolower(*p); p++;}

    for(i=0; *table[i].command;i++){
        if(!strcmp(table[i].command,s))return table[i].tok;
    }
    return 0; // Unknown command
}

int internal_func(char *s){
    int i;

    for(i = 0; intern_func[i].f_name[0];i++)
    {
        if(!strcmp(intern_func[i].f_name,s)) return 1;
    }

    return -1;
}

int isdelim(char c){
    if(strchr(" !;,+-<>",c) || c==9 || c == '\r' || c==0) return 1;
    return 0;
}

int iswhite(char c){
    if(c == ' ' || c == '\t')return 1;
    else return 0;
}