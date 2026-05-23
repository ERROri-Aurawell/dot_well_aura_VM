# AURA - Bytecode da linguagem Well. Versão humana (legível).
DECLARAÇÃO DE LABELS:
"_C_NOME"
---
CRIAÇÃO DE VARIAVEIS:
"NEW B_NOME" 
---
Remoção de variáveis:
"RM B_NOME"
---
Atribuição de valores:
"MOVE"

primeira parte: tipo
0 - String
1 - I8
2 - I16
3 - I32 
4 - F32 (float)
5 - F64 (double)
6 - Operational (usize)

Segunda parte: valor
valor bruto
B_NOME - valor de uma variável
%PONTEIRO - valor do ponteiro de strings

EXEMPLO:
MOVE 1, 42, B_IDADE;

Extras:

Não atribuir o tipo automaticamente atribui como Operational.
"MOVE 60, A0;"

Mover uma variável para um registrador, ou para outro lugar leva o tipo junto automaticamente.
Tentar inferir um tipo diferente do que a variável tem resultará em um comportamento indefinido.
---
CHAMADAS DE SISTEMA:

"CALL"

Chamadas observam os dados nos registradores A0, A1, A2, A3, A4, A5.

A0 - Tipo de chamada
1 - Printf
2 - NOT / Reverse
3 - IF TRUE, JUMP
4 - BOOLEAN EXPRESSION
5 - MATH OPERATION
6 - INCREMENTATION
7 - DECREMENTATION
8 - Conversion
60 - EXIT

---

# Tipo de chamada: Printf
A0 - Tipo de chamada;
A1 - Conteúdo a ser printado;
EXEMPLO:

MOVE 0, B_NOME, A1;
MOVE 1, A0;
CALL;

# Tipo de chamada: NOT / Reverse
A0 - Tipo de chamada;
A1 - Tipo de operação, onde:
0 - NOT
1 - REVERSE
A2 - Valor a ser operado;
A4 - Resultado

EXEMPLO:
MOVE 2, A0;
MOVE 0, A1; // NOT
MOVE 1, A2; // Operational 1 (true)
CALL;

Resultado:
A4 = 0

EXEMPLO:
MOVE 2, A0;
MOVE 1, A1; // REVERSE
MOVE 1, 1, A2; // i8 1
CALL;

Resultado:
A4 = -1 (i8 reverso de 1)

* Aviso
O REVERSE é apenas para tipos numéricos, e o NOT é apenas para tipos booleanos (Operational 0 ou 1).
REVERTER o sinal de um tipo Operational (usize) resultará em um crash.
Igualmente, aplicar NOT em um tipo Operational que não seja 0 ou 1 resultará em um comportamento indefinido.

Qualquer tipo numérico manterá EXATAMENTE o mesmo tipo, só que negativo.

# Tipo de chamada: IF TRUE, JUMP
A0 - Tipo de chamada;
A1 - Valor booleano (usize 0 ou 1);
A2 - Label de destino;
EXEMPLO:

MOVE 3, A0;
MOVE 1, A1;
MOVE "_C_LABEL_DESTINO", A2;
CALL;

# Tipo de chamada: BOOLEAN EXPRESSION

O valor em A2 é o tipo de operação, onde:
0 - ==
1 - !=
2 - >
3 - <
4 - >=
5 - <=
6 - || (OR)
7 - && (AND)

Não, não temos um not. não aqui.

A0 - Tipo de chamada;
A1 = Primeiro opeando
A2 - Operação
A3 - Segundo operando
A4 - Resultado (Operacional, 0 ou 1);
EXEMPLO:

MOVE 4, A0;
MOVE 1, 1, A1; // i8
MOVE 2, A2; // >
MOVE 1, 0, A3; // i8
CALL;

A resposta da expressão booleana estará em A4;

# Tipo de chamada: MATH OPERATION

O valor em A2 é o tipo de operação, onde:

0 - +
1 - -
2 - *
3 - /
4 - %
5 - ^ (potenciação)

A0 - Tipo de chamada;
A1 = Primeiro opeando
A2 - Operação
A3 - Segundo operando
A4 - Resultado (Double);

Os valores em A1 e A3 são convertidos para double, e o resultado da operação é armazenado em A4.
EXEMPLO:

MOVE 5, A0;
MOVE 1, 5, A1; // i8
MOVE 0, A2; // +
MOVE 1, 10, A3; // i8
CALL;

A resposta da expressão matemática estará em A4;

# Tipo de chamada: INCREMENTATION

A0 - Tipo de chamada;
A1 - Variável a ser incrementada;
A4 - Resultado

EXEMPLO:
MOVE 6, A0;
MOVE 1, A1; // i8
CALL;

# Tipo de chamada: DECREMENTATION

A0 - Tipo de chamada;
A1 - Variável a ser decrementada;
A4 - Resultado

EXEMPLO:
MOVE 7, A0;
MOVE 1, A1; // i8
CALL;

# Tipo de chamada: Conversion
A0 - Tipo de chamada;
A1 - Dado original;
A2 - Tipo de dado para conversão;
A4 - Resultado

Valores para A2:
0 - String
1 - I8
2 - I16
3 - I32
4 - F32 (float)
5 - F64 (double)
6 - Operational (usize)

# Tipo de chamada: EXIT
A0 - Tipo de chamada;
A1 - Código de saída (usize);

EXEMPLO:
MOVE 60, A0;
MOVE 0, A1; // código de saída
CALL;

---

Regras de escrita;

- Cada comando deve ser escrito em uma linha separada.
- Os comandos devem ser escritos em letras maiúsculas.
- Os argumentos devem ser separados por vírgulas.
- Os labels devem começar com um underscore (_) e ser seguidos pela letra C
- Os nomes de variáveis devem começar com a letra B e ser seguidos por um nome descritivo.
- Os tipos de dados devem ser representados por números inteiros, conforme descrito na seção de criação de variáveis.
- Os valores devem ser escritos de forma bruta, sem aspas ou formatação adicional.
- Os comandos de chamada de sistema devem seguir a estrutura descrita na seção de chamadas de sistema, com os argumentos apropriados para cada tipo de chamada.
--- 
Observações importantes:
- Todas as labels devem conter um ponteiro no arqivo de labels, usando a estrutura:
"linha, nome_da_label"
"0, _main"

* Em código:
MOVE "_main", A1;
---
- Strings devem ser representados no código por um ponteiro para a posição da string no arquivo de strings, usando o formato %PONTEIRO.

No arquivo de strings, as strings devem ser armazenadas em linhas separadas, com o texto plano em uma única linha, sem aspas ou formatação adicional.

* Em código:
MOVE 0, %PONTEIRO, A1; // onde %PONTEIRO é o ponteiro para a string desejada no arquivo de strings