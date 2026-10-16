/*
* Q14. Ordenação de Array sem if e sem Loops (Programação Branchless) Nível 45/10 • Exponencial IV
Você ainda não aprendeu if/else nem loops, mas precisa colocar um array de 3 inteiros let mut v: [i32; 3]
= [42, 15, 28]; em ordem estritamente crescente! Dica fundamental: em Rust, converter uma comparação
booleana para inteiro com (a > b) as i32 resulta em 1 (se verdadeiro) ou 0 (se falso). Use essa propriedade
matemática para criar uma fórmula que calcule o menor e o maior entre dois números sem usar desvios
condicionais, aplicando-a nas posições do array até que v[0] < v[1] < v[2] .
*/

fn main() {
    let mut v: [i32; 3] = [42, 15, 28];
    // em caso na pior  das formas:let mut v: [i32; 3] = [3, 2, 1];

    let mut ordem: i32;
    let mut pp: i32;

    pp = -((v[1] < v[0]) as i32);
    ordem = (v[1] ^ v[0]) & pp;

    v[1] ^= ordem;
    v[0] ^= ordem;

    pp = -((v[2] < v[1]) as i32);
    ordem = (v[2] ^ v[1]) & pp;

    v[2] ^= ordem;
    v[1] ^= ordem;

    pp = -((v[1] < v[0]) as i32);
    ordem = (v[1] ^ v[0]) & pp;

    v[1] ^= ordem;
    v[0] ^= ordem;

    println!("{:?}", v);
}
