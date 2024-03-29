# 準備

ChatGPTさんにテトリスをプレイしていただいきたいです。

コマンドを受けるとテトリミノを操作できるテトリスゲームを用意いたしました。用語は一般的なテトリスに従いますが、テトリミノのことはミノと略記することがあります。基本的なルールをこれから解説していきます。

テトリスのフィールドは以下のように与えられます。
 0....@@....,
 1....@@....,
 2..........,
 3..........,
 4..........,
 5........@@,
 6.......@@.,
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
19J........L,
20JJJ....LLL,
この行より上までがフィールドの一例になります。

フィールドは21行10列からなります。一番左に書かれているのは行座標です。各行はカンマで区切られています。マスの座標をx行y列や`{ "x": 0, "y": 0 }`のように表し、一番左上のマスの座標を`{ "x": 0, "y": 0 }`、一番右下のマスの座標を`{ "x": 20, "y": 9}`とします。

上からテトリミノが落ちてきます。テトリミノを落とせなくなったらゲームオーバーです。

フィールドの`.`は空白を表します。初期状態ではすべてのマスが`.`です。`@`のマスは現在操作中のテトリミノを表します、上記の例ではSミノが降ってきています。アルファベットは固定されたブロックで、元々なんのテトリミノであったかを表します。

落下中のテトリミノには向きがあります。この向きは、上下方向の移動と混同させないため、方角で表すこととします。右回転ではNorth -> East -> South -> West -> Northという順で回転します。また、左回転は右回転と逆順に変化します。Oミノに向きはなく回転しません。

ミノは最初Northの状態で降ってきます。ワールドルールでは、特にスーパーローテーションシステム等のために、ミノの向きと周辺の空きマスに密接な関係があります。以下はミノと向きの関係を2次元配列で表したものです、しっかりと覚えておいてください。一番左上のマスの座標をテトリミノの座標として扱ってください。

IミノNorthここから
[
    [false, false, false, false],
    [true, true, true, true],
    [false, false, false, false],
    [false, false, false, false]
]
IミノNorthここまで

IミノEastここから
[
    [false, false, true, false],
    [false, false, true, false],
    [false, false, true, false],
    [false, false, true, false]
]
IミノEastここまで

IミノSouthここから
[
    [false, false, false, false],
    [false, false, false, false],
    [true, true, true, true],
    [false, false, false, false]
]
IミノSouthここまで

IミノWestここから
[
    [false, true, false, false],
    [false, true, false, false],
    [false, true, false, false],
    [false, true, false, false]
]
IミノWestここまで

TミノNorthここから
[
    [false, true, false],
    [true, true, true],
    [false, false, false]
]
TミノNorthここまで

TミノEastここから
[
    [false, true, false],
    [false, true, true],
    [false, true, false]
]
TミノEastここまで

TミノSouthここから
[
    [false, false, false],
    [true, true, true],
    [false, true, false]
]
TミノSouthここまで

TミノWestここから
[
    [false, true, false],
    [true, true, false],
    [false, true, false]
]
TミノWestここまで

SミノNorthここから
[
    [false, true, true],
    [true, true, false],
    [false, false, false]
]
SミノNorthここまで

SミノEastここから
[
    [false, true, false],
    [false, true, true],
    [false, false, true]
]
SミノEastここまで

SミノSouthここから
[
    [false, false, false],
    [false, true, true],
    [true, true, false]
]
SミノSouthここまで

SミノWestここから
[
    [true, false, false],
    [true, true, false],
    [false, true, false]
]
SミノWestここまで

ZミノNorthここから
[
    [true, true, false],
    [false, true, true],
    [false, false, false]
]
ZミノNorthここまで

ZミノEastここから
[
    [false, false, true],
    [false, true, true],
    [false, true, false]
]
ZミノEastここまで

ZミノSouthここから
[
    [false, false, false],
    [true, true, false],
    [false, true, true]
]
ZミノSouthここまで

ZミノWestここから
[
    [false, true, false],
    [true, true, false],
    [true, false, false]
]
ZミノWestここまで

