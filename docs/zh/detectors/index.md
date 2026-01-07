---
title: 检测器概述
description: 探索 `archlint` 中的 28+ 个架构异味检测器，包括循环依赖、图层违规、上帝模块等。
---

# 检测器概述

`archlint` 内置了 28+ 个检测器，根据它们识别的架构或代码质量问题的类型进行分类。

## 依赖问题

| 检测器                                       | ID                   | 描述                   | 默认 |
| -------------------------------------------- | -------------------- | ---------------------- | ---- |
| [循环依赖](/zh/detectors/cycles)             | `cycles`             | 文件之间的循环依赖     | ✅   |
| [类型循环](/zh/detectors/circular-type-deps) | `circular_type_deps` | 仅类型的循环依赖       | ❌   |
| [包循环](/zh/detectors/package-cycle)        | `package_cycles`     | 包之间的循环依赖       | ❌   |
| [图层违规](/zh/detectors/layer-violation)    | `layer_violation`    | 违反定义的架构分层     | ❌   |
| [SDP 违规](/zh/detectors/sdp-violation)      | `sdp_violation`      | 违反稳定依赖原则 (SDP) | ❌   |

## 模块与类设计

| 检测器                                     | ID                | 描述                             | 默认 |
| ------------------------------------------ | ----------------- | -------------------------------- | ---- |
| [上帝模块](/zh/detectors/god-module)       | `god_module`      | 承担过多职责的模块               | ✅   |
| [枢纽模块](/zh/detectors/hub-module)       | `hub_module`      | 高度连接的“枢纽”模块             | ❌   |
| [低内聚](/zh/detectors/lcom)               | `lcom`            | 内部内聚力较低的类 (LCOM4)       | ❌   |
| [高耦合](/zh/detectors/high-coupling)      | `high_coupling`   | 依赖项过多的模块                 | ❌   |
| [分散模块](/zh/detectors/scattered-module) | `module_cohesion` | 功能分散在过多的文件中           | ❌   |
| [特性嫉妒](/zh/detectors/feature-envy)     | `feature_envy`    | 使用其他类多于使用自身类的类方法 | ❌   |

## 代码质量与组织

| 检测器                                            | ID                    | 描述                             | 默认 |
| ------------------------------------------------- | --------------------- | -------------------------------- | ---- |
| [死代码](/zh/detectors/dead-code)                 | `dead_code`           | 未使用的导出                     | ✅   |
| [死符号](/zh/detectors/dead-symbols)              | `dead_symbols`        | 未使用的局部函数和变量           | ✅   |
| [孤立类型](/zh/detectors/orphan-types)            | `orphan_types`        | 未连接到代码库的类型             | ✅   |
| [Barrel 滥用](/zh/detectors/barrel-abuse)         | `barrel_file`         | 导致耦合的大型 Barrel 文件       | ✅   |
| [原始类型偏执](/zh/detectors/primitive-obsession) | `primitive_obsession` | 过度使用原始类型而非领域模型类型 | ❌   |

## 复杂度与大小

| 检测器                                 | ID             | 描述               | 默认 |
| -------------------------------------- | -------------- | ------------------ | ---- |
| [高复杂度](/zh/detectors/complexity)   | `complexity`   | 圈复杂度较高的函数 | ✅   |
| [深层嵌套](/zh/detectors/deep-nesting) | `deep_nesting` | 深层嵌套的代码块   | ✅   |
| [参数过多](/zh/detectors/long-params)  | `long_params`  | 参数过多的函数     | ✅   |
| [大文件](/zh/detectors/large-file)     | `large_file`   | 过大的源文件       | ✅   |

## 变更模式

| 检测器                                         | ID                   | 描述                   | 默认 |
| ---------------------------------------------- | -------------------- | ---------------------- | ---- |
| [霰弹式修改](/zh/detectors/shotgun-surgery)    | `shotgun_surgery`    | 更改时需要修改许多文件 | ❌   |
| [不稳定接口](/zh/detectors/unstable-interface) | `unstable_interface` | 频繁更改的公共接口     | ❌   |

## 运行时与安全

| 检测器                                             | ID                   | 描述                   | 默认 |
| -------------------------------------------------- | -------------------- | ---------------------- | ---- |
| [测试泄漏](/zh/detectors/test-leakage)             | `test_leakage`       | 测试代码泄漏到生产环境 | ❌   |
| [供应商耦合](/zh/detectors/vendor-coupling)        | `vendor_coupling`    | 与外部库的紧密耦合     | ❌   |
| [副作用导入](/zh/detectors/side-effect-import)     | `side_effect_import` | 触发副作用的导入       | ✅   |
| [共享可变状态](/zh/detectors/shared-mutable-state) | `shared_state`       | 导出的可变变量         | ❌   |

## 架构指标

| 检测器                                     | ID                 | 描述                   | 默认 |
| ------------------------------------------ | ------------------ | ---------------------- | ---- |
| [抽象性违规](/zh/detectors/abstractness)   | `abstractness`     | 痛苦/无用区 (I+A 指标) | ❌   |
| [分散配置](/zh/detectors/scattered-config) | `scattered_config` | 配置分散在多个文件中   | ❌   |
