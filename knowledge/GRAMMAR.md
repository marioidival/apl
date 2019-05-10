
# EBNF para APL

    =   - definição
    ,   - concatenação
    ;   - finalização de uma definição
    |   - alternação, "ou"
    [ ] - elemento opcional
    { } - pode ser omitido ou repetido
    ( ) - agrupamento
    " " - terminal string, pode ser usado para esperar algo: "(" number ")"
    ?   - pode aparecer 0 ou 1, não mais.
    +   - deve aparecer pelo menos 1 vez.
    *   - pode repetir 0 ou mais vezes. 
    
    
```

suite: simple_stmt | NEWLINE stmt+

retorne_stmt = "retorne" [testlist] ;
passe_stmt = "passe";
interrompa_stmt = "interrompa";
continue_stmt = "continue";
provoque_stmt = "provoque" [cond ["de" cond]]
verifique_stmt = "verifique" cond ["," cond]
remova_stmt = "remova" 
exceto_clause = "exceto" [test ["como" NAME]]
tente_stmt = "try" ":" suite,
             (
                (exceto_clause ":" suite)+
                ["senao" ":" suite]
                ["finalmente" ":" suite]
                | "finalmente" ":" suite
             )

fluxo_stmt = interrompa_stmt | continue_stmt | passe_stmt | retorne_stmt

se_stmt = "se" cond ":" suite ("ouentaose" suite)* ["senao" ":" suite]
paracada_stmt =
enquanto_stmt = "enquanto" cond ":" suite ["senao" ":" suite]

stmt = simple_stmt | compound_stmt
simple_stmt = small_stmt (";" small_stmt)* [";"] NEW_LINE
small_stmt = remova_stmt | passe_stmt | fluxo_stmt | verifique_stmt

compound_stmt: se_stmt | enquanto_stmt | paracada_stmt | tente_stmt | fun_def | classe_def

testlist = cond ("," cond)* [","]

cond = ou_test ["se" ou_test "senao" cond]
ou_test = e_test ("e" nao_test)*
nao_test = "not" nao_test | comparacao
comparacao = expr (comp_op expr)*
comp_op = <'|'>'|'=='|'>='|'<='|'<>'|'!='|'em'|'nao' 'em'|'é'|'é' 'nao'
aritmetico_expr = term (("+" | "-") term)*
term = factor (("*" | "/")) factor)*
factor = 

fun_def = "fun" NAME parametros ":" suite
classe_def = "classe" NAME [ parametros ] ":" suite

parametros = "(" [argumentos] ")"
argumentos = (NAME ["=" VALOR]+ )

literal → inteiro 
    | real
    | Verdadeiro
    | Falso
    | Vazio ;
```

