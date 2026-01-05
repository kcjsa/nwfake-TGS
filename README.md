旧 NWFake TGS v0.9 (タングステン) – 2026-1-5 ビルド
# NWFake Client & Endserver 公開仕様

## 前提条件
- OS: Linux（Ubuntu 系）
- Snap 版 Chromium インストール済み

## 動作確認方法
1. ターミナルを開き `chromium` と入力して起動確認
2. Client を起動し、NWFake モードに切替 → 127.0.0.1 待機
3. Endserver を起動 → 待機メッセージが出ることを確認

## Client
- 内部構造非公開
- 初期状態: HTTPS direct
- NWFake モードに切替直後は 127.0.0.1 で待機
- Endserver に接続するには別設定で IP 指定が必要
- 一部デバッグログのみ出力（通信内容や操作履歴は残りません）
- 追加機能:
  - ドメインブロック: http://block_config.nfk
  - HTTPS/NWFake 切替: http://switch_protocol.nfk/
  - Endserver IP 設定: http://tunnel.nfk/
- **禁止事項**: 改造・逆コンパイル・解析は禁止

## Endserver
- コードは公開されており、自由に改造・解析可能
- WAN 公開専用（Cloudflare 推奨）
- 出力は待機メッセージのみ
- ユーザーが自分でログ出力などの機能を追加可能
- LAN/直接ポート開放は非推奨

## 注意
- 使用はすべて自己責任
- Client 操作は設定画面経由のみ
- 内部構造や loopback 待機の詳細は公開しません
