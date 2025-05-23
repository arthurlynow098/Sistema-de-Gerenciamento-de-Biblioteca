**Sistema de Biblioteca em Rust**

Um sistema de gerenciamento de biblioteca desenvolvido em Rust. O sistema permite cadastrar livros, usuários, realizar empréstimos, devoluções e consultar relatórios e estatísticas.

## 🚀 Começando

Essas instruções permitirão que você obtenha uma cópia do projeto em funcionamento na sua máquina local para desenvolvimento e testes

### 📋 Pré-requisitos

Antes de começar, você vai precisar ter o Rust instalado na sua máquina. Você pode instalar via rustup:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Verifique a instalação:
rustc --version
cargo --version

**🔧 Instalação**
Clone o repositório:
git clone https://github.com/seu-usuario/sistema-biblioteca.git
cd sistema-biblioteca

Compile o projeto:
cargo build

Execute o sistema:
cargo run

----------------------------------------------------
Exemplo de saída:
Empréstimo realizado: 1
=== ESTATÍSTICAS ===
Total de livros: 2
Disponíveis: 1
Emprestados: 1
Total de usuários: 1
Empréstimos ativos: 1

=== RELATÓRIO DO USUÁRIO ===
Nome: Alice
Email: alice@email.com
Telefone: 123456789
Livros emprestados: 1

Livros atualmente emprestados:
 - 1984 por George Orwell
----------------------------------------------------

**⚙️ Executando os testes**
No momento, o sistema não possui testes automatizados, mas você pode adicionar testes unitários em arquivos separados ou no próprio main.rs utilizando o módulo #[cfg(test)].

**🔩 Testes de ponta a ponta**
Você pode simular testes manuais executando ações no main() como:
// Criar livro e usuário
// Realizar empréstimos e devoluções
// Gerar relatórios e estatísticas

**⌨️ Testes de estilo de codificação**

O projeto segue boas práticas do Rust. Utilize cargo fmt para formatação automática:
cargo fmt

E cargo clippy para detectar melhorias:
cargo clippy

**📦 Implantação**
Para implantar em produção, compile em modo release:
cargo build --release

E execute com:
./target/release/sistema-biblioteca


**🛠️ Construído com**
Rust - Linguagem de programação de sistemas
Cargo - Gerenciador de pacotes do Rust

**✒️ Autores**
**Arthur Mendes Entsev** – Desenvolvimento inicial – arthurlynow098


Dê uma **estrela** no repositório ⭐
