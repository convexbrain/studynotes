---
layout: ext
---
# 主双対内点法

[Convex Optimization by Stephen Boyd and Lieven Vandenberghe](http://stanford.edu/~boyd/cvxbook/)
のBookの**11.7 Primal-dual interior-point methods**について、
この節だけ読んでも思い出しづらい（数式参照などが他の節に渡ってしまっている）ので、
問題定義～アルゴリズム部分までself-containedになるようまとめておこうと思います。
（導出のまとめはいつか・・・）

## 不等式制約つき最小化問題

以下の $$ x \in {\bf R}^n $$ に対する最適化問題が対象です。

$$
\begin{array}{ll}
{\rm minimize}     & f_0(x) \\
{\rm subject \ to} & f_i(x) \le 0, \quad i = 1, \ldots, m \\
                   & Ax = b
\end{array}
$$

ここで、

* $$ f_0, \ldots, f_m: {\bf R}^n \rightarrow {\bf R} $$ は凸で2階連続微分可能
* $$ A \in {\bf R}^{p \times n} $$ で $$ {\bf rank} \ A = p < n $$
* 厳密な実行可能点 （実行可能点のうち $$ f_i(x) < 0 \quad (i = 1, \ldots, m) $$ が成立する点）を持つ

を前提条件とします。

なお上記を主問題として、双対問題

$$
\begin{array}{ll}
{\rm maximize}     & g(\lambda, \nu) = \inf_{x \in {\cal D}} L(x, \lambda, \nu) \\
{\rm subject \ to} & \lambda \succeq 0
\end{array}
$$

* $$ \lambda \in {\bf R}^m, \ \nu \in {\bf R}^p $$　
* $$ {\cal D} $$ ： $$ f_0, \ldots, f_m $$ の定義域の共通部分の集合
* $$ f(x) = (f_1(x) \ \cdots \ f_m(x))^T $$ のベクトルとして、ラグランジアン $$ L(x, \lambda, \nu) = f_0(x) + \lambda^T f(x) + \nu^T (Ax - b) $$

をとることができます。

前提条件（凸最適化問題で厳密な実行可能点を持つ）により、強双対性を持ち、主問題・双対問題の最適値が等しくなります。
そしてKKT条件

$$
\begin{array}{rcl}
f(x)                        & \preceq & 0 \\
Ax                          & =       & b \\
\lambda                     & \succeq & 0 \\
\lambda^T f(x)              & =       & 0 \\
\nabla_x L(x, \lambda, \nu) & =       & 0
\end{array}
$$

は最適性の必要十分条件になります。

## Primal-dual interior-point method

### 初期値

* $$ x \in {\cal D} $$ ： $$ f(x) \prec 0 $$ を満たす点。$$ Ax=b $$ である必要はありません。
* $$ \lambda \succ 0 $$　

$$ \nu $$ には初期値として満たすべき条件はありません。

### パラメータ

* $$ \mu > 1 $$ ： KKT条件4つ目の相補性条件の影響を強めてゆく係数、10のオーダーの値
* $$ \epsilon_{\rm feas} > 0 $$ ： 実行可能性に対する残差（最適性条件からの離れ具合）の許容値、終了判定に使用、$$10 ^{-8}$$ のオーダーの値 [^*1]
* $$ \epsilon > 0 $$ ： 代理双対ギャップ（主問題・双対問題の目的関数値の差分に相当）の許容値、終了判定に使用、$$10 ^{-8}$$ のオーダーの値 [^*1]

[^*1] 計算機イプシロンの平方根くらいが良さそうです

* $$ \alpha \in (0, 0.5) $$ ： バックトラッキングラインサーチにおける残差減少係数、通常0.01～0.1
* $$ \beta \in (0, 1) $$ ： バックトラッキングラインサーチにおける反復係数、通常0.3～0.8

### アルゴリズム

1. $$ t := \mu m / {\hat \eta} $$ のセット
  * ここで $$ {\hat \eta} = -f(x)^T \lambda $$ ： 代理双対ギャップ
1. 探索方向 $$ \Delta x_{\rm pd}, \Delta \lambda_{\rm pd}, \Delta \nu_{\rm pd} $$ の計算
  * 以下の線形連立方程式の解
$$
  \left[ \array{
    \nabla^2 f_0(x) + \sum_{i=1}^m \lambda_i \nabla^2 f_i(x) & Df(x)^T           & A^T \\
    -{\bf diag}(\lambda) Df(x)                               & -{\bf diag}(f(x)) & 0 \\
    A                                                        & 0                 & 0
  } \right]
  \left[ \array{
    \Delta x       \\
    \Delta \lambda \\
    \Delta \nu
  } \right]
  = - \left[ \array{
    r_{\rm dual} \\
    r_{\rm cent} \\
    r_{\rm pri}
  } \right]
$$
  * ここで
    * $$ Df(x) = (\nabla f_1(x) \ \cdots \ \nabla f_m(x))^T $$ の行列（ヤコビアン）
    * $$ r_{\rm dual} = \nabla f_0(x) + Df(x)^T \lambda + A^T \nu $$ ： 主残差
    * $$ r_{\rm cent} = -{\bf diag}(\lambda) f(x) - (1/t) {\bf 1} $$ ： 中心残差
    * $$ r_{\rm pri} = Ax - b $$ ： 双対残差
1. バックトラッキングラインサーチで $$ x, \lambda, \nu $$ を更新
  * $$ s^{\max} = \sup \{ s \in [0, 1] \ \mid \ \lambda + s \Delta \lambda \succeq 0 \} $$ を求めて $$ s = 0.99 s^{\max} $$ をセット
  * $$ x^+       = x       + s \Delta x_{\rm pd},       \
       \lambda^+ = \lambda + s \Delta \lambda_{\rm pd}, \
       \nu^+     = \nu     + s \Delta \nu_{\rm pd}      $$ として
    1. $$ f(x^+) \prec 0 $$ となるまで $$ s := \beta s $$
    1. さらに、
    $$ ||r_t(x^+, \lambda^+, \nu^+)||_2 \le (1 - \alpha s) ||r_t(x, \lambda, \nu)||_2 $$ となるまで
    $$ s := \beta s $$
      * ここで
      $$ r_t(x, \lambda, \nu) = \left[ \array{
      r_{\rm dual} \\ r_{\rm cent} \\ r_{\rm pri}
      } \right] $$
  * $$ x := x^+, \ \lambda := \lambda^+, \ \nu := \nu^+ $$　
1. 終了判定 $$ ||r_{\rm dual}||_2 \le \epsilon_{\rm feas}, \
    ||r_{\rm pri}||_2 \le \epsilon_{\rm feas}, \
    {\hat \eta} \le \epsilon $$ を満たすまで繰り返し

## Phase I via infeasible start

上記のアルゴリズムでは、初期値で $$ f(x) \prec 0 $$ を満たす必要がありました。
このような $$ x $$ を求めるために前もって別途最適化問題（Phase Iと言う）を解いてもよいですが、
初期値で「$$ Ax=b $$ である必要はない」「$$ \nu $$ が満たすべき条件はない」ことから、
以下のようにすると一度に扱うことができます。

変数を $$ z \in {\bf R}^n, s \in {\bf R} $$ として、冒頭の最適化問題と同値な

$$
\begin{array}{ll}
{\rm minimize}_{z, s} & f_0(z) \\
{\rm subject \ to}    & f_i(z) \le s, \quad i = 1, \ldots, m \\
                      & Az = b \\
                      & s = 0
\end{array}
$$

を解くことを考えます。

$$ x \leftarrow (z \ \ s)^T $$ と置き換え拡張（$$ f, \nabla f, \nabla^2 f, A, b, \nu $$ も適宜拡張）して、
主双対内点法アルゴリズムを適用します。

$$ s $$ の初期値は $$ z \in {\cal D} $$ の初期値から $$ s > \max f_i(z) $$ となる任意の値とします。
これで、元の最適化問題において実行可能でない点から開始することができます。


## 例

* [QCQP](QCQP)
* [SOCP](SOCP)

[ソースコード](https://github.com/convexbrain/Totsu/tree/master/solver/)
と
[ドキュメント](http://convexbrain.github.io/Totsu/PrimalDualIPM/html/)
