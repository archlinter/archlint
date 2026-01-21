---
title: snapshot
description: "Capture o estado atual da arquitetura do seu projeto e salve-o em um arquivo JSON para uso com o comando diff."
---

# archlint snapshot

O comando `snapshot` captura o estado atual da arquitetura do seu projeto e o salva em um arquivo JSON. Este arquivo pode então ser usado com o comando `diff`.

## Uso

```bash
archlint snapshot [options]
```

## Opções

| Opção                 | Padrão                   | Descrição                        |
| --------------------- | ------------------------ | -------------------------------- |
| `--output, -o <file>` | `archlint-snapshot.json` | O arquivo para salvar o snapshot |

## Exemplos

### Criar uma linha de base para o projeto

```bash
archlint snapshot -o .archlint-baseline.json
```
