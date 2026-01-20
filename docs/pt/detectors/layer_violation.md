# Violação de Camada

**ID:** `layer_violation` | **Gravidade:** High (default)

A violação de camada acontece quando sua "arquitetura limpa" começa a ter goteiras. É quando sua lógica de negócio de alto nível (Domain) começa a perguntar sobre tabelas de banco de dados ou endpoints de API (Infrastructure).

## Por que isso é um smell

- **Abstrações com goteiras**: Sua lógica de negócio não deveria se importar se você usa Postgres ou um arquivo JSON. Quando as camadas vazam, você perde essa liberdade.
- **Testes frágeis**: Você não deveria precisar levantar um mock de banco de dados só para testar uma simples regra de negócio.
- **Fricção ao mudar**: Quer trocar sua biblioteca de logging? Azar o seu, você a importou diretamente no núcleo do seu domínio e agora tem que refatorar tudo.

## Configuração

Você deve definir suas camadas em `.archlint.yaml`:

```yaml
rules:
  layer_violation:
    layers:
  - name: domain
    path: ['**/domain/**']
    allowed_imports: [] # Domain não importa nada

  - name: application
    path: ['**/application/**']
    allowed_imports: ['domain']

  - name: infrastructure
    path: ['**/infrastructure/**']
    allowed_imports: ['domain', 'application']
```

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-layer-violations': 'error',
    },
  },
];
```

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.

## Como corrigir

1. **Inversão de Dependência**: Defina uma interface na camada superior (Domain) e implemente-a na camada inferior (Infrastructure).
2. **Refatorar**: Mova o código mal posicionado para a camada apropriada.
