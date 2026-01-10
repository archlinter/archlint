# Suporte a Frameworks

O archlint não é apenas um linter genérico; ele entende os padrões arquiteturais de frameworks populares e ajusta sua análise de acordo.

## Como funciona

O archlint detecta automaticamente quais frameworks são usados no seu projeto olhando para o `package.json` e as estruturas de arquivos. Você também pode carregar explicitamente presets no seu `.archlint.yaml`:

```yaml
extends:
  - nestjs
  - react
```

## Benefícios da Consciência de Framework

- **Redução de Falsos Positivos**: Alguns padrões que são smells em geral (como alto acoplamento) são necessários e esperados em certos contextos de framework (como módulos NestJS).
- **Pontos de Entrada Inteligentes**: Identifica automaticamente controllers, pages e hooks como pontos de entrada para análise de código morto.
- **Detectores Relevantes**: Desabilita detectores que não fazem sentido para um framework específico (como LCOM para componentes React).

## Frameworks Suportados

- [NestJS](/pt/frameworks/nestjs)
- [Next.js](/pt/frameworks/nextjs)
- [React](/pt/frameworks/react)
- [oclif](/pt/frameworks/oclif)

## Uso Avançado

- [Presets Personalizados](/pt/frameworks/custom-presets)
