---
title: 検出器の概要
description: 循環依存、レイヤー違反、ゴッドモジュールなど、`archlint`にある28以上のアーキテクチャ・スメル検出器を探索しましょう。
---

# 検出器の概要

`archlint`には、特定されるアーキテクチャやコード品質の問題の種類ごとに分類された、28以上の組み込み検出器が備わっています。

::: tip
**誤検知**: アーキテクチャ分析は、特に動的読み込み、リフレクション、または複雑な依存関係注入（DI）コンテナを使用しているプロジェクトにおいて、誤検知が発生することがあります。
:::

## 依存関係の問題

| 検出器                                             | ID                   | 説明                               | デフォルト |
| -------------------------------------------------- | -------------------- | ---------------------------------- | ---------- |
| [循環依存](/ja/detectors/cyclic_dependency)        | `cyclic_dependency`  | ファイル間の循環依存               | ✅         |
| [循環依存クラスター](/ja/detectors/cycle_clusters) | `cycle_clusters`     | 複雑な循環依存の網                 | ✅         |
| [型の循環](/ja/detectors/circular_type_deps)       | `circular_type_deps` | 型のみの循環依存                   | ❌         |
| [パッケージの循環](/ja/detectors/package_cycles)   | `package_cycles`     | パッケージ間の循環依存             | ❌         |
| [レイヤー違反](/ja/detectors/layer_violation)      | `layer_violation`    | 定義されたアーキテクチャ階層の違反 | ❌         |
| [SDP違反](/ja/detectors/sdp_violation)             | `sdp_violation`      | 安定依存原則（SDP）の違反          | ❌         |

## モジュールとクラスの設計

| 検出器                                          | ID                | 説明                                         | デフォルト |
| ----------------------------------------------- | ----------------- | -------------------------------------------- | ---------- |
| [ゴッドモジュール](/ja/detectors/god_module)    | `god_module`      | 責任が多すぎるモジュール                     | ✅         |
| [ハブモジュール](/ja/detectors/hub_module)      | `hub_module`      | 高度に接続された「ハブ」モジュール           | ❌         |
| [低い凝集度](/ja/detectors/lcom)                | `lcom`            | 内部凝集度が低いクラス (LCOM4)               | ❌         |
| [高い結合度](/ja/detectors/high_coupling)       | `high_coupling`   | 依存関係が多すぎるモジュール                 | ❌         |
| [分散モジュール](/ja/detectors/module_cohesion) | `module_cohesion` | 機能が多すぎるファイルに分散している         | ❌         |
| [機能への執着](/ja/detectors/feature_envy)      | `feature_envy`    | 自身のクラスより他のクラスを多く使うメソッド | ❌         |

## コード品質と組織

| 検出器                                                    | ID                    | 説明                                   | デフォルト |
| --------------------------------------------------------- | --------------------- | -------------------------------------- | ---------- |
| [デッドコード](/ja/detectors/dead_code)                   | `dead_code`           | 未使用のエクスポート                   | ✅         |
| [デッドシンボル](/ja/detectors/dead_symbols)              | `dead_symbols`        | 未使用のローカル関数や変数             | ✅         |
| [孤立した型](/ja/detectors/orphan_types)                  | `orphan_types`        | コードベースに接続されていない型       | ✅         |
| [バレル濫用](/ja/detectors/barrel_file)                   | `barrel_file`         | 結合を引き起こす巨大なバレルファイル   | ✅         |
| [基本データ型への執着](/ja/detectors/primitive_obsession) | `primitive_obsession` | ドメイン型の代わりに基本型を過度に使用 | ❌         |

## 複雑度とサイズ

| 検出器                                     | ID             | 説明                             | デフォルト |
| ------------------------------------------ | -------------- | -------------------------------- | ---------- |
| [高い複雑度](/ja/detectors/complexity)     | `complexity`   | 循環的複雑度が高い関数           | ✅         |
| [深いネスト](/ja/detectors/deep_nesting)   | `deep_nesting` | 深くネストされたコードブロック   | ✅         |
| [多すぎる引数](/ja/detectors/long_params)  | `long_params`  | 引数が多すぎる関数               | ✅         |
| [巨大なファイル](/ja/detectors/large_file) | `large_file`   | サイズが大きすぎるソースファイル | ✅         |

## 変更パターン

| 検出器                                                       | ID                   | 説明                                 | デフォルト |
| ------------------------------------------------------------ | -------------------- | ------------------------------------ | ---------- |
| [散弾銃の手術](/ja/detectors/shotgun_surgery)                | `shotgun_surgery`    | 多くのファイルの変更を必要とする変更 | ❌         |
| [不安定なインターフェース](/ja/detectors/unstable_interface) | `unstable_interface` | 頻繁に変更される公開インターフェース | ❌         |

## 実行時と安全性

| 検出器                                                     | ID                     | 説明                           | デフォルト |
| ---------------------------------------------------------- | ---------------------- | ------------------------------ | ---------- |
| [テストの漏洩](/ja/detectors/test_leakage)                 | `test_leakage`         | テストコードの本番環境への漏洩 | ❌         |
| [ベンダー結合](/ja/detectors/vendor_coupling)              | `vendor_coupling`      | 外部ライブラリへの密結合       | ❌         |
| [ハブ依存](/ja/detectors/hub_dependency)                   | `hub_dependency`       | 外部パッケージへの過度な依存   | ❌         |
| [副作用のあるインポート](/ja/detectors/side_effect_import) | `side_effect_import`   | 副作用を引き起こすインポート   | ✅         |
| [共有された可変状態](/ja/detectors/shared_mutable_state)   | `shared_mutable_state` | エクスポートされた可変変数     | ❌         |

## アーキテクチャ・メトリクス

| 検出器                                         | ID                 | 説明                            | デフォルト |
| ---------------------------------------------- | ------------------ | ------------------------------- | ---------- |
| [抽象性違反](/ja/detectors/abstractness)       | `abstractness`     | 苦痛/無用ゾーン (I+Aメトリクス) | ❌         |
| [分散した設定](/ja/detectors/scattered_config) | `scattered_config` | 多くのファイルに分散した設定    | ❌         |
| [コードクローン](/ja/detectors/code_clone)     | `code_clone`       | プロジェクト全体のコードの重複  | ✅         |
