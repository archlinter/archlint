# レイヤー（Layers）

`layers`設定を使用すると、プロジェクトのアーキテクチャレイヤーを定義し、それらの間の依存関係ルールを強制できます。

## レイヤーの定義

各レイヤーの定義は以下で構成されます：

- `name`: レイヤーの一意の識別子。
- `paths`: このレイヤーに含まれるファイルを識別するグロブパターンの配列。
- `can_import`: このレイヤーが依存することを許可されているレイヤー名の配列。

## 例：クリーンアーキテクチャ（Clean Architecture）

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # Domainレイヤーは独立している必要があります

  - name: application
    paths: ['**/application/**', '**/use-cases/**']
    can_import:
      - domain

  - name: infrastructure
    paths: ['**/infrastructure/**', '**/adapters/**']
    can_import:
      - domain
      - application

  - name: presentation
    paths: ['**/controllers/**', '**/api/**', '**/ui/**']
    can_import:
      - domain
      - application
```

## 仕組み

`layer_violation`検出器が有効な場合：

1. `paths`パターンに基づいて、プロジェクト内の各ファイルがレイヤーに割り当てられます。
2. それらのファイル内のすべてのインポート（import）がチェックされます。
3. レイヤー `A` のファイルがレイヤー `B` のファイルをインポートしており、かつ `B` が `A` の `can_import` リストに含まれていない場合、違反（violation）が報告されます。
