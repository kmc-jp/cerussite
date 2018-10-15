# cerussite --- C compiler written in Rust.

Rust で C コンパイラを作ろうという試みです。

## プロジェクトの成果物

* コンパイラ本体 `cerussite`
* テストランチャー `cargo-test-cerussite`
* テストツール `cerussite-test-tool`

## コンパイラ --- `cerussite`

### サポートする機能

現在は、外側の `int main (void) { return (...) ; }` を完全に無視し、 `return` 内部の式を解釈する作業を進めています。

* 最小限の C プログラム
    ```c
    int main(void) {
        return 42;
    }
    ```
* 数式
    * `0` や `42` のような即値
    * `+` による即値同士の演算
    * `-`, `*`, `/`, `%` による即値同士の演算
    * 演算子を複数含む数式
        * 優先順位 (`*`, `/`, `%` の優先)
        * 括弧 (括弧の最優先)
    * 単項 `+`/`-` 演算子

## テストランチャー --- `cargo-test-cerussite`

後述の `cerussite-test-tool` をより便利に実行するためのシンプルなプログラムです。

### すること

1. cerussite プロジェクトのルートディレクトリを検索します。
    * 開発作業の途中で `src/module/submodule/subsub/` などの深いディレクトリなどで作業する必要がでてくるかもしれませんが、この機能があるのでどれだけ深いところにいてもルートにわざわざ戻ることなくテストを実行することができます。
    * カレントディレクトリから上へ上へと辿り、初めて `Cargo.toml` ファイルを発見したディレクトリをルートとします。
        * そのため、他のプロジェクト (テストツールやこのランチャー自体) の中にいると、 cerussite のプロジェクトルートを見つけられず、以降の動作でパスが合わずに失敗してしまいます。
2. `cerussite-test-tool` をコンパイルします。
    * テストツールがアップデートされた場合も気にせず実行することができます。
        * このツールが、当初テストツールが活発に開発されていた黎明期に、テストツールを一々コンパイルして実行するのが面倒だからという経緯で導入されたためです。
    * テスト機能がアップデートされていない / マージされていないブランチに移動するとわざわざ古いバージョンをコンパイルして使い出すという弊害もあります。
        * テストツール開発中にこの傾向があります。
        * 現状、 `master` でもテストツールが最低限の機能を持っているのであまり問題にはならないはずです。
    * テストツールは `cargo build --release` によりコンパイルされ、コンパイルされたバイナリを直接実行することで実行されます。テストツールをパスの通る場所に入れたり、このランチャーがテストツールを勝手にグローバルにインストールしたりといったことはありません。
    * テストツールのコンパイルに失敗した場合、以降の処理は中断されます。
3. コンパイラをコンパイルします。
    * 実はテストツールはコンパイラを `cargo run` により実行するため、単体で使っても「古いバージョンのコンパイラでテストされてしまった」という問題は起こらないように作られています。ただし、いくつか問題があります。
        * その際のコンパイルエラーは他のエラーと同列のものでしかないので、テストツールが stderr 出力としてキャプチャしたものを再表示することになります。見づらくなったり、色情報が失くなっていたり、レイアウト崩れを起こしたりする可能性があります。
        * コンパイルは通るけれど警告がある場合、テストが成功してしまえばテストツールは通常コンパイラ出力を出力しないために気づかないという問題があります。つまり、テストツール単体では、変更→テスト→全て OK でコミット、という流れをとっても警告に気づけません。また cargo は一度コンパイルが成功したバイナリはソースに変更があるまで再コンパイルしないので、その後にもう一度 `cargo build` しても警告は表示されません...。
    * コンパイルに失敗した場合、テストは実行されません。
4. テストを実行します。
    * `cargo test-cerussite` に続いて渡されたオプションは全て `cerussite-test-tool` へ回されます。
    * テストツールが異常終了した場合 (一つでも失敗したテストがある場合) このツールの戻り値も異常終了になります (`main()` を `Result` にして `Err` を投げると何が返ってくるかの規定があるのかないのかは知りませんが、今のところ 1 が返ってきているようです) 。

### 使い方

1. プログラムをビルドし、インストールします。
    * `cargo-test-cerussite` ディレクトリへ移動し、 `cargo install --path . --force` を実行します。
    * これはグローバルのディレクトリ (通常 `~/.cargo`) へバイナリをインストールします。インストールしたくない場合は、普通に `cargo build --release` するといつも通り `cargo-test-cerussite/target/release` 以下にバイナリが生成されます。ただし、利用するときには必ずバイナリが `$PATH` に含まれている必要がありますので注意してください。
    * グローバルにインストールする場合も基本的にバイナリ一つがコピーされるだけなので、それ一つ消せばクリーンな状態に戻るはずです。
