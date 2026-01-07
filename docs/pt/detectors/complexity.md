# Alta Complexidade (High Complexity)

**ID:** `complexity` | **Gravidade:** Média (padrão)

Este detector identifica funções com alta Complexidade Ciclomática.

## Por que isso é um smell

- **Difícil de Entender**: Muitos caminhos de ramificação tornam o código difícil de seguir.
- **Propenso a Bugs**: Maior chance de esquecer casos de borda durante os testes.
- **Pesadelo de Manutenção**: Pequenas mudanças podem ter efeitos imprevisíveis devido à lógica complexa.

## Como corrigir

1. **Extrair Método (Extract Method)**: Divida a lógica complexa em funções menores e nomeadas.
2. **Cláusulas de Guarda (Guard Clauses)**: Use retornos antecipados para reduzir os níveis de aninhamento.
3. **Substituir Condicional por Polimorfismo**: Use objetos ou estratégias em vez de grandes blocos `switch` ou `if/else`.

## Configuração

```yaml
thresholds:
  complexity:
    max_complexity: 15
```
