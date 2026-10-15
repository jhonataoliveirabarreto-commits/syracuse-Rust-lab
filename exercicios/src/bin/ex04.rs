/*
 *  Q04. A Armadilha da Divisão Inteira e Conversão Explícita (as) Nível 04/10 • Intermediário Leve
 * Você recebeu a distância percorrida por um drone como let distancia_metros: u32 = 4_850; e o tempo gasto como let tempo_segundos: u16 = 360;.
 * Calcule a velocidade média exata com casas decimais sem alterar os tipos originais das duas variáveis.
 * Imprima tanto a velocidade truncada (divisão inteira) quanto a velocidade real em f64 com exatamente três casas decimais no println!.
 */

fn main() {
    let distancia_metros: u32 = 4_850;
    let tempo_segundos: u16 = 360;

    let velocidade_media: f64 = (distancia_metros as f64) / (tempo_segundos as f64);
    let velocidade_truncada: u32 = distancia_metros / (tempo_segundos as u32);

    println!("velocidade media: {0:.3}", velocidade_media);
    println!("velocidade trunca: {0}", velocidade_truncada);
}
