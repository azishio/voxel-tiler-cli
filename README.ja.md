[英語](README.md) | 日本語

# voxel-tiler-cli

las若しくはlaz形式の点群データをply形式のボクセル上のメッシュに変換します。
これは[voxel-tiler-core](https://crates.io/crates/voxel-tiler-core)をCLIでラップしたものです。

## インストール

Releaseページから環境に合わせたバイナリをダウンロードしてパスを通すか、以下の手順でインストールしてください。

### Crates.ioからインストール

```sh
cargo install voxel-tiler-cli
```

> [!NOTE]
> `cargo install` によってインストールされた実行ファイルは、通常、Unix系システムでは `$HOME/.cargo/bin`
> 、Windowsでは `%USERPROFILE%\.cargo\bin` に配置されます。
> 実行する際には、フルパスを指定するか、Cargoのbinディレクトリを`PATH`環境変数に追加してください。
>
> ```shell
> # Unix系システムの場合
> ~/.cargo/bin/voxel-tiler-cli
> 
> # または
> export PATH=$PATH:$HOME/.cargo/bin
> voxel-tiler-cli
> 
> # Windowsの場合
> %USERPROFILE%\.cargo\bin\voxel-tiler-cli
> 
> # または
> set PATH=%PATH%;%USERPROFILE%\.cargo\bin
> voxel-tiler-cli
> ```
>
> 参考: [Installing Binaries with cargo install](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html)

### リポジトリをクローンしてビルド

```sh
git clone git@github.com:azishio/voxel-tiler-cli.git
cd voxel-tiler-cli

cargo build --release
```

## 使い方

```shell
voxel-tiler-cli
```

実行ファイルを起動すると、以下の情報の入力が求められ、最終的な確認のあとで変換が開始されます。

+ 出力ファイルをタイル座標を基準に分割するか
+ 入力ファイルのパス
+ XとYを反転するか
+ 平面直角座標系の系
+ タイル座標/ピクセル座標のズームレベル
+ 出力ディレクトリ
+ plyファイルの出力形式

### 出力ファイルをタイル座標を基準に分割するか

デフォルト：いいえ

```shell
? Tiling? (y/N)
```

出力するplyファイルを、要素のタイル座標ごとに分割するか、一つのファイルにまとめるかを選択します。

分割することを選んだ場合、各plyファイルの原点は、要素が属するタイルの左上(ピクセル座標が最小)になります。

分割しないことを選んだ場合、唯一のplyファイルの原点は要素のAABBの最小座標と一致します。
つまり、全ての頂点が正かつ最小の座標を持つようにオフセットされます。

### 入力ファイルのパス

デフォルト：なし
開始位置：カレントディレクトリ

```shell
? Input File Path:  /path/to/some_directory/
  /path/to/some_directory/fiename1.las
  /path/to/some_directory/fiename2.las
  /path/to/some_directory/fiename3.laz
  /path/to/some_directory/child_directory  
[↑↓ to move, tab to autocomplete, enter to submit]
```

変換する.las/.lazファイルの絶対パスを入力してください。
パスを入力中、パス中の有効なディレクトリ内の.las/.lazファイルと子ディレクトリの候補を表示します。
表示された候補は上下キーで選択し、Tabキーで補完を実行できます。

候補の計算時、入力されているパスのうち、最後の`/`が出現するまでの部分がディレクトリとして扱われます。

### XとYを反転するか

デフォルト：いいえ

```shell
? Swap X and Y? (y/N)
```

入力されたファイルのXとYを入れ替えるかを選択します。

### ファイル情報の表示

以下は出力例です。
x,yは前の質問に基づいて入れ替えられています。

```shell
Las File Info

+------------------+---------+
| Number of Points | 6037715 |
+------------------+---------+
+------------+--------------------+-----------+----------+
| coord      | x                  | y         | z        |
+------------+--------------------+-----------+----------+
| max [m]    | -94283.511         | 23761.122 | 309.762  |
+------------+--------------------+-----------+----------+
| min [m]    | -94584.235         | 23482.154 | 269.359  |
+------------+--------------------+-----------+----------+
| center [m] | -94433.87299999999 | 23621.638 | 289.5605 |
+------------+--------------------+-----------+----------+
```

### 平面直角座標系の系

デフォルト：なし

```shell
? JPR Origin:  
```

入力されたファイルがどの平面直角座標系の原点を基準にしているかを入力します。

> [!TIP]
> このツールは、平面直角座標系に基づいた点群をもつ.las/.lazファイルを想定しています。
>
> しかし、平面直角座標系はメートル単位の直交座標系であるため、メートル単位の点群データであればそれなりの結果を得られるかもしれません。
> ただし、出力されたファイルの座標的な正しさは失われることに注意してください。
>
> この場合、`Tiling`を`No`に設定して、出力ファイルを一つにまとめることをお勧めします。

### タイル座標/ピクセル座標のズームレベル

デフォルト：なし

```shell
? Select zoom levels  
^ [ ] ZoomLevel: 14 (  8.09 m/voxel)
  [ ] ZoomLevel: 15 (  4.04 m/voxel)
  [ ] ZoomLevel: 16 (  2.02 m/voxel)
> [ ] ZoomLevel: 17 (  1.01 m/voxel)
  [ ] ZoomLevel: 18 (  0.51 m/voxel)
  [ ] ZoomLevel: 19 (  0.25 m/voxel)
v [ ] ZoomLevel: 20 (  0.13 m/voxel)
[↑↓ to move, space to select one, → to all, ← to none, type to filter]
```

ボクセルがどのズームレベルで生成されるかを選択します。
各ズームレベルに対応するボクセルの1辺の長さも表示されるため、これを参考にしてください。

ボクセルの1辺の長さは、入力されたファイルの中心緯度に基づいて計算されます。

### 出力ディレクトリ

デフォルト：なし
開始位置：カレントディレクトリ

```shell
? Output Directory:  /path/to/some_directory/
  /path/to/some_directory/child_directory1
  /path/to/some_directory/child_directory2
[↑↓ to move, tab to autocomplete, enter to submit]
```

ファイルの出力ディレクトリの絶対パスを入力してください。

ディレクトリが存在しない場合、自動的に作成されます。

> [!WARNING]
> `Tiling`を`Yes`に設定した場合、大量のファイルが出力される可能性があるため、出力先のディレクトリを慎重に選択してください。
> 新しいディレクトリを作成することをお勧めします。

### plyファイルの出力形式

デフォルト：Ascii

```shel
? Select file format  
> Ascii
  Binary (little endian)
  Binary (big endian)
[↑↓ to move, enter to select, type 
```

出力するplyファイルの形式を選択します。

### 最終確認

デフォルト：いいえ

```shell
Params

+------------------------------------------------+
| InputFile  : /path/to/source/las_file.las      |
+------------------------------------------------+
| OutputDir  : /path/to/output/dir/              |
+------------------------------------------------+
| Ply Format : Ascii                             |
+------------------------------------------------+
| Tiling     : false                             |
+------------------------------------------------+

? Continue? (y/N)  
```

入力された設定情報を表示し、変換を開始するか確認します。

## ライセンス

以下のいずれかの下でライセンスされています。

+ Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0)
+ MIT Licence ([LICENSE-MIT](LICENSE-MIT)または http://opensource.org/licenses/MIT)

(ドキュメンテーションコメント及びREADMEファイルの英語はDeepLとChatGPTにより翻訳されています。)
