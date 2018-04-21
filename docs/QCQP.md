---
layout: ext
---
# QCQP

二次制約二次計画問題(quadratically constrained quadratic program)を扱います。

## 問題定義

変数 $$ x \in {\bf R}^n $$ に対して

$$
\begin{array}{ll}
{\rm minimize}     & {1\over2} x^T P_0 x + q_0^T x + r_0 \\
{\rm subject \ to} & {1\over2} x^T P_i x + q_i^T x + r_i \le 0, \quad i = 1, \ldots, m \\
                   & Ax = b
\end{array}
$$

* $$ P_i \in {\bf S}_+^n, \ q_i \in {\bf R}^n, \ r_i \in {\bf R}, \quad i = 0, 1, \ldots, m $$
  * $$ {\bf S}_+^n $$ ： $$ n \times n $$ 半正定値対称行列の集合
  * $$ r_0 $$ は目的関数の定数オフセットなので意味のあるパラメータではない
* $$ A \in {\bf R}^{p \times n}, \ b \in {\bf R}^p $$
  * $$ {\bf rank} \ A = p < n $$

（線形不等式制約つきの）二次計画問題(QP)、最小二乗問題(LS)、線形計画問題(LP)を特殊ケースとして含みます。

## [主双対内点法](PrimalDualIPM.md)の適用

以下のように書けます。

$$
\begin{array}{rcl}
           f_i(x) &=& {1\over2} x^T P_i x + q_i^T x + r_i  \\
  \nabla   f_i(x) &=& P_i x + q_i  \\
  \nabla^2 f_i(x) &=& P_i
\end{array}
\qquad i = 0, 1, \ldots, m
$$

### [Phase I via infeasible start](PrimalDualIPM.md)の適用

変数を $$ z \in {\bf R}^n, s \in {\bf R} $$ として、同値な最適化問題

$$
\begin{array}{ll}
{\rm minimize}     & {1\over2} z^T P_0 z + q_0^T z + r_0 \\
{\rm subject \ to} & {1\over2} z^T P_i z + q_i^T z + r_i \le s, \quad i = 1, \ldots, m \\
                   & Az = b \\
                   & s = 0
\end{array}
$$

を考えることで、任意の $$ z $$ を初期値とすることができます。

$$ x \leftarrow (z \ \ s)^T $$ と置き換えると、
元のQCQPの $$ P, q, A, b $$ を以下のように置き換えることになります
（ $$ r $$ はそのまま）。

* $$ P_0 \leftarrow \left[ \matrix{ P_0 & 0 \\
                                    0   & 0 } \right ], \quad
     P_i \leftarrow \left[ \matrix{ P_i & 0 \\
                                    0   & 0 } \right ] $$
* $$ q_0 \leftarrow \left[ \matrix{ q_i \\
                                    0   } \right ], \quad
     q_i \leftarrow \left[ \matrix{ q_i \\
                                    -1  } \right ] $$
* $$ A \leftarrow \left[ \matrix{ A & 0 \\
                                  0 & 1 } \right ] $$
* $$ b \leftarrow \left[ \matrix{ b \\
                                  0 } \right ] $$

## ソースコード

[主双対内点法](PrimalDualIPM.md)参照
