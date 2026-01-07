# Camadas (Layers)

A configuração de `layers` permite definir as camadas arquiteturais do seu projeto e impor as regras de dependência entre elas.

## Definindo Camadas

Cada definição de camada consiste em:

- `name`: Um identificador único para a camada.
- `paths`: Um array de padrões glob que identificam os arquivos nesta camada.
- `can_import`: Um array de nomes de camadas das quais esta camada pode depender.

## Exemplo: Clean Architecture

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # A camada Domain deve ser independente

  - name: application
    paths: ['**/application/**', '**/use-cases/**']
    can_import:
      - domain

  - name: infrastructure
    paths: ['**/infrastructure/**', '**/adapters/**']
    can_import:
      - domain
      - application

  - name: presentation
    paths: ['**/controllers/**', '**/api/**', '**/ui/**']
    can_import:
      - domain
      - application
```

## Como funciona

Quando o detector `layer_violation` está habilitado:

1. Ele atribui cada arquivo no seu projeto a uma camada com base nos padrões `paths`.
2. Ele verifica cada import nesses arquivos.
3. Se um arquivo na camada `A` importa um arquivo na camada `B`, mas `B` não está na lista `can_import` de `A`, uma violação é relatada.
