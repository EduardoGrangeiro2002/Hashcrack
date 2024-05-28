# Hashcrack: Quebrando Senhas com Força Bruta

## Descrição

Hashcrack é um programa em Rust projetado para demonstrar a eficácia de ataques de força bruta contra diferentes algoritmos de hashing. Ao comparar o tempo necessário para quebrar hashes gerados por algoritmos antigos e modernos, o projeto destaca a importância da escolha de algoritmos robustos e da utilização de senhas fortes.

## Objetivo

Este projeto tem como objetivo:

1. **Explorar o potencial do Rust:** Demonstrar a capacidade da linguagem Rust em desenvolver ferramentas de alta performance e seguras para tarefas computacionalmente intensivas.
2. **Avaliar a segurança de algoritmos de hashing:** Quantificar a resistência de diferentes algoritmos de hashing a ataques de força bruta, comparando o tempo de quebra em diferentes cenários.
3. **Conscientizar sobre a importância da segurança de senhas:** Educar os usuários sobre a necessidade de escolher senhas fortes e utilizar algoritmos de criptografia robustos para proteger seus dados.

## Recursos

- Implementação eficiente de ataques de força bruta em Rust.
- Suporte a diversos algoritmos de hashing, incluindo MD5, SHA-1, SHA-256 e bcrypt.
- Comparação do tempo de quebra para diferentes combinações de algoritmos e senhas.
- Interface de linha de comando intuitiva para facilitar o uso.

## Como Usar

### Instalação

```bash
git clone https://github.com/seu-usuario/hashcrack.git
cd hashcrack
cargo build --release
```

### Execução

```bash
./target/release/hashcrack --algoritmo <algoritmo> --senha <senha>
```

- Substitua `<algoritmo>` por um dos algoritmos suportados (md5, sha1, sha256, bcrypt).
- Substitua `<senha>` pela senha que você deseja testar.

### Exemplo

```bash
./target/release/hashcrack --algoritmo sha256 --senha "senha123"
```

> **Aviso:** Este programa é destinado apenas para fins educacionais e de pesquisa. O uso indevido para quebrar senhas de terceiros é ilegal e antiético.

---
