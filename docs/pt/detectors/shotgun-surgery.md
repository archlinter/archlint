# Cirurgia por Difusão (Shotgun Surgery)

**ID:** `shotgun_surgery` | **Severity:** Medium (default)

A cirurgia por difusão ocorre quando uma única mudança em seus requisitos exige que você faça muitas pequenas alterações em muitos módulos diferentes.

## Por que isso é um smell

Isso indica que as responsabilidades estão muito dispersas pela base de código. Torna as mudanças difíceis, demoradas e propensas a erros.

## Como corrigir

- **Mover Método/Campo (Move Method/Field)**: Consolide a lógica relacionada em um único módulo.
- **Classe Inline (Inline Class)**: Se uma classe é apenas uma coleção de métodos que são sempre usados junto com outra classe, combine-as.
