# Acoplamento de Fornecedor (Vendor Coupling)

**ID:** `vendor_coupling` | **Severity:** Medium (default)

Identifica módulos que estão muito fortemente acoplados a uma biblioteca ou framework externo específico.

## Por que isso é um smell

Se você decidir trocar a biblioteca no futuro, terá que mudar o código em muitos lugares. Também torna os testes mais difíceis porque você tem que simular (mock) a biblioteca externa em todos os lugares.

## Como corrigir

Use o **Padrão Adapter**. Crie uma interface em seu domínio e implemente-a usando a biblioteca externa. O restante do seu código deve depender apenas da sua interface.
