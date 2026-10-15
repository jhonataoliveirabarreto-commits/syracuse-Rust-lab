/*
* Crie uma tupla de 5 elementos representando as coordenadas e metadados de um satélite: (i32, i32, i32,
f64, bool) . Desestruture essa tupla extraindo apenas a primeira e a quarta posição em variáveis nomeadas,
ignorando todas as outras posições com o curinga _ para que o compilador não emita alertas de variáveis não
utilizadas (unused variable warnings). Além disso, crie uma variável que armazene explicitamente o tipo vazio ()
(unit type) e imprima-a com {:?} .
*/
fn main() {
    let satelite: (i32, i32, i32, f64, bool) = (22, 33, 44, 72.5, true);
    let (x, _, _, z, _) = satelite;
    let vazio: () = ();

    println!("{:?}", vazio);
}
