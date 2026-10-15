/*
* Q10. Marco 10: Painel Hexadecimal, Binário e Preenchimento Nível 10/10 • Fechamento da Fase 1
Você possui uma tupla com três registradores de 16 bits: let reg: (u16, u16, u16) = (42, 255, 4095); .
Sem criar variáveis extras, monte um painel perfeitamente alinhado no console usando apenas especificadores
avançados do println! : cada número deve ser exibido em Decimal (alinhado à direita com largura de 6 espaços),
em Hexadecimal maiúsculo com prefixo 0x e 4 dígitos preenchidos com zero à esquerda, e em Binário com 16
dígitos preenchidos com zeros à esquerda.
*/
fn main() {
    let reg: (u16, u16, u16) = (42, 255, 4095);
    println!(
        "decimal:
        1:{0:>6}
        2:{1:>6}
        3:{2:>6}

        hexadecimal:
        1:{0:#06X}
        2:{1:#06X}
        3:{2:#06X}

        binario:
        1:{0:016b}
        2:{1:016b}
        3:{2:016b}
        ",
        reg.0, reg.1, reg.2
    );
}
