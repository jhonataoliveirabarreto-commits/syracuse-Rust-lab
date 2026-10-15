/*
* Q06. Tuplas Heterogêneas, Desestruturação e Acesso por Ponto Nível 06/10 • Intermediário
Construa uma tupla chamada telemetria com anotação explícita de tipos contendo: ID do pacote ( u64 ),
temperatura do núcleo ( f32 ), status de integridade ( bool ) e inicial do módulo ( char ). Primeiro, exiba o primeiro
e o último elemento usando a notação de índice por ponto (ex.: tupla.0 ). Depois, desestruture a tupla inteira em
quatro variáveis individuais em uma única linha let e imprima cada uma
*/
fn main() {
    let telemetria: (u64, f32, bool, char) = (315, 44.5, true, 'C');
    println!(
        "
        {0}
        {1}",
        telemetria.0, telemetria.3
    );

    let (id, temp, itr, modulo) = telemetria;
    println!(
        "
        {0}
        {1}
        {2}
        {3}",
        id, temp, itr, modulo
    );
}
