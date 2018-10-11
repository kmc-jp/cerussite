# cerussite --- C compiler written in Rust.

Rust で C コンパイラを作ろうという試みです。

## テストツール --- `cerussite-test-tool`

### すること

* `test/test-src/ok` の中にある `*.c` ファイルを再帰的にテストします。
    * この過程で `*.c` 意外の名前のファイルは **削除される** ので注意が必要です。
    * 前回テスト時の生成物を削除するためです。
* clang によるコンパイル結果 (参考) と我々のコンパイラによるコンパイル結果を表示します。
    * コンパイル過程の生成物 (LLVM IR と実行可能バイナリ) は同じフォルダに cerussite によるもの (`cur_` から始まる) と clang によるもの (`ref_` から始まる) の両方が保存されます。
* 両者によるバイナリを実行し、標準出力と戻り値が等しいかどうかを見ます。
    * 現在のところ両方ともでコンパイルできるソースはないので比較は意味がありませんが、 Pull Request が承認されれば意味をもつようになります。
    * これは **両者ともにコンパイルエラー** という場合も **OK** と判定されるので、注意が必要です。


### 使い方

1. `cargo build` するか、 `cargo build --release` します。
    * バイナリを `~/.cargo/bin` に入れてよければ `cargo install --path .` としてもよいです。
    * `install` してもバイナリ一つ削除すれば元の状態に戻せるはずなので大丈夫なはずです、おそらく。
    * `install` した場合で、バージョンアップされた際は、 `cargo install --path . --force` とすれば置き換えてくれます。
2. プロジェクトのルートディレクトリ (cerussite のディレクトリ) へ移動して `test/cerussite-test-tool/target/debug/cerussite-test-tool` とします。
    * `--release` でコンパイルした場合は `test/cerussite-test-tool/target/release/cerussite-test-tool` とします。
    * `install` した場合は、きちんと `~/.cargo/bin` にパスが通っていれば `cerussite-test-tool` だけで OK です。

### コマンドラインオプション

* -v, --verbose: cerussite と clang で結果が一致した場合も、アウトプットや LLVM IR などを出力します。
* (`test/test-src/ok` にあるファイル名) テストするファイルを指定できます。
    * 複数指定も可能です。
    * `test/test-src/ok` からの相対パスで指定すれば、 `..` などを活用して表現できるパスであれば、おそらく任意のファイルがテストできます。

### 例

* 全部のファイルをテストする。
    ```
    % cerussite-test-tool
    ```
* 全部のファイルをテストする。成否に関わらず、コンパイラ出力や LLVM IR と実行結果を出力する。
    ```
    % cerussite-test-tool -v
    ```
* `test/test-src/ok` 内の `test01.c` をテストする。
    ```
    % cerussite-test-tool test01.c
    ```
* `test/test-src/ok` 内の `test01.c`, `test02.c` をテストする。冗長出力もする。
    ```
    % cerussite-test-tool -v test01.c test02.c
    または
    % cerussite-test-tool test01.c -v test02.c
    または (以下略。 -v はどこにあってもよい)
    ```
