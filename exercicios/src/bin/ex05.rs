/*
* Q05. Anatomia do char Unicode vs. Byte ASCII Nível 05/10 • Intermediário Leve
Em linguagens antigas como C, um caractere ocupa apenas 1 byte. No Rust, o tipo primitivo char tem tamanho fixo
de 4 bytes para suportar qualquer símbolo Unicode. Declare três variáveis char : a letra 'R' , o símbolo
matemático 'Ω' e o emoji de engrenagem '⚙' . Converta a letra 'R' e o emoji para u32 usando as u32 e
imprima seus respectivos códigos numéricos (code points) ao lado dos símbolos originais.
*/
fn main() {
    let letra: char = 'R';
    let _simbolo: char = 'Ω';
    let emoji: char = '⚙';
    let code_letra: u32 = letra as u32;
    let code_emoji: u32 = emoji as u32;
    println!("{0} versao codigo: {1}", letra, code_letra);
    println!("{0} versao codigo: {1}", emoji, code_emoji);
}
