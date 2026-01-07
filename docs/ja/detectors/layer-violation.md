# レイヤー違反

**ID:** `layer_violation` | **Severity:** High (default)

レイヤー違反（Layer violation）は、あるアーキテクチャレイヤーのコードが、知るべきではないレイヤーのコードをインポートした場合に発生します（例：Domain レイヤーが Infrastructure レイヤーをインポートする）。

## なぜこれがコードの不吉な臭い（smell）なのか

- **抽象化の破壊**: 内部の実装詳細が、高レベルのビジネスロジックに漏れ出します。
- **テストの困難さ**: インフラストラクチャ（DB、API など）のモックなしでは、ビジネスロジックのテストが困難になります。
- **硬直性**: データベースや外部ライブラリを変更する際に、コアなビジネスロジックの変更が必要になります。

## 設定

`.archlint.yaml` でレイヤーを定義する必要があります。

```yaml
layers:
  - name: domain
    paths: ['**/domain/**']
    can_import: [] # Domain は何もインポートしません

  - name: application
    paths: ['**/application/**']
    can_import: ['domain']

  - name: infrastructure
    paths: ['**/infrastructure/**']
    can_import: ['domain', 'application']
```

## 修正方法

1. **依存関係逆転（Dependency Inversion）**: 上位レイヤー（Domain）でインターフェースを定義し、下位レイヤー（Infrastructure）でそれを実装します。
2. **リファクタリング**: 誤って配置されたコードを適切なレイヤーに移動します。
