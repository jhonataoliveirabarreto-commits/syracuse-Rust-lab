# ⚙️ Syracuse Rust Lab 🦀

> *"Brincar é condição fundamental para ser sério."* — Arquimedes de Siracusa

Bem-vindo ao **Syracuse Rust Lab**. Assim como Siracusa foi o palco dos experimentos mecânicos e matemáticos de Arquimedes, este repositório é minha oficina prática para explorar, testar e dominar a linguagem **Rust** — indo desde a "brincadeira" de quebrar a cabeça com o compilador até a engenharia séria de gerenciamento de memória e abstrações de custo zero.

## 🎯 Objetivo da Oficina
Documentar minha evolução técnica em Rust através de experimentos práticos, resolução de algoritmos, estudos do *Borrow Checker* e construção de ferramentas em linha de comando (CLI).

---

## 🗺️ Trilha de Progresso (Roadmap)

### Fundamentos
- [x] Configuração do ambiente (`rustup`, `cargo`) e Hello World
- [x] Variáveis, Mutabilidade e Tipos de Dados
- [ ] Funções e Controle de Fluxo

### O Coração do Rust (Memória)
- [ ] **Ownership** (Posse) e Movimentação
- [ ] **Borrowing** (Empréstimo) e Referências (`&` e `&mut`)
- [ ] **Slices** e manipulação de Strings (`String` vs `&str`)
- [ ] **Lifetimes** (Tempo de vida de referências)

### Estruturação e Abstrações
- [ ] `Structs` e Métodos (`impl`)
- [ ] `Enums` e *Pattern Matching* (`match`, `if let`)
- [ ] Tratamento de Erros (`Result<T, E>` e `Option<T>`)
- [ ] Coleções Comuns (`Vec`, `HashMap`)
- [ ] Generics e **Traits** (Polimorfismo)

### Avançado & Baixo Nível
- [ ] *Smart Pointers* (`Box`, `Rc`, `RefCell`)
- [ ] Concorrência sem medo (*Threads*, *Channels*, `Arc`, `Mutex`)
- [ ] Manipulação de Bits e Ponteiros Brutos (`unsafe` Rust)

---

## 🚀 Como executar os experimentos

Como o Rust utiliza o **Cargo** como gerenciador de pacotes e build system, para rodar qualquer experimento específico, basta navegar até a pasta do projeto pelo terminal e usar:

```bash
# Compilar e rodar o projeto
cargo run --bin nome_do_exercicio

# Apenas checar se o código compila (mais rápido que o build completo)
cargo check

# Rodar os testes unitários
cargo test
