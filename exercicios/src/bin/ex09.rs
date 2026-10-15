/*
* Q09. Estruturas Compostas Aninhadas (Matriz de Tuplas) Nível 09/10 • Avançado Inicial
Declare uma variável grade_sensores cujo tipo seja um array de 3 posições, onde cada posição contém uma tupla
de 2 elementos: um identificador char e um array interno de 3 leituras f32 — isto é, [(char, [f32; 3]); 3] .
Preencha toda a estrutura com dados fictícios e calcule a média aritmética simples das 3 leituras do segundo
sensor, exibindo o caractere identificador e a média calculada.
*/
fn main() {
    let grade_sensores: [(char, [f32; 3]); 3] = [
        ('b', [32.5, 33.5, 66.5]),
        ('c', [44.5, 55.6, 11.5]),
        ('a', [100.5, 88.5, 33.5]),
    ];
    let media_aritmetica =
        (grade_sensores[1].1[0] + grade_sensores[1].1[1] + grade_sensores[1].1[2]) / 3.0;

    println!("{0}, codigo:{1}", media_aritmetica, grade_sensores[1].0);
}
