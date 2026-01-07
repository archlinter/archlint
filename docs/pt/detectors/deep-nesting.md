# Aninhamento Profundo

**ID:** `deep_nesting` | **Severity:** Low (default)

Identifica blocos de código (if, for, while, etc.) que estão aninhados muito profundamente.

## Por que isso é um "smell"

Código profundamente aninhado é exponencialmente mais difícil de ler e entender. Muitas vezes é um sinal de que uma função está fazendo demais ou que a lógica pode ser simplificada.

## Como corrigir

- **Guard Clauses**: Retorne antecipadamente para evitar blocos `else` e reduzir o aninhamento.
- **Extract Function**: Mova o bloco aninhado interno para uma nova função.
- **Flatten Logic**: Reavalie a lógica para ver se ela pode ser expressa de forma mais simples.

## Configuração

```yaml
thresholds:
  deep_nesting:
    max_depth: 4
```
