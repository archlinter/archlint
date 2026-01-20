# Interface Instável

**ID:** `unstable_interface` | **Gravidade:** Medium (default)

Identifica módulos que são um "alvo móvel"—mudam sua API pública constantemente enquanto todo mundo está tentando construir em cima deles.

## Por que isso é um smell

- **O efeito dominó**: Toda vez que você muda um export público em um módulo instável, você está potencialmente quebrando uma dúzia de outros arquivos que dependem dele.
- **Trabalho desnecessário**: Desenvolvedores passam mais tempo consertando imports e se ajustando a mudanças de API do que construindo features.
- **Frustração**: É difícil confiar em um módulo que quebra suas promessas a cada duas semanas.

## Como corrigir

- **Estabilize a API**: Gaste mais tempo projetando a interface antes da implementação.
- **Use Versionamento**: Se possível, suporte múltiplas versões da interface simultaneamente durante uma transição.
- **Restrinja a Interface**: Exporte apenas o que for absolutamente necessário.
