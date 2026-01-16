---
title: Detectors Overview
description: Explore 28+ architecture smell detectors in archlint, including cyclic dependencies, layer violations, god modules, and more.
---

# Detectors Overview

archlint comes with 28+ built-in detectors categorized by the type of architectural or code quality issue they identify.

::: tip
**False Positives**: Architectural analysis can sometimes produce false positives, especially in projects with heavy dynamic loading, reflection, or complex Dependency Injection containers.
:::

## Dependency Issues

| Detector                                            | ID                   | Description                                | Default |
| --------------------------------------------------- | -------------------- | ------------------------------------------ | ------- |
| [Cyclic Dependencies](/detectors/cyclic_dependency) | `cyclic_dependency`  | Circular dependencies between files        | ✅      |
| [Cycle Clusters](/detectors/cycle_clusters)         | `cycle_clusters`     | Complex web of circular dependencies       | ✅      |
| [Type Cycles](/detectors/circular_type_deps)        | `circular_type_deps` | Type-only circular dependencies            | ❌      |
| [Package Cycles](/detectors/package_cycles)         | `package_cycles`     | Cyclic dependencies between packages       | ❌      |
| [Layer Violation](/detectors/layer_violation)       | `layer_violation`    | Violations of defined architectural layers | ❌      |
| [SDP Violation](/detectors/sdp_violation)           | `sdp_violation`      | Stable Dependencies Principle violations   | ❌      |

## Module & Class Design

| Detector                                       | ID                | Description                                           | Default |
| ---------------------------------------------- | ----------------- | ----------------------------------------------------- | ------- |
| [God Module](/detectors/god_module)            | `god_module`      | Modules with too many responsibilities                | ✅      |
| [Hub Module](/detectors/hub_module)            | `hub_module`      | Highly connected "hub" modules                        | ❌      |
| [Low Cohesion](/detectors/lcom)                | `lcom`            | Classes with low internal cohesion (LCOM4)            | ❌      |
| [High Coupling](/detectors/high_coupling)      | `high_coupling`   | Modules with too many dependencies                    | ❌      |
| [Scattered Module](/detectors/module_cohesion) | `module_cohesion` | Functionality scattered across too many files         | ❌      |
| [Feature Envy](/detectors/feature_envy)        | `feature_envy`    | Methods that use more of another class than their own | ❌      |

## Code Quality & Organization

| Detector                                              | ID                    | Description                                   | Default |
| ----------------------------------------------------- | --------------------- | --------------------------------------------- | ------- |
| [Dead Code](/detectors/dead_code)                     | `dead_code`           | Unused exports                                | ✅      |
| [Dead Symbols](/detectors/dead_symbols)               | `dead_symbols`        | Unused local functions and variables          | ✅      |
| [Orphan Types](/detectors/orphan_types)               | `orphan_types`        | Types not connected to the codebase           | ✅      |
| [Barrel Abuse](/detectors/barrel_file)                | `barrel_file`         | Large barrel files causing coupling           | ✅      |
| [Primitive Obsession](/detectors/primitive_obsession) | `primitive_obsession` | Overuse of primitives instead of domain types | ❌      |

## Complexity & Size

| Detector                                                  | ID                      | Description                               | Default |
| --------------------------------------------------------- | ----------------------- | ----------------------------------------- | ------- |
| [Cyclomatic Complexity](/detectors/cyclomatic_complexity) | `cyclomatic_complexity` | Functions with high cyclomatic complexity | ✅      |
| [Cognitive Complexity](/detectors/cognitive_complexity)   | `cognitive_complexity`  | Functions with high cognitive complexity  | ✅      |
| [Deep Nesting](/detectors/deep_nesting)                   | `deep_nesting`          | Deeply nested code blocks                 | ✅      |
| [Long Parameters](/detectors/long_params)                 | `long_params`           | Functions with too many parameters        | ✅      |
| [Large File](/detectors/large_file)                       | `large_file`            | Source files that are too large           | ✅      |

## Change Patterns

| Detector                                            | ID                   | Description                                  | Default |
| --------------------------------------------------- | -------------------- | -------------------------------------------- | ------- |
| [Shotgun Surgery](/detectors/shotgun_surgery)       | `shotgun_surgery`    | Changes requiring modification in many files | ❌      |
| [Unstable Interface](/detectors/unstable_interface) | `unstable_interface` | Frequently changing public interfaces        | ❌      |

## Runtime & Safety

| Detector                                                | ID                     | Description                          | Default |
| ------------------------------------------------------- | ---------------------- | ------------------------------------ | ------- |
| [Test Leakage](/detectors/test_leakage)                 | `test_leakage`         | Test code leaking into production    | ❌      |
| [Vendor Coupling](/detectors/vendor_coupling)           | `vendor_coupling`      | Tight coupling to external libraries | ❌      |
| [Hub Dependency](/detectors/hub_dependency)             | `hub_dependency`       | Over-reliance on external packages   | ❌      |
| [Side Effect Import](/detectors/side_effect_import)     | `side_effect_import`   | Imports that trigger side effects    | ✅      |
| [Shared Mutable State](/detectors/shared_mutable_state) | `shared_mutable_state` | Exported mutable variables           | ❌      |

## Architectural Metrics

| Detector                                          | ID                 | Description                            | Default |
| ------------------------------------------------- | ------------------ | -------------------------------------- | ------- |
| [Abstractness Violation](/detectors/abstractness) | `abstractness`     | Pain/Useless zones (I+A metric)        | ❌      |
| [Scattered Config](/detectors/scattered_config)   | `scattered_config` | Configuration spread across many files | ❌      |
| [Code Clone](/detectors/code_clone)               | `code_clone`       | Duplicated code across the project     | ✅      |
