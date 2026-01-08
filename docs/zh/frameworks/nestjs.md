# NestJS 支持

archlint 理解 NestJS 的模块化架构，并为其提供专门的分析。

## 主要特性

- **模块分析**：识别 `@Module` 作为协调点，并放宽对其耦合规则的限制。
- **入口点**：自动将控制器（Controllers）和提供者（Providers）标记为入口点。
- **架构层强制执行**：完美适用于 NestJS 风格的层级架构（Controllers -> Services -> Repositories）。
- **LCOM 覆盖**：在内聚性分析中忽略 NestJS 装饰器，以专注于实际逻辑。

## 推荐配置

```yaml
framework: nestjs

rules:
  layer_violation:
    layers:
  - name: presentation
    path: ['**/*.controller.ts']
    allowed_imports: ['application']

  - name: application
    path: ['**/*.service.ts']
    allowed_imports: ['domain']

  - name: domain
    path: ['**/entities/**']
    allowed_imports: []
```
