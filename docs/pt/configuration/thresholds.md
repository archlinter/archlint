# Limites (Thresholds)

Os limites permitem que você ajuste quando um detector deve relatar um "cheiro" (smell).

## Limites Comuns

| Detector       | Opção              | Padrão | Descrição                                                    |
| -------------- | ------------------ | ------ | ------------------------------------------------------------ |
| `cycles`       | `exclude_patterns` | `[]`   | Padrões glob para ignorar na detecção de ciclos              |
| `god_module`   | `fan_in`           | `10`   | Máximo de dependências de entrada                            |
| `god_module`   | `fan_out`          | `10`   | Máximo de dependências de saída                              |
| `god_module`   | `churn`            | `20`   | Máximo de commits git no histórico                           |
| `god_module`   | `max_lines`        | `500`  | Máximo de linhas de código no arquivo                        |
| `complexity`   | `max_complexity`   | `15`   | Máxima complexidade ciclomática por função                   |
| `deep_nesting` | `max_depth`        | `4`    | Máxima profundidade de aninhamento para blocos               |
| `long_params`  | `max_params`       | `5`    | Máximo de parâmetros por função                              |
| `large_file`   | `max_lines`        | `1000` | Máximo de linhas por arquivo                                 |
| `lcom`         | `threshold`        | `1`    | Máximo permitido de componentes não conectados em uma classe |

## Exemplo de Configuração

```yaml
thresholds:
  god_module:
    fan_in: 20
    max_lines: 800

  complexity:
    max_complexity: 10

  large_file:
    max_lines: 2000
```
