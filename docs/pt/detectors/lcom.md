# Baixa Coesão (LCOM4)

**ID:** `lcom` | **Gravidade:** Medium (default)

A coesão mede se os métodos e campos da sua classe realmente pertencem juntos. Se não pertencem, você provavelmente tem uma classe "monstro de Frankenstein".

## Por que isso é um smell

- **Violação do SRP**: Sua classe provavelmente está usando muitos chapéus e tentando fazer três trabalhos diferentes ao mesmo tempo.
- **Fragilidade**: Você muda um método relacionado a "avatares de usuário" e de alguma forma quebra a lógica de "hash de senha" porque eles compartilham a mesma classe inchada.
- **Difícil de Reutilizar**: Se você só precisa da lógica de "avatar", você é forçado a trazer toda a máquina de "senha" também.

## Como consertar

1. **Extract Class**: Divida a classe em duas ou mais classes menores, cada uma com uma única responsabilidade.
2. **Move Method**: Mova métodos que não usam o estado da classe para um local mais apropriado (por exemplo, um módulo de utilitários).

## Configuração

```yaml
rules:
  lcom:
    severity: medium
    max_lcom: 4
    min_methods: 3
```
