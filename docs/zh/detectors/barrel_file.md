---
title: Barrel 文件滥用
description: "Barrel 文件（index.ts）如果变得过大，可能会导致间接循环依赖和性能问题。"
---

# Barrel 文件滥用 (Barrel Abuse)

**ID:** `barrel_file` | **严重程度:** 中 (默认)

Barrel 文件（例如仅重新导出其他文件的 `index.ts` 文件）在变得过大或包含过多不相关的导出时可能会产生问题。

## 为什么这是一种坏味道

- **循环依赖**: 大型 Barrel 文件是间接循环依赖的常见原因。
- **不必要的耦合**: 从一个大型 Barrel 文件导入一个内容可能会导致打包工具引入许多不相关的模块。
- **性能**: 可能会降低开发效率（IDE 索引）和生产环境性能（包大小/加载时间）。

## 配置

```yaml
rules:
  barrel_file:
    severity: high
    max_reexports: 10
```

## ESLint 规则

此检测器可作为 ESLint 规则使用，以便在编辑器中获得实时反馈。

```javascript
// eslint.config.js
export default [
  {
    rules: {
      '@archlinter/no-barrel-abuse': 'warn',
    },
  },
];
```

详见 [ESLint 集成](/zh/integrations/eslint) 了解设置说明。

## 如何修复

- 避免在大型目录的根部使用"包罗万象"的 Barrel 文件。
- 如果 Barrel 文件导致问题，请优先使用直接导入。
- 将导出分组到更小、更具体的 Barrel 文件中。
