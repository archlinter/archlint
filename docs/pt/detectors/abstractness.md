# Violação de Abstração (Abstractness Violation)

**ID:** `abstractness` | **Gravidade:** Baixa (padrão)

Baseado nas métricas "Main Sequence" de Robert C. Martin. Mede o equilíbrio entre estabilidade (I) e abstração (A). Um módulo deve ser estável e abstrato, ou instável e concreto.

## Por que isso é um smell

Módulos que são estáveis e concretos estão na "Zona de Dor" (difíceis de mudar, mas outros dependem deles). Módulos que são instáveis e abstratos estão na "Zona de Inutilidade" (ninguém depende deles, mas são abstratos).

## Como corrigir

- **Na Zona de Dor**: Introduza abstrações (interfaces, classes abstratas) para desacoplar a implementação do módulo de seus usuários.
- **Na Zona de Inutilidade**: Considere tornar o módulo mais concreto ou remover abstrações não utilizadas para simplificar o código.

## Configuração

```yaml
rules:
  abstractness:
    severity: medium
```
