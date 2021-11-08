# Parser grammer rules

### Syntax for grammer lang

```
|           -> or
*           -> what comes before can be repeated 0 or more times
+           -> what comes before can be repeated 1 or more times
```

### Actual Grammer

```
code_block  ->  scope
scope       ->  "{" (scope | stmt | expr)* "}"
stmt        ->  decl | return

decl        ->  STRING "=" expr ";"
return      ->  "return" expr ";"

expr        ->  term (("+" | "-") term)* ";"
term        ->  factor (("*" | "/") factor)*
factor      ->  NUMBER
            |   ("+" | "-") factor
            |   "(" expr ")"
```
