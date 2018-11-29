#include <stdio.h>
#include <stdlib.h>

extern char *prog;//Points to program location
extern char token[80];
extern char token_type;
extern char tok;

enum tok_types
{
    DELIMITER,
    IDENTIFIER,
    NUMBER,
    COMMAND,
    STRING,
    QUOTE,
    VARIABLE,
    BLOCK,
    FUNCTION
};

enum error_message
{
    SYNTAX,
    UNBAL_PARENS,
    NO_EXP,
    EQUALS_EXPECTED,
    NOT_VAR,
    PARAM_ERR,
    SEMI_EXPECTED,
    UNBAL_BRACES,
    FUNC_UNDEFINED,
    TYPE_EXPECTED,
    NEST_FUNC,
    RET_NOCALL,
    PAREN_EXPECTED,
    WHILE_EXPECTED,
    QUOTE_EXPECTED,
    NOT_TEMP,
    TOO_MANY_LVARS
};

int get_token(void);
void syntx_error(int error), eval_exp(int *result);
void putback(void);

int call_getchar(){
    char ch = getchar();
    while(*prog!= ')') prog++;
    prog++;//Advance toend of line
    return ch;
}

int call_putchar(){
    int value;
    eval_exp(&value);
    printf("%c",value);
    return value;
}

int call_puts(void){
    get_token();
    if(*token!='(') syntx_error(PAREN_EXPECTED);
    get_token();
    if(token_type!=QUOTE) syntx_error(QUOTE_EXPECTED);
    puts(token);
    get_token();
    if(*token!=')') syntx_error(PAREN_EXPECTED);

    get_token();
    if(*token!=';') syntx_error(SEMI_EXPECTED);
    putback();
    return 0;
}

int print(void){
    int i;
    get_token();
    if(*token != '(') syntx_error(PAREN_EXPECTED);
    get_token();
    if(token_type == QUOTE)
        printf("%s",token);
    else{
        putback();
        eval_exp(&i);
        printf("%d",i);
    }

    get_token();
    if(*token!=')') syntx_error(PAREN_EXPECTED);
    get_token();
    if(*token!=';') syntx_error(SEMI_EXPECTED);
    putback();
    return 0;
}

int getnum(void){
    char s[80];
    gets(s);
    
    while(*prog!=')')prog++;
    prog++;//Go to end of line
    return atoi(s);
}