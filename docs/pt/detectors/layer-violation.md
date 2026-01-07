# Violação de Camada

**ID:** `layer_violation` | **Severity:** High (default)

A violação de camada (Layer violation) ocorre quando o código em uma camada arquitetural importa código de uma camada que não deveria conhecer (por exemplo, a camada Domain importando da Infrastructure).

## Por que isso é um "smell"

- **Quebra a Abstração**: Detalhes de implementação interna vazam para a lógica de negócio de alto nível.
- **Dificuldade de Teste**: A lógica de negócio torna-se difícil de testar sem mocks para a infraestrutura (BD, API, etc.).
- **Rigidez**: Alterar um banco de dados ou biblioteca externa requer a alteração da lógica de negócio principal.

## Configuração

Você deve definir suas camadas em `.archlint.yaml`:

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # Domain não importa nada

  - name: application
    paths: ['**/application/**']
    can_import: ['domain']

  - name: infrastructure
    paths: ['**/infrastructure/**']
    can_import: ['domain', 'application']
```

## Como corrigir

1. **Inversão de Dependência**: Defina uma interface na camada superior (Domain) e implemente-a na camada inferior (Infrastructure).
2. **Refatorar**: Mova o código mal posicionado para a camada apropriada.
