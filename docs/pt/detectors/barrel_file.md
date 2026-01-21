---
title: Abuso de Barrel Files
description: "Arquivos barrel (index.ts) podem causar dependências circulares indiretas e problemas de desempenho se crescerem muito."
---

# Abuso de Barrel Files (Barrel Abuse)

**ID:** `barrel_file` | **Gravidade:** Média (padrão)

Arquivos barrel (ex: arquivos `index.ts` que apenas reexportam outros arquivos) podem se tornar problemáticos quando crescem demais ou incluem muitas exportações não relacionadas.

## Por que isso é um smell

- **Dependências Circulares**: Arquivos barrel grandes são uma causa comum de dependências circulares indiretas.
- **Acoplamento Desnecessário**: Importar uma única coisa de um arquivo barrel grande pode fazer com que o bundler inclua muitos módulos não relacionados.
- **Performance**: Pode deixar mais lento tanto o desenvolvimento (indexação da IDE) quanto a produção (tamanho do bundle/tempo de carregamento).

## Configuração

```yaml
rules:
  barrel_file:
    severity: high
    max_reexports: 10
```

## Regra ESLint

Este detector está disponível como uma regra ESLint para feedback em tempo real no seu editor.

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-barrel-abuse': 'warn',
    },
  },
];
```

Veja [Integração ESLint](/pt/integrations/eslint) para instruções de configuração.

## Como corrigir

- Evite arquivos barrel "pega-tudo" na raiz de diretórios grandes.
- Prefira imports diretos se um arquivo barrel estiver causando problemas.
- Agrupe exportações em arquivos barrel menores e mais específicos.
