---
title: Presets de Framework
description: "Aprenda como criar presets personalizados baseados em YAML para entender padrões específicos de frameworks e reduzir falsos positivos na análise do archlint."
---

# Presets de Framework

O archlint usa presets baseados em YAML para entender padrões específicos de frameworks e reduzir falsos positivos.

## Como funciona

O archlint detecta automaticamente frameworks analisando dependências em `package.json` e arquivos de configuração. Você também pode estender explicitamente presets no seu `.archlint.yaml`:

```yaml
extends:
  - nestjs
  - ./my-company-preset.yaml
```

## Presets Integrados

- **nestjs**: Para aplicações NestJS.
- **nextjs**: Para projetos Next.js.
- **react**: Para bibliotecas e aplicações React.
- **oclif**: Para ferramentas CLI construídas com oclif.

## Presets Personalizados

Um arquivo preset é um arquivo de configuração padrão do archlint com uma seção adicional `detect` para autodescoberta.

### Estrutura

```yaml
name: my-framework
version: 1

# Regras para autodetecção
detect:
  packages:
    any_of: ['my-core-pkg']
  files:
    any_of: ['my-framework.config.js']

# Regras globais
rules:
  layer_violation: high
  dead_symbols:
    ignore_methods: ['onInit', 'onDestroy']
  vendor_coupling:
    ignore_packages: ['my-framework/*']

# Substituições específicas de caminho
overrides:
  - files: ['**/*.controller.ts']
    rules:
      lcom: off

# Padrões para análise de código morto
entry_points:
  - '**/*.controller.ts'
```

### Carregando Presets Personalizados

Você pode carregar presets de arquivos locais ou URLs:

```yaml
extends:
  - ./presets/shared.yaml
  - https://raw.githubusercontent.com/org/archlint-presets/main/standard.yaml
```

## Lógica de Mesclagem

Os presets são mesclados na ordem em que são especificados. A prioridade é:

1. Configuração do usuário em `.archlint.yaml` (maior)
2. Presets na lista `extends`
3. Presets detectados automaticamente
4. Configurações padrão do archlint (menor)

Para configurações baseadas em listas (como `entry_points` ou `ignore_packages` dentro de regras), o archlint realiza uma união de todos os valores. Regras e substituições são mescladas recursivamente.
