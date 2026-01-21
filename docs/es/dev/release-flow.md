---
title: Flujo de Lanzamiento
description: "Este documento describe el proceso de lanzamiento de archlint."
---

# Flujo de Lanzamiento (Release Flow)

Este documento describe el proceso de lanzamiento de archlint.

## Descripción General

archlint utiliza **semantic-release** para automatizar todo el flujo de trabajo de lanzamiento. Los números de versión se calculan basándose en los mensajes de commit siguiendo el formato de Conventional Commits.

## Formato del Mensaje de Commit

Todos los commits **deben** seguir el formato de Conventional Commits. Esto es verificado por commitlint en la CI.

### Formato

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Tipos (Types)

| Tipo       | Descripción               | Incremento de Versión |
| ---------- | ------------------------- | --------------------- |
| `feat`     | Nueva característica      | **Minor** (0.x.0)     |
| `fix`      | Corrección de errores     | **Patch** (0.0.x)     |
| `perf`     | Mejora de rendimiento     | **Patch** (0.0.x)     |
| `refactor` | Refactorización de código | Ninguno               |
| `docs`     | Documentación             | Ninguno               |
| `test`     | Pruebas                   | Ninguno               |
| `chore`    | Mantenimiento             | Ninguno               |
| `ci`       | Cambios en CI/CD          | Ninguno               |
| `build`    | Sistema de construcción   | Ninguno               |

### Breaking Changes (Cambios Disruptivos)

Añade `!` después del tipo o `BREAKING CHANGE:` en el pie de página (footer) para activar un incremento de versión **major**:

```bash
# Incremento de versión major (1.0.0)
git commit -m "feat!: change API signature"

# O
git commit -m "feat: new feature

BREAKING CHANGE: This changes the public API"
```

## Proceso de Lanzamiento

### 1. Desarrollo

Desarrolla características en ramas de funcionalidad (feature branches) y fusiónalas en `main`.

### Ramas de Prerelease

El archivo `.releaserc.json` contiene configuraciones estáticas de ramas para los canales `beta` y `alpha`. Sin embargo, **las ramas de prerelease se configuran dinámicamente mediante la CI** durante el flujo de trabajo de lanzamiento. El flujo de trabajo crea automáticamente configuraciones de rama basadas en el canal seleccionado y el nombre de la rama actual, por lo que las entradas estáticas en `.releaserc.json` no se utilizan durante los lanzamientos reales.

### 2. Activar Lanzamiento

Cuando estés listo para lanzar, activa manualmente el flujo de trabajo de Release:

1. Ve a **Actions** -> flujo de trabajo de **Release**.
2. Haz clic en **Run workflow**.
3. (Opcional) Establece `dry_run` a `true` para ver qué sucedería sin publicar realmente.

### 3. Pasos Automáticos

El flujo de trabajo realizará lo siguiente:

1. **Calcular versión**: `semantic-release` analiza los commits desde el último lanzamiento.
2. **Actualizar archivos**: Actualiza automáticamente `Cargo.toml`, `package.json` y `CHANGELOG.md`.
3. **Commit y tag**: Crea un nuevo commit y una etiqueta (tag) de Git para el lanzamiento.
4. **Activar CI**: El envío del tag activa el flujo de trabajo de CI, que construye todos los binarios.
5. **Publicar en npm**: La CI publica todos los paquetes en el registro de npm (solo en etiquetas).
6. **Adjuntar binarios**: La CI sube los binarios independientes a la Release de GitHub.

## Números de Versión

Todos los paquetes comparten la misma versión (versión unificada):

- `@archlinter/cli@0.2.0`
- `@archlinter/cli-darwin-arm64@0.2.0`
- `@archlinter/cli-linux-x64@0.2.0`
- etc.

## Comprobación del Estado del Lanzamiento

### Ver Estado del Flujo de Trabajo

https://github.com/archlinter/archlint/actions

### Verificar Publicación en npm

```bash
npm view @archlinter/cli
```

### Probar Instalación

```bash
npx @archlinter/cli@latest --version
```

## Solución de Problemas

### Commit Rechazado por commitlint

**Solución**: Sigue el formato de conventional commits:

```bash
git commit --amend -m "feat: correct commit message"
```

### Fallo en el Flujo de Trabajo de Lanzamiento

Comprueba:

1. ¿Está configurado el secreto NPM_TOKEN?
2. ¿Está configurado el secreto GH_PAT?
3. ¿Falló la construcción en CI?

## Referencias

- [Conventional Commits](https://www.conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [semantic-release](https://github.com/semantic-release/semantic-release)
