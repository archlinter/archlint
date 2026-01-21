---
title: フレームワークのサポート
description: "archlintがNestJS、Next.js、Reactなどの人気フレームワークのアーキテクチャパターンを理解し、それに応じて分析を調整する方法を学びます。"
---

# フレームワークのサポート

archlintは単なる汎用リンターではありません。人気のあるフレームワークのアーキテクチャパターンを理解し、それに応じて分析を調整します。

## 仕組み

archlintは、`package.json`やファイル構造を確認することで、プロジェクトで使用されているフレームワークを自動的に検出します。また、`.archlint.yaml`で明示的にプリセットを読み込むこともできます。

```yaml
extends:
  - nestjs
  - react
```

## フレームワーク対応のメリット

- **偽陽性（False Positives）の削減**: 一般的には不吉なにおいとされるパターン（結合度が高いなど）でも、特定のフレームワークのコンテキスト（NestJSモジュールなど）では必要かつ期待される場合があります。
- **スマートなエントリポイント**: デッドコード分析のために、コントローラー、ページ、フックをエントリポイントとして自動的に識別します。
- **関連する検出器**: 特定のフレームワークでは意味をなさない検出器（Reactコンポーネントに対するLCOMなど）を無効にします。

## サポートされているフレームワーク

- [NestJS](/ja/frameworks/nestjs)
- [Next.js](/ja/frameworks/nextjs)
- [React](/ja/frameworks/react)
- [oclif](/ja/frameworks/oclif)

## 高度な使用方法

- [カスタムプリセット](/ja/frameworks/custom-presets)
