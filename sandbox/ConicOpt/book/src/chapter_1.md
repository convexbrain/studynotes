# 錐線形計画問題

## 主問題

\\[
    \begin{array}{l}
    {\rm minimize} & c^Tx \\\\
    {\rm subject\ to} & Ax \preceq_\mathcal{K} b
    \end{array}
\\]
ここで、
* 変数 \\(x\in{\bf R}^n\\)
* \\(c\in{\bf R}^n,\ A\in{\bf R}^{m\times n},\ b\in{\bf R}^m\\)
* 閉凸錐 \\(\mathcal{K}\ne\emptyset\\)

とする。
また \\(\preceq_\mathcal{K}\\) の関係は
\\[
    x\preceq_\mathcal{K}y \Longleftrightarrow
    0\preceq_\mathcal{K}y-x \Longleftrightarrow
    y-x\in\mathcal{K}
\\]
であり、スラック変数 \\(s\in{\bf R}^m\\) を導入して主問題は
\\[
    \begin{array}{l}
    {\rm minimize} & c^Tx \\\\
    {\rm subject\ to} & Ax + s = b \\\\
    & s \in \mathcal{K}
    \end{array}
\\]
と書くことができる。

## 双対問題

双対変数あるいはラグランジュ乗数を \\(y\\) とし、
ラグランジアン
\\[
    L = c^Tx + y^T(Ax+s-b)
\\]
を導入する。
ラグランジュ双対をとるために \\(\inf_{x,s\in\mathcal{K}} L\\) を評価するが、
\\[
    L = (c + A^Ty)^Tx + y^Ts - b^Ty
\\]
から、
* \\(c + A^Ty \ne 0\\) のとき、適当な \\(x\\) でいくらでも小さくできる
* ある \\(y,s\\) で \\(y^Ts<0\\) となるとき、
  \\(\mathcal{K}\\) は錐なので任意の \\(\lambda>0\\) に対して \\(\lambda s\\) も \\(\mathcal{K}\\) に含まれ、
  \\(y^T\lambda s\\) はいくらでも小さくできる
  * なお \\(\mathcal{K}\\) は閉錐なので \\(0\in\mathcal{K}\\)、よって \\(y^Ts=0\\) となりうる

ことがわかる。

したがって \\(\inf_{x,s\in\mathcal{K}} L > -\infty\\) のためには
* \\(c + A^Ty = 0\\)
* \\(y^Ts \ge 0,\ \forall s\in\mathcal{K}\\)、これは双対錐の定義と一致するので \\(y\in\mathcal{K}^*\\)

となる必要があり、このとき \\(\inf_{x,s\in\mathcal{K}} L = -b^Ty\\) である。

結果として、\\(g(y)=\inf_{x,s\in\mathcal{K}} L\\) を最大化するラグランジュ双対問題をとると
\\[
    \begin{array}{l}
    {\rm maximize} & -b^Ty \\\\
    {\rm subject\ to} & -A^Ty = c \\\\
    & y \in \mathcal{K}^*
    \end{array}
\\]
が得られる。

錐線形計画問題は凸最適化問題であり、制約想定（スレーターの条件など）の下で強双対性が成立し、
双対ギャップがゼロ、つまり主問題と双対問題の最適値が一致する。
