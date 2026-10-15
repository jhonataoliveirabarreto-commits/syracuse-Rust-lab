/*
* Q13. As Quatro Faces do Estouro de Inteiros (Integer Overflow) Nível 30/10 • Exponencial III
Em modo debug, somar 1 a um u8 que vale 255 faz o programa em Rust entrar em pânico (panic). Em sistemas
embarcados e criptografia, porém, você precisa controlar o que acontece na borda do tipo. Dado let base: u8 =
250; e let incremento: u8 = 20; , armazene em uma única tupla os resultados dos três métodos explícitos de
controle de transbordo do Rust: wrapping_add , saturating_add e overflowing_add (este último retorna uma
sub-tupla (u8, bool) ). Imprima e explique no console a diferença entre os três valores obtidos
*/

fn main() {
    let base: u8 = 250;
    let incremento: u8 = 20;

    let mut controle: (u8, u8, (u8, bool)) = (0, 0, (0, false));

    controle.0 = base.wrapping_add(incremento);
    controle.1 = base.saturating_add(incremento);
    controle.2 = base.overflowing_add(incremento);

    println!(
        "
        {0}
        {1}
        {2} {3}",
        controle.0, controle.1, controle.2.0, controle.2.1
    );
}
