/*
* Q11. Permutação de Variáveis via XOR Bit-a-Bit (Sem Variável Temporária) Nível 15/10 • Exponencial I
Dadas duas variáveis mutáveis let mut a: u32 = 0b1011_0011; e let mut b: u32 = 0b0110_1100; ,
inverta os valores entre a e b sem declarar nenhuma variável auxiliar, sem usar tuplas de reatribuição e sem usar
operações aritméticas de soma/subtração (que poderiam causar estouro de limite). Utilize exclusivamente três
operações consecutivas do operador lógico bit-a-bit XOR ( ^ ). Imprima os valores em binário antes e depois de cada
um dos três passos.
*/

fn main() {
    let mut a: u32 = 0b1011_0011;
    let mut b: u32 = 0b0110_1100;

    println!(
        "
        {0:08b}
        {1:08b}",
        a, b
    );
    a = a ^ b;
    println!(
        "
        {0:08b}
        {1:08b}",
        a, b
    );
    b = a ^ b;
    println!(
        "
        {0:08b}
        {1:08b}",
        a, b
    );
    a = a ^ b;

    println!(
        "
        {0:08b}
        {1:08b}",
        a, b
    );
}
