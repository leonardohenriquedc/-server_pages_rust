 ---

title: Um resumo sobre Cyclic Redundance Check
author: leonardo
date: 2025-09-22
tags: ["CRC", "Redes", "Segurança de redes"]
---

No artigo de hoje vamos entender um pouco sobre o que é o **Cyclic Redundance Check** e qual a sua importância nos dias de hoje para a segurança de dados e controle de falhas em uma troca de informações pela internet.  

O CRC compõe uma parte importante no controle de erros em segmentos de dados enviados de um transmissor a um receptor. Ele é responsável por validar se a "rajada" de bits enviada está em perfeitas condições de prosseguir para a camada superior (Aplicação). O CRC é um valor resultante de um cálculo que é feito sobre a quantidade de bits a ser enviada (contando com o cabeçalho da camada de transporte, que é onde esse valor é implementado e validado).  

Vamos chamar a quantidade de bits a ser enviada de *valor*. Para adicionarmos um CRC a esse valor, precisamos determinar um **"Gerador Polinomial"**. Esse gerador é uma função cujos coeficientes são 0 e 1.  

Vamos a um exemplo:  
Enviaremos de um remetente qualquer o seguinte segmento de dados '1010011' e vamos dizer que nosso Gerador Polinomial é:

> G(x) = x⁵ + x³ + x + 1 => 1*x⁵ + 0*x⁴ + 1*x³ + 0*x² + 1*x + 1*x⁰ (1) => 101011

Perceba que, na construção do gerador, o coeficiente 1 é usado para monômios presentes na equação polinomial: x⁵, x³, x, x⁰ (1), e o coeficiente 0 para monômios ausentes na função.  

Agora que temos o G, vamos pegar o dado que queremos enviar e adicionar uma quantidade de bits 0 à direita, cuja quantidade seja igual ao grau da função G(x), que no nosso caso é 5. Logo: 1010011<u>00000</u>.  

Agora vamos ser um pouco mais técnicos e precisamos que o leitor tenha conhecimentos em relação a operadores lógicos, mais especificamente ao operador **XOR** (ou exclusivo), pois vamos fazer uma divisão usando este operador bit a bit:  

```
101001100000  
101011  
_________________________  
000010100000  
    101011  
_________________________  
000000001100  
```
  
Perceba que fizemos uma operação XOR bit a bit. Porém, quando o dividendo for 0, pulamos até que haja um bit divisível, ou seja, 1.  

Agora temos o nosso código CRC (valor sublinhado na divisão). Portanto, substituiremos os bits 0 adicionados anteriormente pelo código CRC, ficando assim: 1010011<u>01100</u>.  

O código CRC é definido contando da direita para a esquerda, num total igual à quantidade de bits 0 adicionados ao dividendo anteriormente.  

Vamos agora validar se o resultado dessa operação está correto. Essa validação é feita colocando o resultado da operação anterior para fazer uma divisão bit a bit de **ou exclusivo**, onde o divisor é o mesmo da operação anterior:  

```
101001101100  
101011  
_________________________  
000010101100  
    101011  
_________________________  
000000000000
```

Como vemos, o resultado dessa operação foi 0, ou seja, o cálculo está correto, constatando assim que o segmento de dados está intacto.

### Observações

- Por que, no Gerador Polinomial, os monômios ausentes são levados em consideração?  
  - Porque a posição binária de cada expoente precisa ser preservada.  

Como podemos ver um cálculo relativamente de se entender, agora vou mostrar ao leitor dois videos do Professor Orthon Batista, que explica de forma visual e
intuitiva como funciona a divisão:

[Calculo de CRC - Parte 1](https://www.youtube.com/watch?v=XWcJcybL3JQ&list=PL7KqjbmLa7gHguFAPlqc-f_gx8TvVbTYE)  
[Calculo de CRC - Parte 2](https://www.youtube.com/watch?v=wyUNSzDbFjg&list=PL1iys8ibudKeD2f4c0-CdzHzmcd4bKvt1)
