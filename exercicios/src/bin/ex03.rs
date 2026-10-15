/*
* O Rust exige precisão absoluta na escolha de espaço em memória. Declare oito variáveis representando os limites
mínimos e máximos exatos dos tipos i8 , u8 , i16 e u32 . Para valores acima de mil, utilize obrigatoriamente o
sublinhado ( _ ) como separador visual de milhar (ex.: 1_000 ) e anexe o sufixo do tipo diretamente no literal
numérico. Exiba todos em uma tabela alinhada no terminal.
*/

fn main() {
    let unsigned8b = 255u8;
    let unsigned32b = 4_294_967_295u32;

    let unsigned8b_min = 0u8;
    let unsigned32b_min = 0u32;

    println!(
        "max:
        {0}
        {1}
        min:
        {2}
        {3}",
        unsigned8b, unsigned32b, unsigned8b_min, unsigned32b_min
    );

    let signed8b = 127i8;
    let signed16b = 32_767i16;
    let signed8b_min = -128i8;
    let signed16_min = -32_768i16;

    println!(
        "max:
        {0}
        {1}
        min:
        {2}
        {3}",
        signed8b, signed16b, signed8b_min, signed16_min
    );
}
