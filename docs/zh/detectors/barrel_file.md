# Barrel 文件滥用 (Barrel Abuse)

**ID:** `barrel_file` | **严重程度:** 中 (默认)

Barrel 文件（比如一个简单地重新导出所有东西的 `index.ts`）本意是为了简化导入，但它们往往会变成架构黑洞。

## 为什么这是一种坏味道

- **循环依赖制造工厂**: 巨大的 barrel 文件是那些烦人的、无法追踪的间接循环依赖的头号原因。
- **导入整个世界**: 当你从一个巨大的 barrel 文件中导入一个微小的常量时，打包工具通常会把该 barrel 引用的每个模块都拉进来。
- **拖慢速度**: 它们让 IDE 索引变慢，如果 tree-shaking 不完美，还会让你的生产包膨胀。

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
