/*
* Q07. Arrays de Tamanho Fixo, Sintaxe de Repetição e Mutação Nível 07/10 • Intermediário
Declare um array mutável de 8 posições do tipo i16 onde todos os elementos iniciem com o valor -1 , utilizando a
sintaxe curta de inicialização do Rust (sem digitar -1 oito vezes). Em seguida, substitua manualmente o primeiro
elemento por 100 , o elemento central (índice 4) por 250 e o último elemento pela soma do primeiro com o central.
Exiba o array completo em uma única linha usando o formatador de depuração
*/
fn main() {
    let mut array: [i16; 8] = [-1; 8];

    array[0] = 100;
    array[4] = 250;
    array[7] = array[0] + array[4];

    println!("{:?}", array);
}
