---
title: Violación de Abstracción
description: 'Detecta módulos que son demasiado concretos y estables (difíciles de cambiar) o demasiado abstractos e inestables (sobreingeniería), violando el Principio de Abstracciones Estables.'
---

# Violación de Abstracción (Abstractness Violation)

**ID:** `abstractness` | **Severidad:** Media (por defecto)

## Qué detecta esta regla (TL;DR)

Esta regla marca módulos que son:

- **Demasiado concretos y demasiado estables** — muchos archivos dependen de una clase concreta (difícil de cambiar de forma segura).
- **Demasiado abstractos y demasiado inestables** — abstracciones de las que nadie depende (sobreingeniería/YAGNI).

En resumen:  
👉 **Los módulos estables** (la base) deben ser **abstractos**.  
👉 **Los módulos inestables** (las hojas) deben ser **concretos**.

---

## ¿Qué es un "Módulo"?

En `archlint`, un **módulo** es un único archivo fuente (`.ts`, `.tsx`, etc.) que exporta al menos un símbolo.

- Cada archivo se analiza de forma independiente.
- Los archivos barrel (`index.ts`) se tratan como módulos de agregación.
- Las reexportaciones e importaciones se cuentan como dependencias entre módulos.

---

## Métricas e Intuición

### 1. Inestabilidad (I)

Mide qué tan propenso es un módulo a cambiar.

`I = Acoplamiento Eferente (Ce) / (Acoplamiento Aferente (Ca) + Acoplamiento Eferente (Ce))`

**Intuición**:

- **Estable (I ≈ 0)**: Muchos archivos te importan, pero tú casi no importas a nadie. Eres un componente fundamental.
- **Inestable (I ≈ 1)**: Importas muchas cosas, pero nadie te importa. Estás en el borde del sistema.

### 2. Abstracción (A)

Utilizamos un **Cálculo Semántico** basado en el uso real:

`A = (Clientes que importan solo Interfaces/Tipos) / (Total de Clientes)`

**Diferencia importante respecto al A clásico**:
**NO** solo contamos palabras clave o interfaces dentro del archivo. En su lugar, medimos **cómo se usa realmente el módulo**:

- Importar una `class` concreta → **dependencia concreta**.
- Importar una `interface` o `type` → **dependencia abstracta**.

Esto refleja el _acoplamiento arquitectónico real_, no solo la sintaxis. Usar `import type` es una señal fuerte de intención abstracta.

### 3. Distancia (D)

La distancia desde la línea ideal de "Secuencia Principal" donde `A + I = 1`.

`D = |A + I - 1|`

---

## Zonas de Riesgo (Interpretación)

Según los valores de **A** e **I**, los módulos caen en zonas específicas:

### 🧱 Zona de Dolor

- **Métricas**: I ≈ 0–0.3 (estable), A ≈ 0–0.3 (concreto).
- **Problema**: Todos dependen de una implementación concreta. Cambiarla es peligroso porque es tanto rígida como altamente acoplada.

**Ejemplo malo (Dependencia concreta):**

```typescript
// database.service.ts
export class DatabaseService {
  save(data: any) {
    /* lógica concreta */
  }
}

// client.ts (100+ archivos haciendo esto)
import { DatabaseService } from './database.service'; // Importación directa de clase
const db = new DatabaseService();
```

**Por qué se marca**:

- `Ca` = 100+ (muy estable, `I ≈ 0`).
- `A` = 0 (los clientes importan la clase directamente).
- `D` ≈ 1 → Distancia máxima de la secuencia principal.

### 💨 Zona de Inutilidad

- **Métricas**: I ≈ 0.7–1.0 (inestable), A ≈ 0.7–1.0 (abstracto).
- **Problema**: Abstracciones sobreingeniadas que nadie usa.

**Ejemplo:**

```typescript
// complex-plugin.interface.ts
export interface IComplexPlugin {
  execute(context: any): Promise<void>;
}
// 0 implementaciones y 0 clientes usando esta interfaz.
```

**Por qué se marca**:

- `I` ≈ 1 (nadie depende de él).
- `A` = 1 (es puramente abstracto).
- `D` ≈ 1 → La abstracción existe sin propósito.

---

## Heurísticas para Reducir Falsos Positivos

El análisis estático puede ser ruidoso. Estas heurísticas enfocan la regla en **decisiones arquitectónicas**, no en código incidental:

1.  **Umbral de Estabilidad (Fan-in)**: Solo se analizan módulos con al menos `fan_in_threshold` (por defecto: 10) dependientes. Si solo unos pocos archivos usan un módulo, su impacto arquitectónico es bajo.
2.  **DTOs y Entidades**: Las clases sin métodos (solo datos) se ignoran. Son **transportadores de datos**, no componentes de comportamiento.
3.  **Errores**: Las clases que extienden `Error` se ignoran. Son **siempre concretas por diseño**.
4.  **Scripts de Infraestructura**: Las migraciones de base de datos (`up`/`down`) se ignoran ya que son **scripts procedimentales**, no parte de la arquitectura a largo plazo.

---

## Cómo Corregir (Guía de Decisión)

1. **¿El módulo es estable (tiene muchos dependientes)?**
   - **Sí**: Extrae una `interface`. Asegúrate de que los clientes usen `import type { ... }`. Usa Inyección de Dependencias.
   - **No**: Las abstracciones podrían ser innecesarias. Manténlo concreto hasta que la estabilidad aumente.

2. **¿Hay más de una implementación?**
   - **No**: Si es inestable, considera eliminar la interfaz (YAGNI).
   - **Sí**: La interfaz está justificada, pero asegúrate de que los clientes dependan de la interfaz, no de las clases.

---

## Configuración

```yaml
rules:
  abstractness:
    severity: medium
    distance_threshold: 0.85 # Umbral de activación para la distancia D
    fan_in_threshold: 10 # Mínimo de dependencias entrantes (Fan-in) para activar el análisis
```
