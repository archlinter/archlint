---
title: Suporte ao oclif
description: "Suporte integrado para o framework CLI oclif, reconhecendo arquivos de comando como pontos de entrada e fornecendo presets arquiteturais."
---

# Suporte ao oclif

O archlint oferece suporte integrado ao [oclif](https://oclif.io/), o Open CLI Framework.

## Recursos

- **Pontos de entrada da CLI**: Reconhece automaticamente arquivos de comando como pontos de entrada.
- **Detecção de hooks**: Identifica hooks do oclif para evitar falsos positivos na análise de código morto.
- **Regras arquiteturais**: Fornece presets que seguem a estrutura de diretórios recomendada pelo oclif.

## Configuração

Para ativar o suporte ao oclif, adicione-o à sua lista `extends`:

```yaml
extends:
  - oclif
```

## Lógica de detecção

O preset do oclif é detectado automaticamente se:

1. O `package.json` contiver `@oclif/core` nas dependências.
2. Houver um arquivo `oclif.manifest.json` no projeto.
