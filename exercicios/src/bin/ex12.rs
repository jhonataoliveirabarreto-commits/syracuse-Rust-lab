/*
* Q12. Desempacotamento de Cor ARGB de 32-bits em Tupla de 4 Canais Nível 22/10 • Exponencial II
Em motores gráficos, uma cor com transparência (Alpha, Red, Green, Blue) costuma ser compactada em um único
inteiro u32 no formato 0xAA_RR_GG_BB . Dada a constante let cor_hex: u32 = 0x80_FF_57_33; , extraia
matematicamente cada um dos 4 canais usando deslocamento de bits (bit shifting >> ) e máscaras binárias ( &
0xFF ), armazenando o resultado final em uma tupla (u8, u8, u8, u8) . Ao final, reconstrua o u32 original a
partir da tupla usando o operador | (OR bit-a-bit) e << .
*/
fn main() {
    let cor_hex: u32 = 0x80_FF_57_33;

    //minhas notas: cada cor com transparência tem 8 bits(0000_0000), para colocar o valor certo em cada variavel transforma de hexadecimal para binario.
    // ou faça o shifting de 8 em 8

    let a: u8;
    let r: u8;
    let g: u8;
    let b: u8;

    a = (cor_hex >> 24 & 0xFF) as u8;
    r = (cor_hex >> 16 & 0xFF) as u8;
    g = (cor_hex >> 8 & 0xFF) as u8;
    b = (cor_hex & 0xFF) as u8;

    let armazenamento: (u8, u8, u8, u8) = (a, r, g, b);

    let mut cor_original: u32 = 0;

    cor_original |= armazenamento.0 as u32;
    cor_original = cor_original << 8;
    cor_original |= armazenamento.1 as u32;
    cor_original = cor_original << 8;
    cor_original |= armazenamento.2 as u32;
    cor_original = cor_original << 8;
    cor_original |= armazenamento.3 as u32;
    cor_original = cor_original << 0;

    println!("{0:#X}", cor_original);
}
