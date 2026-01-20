# Cohesión Baja (LCOM4)

**ID:** `lcom` | **Severidad:** Medium (default)

La cohesión mide si los métodos y campos de tu clase realmente pertenecen juntos. Si no lo hacen, probablemente tienes una clase "Frankenstein".

## Por qué esto es un problema

- **Violación del SRP**: Tu clase probablemente está usando demasiados sombreros e intentando hacer tres trabajos diferentes a la vez.
- **Fragilidad**: Cambias un método relacionado con "avatares de usuario" y de alguna manera rompes la lógica de "hashing de contraseñas" porque comparten la misma clase inflada.
- **Difícil de Reutilizar**: Si solo necesitas la lógica de "avatar", estás obligado a llevar también toda la maquinaria de "contraseñas".

## Cómo solucionarlo

1. **Extract Class**: Divide la clase en dos o más clases más pequeñas, cada una con una única responsabilidad.
2. **Move Method**: Mueve los métodos que no utilizan el estado de la clase a una ubicación más apropiada (por ejemplo, un módulo de utilidades).

## Configuración

```yaml
rules:
  lcom:
    severity: medium
    max_lcom: 4
    min_methods: 3
```
