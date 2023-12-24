# チャットベーステトリス

「ChatGPTはテトリスをプレイできるか」を検証するために作成したプログラムになります！

自分向けに作ったのであんまり親切な設計にはなってないです

# テキストベースで実行

```bash
$ cargo run --bin chatris
```

## 出力例

```plaintext
 0....@.....,
 1....@@@...,
 2..........,
 3..........,
 4..........,
 5..........,
 6..........,
 7..........,
 8..........,
 9..........,
10..........,
11..........,
12..........,
13..........,
14..........,
15..........,
16..........,
17..........,
18..........,
19..........,
20..........,
==========
Current: J
Hold: -
Next: I S T

目標: 1ライン消し
Score: 0
Lines: 0
feedback: 

ChatGPT: 
```

- `Current` : 現在降っているテトリミノの種類
- `Hold` : 現在ホールドしているテトリミノ
- `Next` : 次に降ってくるテトリミノ。一番左が一番早いです
- `目標` : ChatGPTに目標を認識させる
- `Score` : スコアです
- `Lines` : 消したライン数です
- `feedback` : ChatGPTに連絡事項がある際に使いやすいフィールドです。コピペする時にここを活用しましょう
- `ChatGPT` : ここにコマンドを入力します

## コマンド

|コマンド|意味|
|:-------|:---|
|R|右回転|
|L|左回転|
|V|テトリミノが下方向に1下がります。|
|>|テトリミノが右方向に1動きます。|
|<|テトリミノが左方向に1動きます。|
|H|現在のテトリミノをホールド(Hold)します。|
|;|テトリミノを一番下までおろし、現在操作中のテトリミノを固定します。ハードドロップに相当する操作です。|

これ以上下ることができなくなっても固定されない仕様になっています。位置を決定したら必ず `;` コマンドでハードドロップしてください。

# フィールドを画像として出力しつつ実行

```bash
$ cargo run --bin with_img -- -d <DIRECTORY>
```

## 出力例

![field.png](.github/field.png)

```plaintext
Current: I
Hold: -
Next: L O J

目標: 1ライン消し
Score: 0
Lines: 0
feedback: 

ChatGPT: 
```

指定したディレクトリ(指定しない場合 `tetris_imgs` ディレクトリ)にフィールド画像が出力されます。

フィールド部分が画像である以外は、テキストベースの場合と変わりません。

# 捕捉事項

- リポジトリ内にプロンプトの残骸が散らばっていますが、現在の仕様と互換性がないものが多数混ざっているためあまり参考にしないでください