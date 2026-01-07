# NestJSのサポート

archlintはNestJSのモジュール型アーキテクチャを理解し、それに特化した分析を提供します。

## 主な機能

- **モジュール分析**: `@Module` を調整点（coordination point）として認識し、それに対する結合ルールの制約を緩和します。
- **エントリポイント**: コントローラー（Controllers）とプロバイダー（Providers）をエントリポイントとして自動的にマークします。
- **レイヤーの強制**: NestJSスタイルのレイヤーアーキテクチャ（Controllers -> Services -> Repositories）と完璧に連動します。
- **LCOMの上書き**: 凝集度分析においてNestJSデコレータを無視し、実際のロジックに焦点を当てます。

## 推奨設定

```yaml
frameworks:
  - nestjs

layers:
  - name: presentation
    paths: ['**/*.controller.ts']
    can_import: ['application']

  - name: application
    paths: ['**/*.service.ts']
    can_import: ['domain']

  - name: domain
    paths: ['**/entities/**']
    can_import: []
```
