# NestJSのサポート

archlintはNestJSのモジュール型アーキテクチャを理解し、それに特化した分析を提供します。

## 主な機能

- **モジュール分析**: `@Module` を調整点（coordination point）として認識し、それに対する結合ルールの制約を緩和します。
- **エントリポイント**: コントローラー（Controllers）とプロバイダー（Providers）をエントリポイントとして自動的にマークします。
- **レイヤーの強制**: NestJSスタイルのレイヤーアーキテクチャ（Controllers -> Services -> Repositories）と完璧に連動します。
- **LCOMの上書き**: 凝集度分析においてNestJSデコレータを無視し、実際のロジックに焦点を当てます。

## 推奨設定

```yaml
extends:
  - nestjs

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
