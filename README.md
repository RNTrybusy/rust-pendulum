# Simulação de Pêndulo em Rust
Este projeto é uma simulação simples de um pêndulo implementada na linguagem Rust. O objetivo é estudar conceitos básicos de programação gráfica e física utilizando a biblioteca [speedy2d](https://github.com/QuantumBadger/Speedy2D).

## 🛠 Funcionalidades
- Simulação de um pêndulo com movimento baseado em física.
- Renderização gráfica com animação.

## 📚 Objetivo
O projeto foi criado para estudo da linguagem Rust, incluindo:
- Implementação de estruturas e métodos.
- Uso de bibliotecas para gráficos.
- Simulação de movimento físico.

## 🚀 Como Executar

1. **Clone o Repositório**:
   ```bash
   git clone https://github.com/RNTrybusy/rust-pendulum.git
   cd rust-pendulum
   ```

2. **Instale as Dependências**:
   Certifique-se de ter o [Rust](https://www.rust-lang.org/) instalado. Em seguida, adicione a biblioteca `speedy2d` ao projeto:
   ```bash
   cargo add speedy2d
   ```

3. **Compile e Execute**:
   ```bash
   cargo run
   ```

## 🔧 Estrutura do Código
- **`Vector`**: Representa posições e cálculos básicos de vetores.
- **`Pendulum`**: Contém lógica para atualizar e renderizar o movimento do pêndulo.
- **`MyWindowHandler`**: Manipula o loop de desenho e animação da janela.
