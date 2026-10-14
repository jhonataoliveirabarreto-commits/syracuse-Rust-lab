/*
* Q01. Interpolação Posicional e Nomeada no println! Nível 01/10 • Básico
Declare três variáveis imutáveis: um código numérico de agente ( u16 ), uma letra de setor ( char ) e uma taxa de
eficiência ( f64 ). Em seguida, utilizando uma única chamada da macro println! , exiba um relatório onde o
código do agente apareça duas vezes na frase sem repetir a variável na lista de argumentos da macro. Na linha
seguinte, faça outra impressão usando argumentos nomeados diretamente dentro das chaves.
*/

fn main() {
    let cod_num: u16 = 2;

    let setor: char = 'B';

    let tx_eficiencia: f64 = 23.5;

    println!(
        "o agente de codigo: {0}, subiu para o setor {1} e apresentou uma taxa de eficiencia de {2}. Parabens agente {0} ",
        cod_num, setor, tx_eficiencia
    );

    println!(
        "o agente de codigo: {cod_num}, subiu para o setor {setor} e apresentou uma taxa de eficiencia de {tx_eficiencia}. Parabens agente {cod_num} "
    );
}
