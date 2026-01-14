# Clone de Código

**ID:** `code_clone` | **Gravidade:** Média (padrão)

Este detector identifica blocos de código duplicados em seu projeto. Utiliza a tokenização baseada em AST para encontrar correspondências exatas, ignorando diferenças de formatação e comentários.

## Por que isto é um "cheiro"

- **Sobrecarga de manutenção**: Corrigir um bug ou fazer uma alteração em um lugar exige a atualização de todas as duplicatas.
- **Violação de DRY**: A duplicação é um sinal claro de que falta abstração ou reutilização.
- **Evolução inconsistente**: Com o tempo, as duplicatas podem divergir, levando a bugs sutis e dificultando a refatoração.

## Como corrigir

1. **Extract Method**: Mova a lógica compartilhada para uma única função e chame-a de vários lugares.
2. **Componentes Genéricos**: Para código de UI, crie um componente reutilizável com props.
3. **Módulos de Utilidade**: Mova a lógica de ajuda comum para um arquivo de utilidade compartilhado.

## Configuração

```yaml
rules:
  code_clone:
    enabled: true
    severity: medium
    min_tokens: 50
    min_lines: 6
```

### Opções

- `min_tokens`: O número mínimo de tokens normalizados para acionar a detecção de um clone (padrão: 50).
- `min_lines`: O número mínimo de linhas que o clone deve abranger (padrão: 6).

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-code-clone': 'warn',
    },
  },
];
```

Consulte [Integração com ESLint](/pt/integrations/eslint) para instruções de configuração.
