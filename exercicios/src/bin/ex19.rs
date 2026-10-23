/*
* Q19. Emulador de CPU de 4-Bits com Tabela de Busca (Lookup Table) Nível 260/10 • Exponencial IX
Como executar uma operação diferente escolhida pelo usuário se você não pode usar if nem match ? Usando
arrays como Tabelas de Busca (LUT)! Dada uma instrução representada por uma tupla let instr: (usize, i32,
i32) = (2, 18, 6); onde o primeiro item ( 0 a 3 ) é o código da operação (Opcode: 0=Soma , 1=Subtração ,
2=Multiplicação , 3=Divisão ) e os outros dois são os operandos: pré-calcule os 4 resultados em um array
[i32; 4] , crie um segundo array paralelo [char; 4] com os símbolos ['+', '-', '*', '/'] e use
instr.0 como índice direto para imprimir apenas a expressão e o resultado escolhido!
*/

fn main() {
    let instr: (usize, i32, i32) = (2, 18, 6);
    let pre: [i32; 4] = [
        instr.1 + instr.2,
        instr.1 - instr.2,
        instr.1 * instr.2,
        instr.1 / instr.2,
    ];
    let paralelo: [char; 4] = ['+', '-', '*', '/'];

    println!(
        "o resultado foi: {0} e a operaçao feita foi a '{1}'",
        pre[instr.0], paralelo[instr.0]
    );
}
