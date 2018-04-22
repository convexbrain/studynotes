---
layout: ext
---
# SVMによる回帰

## 回帰問題

ハードマージンやソフトマージンでは $$y_i\in\{+1,-1\}$$ の2値判別問題を扱いましたが、SVMは $$y_i\in \mathbb R$$ の場合の回帰（または関数近似）にも適用することができます。

問題を改めて書き直します。
$$l$$ 個の入力ベクトル $$ x_i \ (i=1,\ldots,l) $$ と、各 $$x_i$$ に対する値 $$y_i$$ が学習データセットとして与えられたときに、この関係を近似する $$y=f(x)=wx+b$$ を求めることを考えます。
もちろんこのような線形近似だけでなく、[SVM](SVM) と同様にカーネルを適用することで自然に非線形関数近似に拡張できます。

## 線形 $$\epsilon$$-insensitive 損失

簡単に言えば誤差 $$|f(x_i)-y_i|$$ をゼロにできればいいわけですが、
そもそも本質的に、回帰は完全一致を目指す問題ではありません。
まず、観測されるデータにはノイズがのっていることが一般的です。
また、選択されたカーネルと、実際のデータ生成過程の性質とが一致しなければ、モデル化誤差が生じます。

そこではじめに、ある程度の誤差 $$\epsilon(>0)$$ は許すことにしましょう。
この条件を制約として、[SVM](SVM) 同様、重みベクトル $$w$$ のノルムを最小化します。
つまり

$$
\text{ find } w, b \text{ s.t. }
\begin{array}{ll}
\text{ minimize }
& \frac12w^2 \\
\text{ subject to }
& |(wx_i+b)-y_i| \le \epsilon
\end{array}
$$

です。
カーネルをうまく選べば、モデル化誤差はこれでなんとか収まりそうな感じです。
でもノイズはノイズ源によっては大きく外れたデータ点を生むこともあるので、このままでは解がみつからなくなる気がします。
というわけで次に [ソフトマージンSVM](SoftMarginSVM) と同様にスラック変数 $$\xi$$ を導入しましょう。
$$\xi$$ が（正の）値を持つときは、その分さらに誤差を許すことにして、一緒に最小化します。

$$
\text{ find } w, b \text{ s.t. }
\begin{array}{ll}
\text{ minimize }
& \frac12w^2 + C\sum_i\xi_i \\
\text{ subject to }
& |(wx_i+b)-y_i| \le \epsilon + \xi_i, \ \xi_i \ge 0
\end{array}
$$

改めて $$\xi$$ について見てみると、誤差 $$\mid(wx_i+b)-y_i\mid$$ が $$\epsilon$$ 以下ならば $$\xi_i=0$$ と取ること（ $$\epsilon$$-insensitive ）ができ、一方 $$\epsilon$$ を超える場合は超えた分に比例してだけペナルティ（線形損失）として $$\xi_i$$ が値を持つ必要があります。

## 双対形式の導出

制約条件の絶対値を正・負ではずして二通りの式になることに注意して、一般ラグランジアンは

$$
\begin{array}{ll}
L=
& \frac12w^2 + C\sum_i\xi_i \\
& + \sum_i p_i(wx_i+b-y_i-\epsilon-\xi_i) \\
& + \sum_i q_i(-wx_i-b+y_i-\epsilon-\xi_i) \\
& + \sum_i r_i(-\xi_i)
\end{array}
$$

となります。
$$p,q,r$$ をラグランジュ乗数としました。
$$L(w,b,\xi)$$ の最小値（極値）をとるために
$$ {\partial L \over \partial w} = 0,
   {\partial L \over \partial b} = 0,
   {\partial L \over \partial \xi_i} = 0 $$
として、

$$
\begin{array}{ll}
w = \sum_i(q_i-p_i)x_i = \sum_i\alpha_ix_i \\
0 = \sum_i(q_i-p_i) = \sum_i\alpha_i  \\
r_i = C-(p_i+q_i) = C-|\alpha_i|
\end{array}
$$

を得ます。
ここで $$q_i-p_i=\alpha_i$$ とおき、さらにKKT条件から実は $$p_iq_i=0$$ だとわかるので、 $$p_i+q_i=\sqrt{(q_i-p_i)^2}=|\alpha_i|$$ としました。
これらの関係を再度 $$L$$ に代入して最大化目的関数とし、ラグランジュ乗数が非負の制約条件をつけて、式を整理すると、以下の双対問題を導出できます。

$$
\text{ find } \alpha \text{ s.t. }
\begin{array}{ll}
\text{ maximize }
& \sum_i\alpha_iy_i -
  \epsilon\sum_i|\alpha_i| -
  \frac12\sum_{i,j}\alpha_i\alpha_jK(x_i,x_j)  \\
\text{ subject to }
& \sum_i\alpha_i=0, \ -C \ge \alpha_i \ge C
\end{array}
$$

（カーネル適用も [SVM](SVM) と同様に行いました。）
