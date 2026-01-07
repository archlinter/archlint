# archlint scan

O comando `scan` realiza uma análise arquitetural completa do seu projeto.

## Uso

```bash
archlint scan [path] [options]
```

## Opções

| Opção                       | Padrão   | Descrição                                                  |
| --------------------------- | -------- | ---------------------------------------------------------- |
| `--format <format>`         | `table`  | Formato de saída: `table`, `json`, `markdown`              |
| `--report <file>`           | `stdout` | Salva o relatório em um arquivo                            |
| `--min-severity <sev>`      | `low`    | Filtra por severidade: `low`, `medium`, `high`, `critical` |
| `--detectors <ids>`         | `all`    | Lista de detectores separados por vírgula para executar    |
| `--exclude-detectors <ids>` | `none`   | Detectors para pular                                       |
| `--no-cache`                | `false`  | Desabilita o cache de análise                              |

## Exemplos

### Scan com relatório em Markdown

```bash
archlint scan --format markdown --report report.md
```

### Executar apenas detecção de ciclos

```bash
archlint scan --detectors cycles,circular_type_deps
```

### Apenas severidade alta

```bash
archlint scan --min-severity high
```
