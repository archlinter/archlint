# Dependência Hub

**ID:** `hub_dependency` | **Gravidade:** Média (padrão)

Identifica pacotes externos que são importados por muitos arquivos no seu projeto, criando um ponto central de falha.

## Por que isso é um smell

Quando seu projeto depende muito de uma única biblioteca externa, torna-se difícil substituir ou atualizar essa biblioteca. Também sugere que você pode estar vazando detalhes de infraestrutura na lógica do seu aplicativo.

## Configuração

```yaml
rules:
  hub_dependency:
    severity: medium
    min_dependents: 20
    ignore_packages:
      - 'react'
      - 'lodash'
      - 'typescript'
```

### Opções

- `min_dependents` (padrão: 20): O número mínimo de arquivos importando um pacote para acionar este smell.
- `ignore_packages`: Uma lista de nomes de pacotes ou padrões glob para ignorar.

## Como corrigir

Identifique por que o pacote é usado tão amplamente. Se for uma biblioteca utilitária como `lodash`, considere se você realmente precisa de todas essas importações ou se pode usar recursos nativos da linguagem. Para bibliotecas de infraestrutura, use o **Padrão Adapter** para isolar a dependência.
