# Arquivo Grande

**ID:** `large_file` | **Gravidade:** Medium (default)

Identifica arquivos de origem que excedem um determinado número de linhas.

## Por que isso é um smell

Arquivos extremamente grandes são difíceis de navegar, entender e manter. Eles geralmente indicam uma violação do Princípio da Responsabilidade Única.

## Como corrigir

Divida o arquivo em módulos menores e mais focados.

## Configuração

```yaml
rules:
  large_file:
    max_lines: 1000
```