JミノNorthここから
[
    [true, false, false],
    [true, true, true],
    [false, false, false]
]
JミノNorthここまで

JミノEastここから
[
    [false, true, true],
    [false, true, false],
    [false, true, false]
]
JミノEastここまで

JミノSouthここから
[
    [false, false, false],
    [true, true, true],
    [false, false, true]
]
JミノSouthここまで

JミノWestここから
[
    [false, true, false],
    [false, true, false],
    [true, true, false]
]
JミノWestここまで

LミノNorthここから
[
    [false, false, true],
    [true, true, true],
    [false, false, false]
]
LミノNorthここまで

LミノEastここから
[
    [false, true, false],
    [false, true, false],
    [false, true, true]
]
LミノEastここまで

LミノSouthここから
[
    [false, false, false],
    [true, true, true],
    [true, false, false]
]
LミノSouthここまで

LミノWestここから
[
    [true, true, false],
    [false, true, false],
    [false, true, false]
]
LミノWestここまで

ここまでのルールは大丈夫ですか？

(テトリミノの種類、向き、座標を与えるので空のフィールドに反映させてください、とかやる...?)

# Analyzer

ChatGPTさんがテトリスをプレイできるように、その下準備として、これからいくつかの命令を定義していきます。

まず最初の命令はAnalyzeです。この命令を実行してほしい場所では、「Analyzeしてください」などのように表現します。フィールドを解析してほしいためこのような名前としています。

Analyzeの入力: 前述した21行10列のフィールド
Analyzeの出力: 下記フォーマットのJSONと、フィールドの分析考察。()内は型を自然言語で表しています。

- field: フィールドの状況を配列に起こしてください。各行に番号も振ってください。「埋まっているマス、すなわち『`.`や`@`』ではないマス」はtrue、「埋まっていないマス、すなわち『`.`または`@`』のマス」はfalseです。全てがfalseの行は出力しないでください。`@`のマスは現在操作中のテトリミノのマスですから埋まっておらずfalseとするべきことに十分に気をつけてください。(booleanの2次元配列)
- current: 現在操作中のテトリミノの種類、方角と座標を記述してください。この情報は、先程列挙したテトリミノの向きに関する情報と照らし合わせることで得られるはずです。テトリミノは複数行に渡るので、早とちりせず複数の行を解析した後に操作中のテトリミノを判断しましょう。横にしたTミノとLミノ、Jミノは間違いやすいです。`@`の位置関係は横と縦両方を見て気をつけて確認してください。方角と座標が複数考えられる場合はペアですべて列挙してください。(下記構造体)
  - mino_kind: テトリミノの種類(文字列)
  - dirpos: 方角と座標の配列(下記構造体の配列)
    - direction: 向き(文字列)
    - position: フィールド左上の座標を{ x: 0, y: 0 }としたときのテトリミノの座標。xは行数でyは列数(行数と列数の構造体)

- 考察: 埋まっている行に関する現状整理。例は後述します。JSONの外に書いてください。

以下の例は一番最初に出したフィールドを入力としたものです。

```出力例
{
    "field": [
        [19, [true, false, false, false, false, false, false, false, false, true]],
        [20, [true, true, true, false, false, false, false, true, true, true]]
    ],
    "current": {
        "mino_kind": "S",
        dirpos: [
            {
                "direction": {
                    "x": 5,
                    "y": 7,
                },
                "position": "North"
            },
            {
                "direction": {
                    "x": 4,
                    "y": 7
                },
                "position": "South"
            }
        ],
    },
}
考察: 19行目からブロックがあります。19行目は両端が2マスのみ埋まっていますが、残りは空いておりほとんど揃っていません。20行目は、両端から3マスが埋まっており真ん中に横連続空き4マスがあります。空いている箇所の真上も空いているため、ここにIミノをNorthまたはSouthの向きで置けばラインを消すことができそうです。
```

