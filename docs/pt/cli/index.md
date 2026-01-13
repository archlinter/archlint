---
title: Referência da CLI
description: Referência completa para os comandos da CLI do archlint, incluindo scan, diff, snapshot e watch.
---

# Referência da CLI

A CLI do archlint é a principal forma de interagir com a ferramenta.

## Uso Geral

```bash
archlint [command] [options]
```

## Comandos

| Comando                        | Descrição                                         |
| ------------------------------ | ------------------------------------------------- |
| [`scan`](/pt/cli/scan)         | Executa uma análise arquitetural única            |
| [`diff`](/pt/cli/diff)         | Compara o estado atual com uma linha de base      |
| [`snapshot`](/pt/cli/snapshot) | Salva o estado atual em um arquivo JSON           |
| [`watch`](/pt/cli/watch)       | Executa em modo watch para feedback em tempo real |

## Opções Globais

| Opção                 | Descrição                                          |
| --------------------- | -------------------------------------------------- |
| `-c, --config <path>` | Caminho para o arquivo de configuração             |
| `-v, --verbose`       | Habilita o log detalhado                           |
| `-q, --quiet`         | Modo amigável para CI/CD (sem barras de progresso) |
| `-V, --version`       | Mostra informações da versão                       |
| `-h, --help`          | Mostra ajuda para um comando                       |
