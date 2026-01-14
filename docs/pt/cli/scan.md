# archlint scan

O comando `scan` realiza uma análise arquitetural completa do seu projeto.

## Uso

```bash
archlint scan [path] [options]
```

## Opções

| Opção                           | Padrão   | Descrição                                                   |
| ------------------------------- | -------- | ----------------------------------------------------------- |
| `-f, --format <format>`         | `table`  | Formato de saída: `table`, `json`, `markdown`, `sarif`      |
| `-j, --json`                    | `false`  | Atalho para `--format json`                                 |
| `-r, --report <file>`           | `stdout` | Salva o relatório em um arquivo                             |
| `-s, --min-severity <sev>`      | `low`    | Filtra por severidade: `low`, `medium`, `high`, `critical`  |
| `-S, --min-score <score>`       | `none`   | Filtra pela pontuação mínima de saúde                       |
| `-d, --detectors <ids>`         | `all`    | Lista de detectores separados por vírgula para executar     |
| `-e, --exclude-detectors <ids>` | `none`   | Detectors para pular                                        |
| `-A, --all`                     | `false`  | Executa todos os detectores (incluindo desativados)         |
| `--no-cache`                    | `false`  | Desabilita o cache de análise                               |
| `--no-git`                      | `false`  | Desabilita a integração com o git (pula a análise de churn) |

## Exemplos

### Scan com relatório em Markdown

```bash
archlint scan --format markdown --report report.md
```

### Exportar para SARIF (para GitHub Code Scanning)

```bash
archlint scan --format sarif --report results.sarif
```

### Executar apenas detecção de ciclos

```bash
archlint scan --detectors cycles,circular_type_deps
```

### Apenas severidade alta

```bash
archlint scan --min-severity high
```