考察には、fieldの埋まっているマス(trueのマス)が存在する各行を1行ずつ丁寧に調べたのち、行同士の上下関係も調べ、最終的に、どの向きにどのテトリミノを置けばよいかといった、ラインを消すのに役立つ情報を記述してください。先程記述した例はあくまでも例であるため記述がただ似通うといったことがないように気をつけ、どのマスが埋まっているかについてよく注意して記述してください。ラインを消すことに注力すると、埋まっているマスがある最も上部の行(言い換えると、x座標が小さい行)について最もよく注意して記述することが大切です。

各行で連続して空いているマス(横連続空きマス)にテトリミノを上手く入れることで行を消せる可能性が高くなります。以下、連続して空いているマスの数とどういったテトリミノ及びその向きを落とせば消せる可能性が高いかの関係を列挙します。

横連続空き1マス: 「East、South、Westの向きのTミノ」、「East、Westの向きのIミノ」、「East、Westの向きのZミノ」、「East、Westの向きのSミノ」、「West向きのLミノ」、「East向きのJミノ」

横連続空き2マス: 「North、Southの向きのSミノ」、「North、South向きのZミノ」、「West向きのJミノ」、「East向きのLミノ」、「すべての向きのOミノ」

横連続空き3マス: 「Northの向きのTミノ」、「Northの向きのLミノ」、「Northの向きのJミノ」

横連続空き4マス以上: 「North、Southの向きのIミノ」

ラインを消すことは叶いませんが、揃えるためにn > mについて横連続空きnマスの場所に横連続空きmマスのテトリミノを置くのは候補として有効です。

候補を絞る際には、今考えている行、空白マスの上にあるマスに注目してください。空いているマスに、候補のミノを候補の向きに実際に置けるかを、ミノの形状から判断し、無理ならば候補として出さないようにしてください。横連続空きマスがあっても、上が埋まってしまっている場合は候補を考えても意味がない点に注意してください。

横連続空きマスだけではなく、縦に連続して空いているマスについても、同様に考えて候補を挙げることが可能です。

縦連続空き2マス: 「East、Westの向きのIミノ」、「Eastの向きのJミノ」、「Westの向きのLミノ」

縦連続空き3マス以上: 「East、Westの向きのIミノ」

以上、とりあえずラインを消すことだけに注力した場合に考えるべき事項を挙げましたが、「4行同時消し(テトリス)を狙うために縦に連続した穴を作る」といった別な戦略を導入し拡張しても良いです。そういった戦略を取る際には考察もその戦略に合ったものにしましょう。

考察は停止せず最後まで書いてください。ここまでがAnalyze命令の説明になります。

では、早速フィールドを与えるので、与えられたフィールドをAnalyzeしてください。

- 「思考している」のではなく「一番矛盾がない回答をしている」だけに見える
- テトリミノの判断が苦手みたい...判断するためのアルゴリズムを与える?
- 行省略と行番号付与をしたほうがいいかも
- 結構指示を覚えていないのでアルゴリズムを説明する時には都度必要なものをすべて書いてあげたほうが良さそう

- 時間経過とともにルールを忘れている説がある...?
- テトリミノの種類、向きの認識が未だに怪しい
- 数を考えるのが苦手らしい
- 与えるフィールドのフォーマットも考え直すか

ここからは、ChatGPTさんに答えてもらいたいコマンドに関する説明をします。わかりやすさのためMarkdownのテーブルで与えます。

|コマンド|意味|
|:-------|:---|
|R|右回転|
|L|左回転|
|V|テトリミノが下方向に1下がります。|
|>|テトリミノが右方向に1動きます。|
|<|テトリミノが左方向に1動きます。|
|H|現在のテトリミノをホールド(Hold)します。|
|;|StatusがDroppingの時は、テトリミノを下に移動できるだけ移動します。StatusがGroundedの時はテトリミノを固定し、新しいテトリミノを降らせます。ハードドロップに近い動作です。|



;コマンドの扱いには注意してください。テトリミノがまだ下に動ける時は下まで降りる動作のみで、テトリミノがもう下に動けない時にテトリミノが固定されるコマンドです。もし単にハードドロップしたいだけであれば;;のように2つ連続で記述してください。1つ目の;で一番下まで落ち、2つ目の;で固定が行われます。

