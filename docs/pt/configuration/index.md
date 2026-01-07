---
title: Configuração
description: Saiba como configurar o archlint usando archlint.yaml, definir camadas arquiteturais e definir limites personalizados para detectores.
---

# Configuração

O archlint pode ser configurado usando um arquivo `archlint.yaml` na raiz do seu projeto. Se nenhum arquivo de configuração for encontrado, a ferramenta usa padrões sensatos para todos os detectores.

## Estrutura do Arquivo de Configuração

```yaml
# Arquivos para ignorar
ignore:
  - '**/dist/**'

# Aliases de caminho (ex: do tsconfig.json)
aliases:
  '@/*': 'src/*'

# Pontos de entrada para análise de código morto
entry_points:
  - 'src/index.ts'

# Limites personalizados para detectores
thresholds:
  cycles:
    exclude_patterns: []
  god_module:
    fan_in: 15
    fan_out: 15

# Camadas arquiteturais
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: []

# Presets de frameworks
frameworks:
  - nestjs

# Sobrescritas de severidade
severity:
  cycles: critical
```

## Configuração via CLI

Você também pode especificar o caminho do arquivo de configuração via CLI:

```bash
archlint scan --config custom-config.yaml
```
