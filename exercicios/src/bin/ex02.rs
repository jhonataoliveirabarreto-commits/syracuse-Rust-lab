/*
* Q02. Mutabilidade Estrita vs. Sombreamento (Shadowing) Nível 02/10 • Básico
Crie uma variável mutável saldo do tipo i32 iniciando em 1500 e altere seu valor subtraindo 350 . Logo
abaixo, utilize o recurso de sombreamento (shadowing) declarando novamente let saldo para converter esse
valor em um número de ponto flutuante ( f64 ) acrescido de 5% de rendimento. Por fim, sombreie saldo uma
terceira vez para guardar um valor booleano ( bool ) que indique se o valor é maior que 1200.0 . Imprima o
resultado de cada etapa.
*/
fn main() {
    let mut saldo: i32 = 1500;

    println!("{0}", saldo);

    saldo = saldo - 350;

    println!("{0}", saldo);

    let saldo: f64 = (saldo as f32 + (saldo as f32 * 0.05)).into();

    println!("{0}", saldo);

    let saldo: bool = saldo > 1200.0;

    println!("{0}", saldo);
}
