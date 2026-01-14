# Envidia de Funcionalidades

**ID:** `feature_envy` | **Severidad:** Medium (default)

La envidia de funcionalidades (Feature envy) ocurre cuando un método en una clase parece más interesado en los datos de otra clase que en los datos de su propia clase.

## Por qué esto es un problema

Indica una violación de la encapsulación. Es probable que la lógica esté en el lugar equivocado.

## Cómo solucionar

Mueva el método (o la parte del método que tiene la envidia) a la clase cuyos datos está utilizando.
