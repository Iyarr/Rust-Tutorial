# 計算処理の設計

## make_rpm

この関数は、入力された計算式を逆ポーランド記法の計算式に書き換える関数。その処理内容を表す

- 引数の文字列が 1 文字以下だったら引数をそのまま返す
- 括弧が式の中にないか探す
  - あった場合はその括弧の中の式を引数に make_rpm 関数を呼び出す
  - 式の中にある演算子を加減算、乗算の順に探す
- 式の中に＋や－の演算子がないか探す
  - あった場合はその演算子を中心に二つの式に分割し、それぞれの式を引数に make_rpm 関数を呼び出す
  - 最終的な式を左側の式、右側の式、演算子の順番につなげる
- 式の中に\*や式がつながってる場所がないか探す
  - あった場合は二つの式に分割し、それぞれの式を引数に make_rpm 関数を呼び出す
  - 最終的な式を左側の式、右側の式、演算子の順番につなげる
- 演算子がなかった場合、引数をそのまま返す

## 設計メモ

単純な左辺と右辺と演算子に分けるアルゴリズムがいいと考えたけど、以下の形態で不具合が発生する

> 4 x 4 + 4 x 4 + 4

この式において 2 個目の＋演算子よりも先に 1 個目の x 演算子が評価されてしまう
