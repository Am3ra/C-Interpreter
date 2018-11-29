#include <stdio.h>
#include <setjmp.h>
#include <math.h>
#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define NUM_FUNC 100
#define NUM_GLOBAL_VARS 100
#define NUM_LOCAL_VARS 200
#define NUMB_BLOCK 100
#define NUM_PARAMS 31
#define ID_LEN 31
#define FUNC_CALLS 31
#define PROG_SIZE 10000
#define LOOP_NEST 31

enum tok_types
{
    DELIMITER,
    IDENTIFIER,
    NUMBER,
    KEYWORD,
    TEMP,
    STRING,
    BLOCK
};

enum tokens
{
    ARG,
    CHAR,
    INT,
    IF,
    ELSE,
    FOR,
    DO,
    WHILE,
    SWITCH,
    RETURN,
    EOL,
    FINISHED,
    END
};

enum double_ops
{
    LT = 1,
    LE,
    GT,
    GE,
    EQ,
    NE
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

char *prog;    //current location in source
char *p_buff;  // start of program buffer
jmp_buf e_buf; // Hold environment for longjump()

struct var_type
{
    char var_name[32];
    int var_type;
    int value;
} global_vars[NUM_GLOBAL_VARS];

struct var_type local_var_stack[NUM_GLOBAL_VARS];

struct func_type
{
    char func_name[32]; //Try to remove magic numbers
    char *loc;          //location of function entry point in file

} func_table[NUM_FUNC];

int call_stack[NUM_FUNC];

struct commands
{
    char command[20];
    char tok;
} table[] = {
    "if", IF,
    "else", ELSE,
    "for", FOR,
    "do", DO,
    "while", WHILE,
    "char", CHAR,
    "int", INT,
    "return", RETURN,
    "end", END,
    "", END};

char token[80];
char token_type, tok;

int functos;    //index to top of function call stack
int func_index; // index into function table
int gvar_index; // Index into global variable table
int lvartos;    //index into local variable stack

int ret_value; // fucntion return value;

void print(void), prescan(void);
void dec_global(void), call(void), putback(void);
void dec_local(void), local_push(struct var_type i);
void eval_exp(int *value), syntx_error(int error);
void exec_if(void), find_eob(void), exec_for(void);
void get_params(void), get_args(void);
void exec_while(void), func_push(int i), exec_do(void);
void assign_var(char *var_name, int value);
int load_program(char *p, char *fname), find_var(char *s);
void interp_block(void), func_ret(void);
int func_pop(void), is_var(char *s), get_token(void);
char *find_func(char *name);



int main(int argc, char *argv[])
{
    if (argc != 2)
    {
        printf("Usage: littlec <FILENAME>\n");
        exit(EXIT_FAILURE);
    }

    //allocate program memory
    printf("allocate program memory\n");

    if ((p_buff = (char *)malloc(PROG_SIZE)) == NULL)
    {
        printf("ALLOCATION FAILURE\n");
        exit(EXIT_FAILURE);
    }

    // load program

    printf("loadingprogram\n");
    if (!load_program(p_buff, argv[1]))
        exit(EXIT_FAILURE);

    //INITIALIZA long jump buffer
    printf("INIRIALIZING jmp buffer\n");
    if (setjmp(e_buf))
        exit(EXIT_FAILURE);

    //initialize prog pointer
    prog = p_buff;
    printf("Initializing pre-scan\n");
    prescan(); // find function locations
    printf("Finished prescan\n");
    gvar_index = 0;
    lvartos = 0;
    functos = 0;

    printf("LOOKING FOR MAIN\n");
    prog = find_func("main"); //Program starting point
    prog--;                   // goto opening (
    strcpy(token, "main");
    call(); //call main to begin interpretation
    return 0;
}

void interp_block(void)
{
    int value;
    char block = 0;

    do
    {
        token_type = get_token();
        // if single statement, return on first semicolon

        if (token_type == IDENTIFIER)
        {
            putback();
            eval_exp(&value);

            if (*token != ';')
                syntx_error(SEMI_EXPECTED);
        }

        else if (token_type == BLOCK)
        {

            if (*token == '{')
            {
                block = true;
            }
            else
                return; // end of block }
        }
        else

            switch (tok)
            {
            case CHAR:
            case INT:
                putback();
                dec_local();
                break;
            case RETURN:
                func_ret();
                return;
            case IF:
                exec_if();
                break;
            case ELSE:
                find_eob();
                break;
            case WHILE:
                exec_while();
                break;
            case DO:
                exec_do();
                break;
            case FOR:
                exec_for();
                break;
            case END:
                exit(EXIT_SUCCESS);
            }

    } while (tok != FINISHED && block);
}

int load_program(char *p, char *fname)
{
    FILE *fp;
    int i = 0;

    if ((fp = fopen(fname, "rb")) == NULL)
        return false;
    i = 0;

    do
    {
        *p = getc(fp);
        p++;
        i++;
    } while (!feof(fp) && i < PROG_SIZE);

    *(p - 2) = '\0'; // Null terminate program
    fclose(fp);
    return true;
}

//Find Functions and store Globals

void prescan(void)
{
    char *p;
    char temp[32];
    int brace = 0; // if 0, this means that source is outside of any function

    p = prog;
    func_index = 0;

    do
    {
        while (brace)
        { //Bypass functions
            get_token();
            if (*token == '{')
                brace++;
            if (*token == '}')
                brace--;
        }

        get_token();

        if (tok == CHAR || tok == INT)
        { //Global var
            putback();
            dec_global();
        }
        else if (token_type == IDENTIFIER)
        {
            strcpy(temp, token);
            get_token();
            if (*token == '(')
            { // assume its a function
                func_table[func_index].loc = prog;
                strcpy(func_table[func_index].func_name, temp);
                func_index++;
                while (*prog != ')')
                    prog++;
                prog++; // Prog now points to opening curly brace of function. Supposedly
            }
            else
                putback();
        }
        else if (*token == '{')
            brace++;
    } while (tok != FINISHED);
    prog = p;
}

char *find_func(char *name){
    int i;

    for(i=0;i<func_index;i++)
        if(!strcmp(name,func_table[i].func_name))
            return func_table[i].loc;
    return NULL;
}

void dec_global(void)
{
    get_token();

    global_vars[gvar_index].var_type = tok;
    global_vars[gvar_index].value = 0; /// INITIALIZE TO 0, we're the good guys

    do
    {                // Process comma-seperated list
        get_token(); // get name
        strcpy(global_vars[gvar_index].var_name, token);
        get_token();
        gvar_index++;
    } while (*token == ',');

    if (*token != ';')
        syntx_error(SEMI_EXPECTED);
}

void dec_local(void)
{
    struct var_type i;

    get_token();

    i.var_type = tok;
    i.value = 0; // init to 0

    do
    {                // Process comma-seperated list
        get_token(); // get name
        strcpy(i.var_name, token);
        local_push(i);
        get_token();
    } while (*token == ',');

    if (*token != ';')
        syntx_error(SEMI_EXPECTED);
}

//Call Function
void call(void)
{
    char *loc, *temp;
    int lvartemp;

    loc = find_func(token);

    if (loc == NULL)
        syntx_error(FUNC_UNDEFINED);
    else
    {
        lvartemp = lvartos; // Save local var stack index

        get_args();          // get function args
        temp = prog;         // save return location
        func_push(lvartemp); // save local var stack index
        prog = loc;          // reset prog to start of function
        get_params();        //load params with values of arguments

        interp_block();       //Interpret function
        prog = temp;          // reset program pointer
        lvartos = func_pop(); // reset var stacl
    }
}

void get_args(void)
{
    int value, count, temp[NUM_PARAMS];
    struct var_type i;

    count = 0;
    get_token();
    if (*token != '(')
        syntx_error(PAREN_EXPECTED);
    //Process comma seperated list.

    do
    { // Process comma-seperated list
        eval_exp(&value);
        temp[count] = value;
        get_token();
        count++;
    } while (*token == ',');

    count--;

    for (; count >= 0; count--)
    { // push to local var stack backwards
        i.value = temp[count];
        i.var_type = ARG;
        local_push(i);
    }
}

void get_params(void)
{
    struct var_type *p;
    int i;

    i = lvartos - 1;

    do
    {
        get_token();
        p = &local_var_stack[i];
        if (*token != ')')
        {
            if (tok != INT && tok != CHAR)
                syntx_error(TYPE_EXPECTED);
            p->var_type = token_type;
            get_token();

            strcpy(p->var_name, token);
            get_token();
            i--;
        }
        else
            break;
    } while (*token == ',');
    if (*token != ')')
        syntx_error(PAREN_EXPECTED);
}

void func_ret(void)
{
    int value = 0;
    eval_exp(&value);
    ret_value = value;
}

void local_push(struct var_type i)
{
    if (lvartos > NUM_LOCAL_VARS)
        syntx_error(TOO_MANY_LVARS);

    local_var_stack[lvartos] = i;
    lvartos++;
}

int func_pop(void)
{
    functos--;
    if (functos < 0)
        syntx_error(RET_NOCALL);
    return (call_stack[functos]);
}

void func_push(int i)
{
    if (functos > NUM_FUNC)
        syntx_error(NEST_FUNC);
    call_stack[functos] = i;
    functos++;
}

void assign_var(char *var_name, int value)
{
    int i;

    for (i = lvartos - 1; i >= call_stack[functos - 1]; i--)
        if (!strcmp(local_var_stack[i].var_name, var_name))
        {
            global_vars[i].value = value;
            return;
        }
    if (i < call_stack[functos - 1])
        for (i = 0; i < NUM_GLOBAL_VARS; i++)
            if (!strcmp(global_vars[i].var_name, var_name))
            {
                global_vars[i].value = value;
                return;
            }
    syntx_error(NOT_VAR); //Var not found;
}

int find_var(char *s){
    int i;
    for (i = lvartos - 1; i >= call_stack[functos - 1]; i--)
        if (!strcmp(local_var_stack[i].var_name, token))
            return true;

    //Check globals
    for (i = 0; i < NUM_GLOBAL_VARS; i++)
        if (!strcmp(global_vars[i].var_name, s))
            return true;
    
    syntx_error(NOT_VAR);
    return 0;
}

int is_var(char *s)
{
    int i;

    //check if local var
    for (i = lvartos - 1; i >= call_stack[functos - 1]; i--)
        if (!strcmp(local_var_stack[i].var_name, token))
            return true;

    //Check globals
    for (i = 0; i < NUM_GLOBAL_VARS; i++)
        if (!strcmp(global_vars[i].var_name, s))
            return true;
    return 0;
}

void exec_if(void)
{
    int cond;

    eval_exp(&cond); // Get left expression;

    if (cond)
        interp_block();
    else{
        find_eob();
        get_token();

        if(tok!=ELSE){
            putback();
            return;
        }
        interp_block();
    }
}

void exec_while(void){
    int cond;
    char *temp;

    putback();
    temp=prog; // Save location of top of loop;
    get_token();

    eval_exp(&cond);
    if(cond)
        interp_block();
    else{
        find_eob();
        return;
    }

    prog= temp;//Loop to top of function
}

void exec_do(void){
    int cond;
    char *temp;

    putback();
    temp = prog; //Save position

    get_token();
    interp_block();
    get_token();
    if(tok!=WHILE) syntx_error(WHILE_EXPECTED);

    eval_exp(&cond);
    if(cond) 
        prog = temp;
}

void find_eob(void){
    int brace = 1;
    get_token();

    do
    {
        get_token();
        if(*token == '{')
            brace++;
        else if(*token=='}') brace --;
    } while(brace);
}


void exec_for(void){
    int cond; 
    char *temp, *temp2;
    int brace =1;

    get_token();
    eval_exp(&cond); //Initialization expression, can be empty
    if(*token != ';')
        syntx_error(SEMI_EXPECTED);
    prog++; //Get past ';'
    temp = prog;
    for(;;){
        eval_exp(&cond);
        if (*token != ';')
            syntx_error(SEMI_EXPECTED);
        prog++; //get past second ';'
        temp2=prog; //Get closing expression

        while(brace){
            get_token();
            if (*token == '(') brace++;
            if (*token == ')') brace--;

        }
        if(cond)interp_block();
        else {
            find_eob();
            return;
        }
        prog=temp2;
        eval_exp(&cond);
        prog = temp; //loop to top;
    }
}