# `bool-eval`

Tiny school project.

### Syntax

```bnf
<Program> ::= <Control> <Expr>

<Control> ::= <Number> ( <Bit> )*

<Ident> ::= ([a-z] | [A-Z])+
<Number> ::= [0-9]+
<Bit> ::= "0" | "1"

<Expr> ::= <VarExpr> | <AppExpr>

<VarExpr> ::= <Ident>
<AppExpr> ::= <Ident> "(" <AppArgs>? ")"
<AppArgs> ::= <Expr> ( "," <Expr> )* ","?
```

### Examples:

```
> 2 1 0 and(A, B)
< false

> 2 1 0 or(A, B)
< true

> 2 1 0 and(not(B), A)
< true
```
