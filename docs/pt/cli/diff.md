# archlint diff

O comando `diff` é a funcionalidade principal que implementa o enfoque Ratchet (melhoria progressiva). Ele compara sua base de código atual com um snapshot salvo anteriormente ou outro branch/commit do git.

## Uso

```bash
# Comparar com um arquivo de snapshot
archlint diff <baseline.json> [options]

# Comparar com uma referência do git
archlint diff <git-ref> [options]
```

## Como funciona

O archlint não apenas conta problemas. Ele realiza um **diff semântico** dos defeitos arquiteturais (smells):

1. **Novos problemas**: Defeitos que existem agora, mas não existiam na linha de base (ex: um novo ciclo).
2. **Problemas agravados**: Defeitos existentes que se tornaram mais graves (ex: um ciclo cresceu de 3 arquivos para 5).
3. **Problemas corrigidos**: Defeitos que existiam na linha de base, mas agora se foram.

## Opções

| Opção                  | Padrão  | Descrição                                                                     |
| ---------------------- | ------- | ----------------------------------------------------------------------------- |
| `-j, --json`           | `false` | Saída do relatório em formato JSON                                            |
| `-v, --verbose`        | `false` | Habilitar saída detalhada (verbose)                                           |
| `-p, --path <path>`    | `.`     | Caminho do projeto                                                            |
| `--fail-on <severity>` | `low`   | Sai com código 1 se uma regressão desta severidade ou superior for encontrada |
| `--explain`            | `false` | Fornece uma explicação detalhada para cada regressão                          |

## Configuração

Você pode ajustar o mecanismo de diff em seu arquivo `.archlint.yaml`:

```yaml
diff:
  metric_threshold_percent: 20 # relatar como regressão apenas se a métrica piorar >20%
  line_tolerance: 50 # ignorar deslocamentos de até 50 linhas na correspondência difusa
```

Consulte o [Guia de Configuração](/pt/configuration/index#configuração-de-diff) para mais detalhes.

## Exemplos

### Verificar contra o branch main no CI/CD

```bash
archlint diff origin/main --fail-on low --explain
```

### Verificar contra uma linha de base local

```bash
archlint diff .archlint-baseline.json
```
