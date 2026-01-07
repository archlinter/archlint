# 框架支持

archlint 不仅仅是一个通用的 linter；它理解流行框架的架构模式，并据此调整其分析。

## 工作原理

archlint 通过查看 `package.json` 和文件结构，自动检测您的项目中使用了哪些框架。您还可以在 `archlint.yaml` 中显式设置或覆盖此设置：

```yaml
frameworks:
  - nestjs
  - react
```

## 框架感知的优势

- **减少误报**：某些在通常情况下被视为坏味道（如高耦合）的模式，在特定的框架上下文（如 NestJS 模块）中是必要且符合预期的。
- **智能入口点**：自动将控制器、页面和 hook 识别为死代码分析的入口点。
- **相关探测器**：禁用对特定框架没有意义的探测器（如 React 组件的 LCOM 探测）。

## 支持的框架

- [NestJS](/zh/frameworks/nestjs)
- [Next.js](/zh/frameworks/nextjs)
- [React](/zh/frameworks/react)
