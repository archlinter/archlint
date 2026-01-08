# Camadas

A configuração de camadas permite definir níveis arquiteturais do seu projeto e impor regras de dependência entre eles.

## Definindo Camadas

As camadas são configuradas dentro da regra `layer_violation`. Cada definição de camada consiste em:

- `name`: Nome único da camada.
- `path` (ou `paths`): Padrão glob identificando os arquivos nesta camada.
- `allowed_imports` (ou `can_import`): Lista de nomes de camadas que esta camada tem permissão para importar.

## Exemplo: Arquitetura Limpa (Clean Architecture)

```yaml
rules:
  layer_violation:
    severity: error
    layers:
      - name: domain
        path: '**/domain/**'
        allowed_imports: [] # A camada Domain não deve depender de nada

      - name: application
        path: '**/application/**'
        allowed_imports:
          - domain

      - name: infrastructure
        path: '**/infrastructure/**'
        allowed_imports:
          - domain
          - application

      - name: presentation
        path: '**/presentation/**'
        allowed_imports:
          - domain
          - application
```

## Como Funciona

Quando o detector `layer_violation` está ativado:

1. Ele mapeia cada arquivo no seu projeto para uma camada específica com base no padrão `path`.
2. Se um arquivo corresponder a vários padrões, o mais específico (padrão mais longo) será escolhido.
3. A ferramenta verifica cada importação. Se um arquivo na camada `A` importar um arquivo na camada `B`, mas `B` não estiver na lista `allowed_imports` da camada `A`, uma violação será relatada.
