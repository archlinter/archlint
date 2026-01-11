# Importações com Efeito Colateral

**ID:** `side_effect_import` | **Gravidade:** Low (default)

Identifica importações que são realizadas apenas por seus efeitos colaterais (ex: `import './globals';`), que frequentemente modificam o estado global ou protótipos.

## Por que isso é um smell

Importações com efeito colateral tornam o gráfico de dependências menos explícito e podem levar a comportamentos não determinísticos dependendo da ordem de importação. Geralmente são dependências "ocultas" difíceis de rastrear.

## Padrões Excluídos

O archlint ignora automaticamente as seguintes importações com efeito colateral:

- **CSS/Assets**: `import './styles.css'`, `import './image.png'`, etc.
- **Importações Dinâmicas**: `import('./module')` ou `require('./module')` dentro de funções (geralmente usadas para lazy loading ou importações condicionais).

## Как исправить

Tente tornar a inicialização explícita. Em vez de depender de uma importação com efeito colateral, exporte uma função `init()` e chame-a explicitamente.