スーパーローテーションシステムなど、その他ルールの詳細は、ワールドルールに則るものとします。

ChatGPTさんには以下の項目をJSONで返していただきたいです。

- 現在の"状況"の解析
- 目標達成のために取るべき"行動"
- 行動の"理由"
- 以下内容の配列 "command候補"
    - フィールド上でのテトリミノの"目標地点"。フィールドを配列と文字列で表現し、目標地点でテトリミノを固定する場合はテトリミノの文字で、固定しない場合は#で示してください。
    - 目標地点への配置を実現する"command"
    - commandの"実行結果"。ライン消去は行わないでください
    - commandの実行結果が最初に決めた目標地点と一致するかの"判定"。一致する際はtrue, 一致しない際はfalse
    - 判定で一致しなかった場合に何が問題であったかの"考察"。一致した場合は空文字列にしてください。

command候補はそれまでに出力した内容を踏まえて出力してください。

command候補配列は、判定がtrueになるか、要素が10個になるまで出力してください。判定で一致しなかった場合、次に出力するcommandは考察を参考に改善を試みてください。

command候補の最後のcommandを実行した後のフィールド状況を返します。

説明には座標を使用して構いません。プログラミングにおける配列の表現に合わせ0オリジンとし、(x, y)はx列目のy行目を表します。

以下は私とChatGPTさんのやりとりの例です。

私:
.....#....
....###...
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
J.........
JJJ...IIII
==========
Current: T
Hold: -
Next: L O S
Status: Dropping

目標: 1ライン消し
Score: 0
Lines: 0
feedback: -

ChatGPT:
{
    "状況": "Tミノが降り始めており、(0, 4)にあります。Holdはありません。最下段の左から4番目から3つ穴が空いており、そこにTミノを入れればラインが消せそうです。",
    "行動": "Tミノを(18, 3)に移動させ最下段を固定します。",
    "理由": "これで最下段を消すことができるためです。",
    "command候補": [
        {
            "目標地点": [
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "J...T.....",
                "JJJTTTIIII",
            ],
            "command": "<<;;",
            "実行結果": [
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "...T......",
                "J.TTT.....",
                "JJJ...IIII",
            ],
            "判定": false,
            "考察": "左に移動させすぎました。"
        },
{
            "行き先": [
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "J...T.....",
                "JJJTTTIIII",
            ],
            "command": "<;;",
            "実行結果": [
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "..........",
                "J...T.....",
                "JJJTTTIIII",
            ],
            "判定": true,
            "考察": ""
        },
    ]
}

私:
......#...
....###...
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
J...T.....
==========
Current: L
Hold: -
Next: O S Z
Status: Dropping

目標: 1ライン消し
Score: 119
Lines: 1
feedback: -
ここより上の行までがやり取りの例です。

次は特にChatGPTさんに守っていただきたい事項です。

- プレイするのは私ではなくChatGPTさんです。コマンドを返答してください。
    - 「申し訳ありませんがプレイできません」や「最初のコマンドをお知らせください」みたいな返答はダメです。そういう時は`;`を返してください。
- 1ライン以上消せることをChatGPTさんの勝利条件かつ目標とします。
- 勝つまでコマンドを返答し続けてください。

では以下の状態からゲームスタートです。

TODO
- 東西南北で書いてたけど考えてみればこれはテトリミノの向きだったので修正したい..と思ったけどやっぱ回転と衝突するし直す必要ないかも: <>Vでどう？
- 操作の具体例をもっと増やす
- 操作後の盤面を予想させる。(間違っていたら手戻りさせる...?)

- Step By Stepよりも例示の方が効果があった。
- それまでの出力にかなり引っ張られるみたいなので、いきなり答えを求めるのではなくて
- やはり赤子を相手にするように簡単な問題から解かせるべきかもしれない。

- 実行させるのも大変みたい。こっちが作ったルールをまず理解しているかを問うたほうがいいかも。
- 「命令を解釈する」などにアルゴリズムを付けさせる手は有効かも