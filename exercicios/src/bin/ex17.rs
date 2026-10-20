/*
* Um Ring Buffer (fila circular) usa um array de tamanho fixo onde, ao passar do último índice, voltamos
automaticamente ao índice zero sem precisar de if . Dado um array let historico: [char; 5] = ['A', 'B',
'C', 'D', 'E']; e uma posição inicial let cursor: usize = 3; , utilize apenas o operador de resto da divisão
( % ) e o tipo de indexação de arquitetura usize para acessar e imprimir os próximos 4 elementos em sequência
circular após o cursor (saltos +1 , +2 , +3 e +4 ) e o elemento exatamente 2 posições atrás do cursor sem gerar
números negativos.
*/
fn main() {
    let historico: [char; 5] = ['A', 'B', 'C', 'D', 'E'];
    let cursor: usize = 3;
    let mut modifica: usize;

    //println!("{:?}", historico);

    modifica = (cursor + 1) % 5;

    println!("primeira:{0}", historico[modifica]);
    modifica = (cursor + 2) % 5;
    println!("segunda:{0}", historico[modifica]);
    modifica = (cursor + 3) % 5;
    println!("terceira:{0}", historico[modifica]);
    modifica = (cursor + 4) % 5;
    println!("quarta:{0}", historico[modifica]);
    modifica = (cursor + 5 - 2) % 5;
    println!("duas atras:{0}", historico[modifica]);
}
