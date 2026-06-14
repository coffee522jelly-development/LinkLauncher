# LinkLauncher (リンクランチャー)

LinkLauncher は、業務で頻繁に使用する URL やファイルパスを効率的に管理・起動するための、Windows 最適化デスクトップアプリケーションです。ブラウザのブックマークとは異なり、業務に必要なリソースだけに絞り込むことで、迷いなく目的の場所にアクセスできる環境を提供します。

Tauri v2 と Svelte 5 (Runes) を使用して構築されており、軽量で高速に動作します。

## 主な機能

- **リンク管理**: 名称、URL またはファイルパス、カテゴリーを登録・編集・削除できます。
- **インテリジェントな自動分類**: カテゴリーを空欄で登録すると、パスの内容に応じて「Web」または「Local」が自動的に割り当てられます。
- **マルチビュー表示**:
  - **リスト形式 (Table)**: 詳細な情報を一覧で確認できる表示モード。
  - **ボタン形式 (Grid)**: アイコン感覚で直感的に操作できる表示モード。
- **整理と並び替え**:
  - **ピン留め**: 重要なリンクをリストの最上部に固定できます。
  - **ドラッグ＆ドロップ**: ボタン形式において、マウス操作で直感的に順序を入れ替え可能です（カスタムソート時）。
  - **高度なソート**: 名称、カテゴリーによる昇順・降順ソートに対応。
- **クイックアクション**:
  - **スマート起動**: URL ならブラウザ、ファイルなら規定のアプリで開きます（.xlsx なら「Excel」ボタンとして表示）。
  - **場所を表示 (Reveal)**: ファイルが保存されているフォルダを開き、そのファイルを選択状態にします。
  - **ターミナルで開く**: 指定されたパス（またはファイルの親フォルダ）でコマンドプロンプトを起動します（自動的に `cd` します）。
  - **コピー**: パスをクリップボードにコピーします。
- **テーマとカスタマイズ**:
  - **6つのカラーテーマ**: Zinc, Blue, Rose, Green, Orange, Slate から選択可能。
  - **ダークモード対応**: システム設定に連動し、視認性の高い配色を提供。
  - **ウィンドウ状態の保持**: サイズや位置を終了時に保存し、次回起動時に復元します。
- **データ管理**: CSV 形式でのインポート・エクスポートに対応しており、データのバックアップや移行が容易です。

## 技術スタック

- **Frontend**: Svelte 5 (Runes), TypeScript, Tailwind CSS v3
- **Backend**: Rust (Tauri v2)
- **Persistence**: @tauri-apps/plugin-store (JSON), tauri-plugin-window-state
- **Native Integration**: tauri-plugin-opener, tauri-plugin-fs, tauri-plugin-dialog, tauri-plugin-notification, tauri-plugin-clipboard-manager

## 開発環境のセットアップ

### 前提条件

- [Node.js](https://nodejs.org/) (LTS)
- [Rust](https://www.rust-lang.org/) (Tauri v2 のシステム要件を満たしていること)

### インストール

```bash
# 依存関係のインストール
npm install
```

### 開発用サーバーの起動

```bash
# 開発モードで起動 (localhost:1420)
npm run dev
```

### ビルド

```bash
# インストーラーの作成 (MSI/NSIS)
npm run tauri build
```

## ライセンス

MIT License
