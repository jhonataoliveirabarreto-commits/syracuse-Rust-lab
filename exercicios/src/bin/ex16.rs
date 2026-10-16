/*
*Q16. Autopsia de Ponto Flutuante IEEE 754 (Imprecisão, NaN e to_bits) Nível 85/10 • Exponencial VI
* No mundo inteiro, iniciantes se assustam ao descobrir que 0.1_f64 + 0.2_f64 == 0.3_f64 resulta em false!
Armazene uma tupla contendo:
(1) o booleano dessa comparação,
(2) a diferença absoluta entre (0.1 + 0.2) e 0.3 impressa com 20 casas decimais ({:.20}),
(3) o resultado de 0.0_f32 / 0.0_f32 (NaN) comparado consigo mesmo (nan == nan), e
(4) a representação binária bruta da memória do número -42.625_f32 extraída com o método .to_bits().
Exiba todos os resultados e o bit de sinal, expoente e mantissa do -42.625_f32.
*/
fn main() {
    let ab = 0.1_f64 + 0.2_f64 == 0.3_f64;
    println!("{0}", ab);
    let diff = 0.1 + 0.2;
    let res = (0.0_f32 / 0.0f32) == f32::NAN;
    let binary_brut = (-42.625_f32).to_bits();

    //nota do usuario: o uso de hexa decimal usando a porta AND e para evitar passar valores nao desejaveis.
    let sinal = (binary_brut >> 31) & 1;
    let expoente_b = (binary_brut >> 23) & 0xFF;
    let _expoente_real = (expoente_b as i32) - 127;
    let mantissa = binary_brut & 0x007F_FFFF;

    let autopsia: (bool, f64, bool, u32) = (ab, diff, res, binary_brut);

    println!("{:?}", autopsia);
    println!(
        " binary brut: {3:b}

        {0:b}
        {1:b}
        {2:b}",
        sinal, expoente_b, mantissa, binary_brut
    );
}
