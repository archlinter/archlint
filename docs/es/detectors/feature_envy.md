# Envidia de Funcionalidades

**ID:** `feature_envy` | **Severidad:** Medium (default)

La envidia de funcionalidades es como ese vecino entrometido que sabe más sobre lo que pasa en tu casa que tú mismo. Sucede cuando un método parece mucho más interesado en los datos de otra clase que en los suyos propios.

## Por qué esto es un problema

Es una señal clásica de lógica mal ubicada. Si un método está constantemente hurgando en otro objeto para sacar datos y hacer cálculos, esa lógica probablemente pertenece dentro del otro objeto. Rompe el encapsulamiento y hace que tus clases estén fuertemente acopladas.

## Cómo solucionar

Mueva el método (o la parte del método que tiene la envidia) a la clase cuyos datos está utilizando.
