# テストツール

## すること

* `test/test-src/ok` の中にある `*.c` ファイルを再帰的にテストします。
    * この過程で `*.c` 意外の名前のファイルは **削除される** ので注意が必要です。
* clang によるコンパイル結果 (参考) と我々のコンパイラによるコンパイル結果を表示します。
* 両者によるバイナリを実行し、標準出力と戻り値が等しいかどうかを見ます。

現在のところ両方ともでコンパイルできるバイナリはないので比較は意味がありませんが...。最低限の C の形でコンパイルできるようになれば意味をもつはずです。

## 使い方

1. `cargo build` するか、 `cargo build --release` します。
    * バイナリを `~/.cargo/bin` に入れてよければ `cargo install --path .` としてもよいです。
    * `install` してもバイナリ一つ削除すれば元の状態に戻せるはずなので大丈夫なはずです、おそらく。
2. プロジェクトのルートディレクトリ (cerussite のディレクトリ) へ移動して `test/cerussite-test-tool/target/debug/cerussite-test-tool` とします。
    * `--release` でコンパイルした場合は `test/cerussite-test-tool/target/release/cerussite-test-tool` とします。
    * `install` した場合は、きちんと `~/.cargo/bin` にパスが通っていれば `cerussite-test-tool` だけで OK です。
