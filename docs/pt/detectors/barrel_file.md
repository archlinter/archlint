# Abuso de Barrel Files (Barrel Abuse)

**ID:** `barrel_file` | **Gravidade:** Média (padrão)

Arquivos barrel (como um `index.ts` que só reexporta tudo) são feitos para simplificar imports, mas frequentemente se transformam em um buraco negro arquitetural.

## Por que isso é um smell

- **Fábrica de dependência circular**: Barrels gigantes são a causa #1 daquelas dependências circulares indiretas irritantes que são impossíveis de rastrear.
- **Importar o mundo inteiro**: Quando você importa uma constante minúscula de um barrel massivo, o bundler geralmente acaba puxando cada módulo que aquele barrel referencia.
- **Te deixa mais lento**: Eles fazem a indexação da IDE rastejar e podem inflar seu bundle de produção se o tree-shaking não for perfeito.

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
