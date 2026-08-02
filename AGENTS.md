# AI Agent Instructions (Tauri v2 + Svelte 5 Project Guidelines)

このドキュメントは、本技術スタック（Tauri v2 + Svelte 5）を用いたプロジェクト開発を支援するAIエージェント向けの設定・アーキテクチャ・コーディング規約の共通ガイドラインです。開発や修正を開始する前に必ず一読してください。

## 1. 必須の基本ルール
- **言語設定**: 全てのドキュメント（特に `README.md`）、コミットメッセージの詳細、AIエージェントによるコードレビューおよびサマリーは**必ず日本語**で記述してください。
- **テストの実施**: UIに変更を加えた場合など、テスト実行時には毎回必ず **Playwright を使用したテスト（UIの自動検証・スクリーンショット確認など）** を実施するよう計画に組み込んでください。

## 2. 技術スタックとインストーラー設定
- **Frontend**: Svelte 5, Tailwind CSS, Vite
- **Backend**: Tauri v2, Rust
- **パッケージマネージャー**: npm
- **インストーラーの作成**:
  - インストーラーは必ず **MSI形式 (`wix`)** で作成してください（`tauri.conf.json` で指定）。
  - インストーラーの言語（ローカライズ設定）は必ず **`ja-JP`** に設定して作成してください。

### UIコンポーネントの方針
- 可能であれば、**`shadcn-svelte`** や **`bits-ui`** を積極的に活用・導入していくことを推奨します。
- UIのレイアウトは、狭いウィンドウサイズでも要素が重ならず視認性を保てるように配慮してください（レスポンシブなFlex/Gridレイアウトを推奨）。
- テーマ切り替え（Zinc, Blue, Rose等）に対応するため、テキスト入力やボタンなどには必ず適切な色変数（例: `text-foreground`, `bg-background`, `bg-primary`）を適用してください。

## 3. アーキテクチャとTauri v2の制約

### OSネイティブ操作
- ファイルを開く、ブラウザを開く、ディレクトリを表示するなどのOSレベルの操作は、自作のC++ロジックなどを実装せず、可能な限り**公式の Tauri v2 プラグイン（`tauri-plugin-opener` など）**を使用してください。
- ターミナルの起動など、プラグインで対応できない処理についてはRust側で `std::process::Command` を使用して実装し、フロントエンドからは `invoke` で呼び出してください。

### ウィンドウ状態と常駐（トレイ）アプリの実装
- アプリがシステムトレイに常駐する仕様の場合、ウィンドウを閉じるイベント（`tauri::WindowEvent::CloseRequested`）をフックし、`api.prevent_close()` を呼んだ上で `window.hide()` を実行します。
- **重要**: ウィンドウを隠す前に、必ず `tauri-plugin-window-state` の `save_window_state(StateFlags::all())` を手動で呼び出して、ウィンドウサイズと位置を保存してください。

### プラグインのパーミッション設定
- Tauri v2 では権限が厳格化されています。`src-tauri/capabilities/default.json` 等にて、必要なプラグインの権限を明示的に許可してください（例: `opener` にはファイルパス用の `**` とURL用の `http://**`, `https://**` を指定）。

## 4. Svelte 5 (Frontend) のルール
- 状態管理には Svelte 5 の Runes (`$state`, `$derived`, `$effect`, `$props`, `$bindable`) を使用してください。古い Svelte 4 の記法（`export let` 等）は避けてください。
- `Input` のようなラップされたUIコンポーネントからDOM要素への参照（`ref`）を引き回す場合は、`$bindable()` を使用して親コンポーネントからアクセスできるようにしてください。
- 通知（`@tauri-apps/plugin-notification` 等）は、ユーザーの明示的なアクションが成功・失敗した際のみ表示し、アプリ起動時など暗黙的なタイミングでの呼び出しは避けてください。

## 5. Rust (Backend) のデータ連携ルール
- アプリのデータや設定の永続化には `@tauri-apps/plugin-store` 等を利用します。
- Rust側でJSONデータをパースする際、JavaScript側でキャメルケース（例: `isFavorite`）で保存されているプロパティ名と、Rust側のスネークケース（`is_favorite`）を一致させるため、必ず `#[serde(rename = "camelCaseName")]` を指定してください。
- 新しいフィールドを追加する際は、過去のJSONデータとの互換性を保つために `#[serde(default)]` を付与してください。
- トレイアイコンのIDは `TrayIconBuilder::with_id("tray_name")` で初期化してください（`.id()` メソッドはv2.11+ではGetterとして機能するためビルドエラーになります）。

## 6. ビルドと検証のガイドライン
- 依存関係のインストール: `npm install`
- 開発サーバーの起動: `npm run dev`
- Tauri本番ビルド: `npm run tauri build`
- PRやコード提出の前には、必ず `npm run tauri build` を実行し、コンパイルエラーや依存関係のエラーが発生しないことを検証してください。
- Windows環境特有の問題（Smart App Controlによるブロックなど）が懸念される場合は、必要に応じてレビューコメントにトラブルシューティングを残してください。
