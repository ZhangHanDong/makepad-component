# Makepad Component

[![バージョン](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/ZhangHanDong/makepad-component)
[![ライセンス](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](LICENSE)

**[English](README.md) | [中文](README_zh.md)**

[Makepad](https://github.com/makepad/makepad) 向けのモダンなUIコンポーネントライブラリ。Longbridge の [gpui-component](https://github.com/longbridge/gpui-component) からインスピレーションを得ています。

![Makepad Component プレビュー](asserts/mc1.png)

## Makepad について

[Makepad](https://github.com/makepad/makepad) は Rust で書かれた次世代 UI フレームワークで、以下の特徴があります：

- **GPU アクセラレーション描画** - SDF（符号付き距離場）を使用したカスタムシェーダーベースの描画
- **クロスプラットフォーム** - デスクトップ（Windows、macOS、Linux）、モバイル（iOS、Android）、Web（WebAssembly）
- **ライブデザイン** - ホットリロード対応の DSL で高速な UI イテレーション
- **高パフォーマンス** - IDE やリアルタイムツールなど、要求の厳しいアプリケーション向けに設計

### プロダクション事例

| プロジェクト | 説明 |
|-------------|------|
| [Robrix](https://github.com/project-robius/robrix) | Makepad で構築された Matrix チャットクライアント |
| [Moly](https://github.com/moxin-org/moly) | AI モデル管理・推論ツール |
| [Makepad Studio](https://github.com/makepad/makepad) | Makepad IDE 自体 |

これらのプロジェクトは [Robius](https://github.com/project-robius) イニシアチブの下で開発され、クロスプラットフォーム Rust GUI 開発を推進しています。

## スクリーンショット

| コンポーネント | Slider 機能 |
|----------------|-------------|
| ![コンポーネント](asserts/mc1.png) | ![Slider](asserts/mc2.png) |

| その他のコンポーネント | フルデモ |
|------------------------|----------|
| ![その他](asserts/mc3.png) | ![デモ](asserts/mc4.png) |

## 機能

### コンポーネント (v0.1.0)

- **Button** - Primary、Secondary、Danger、Ghost バリアント、複数サイズ対応
- **Checkbox** - ラベルと不確定状態をサポート
- **Switch** - アニメーション付きトグルスイッチ
- **Radio** - ラジオボタングループ
- **Divider** - 水平/垂直セパレーター
- **Progress** - 線形プログレスバー
- **Slider** - シングル/レンジモード、垂直方向、対数スケール、無効状態

### 今後追加予定

- TextInput（テキスト入力）
- Badge（バッジ）
- Tooltip（ツールチップ）
- Spinner（ローディング）
- Modal（モーダル）
- Dropdown（ドロップダウン）
- その他...

## インストール

`Cargo.toml` に追加：

```toml
[dependencies]
makepad-component = { git = "https://github.com/ZhangHanDong/makepad-component", branch = "main" }
```

## 使用方法

```rust
use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_component::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {
                    flow: Down, spacing: 20, padding: 20

                    <MpButtonPrimary> { text: "プライマリボタン" }
                    <MpCheckbox> { text: "チェックして" }
                    <MpSwitch> {}
                    <MpSlider> { value: 50.0, min: 0.0, max: 100.0 }
                }
            }
        }
    }
}
```

## デモの実行

```bash
# リポジトリをクローン
git clone https://github.com/ZhangHanDong/makepad-component
cd makepad-component

# コンポーネントズーデモを実行
cargo run -p component-zoo
```

---

## AI 支援開発

本コンポーネントライブラリは、[makepad-skills](https://github.com/ZhangHanDong/makepad-skills) を使用して AI（Claude Code）と共同で構築されました。

makepad-skills は、Makepad 開発向けに設計された Claude Code スキルセットで、ウィジェット作成、シェーダープログラミング、プロダクションパターンをカバーしています。

---

## インスピレーション

本プロジェクトは Longbridge の [gpui-component](https://github.com/longbridge/gpui-component) からインスピレーションを得ています。gpui-component は GPUI フレームワーク（Zed エディタで使用）向けのコンポーネントライブラリです。gpui-component が GPUI をターゲットにしているのに対し、**makepad-component** は同様のデザイン原則とコンポーネントパターンを Makepad エコシステムにもたらします。

主な違い：
- **Makepad** は `live_design!` DSL を使用、GPUI は純粋な Rust アプローチ
- **Makepad** は組み込みのシェーダー/アニメーションシステムを持つ
- **Makepad** はより多くのプラットフォーム（モバイル/Web を含む）をサポート

## コントリビューション

> **注意：** 本コンポーネントライブラリはまだ開発初期段階です。皆さんと一緒に作り上げていきたいと思います！

コントリビューションを歓迎します！issue や pull request をお気軽にお送りください。

## ライセンス

以下のいずれかのライセンスの下で提供されます：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) または http://opensource.org/licenses/MIT)

お好きな方をお選びください。
