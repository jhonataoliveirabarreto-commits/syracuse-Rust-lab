/*
* Q18. Regra de Cramer & Determinante 3×3 em Arrays Multidimensionais Nível 180/10 • Exponencial VIII
Represente uma matriz matemática 3×3 usando um array composto aninhado: let m: [[f64; 3]; 3] = [[2.0,
-1.0, 3.0], [0.0, 4.0, -2.0], [1.0, 5.0, 1.0]]; . Utilizando a indexação multidimensional ( m[linha]
[coluna] ) e variáveis intermediárias para organizar os cofatores (Regra de Sarrus), calcule o determinante exato da
matriz m . Em seguida, armazene em uma tupla a diagonal principal [f64; 3] , a diagonal secundária [f64; 3]
e o valor do determinante, exibindo tudo formatado.
*/
fn main() {
    let m: [[f64; 3]; 3] = [[2.0, -1.0, 3.0], [0.0, 4.0, -2.0], [1.0, 5.0, 1.0]];

    let principaldg_determinates = (m[0][0] * m[1][1] * m[2][2])
        + (m[0][1] * m[1][2] * m[2][0])
        + (m[0][2] * m[1][0] * m[2][1]);

    let secundariadg_determinates = (m[0][1] * m[1][0] * m[2][2])
        + (m[0][0] * m[1][2] * m[2][1])
        + (m[0][2] * m[1][1] * m[2][0]);

    let determinante = principaldg_determinates + (-1.0 * secundariadg_determinates);

    let f: ([f64; 3], [f64; 3], f64) = (
        [m[0][0], m[1][1], m[2][2]],
        [m[0][2], m[1][1], m[2][0]],
        determinante,
    );
    println!("{0}", determinante);

    println!(
        "
        {0:?}
        {1:?}
        {2}",
        f.0, f.1, f.2
    );
}