2. 成果物のバイナリ `cargo-test-cerussite` が `$PATH` にある場合、 `cerussite` プロジェクト内の、 **他のプロジェクト配下でない** 任意のディレクトリで `cargo test-cerussite` と実行することでプログラムを起動することができます。
    * もしグローバルにインストールしない選択をした場合、バイナリのあるディレクトリを `$PATH` に追加すれば同様に呼び出せます。
    * 直接パスを指定して実行することでは実行できません。これは引数解析の都合上、第一引数に `cargo` という文字列が含まれることと、第二引数に `test-cerussite` という文字列が来ることをチェックしているからです。

### コマンドラインオプション

* `cargo test-cerussite` 以降の引数は全て `cerussite-test-tool` の第二引数以降として渡されます。 `cerussite-test-tool` のコマンドラインオプションについてはその項を参照ください。

### 例

* 全てをテストする。
    ```console
    $ cargo test-cerussite
    ```
* `cerussite-test-tool` のオプションを指定する。
    ```console
    $ cargo test-cerussite test1.c -v test2.c
    ```
    オプションについての詳細はテストツールのコマンドラインオプションの項を参照ください。

## テストツール --- `cerussite-test-tool`

### すること

* `test/ok` の中にある `*.c` ファイルを再帰的にテストします。
    * この過程で `*.c` 以外の名前のファイルは **削除される** ので注意が必要です。
    * 前回テスト時の生成物を削除するためです。
* clang によるコンパイル結果 (参考) と我々のコンパイラによるコンパイル結果を表示します。
    * コンパイル過程の生成物 (LLVM IR と実行可能バイナリ) は同じフォルダに cerussite によるもの (`cur_` から始まる) と clang によるもの (`ref_` から始まる) の両方が保存されます。
* 両者によるバイナリを実行し、標準出力と戻り値が等しいかどうかを見ます。
    * **両者ともにコンパイルエラー** という場合は **テスト失敗** とされるようになりました。これは `test/ok` の中にあるファイルは全てコンパイルが通るべきものだからです。テストファイルの記述ミスによる意図しないテスト成功を防ぐ意味もあります。意図的にエラーを起こしたい場合は `test/ng` にでも入れておいてください (このディレクトリに対してテストを行うツールなどはまだありません。このツールで一つずつファイルを指定することはできますが) 。


### 使い方

現状このツールを手動で呼び出す意味はあまりないと考えられます。この手順は今も有効ですが、通常は `cargo-test-cerussite` を使うと良いと思います。

1. `cargo build` するか、 `cargo build --release` します。
    * バイナリを `~/.cargo/bin` に入れてよければ `cargo install --path .` としてもよいです。
    * `install` してもバイナリ一つ削除すれば元の状態に戻せるはずなので大丈夫なはずです、おそらく。
    * `install` した場合で、バージョンアップされた際は、 `cargo install --path . --force` とすれば置き換えてくれます。
2. プロジェクトのルートディレクトリ (cerussite のディレクトリ) へ移動して `cerussite-test-tool/target/debug/cerussite-test-tool` とします。
    * `--release` でコンパイルした場合は `cerussite-test-tool/target/release/cerussite-test-tool` とします。
    * `install` した場合は、きちんと `~/.cargo/bin` にパスが通っていれば `cerussite-test-tool` だけで OK です。

### コマンドラインオプション

* -v, --verbose: cerussite と clang で結果が一致した場合も、アウトプットや LLVM IR などを出力します。
* (`test/ok` にあるファイル名) テストするファイルを指定できます。
    * 複数指定も可能です。
    * `test/ok` からの相対パスで指定すれば、 `..` などを活用して表現できるパスであれば、おそらく任意のファイルがテストできます。

### 例

* 全部のファイルをテストする。
    ```console
    $ cerussite-test-tool
    ```
* 全部のファイルをテストする。成否に関わらず、コンパイラ出力や LLVM IR と実行結果を出力する。
    ```console
    $ cerussite-test-tool -v
    ```
* `test/ok` 内の `test01.c` をテストする。
    ```console
    $ cerussite-test-tool test01.c
    ```
* `test/ok` 内の `test01.c`, `test02.c` をテストする。冗長出力もする。
    ```console
    $ cerussite-test-tool -v test01.c test02.c
    または
    $ cerussite-test-tool test01.c -v test02.c
    または (以下略。 -v はどこにあってもよい)
    ```
