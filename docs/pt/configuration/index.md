---
title: Configuração
description: Aprenda como configurar o archlint usando .archlint.yaml, definir camadas arquiteturais e configurar regras para detectores.
---

# Configuração

O archlint pode ser configurado usando um arquivo `.archlint.yaml` na raiz do seu projeto. Se nenhum arquivo de configuração for encontrado, a ferramenta usará padrões sensatos para todos os detectores.

## Estrutura do Arquivo de Configuração

```yaml
# Arquivos e diretórios a serem ignorados (global)
ignore:
  - '**/dist/**'
  - '**/node_modules/**'

# Aliases de caminho (semelhante ao tsconfig.json ou webpack)
# Por padrão, o archlint carrega automaticamente os aliases do tsconfig.json.
# Aliases explícitos definidos aqui têm precedência sobre os valores derivados do tsconfig.json.
aliases:
  '@/*': 'src/*'

# Configurações de integração com TypeScript (true, false ou caminho do arquivo)
tsconfig: true

# Estender a partir de presets integrados ou personalizados
extends:
  - nestjs
  - ./my-company-preset.yaml

# Pontos de entrada para análise (usados para detecção de código morto)
entry_points:
  - 'src/main.ts'

# Configuração de regras para cada detector
rules:
  # Forma curta: nível de severidade ou "off"
  cycles: high
  dead_code: medium

  # Forma completa: com opções adicionais
  god_module:
    severity: high
    enabled: true
    exclude: ['**/generated/**']
    # Opções específicas do detector
    fan_in: 15
    fan_out: 15
    churn: 20

  vendor_coupling:
    severity: medium
    ignore_packages: ['lodash', 'rxjs']

# Substituições de regras para caminhos específicos
overrides:
  - files: ['**/legacy/**']
    rules:
      complexity: medium
      god_module: off

# Configuração de pontuação e graduação
scoring:
  # Nível mínimo de severidade para relatar (low, medium, high, critical)
  minimum: low
  # Pesos para o cálculo da pontuação total
  weights:
    critical: 100
    high: 50
    medium: 20
    low: 5
  # Limites para graduação (Densidade = Pontuação Total / Arquivos)
  grade_rules:
    excellent: 1.0
    good: 3.0
    fair: 7.0
    moderate: 15.0
    poor: 30.0

# Detecção automática de framework (padrão true)
auto_detect_framework: true

# Configurações do Git
git:
  enabled: true # habilitar análise (padrão true)
  history_period: '1y'
```

## Extends (Extensão)

O campo `extends` permite carregar presets de diferentes fontes:

- **Presets integrados**: `nestjs`, `nextjs`, `react`, `oclif`.
- **Arquivos locais**: Caminho relativo para um arquivo YAML (por exemplo, `./archlint-shared.yaml`).
- **URLs**: URL direta para um arquivo YAML (por exemplo, `https://example.com/preset.yaml`).

Os presets são mesclados na ordem em que são listados. A configuração do usuário sempre tem a maior prioridade.

## Regras e Níveis de Severidade

Na seção `rules`, você pode usar os seguintes níveis:

- `critical`: Problema crítico que requer atenção imediata.
- `high`: Erro arquitetural de alta severidade.
- `medium`: Aviso ou problema de severidade média.
- `low`: Mensagem informativa ou de baixa severidade.
- `off`: Desativa completamente o detector.

## Configuração via CLI

Você pode especificar o caminho do arquivo de configuração explicitamente:

```bash
archlint scan --config custom-config.yaml
```

## Integração com TypeScript

O archlint pode sincronizar automaticamente com seu `tsconfig.json`. Use o campo `tsconfig` para controlar isso:

- `tsconfig: true` (padrão): Busca automaticamente `tsconfig.json` na raiz do projeto.
- `tsconfig: false` ou `tsconfig: null`: Desativa a integração com TypeScript.
- `tsconfig: "./caminho/para/tsconfig.json"`: Utiliza um arquivo de configuração específico.

Quando habilitado, a ferramenta:

1. **Carrega Aliases**: Extrai `compilerOptions.paths` e `compilerOptions.baseUrl` para configurar automaticamente os `aliases`.
2. **Auto-ignorar**: Adiciona `compilerOptions.outDir` à lista global de `ignore`.
3. **Exclusões**: Incorpora padrões do campo `exclude` na lista de `ignore`.

A ferramenta procura pelo arquivo `tsconfig.json` na raiz do projeto. Se você tiver uma configuração personalizada, use o campo `tsconfig` para apontar para o arquivo correto.
