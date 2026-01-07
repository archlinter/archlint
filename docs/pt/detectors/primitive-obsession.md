# Obsessão por Primitivos

**ID:** `primitive_obsession` | **Severity:** Low (default)

A obsessão por primitivos é o uso excessivo de tipos primitivos (strings, números, booleanos) para representar conceitos de domínio que poderiam ser melhor representados por um tipo ou classe específica (por exemplo, usar uma `string` para um endereço de e-mail ou um `number` para uma moeda).

## Por que isso é um "smell"

Os primitivos não têm comportamento ou validação. Ao usar um tipo específico de domínio, você pode encapsular a lógica de validação e tornar o código mais autodocumentado.

## Como consertar

Crie uma classe ou um alias de tipo (no TypeScript) com lógica de validação para o conceito de domínio.
