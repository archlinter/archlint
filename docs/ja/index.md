---
layout: home
title: Archlint — TypeScript & JavaScript 向けアーキテクチャリンター
description: "TypeScript/JavaScriptプロジェクト向けの高速でASTベースのアーキテクチャの問題検出器。28以上の検出器と驚異的な速さの分析でアーキテクチャの劣化を阻止します。"

hero:
  name: 'archlint'
  text: '私たちはあなたのアーキテクチャを修正しません。劣化を止めるだけです。'
  tagline: TypeScript/JavaScriptプロジェクト向けの高速でASTベースのアーキテクチャの問題検出器。
  image:
    src: /logo.svg
    alt: archlint logo
  actions:
    - theme: brand
      text: はじめに
      link: /ja/getting-started/
    - theme: alt
      text: GitHubで見る
      link: https://github.com/archlinter/archlint

features:
  - title: 28以上の検出器
    details: 循環依存からゴッドモジュール、レイヤー違反まで。Rustとoxcで構築され、最高のパフォーマンスを実現。
  - title: Diffモード
    details: 「ラチェット（継続的改善）」の哲学。現在の状態を固定し、新しいアーキテクチャの退行に対してのみアラートを出します。
  - title: フレームワーク認識
    details: NestJS、Next.js、React、oclifのプリセットを内蔵。フレームワークのアーキテクチャパターンを理解しています。
  - title: 驚異的な速さ
    details: 200以上のファイルを5秒以内に分析。並列処理とスマートなコンテンツベースのキャッシュ。
  - title: 実用的な洞察
    details: すべてのレポートには、重要度スコア、明確な説明、リファクタリングの推奨事項が含まれます。
  - title: 統合の準備
    details: ESLintプラグイン、GitHub Actions、GitLab CI、さらにはAIコーディングアシスタント用のMCPサーバーも提供。
---

## なぜ archlint なのか？

現代のコードベースは急速に複雑化します。archlintは、アーキテクチャの問題が技術的負債になる前に、早期に発見するのを助けます。

```bash
# PRで退行をキャッチ
npx -y @archlinter/cli diff HEAD~1 --explain
```

```
🔴 REGRESSION: New cycle detected

  src/orders/service.ts → src/payments/processor.ts → src/orders/service.ts

  Why this is bad:
    Circular dependencies create tight coupling between modules.
    Changes in one module can cause unexpected failures in the other.

  How to fix:
    Extract shared logic into a separate module, or use dependency injection.
```
