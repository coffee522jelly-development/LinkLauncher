# AGENTS.md

# AI Agent Instructions
## Tauri v2 + Svelte 5 Project Guidelines

このドキュメントは、本プロジェクトへ参加するAIエージェント向けの共通ガイドラインです。

実装・修正・レビューを開始する前に必ず本ドキュメントを確認し、記載された設計思想・コーディング規約に従ってください。

---

# 1. 基本ルール

## 必須

### 言語

以下は必ず日本語で記述してください。

- README.md
- ドキュメント
- AIレビュー
- 修正サマリー
- コミットメッセージ（詳細）

---

### テスト

UIへ変更を加えた場合は、必ず Playwright を利用したUIテストを実施してください。

最低限確認する内容

- レイアウト崩れ
- ライトテーマ
- ダークテーマ
- レスポンシブ表示
- スクリーンショット比較

---

### ビルド

コード提出前には必ず実行してください。

```bash
npm run tauri build
```

コンパイルエラーが残った状態では提出しないでください。

---

### アイコン

プロジェクト作成直後は必ず仮アイコン（SVG可）を作成してください。

その後

```bash
npm run icon
```

を実行し、

- ico
- png
- icns

を生成してください。

---

# 2. 技術スタック

|項目|内容|
|----|----|
|Frontend|Svelte 5|
|Desktop|Tauri v2|
|Backend|Rust|
|CSS|Tailwind CSS|
|Build|Vite|
|Package Manager|npm|

---

# 3. UI設計

## コンポーネント

可能な限り以下を優先してください。

- shadcn-svelte
- bits-ui

独自実装は必要最小限としてください。

---

## レイアウト

レスポンシブデザインを基本とします。

推奨

- Flex
- Grid
- Scroll Area

狭いウィンドウでもUIが崩れない設計を心掛けてください。

固定サイズは必要最低限にしてください。

---

# 4. テーマ設計

## 基本方針

テーマはCSS変数によって管理してください。

コンポーネント内で固定色を書くことは禁止します。

例

❌

```css
color:#2196F3;
background:red;
```

⭕

```css
color:var(--primary);
background:var(--background);
```

---

## ブランドカラー

ブランドカラーは次の3色を定義してください。

- Primary
- Secondary
- Tertiary

これらはブランドカラーであり、役割を表します。

---

## UIカラー

ブランドカラーとは別にUIカラーを定義してください。

最低限

- Background
- Surface
- Foreground
- Border
- Accent
- Success
- Warning
- Destructive

を定義してください。

---

## テーマ

テーマは以下を実装してください。

- Light
- Dark
- System（OS設定に追従）
- Custom（将来拡張）

テーマ追加時にコンポーネントの修正が不要な構造を維持してください。

---

# 5. Tauri v2

## OSネイティブ操作

OS機能は以下の優先順位で実装してください。

1. Tauri公式Plugin
2. Rust標準ライブラリ
3. OS API

独自C++実装は原則行わないでください。

例

- opener
- dialog
- shell
- fs
- store
- notification

---

## invoke

フロントエンドからRustを呼ぶ場合は invoke を利用してください。

同じAPIを複数利用する場合はラッパー関数を作成してください。

---

## Trayアプリ

CloseRequestedでは必ず

```
prevent_close()

↓

save_window_state()

↓

window.hide()
```

の順番で実行してください。

---

## Window State

ウィンドウ位置・サイズは

```
tauri-plugin-window-state
```

を利用してください。

手動保存

```rust
save_window_state(StateFlags::all())
```

を必ず実施してください。

---

## Permission

default.jsonには必要最低限のPermissionのみを追加してください。

Plugin追加時はPermissionも更新してください。

---

# 6. Svelte 5

状態管理はRunesを使用してください。

```
$state
$derived
$effect
$props
$bindable
```

Svelte4構文は禁止です。

```
export let
```

は使用しないでください。

---

## DOM参照

ラップされたコンポーネントからDOMを参照する場合は

```
$bindable()
```

を利用してください。

---

## Notification

通知は

- 成功
- 失敗

などユーザー操作に対してのみ表示してください。

起動時など暗黙タイミングでは表示しないでください。

---

# 7. Rust

JSONとの互換性維持のため

```rust
#[serde(rename="camelCaseName")]
```

を利用してください。

新規追加項目には

```rust
#[serde(default)]
```

を付与してください。

---

## Store

設定保存には

```
plugin-store
```

を利用してください。

---

## Tray Icon

Tray Iconは

```rust
TrayIconBuilder::with_id("tray")
```

を利用してください。

`.id()` は使用しないでください。

---

# 8. コーディングルール

## ライブラリ追加

新しいライブラリは安易に追加しないでください。

追加する場合は

- 必要性
- メリット
- デメリット

を説明してください。

---

## リファクタリング

要求されていないリファクタリングは禁止します。

例

- 命名変更のみ
- フォルダ移動のみ
- フォーマット変更のみ
- Import整理のみ

---

## コメント

コードから分かるコメントは不要です。

コメントは

- 設計意図
- 制約
- 注意事項

のみ記述してください。

---

## CSS

Tailwind CSSを優先してください。

style="" の利用は必要最低限とします。

共通スタイルはCSS変数またはコンポーネントへ切り出してください。

---

## コンポーネント分割

コンポーネントが肥大化した場合は責務ごとに分割してください。

再利用可能なUIは components 配下へ配置してください。

---

# 9. エラーハンドリング

例外を握り潰さないでください。

必要に応じて

- console.error
- Rustログ

へ出力してください。

Notificationはユーザーへ通知すべき場合のみ表示してください。

---

# 10. ビルド

提出前に最低限実施してください。

```bash
npm install

npm run dev

npm run tauri build
```

---

# 11. AIエージェントの行動指針

以下を遵守してください。

- 要件外の大規模リファクタリングを行わない
- 既存アーキテクチャを尊重する
- フォルダ構成を勝手に変更しない
- メジャーバージョンアップは指示がある場合のみ行う
- 既存ライブラリで実現可能なら新規ライブラリを追加しない
- 後方互換性を維持する
- ビルドが成功した状態で提出する

---

# 12. 作業完了時の報告

作業終了時には必ず以下を報告してください。

- 変更内容
- 変更理由
- 影響範囲
- 実施したテスト
- ビルド結果

---

# 13. AIへの最終指示

本ドキュメントに記載されていない事項については、

- 一貫性
- 保守性
- 可読性
- 後方互換性

を優先して実装してください。

判断に迷う場合は、既存コードベースの設計思想を優先してください。
