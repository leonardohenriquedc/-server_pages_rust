---
title: Um resumo sobre Cyclic Redundance Check
author: leonardo
data: 2025/09/22
tags: ["CRC", "Redes", "Segurança de redes"]
---

No artigo de hoje vamos entender um pouco sobre o que e o **Cyclic Redundance Check** e qual a sua importancia nos dias de hoje para
a Segurança de dados e controle de falhas em uma troca de redes pela internet.  
O CRC compoem uma parte importante no controle de erros em segmentos de dados enviados de um transmissor a um receptor, ele e responsavel por
validar se "Rajada" de bits enviados está em perfeitas condições de prosseguir para a camada superior (Aplicação). O CRC e um valor resultante
de um calculo que e feito sobre a quantidade de bits a ser enviado (contando com o cabeçalho da camada de transporte, que é onde
esse valor e implementado e validado).  
Vamos chamar a quantidade de bits a ser enviado de *valor*, para adicionarmos um CRC a esse valor precisamos da determinação de um **"Gerador Polinomial"**
esse Gerador e uma função cujo seus coeficientes são 0 e 1.  
Vamos a um exemplo:  
Enviaremos de um remetente qualquer o seguinte segmento de dados '1010011' e vamos dizer que nosso Gerador Polinomial é

> G(x) = X⁵ + x³ + x + 1 =>  1*x⁵ + 0*x⁴ + 1*x³ + 0*x² + 1*x + 1*x⁰ (1) => 101011

Perceba que na construção do Gerador o coeficiente 1 e usados para monomios presentes na equação Polinomial: x⁵, x³, x, x⁰ (1) e o coeficiente 0 para
monomios ausentes na função.  
Agora que temos o no G, vamos pegar o dado que queremos enviar e adicionar uma quantidade de bit 0 a direita cuja a sua quantidade seja igual
ao grau da função G(x), que no nosso caso e 5, logo 1010011<u>00000</u>.  
Agora vamos ser um pouco mais tecnicos e preciso que o leitor tenha conhecimentos em relação a operadores logicos, mais especificamente ao
operador **XOR** (ou exclusivo) pois vamos fazer uma divisão usando este operador bit a bit:  
  
```
101001100000  
101011  
_________________________  
000010100000  
    101011  
_________________________  
000000001100  
```
  
Perceba que fizemos uma operação XOR bit a bit, porem quando o dividendo for 0 pulamos ate que haja um bit divisivel ou seja 1.  
Agora temos o nosso codigo CRC (valor sublinhado na divisão), portanto agora substituiremos os bits 0 adicionados anteriormente
pelo codigo CRC ficando portanto: 1010011<u>01100</u>.  
O Codigo CRC e definido contando da direita para a esquerda num total igual a quantidade de bits 0 adicionados ao dividendo anteriormente.  
Vamos agora validar se o resultado dessa operação esta correto, essa validação e feita colocando o resultado da operação anterior para fazer
uma divisão bit a bit de **ou exclusivo** onde o divisor e o mesmo da operação anterior:  

```
101001101100  
101011  
_________________________  
000010101100  
    101011  
------------------------  
000000000000
```

Como vemos o resultado dessa operação foi 0 ou seja o calculo esta correto, constatando assim de que o segmento de dados esta intacto.

### Observações

- Porque no Gerador Polinomial os monomios ausentes são levados em consideração?  
  - Por que a posição binaria de cada expoente precisa ser preservada.
