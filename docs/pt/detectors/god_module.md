# Módulo Deus

**ID:** `god_module` | **Gravidade:** High (default)

Um "Módulo Deus" é aquele arquivo no seu projeto que todo mundo tem medo de mexer porque ele faz absolutamente tudo. Ele geralmente começa como um simples utilitário e acaba virando um monstro que cuida da autenticação, das consultas ao banco de dados e do estado da UI ao mesmo tempo.

## Por que isso é um smell

- **Pesadelo da Responsabilidade Única**: Quando um módulo faz de tudo, qualquer mudança — por menor que seja — parece um jogo de Jenga com a sua arquitetura.
- **Ímã de conflitos**: Como é o "centro do universo", todos os desenvolvedores do time estão mexendo nele constantemente, o que garante conflitos infinitos de merge.
- **Fragilidade**: Mudanças em um canto do módulo podem quebrar algo inesperadamente do outro lado, porque tudo está implicitamente conectado.
- **Dor de cabeça para testar**: Você não deveria ter que simular um banco de dados e um serviço de e-mail só para testar um simples formatador de texto.

## Critérios de Detecção

O `archlint` identifica Módulos Deus com base em:

- **Fan-in**: Número de outros módulos que dependem dele.
- **Fan-out**: Número de módulos dos quais ele depende.
- **Churn**: Frequência de mudanças no git.
- **Lines of Code**: Tamanho total do arquivo.

## Como corrigir

1. **Identificar Responsabilidades**: Liste todas as diferentes tarefas que o módulo executa.
2. **Extrair Módulos**: Divida o arquivo em módulos menores e mais focados.
3. **Padrão Facade**: Se o módulo atua como um coordenador, mantenha apenas a lógica de coordenação e delegue o trabalho para submódulos.

## Configuração

```yaml
rules:
  god_module:
    severity: high
    fan_in: 15
    fan_out: 15
    churn: 20
```

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-god-modules': 'warn',
    },
  },
];
```

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.
