# Arquivo Grande

**ID:** `large_file` | **Gravidade:** Medium (default)

Identifica arquivos que cresceram tanto que provavelmente merecem seu próprio CEP.

## Por que isso é um smell

Arquivos extremamente grandes são um pesadelo para navegar. Você passa mais tempo scrollando e procurando símbolos do que realmente escrevendo código. Geralmente, um arquivo de 2000 linhas é só três ou quatro módulos lógicos menores disfarçados de um só. Viola o Princípio da Responsabilidade Única e quase garante conflitos de merge.

## Como corrigir

Divida o arquivo em módulos menores e mais focados.

## Configuração

```yaml
rules:
  large_file:
    max_lines: 1000
```
