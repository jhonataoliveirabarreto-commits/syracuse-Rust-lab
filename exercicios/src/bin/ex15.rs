/*
 * Q15. Ordem de Bytes na Memória (Big-Endian vs. Little-Endian) Nível 60/10 • Exponencial V
 * Quando um número u32 como 0x12_34_56_78 é salvo na memória RAM ou enviado pela internet, a ordem dos seus 4 bytes depende da arquitetura do processador.
 * Utilize os métodos nativos de tipos primitivos do Rust to_be_bytes(), to_le_bytes() e to_ne_bytes() para transformar esse número em três arrays do tipo [u8; 4].
 * Depois, inverta os bytes manualmente em um novo array e reconstrua um u32 usando u32::from_be_bytes(...). Imprima os três arrays em hexadecimal para descobrir qual é a arquitetura da sua máquina.
 *
 */

fn main() {
    let ram: u32 = 0x12_34_56_78;

    let mut msb: [u8; 4] = ram.to_be_bytes();
    let mut lsb: [u8; 4] = ram.to_le_bytes();
    let mut native: [u8; 4] = ram.to_ne_bytes();
    let invertido: [u8; 4] = [lsb[3], lsb[2], lsb[1], lsb[0]];
    let mut recon = u32::from_be_bytes(invertido);

    println!("{:#X?}", lsb);
    println!("{:#X?}", msb);
    println!("{:#X?}", native);
    println!("{:#X?}", recon);
}
