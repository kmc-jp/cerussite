# テストツール

## 使い方

1. `cargo build` するか、 `cargo build --release` する。
    * バイナリを `~/.cargo/bin` に入れてよければ `cargo install --path .` としてもよい。
    * `install` してもバイナリ一つ削除すれば元の状態に戻せるはずなので大丈夫、おそらく。
2. プロジェクトのルートディレクトリ (cerussite のディレクトリ) へ移動して `test/cerussite-test-tool/target/debug/cerussite-test-tool` とする。
    * `--release` でコンパイルした場合は `test/cerussite-test-tool/target/release/cerussite-test-tool` とする。
    * `install` した場合は、きちんと `~/.cargo/bin` にパスが通っていれば `cerussite-test-tool` だけでよい。
