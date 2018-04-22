---
layout: ext
---
# Gaussian Mixture Model

## 定義

Gaussian Mixture Model（混合正規分布モデル）は以下のようなデータ生成モデルです。

* K個のクラスがあるとする。対応して、K個の正規分布が存在する。
* まず、K個からひとつランダムに選ぶ。
* 次に、選ばれた正規分布に従ってデータベクトル \(x_n\in {\bf R}^D\) を生成する。
* これを N回繰り返して \(X=(x_1,\ldots,x_N)\) を得る。

この \(X\) が観測データとなるわけです。
一方、\(x_n\) を生成したクラスはどれかを表す変数を \(y_n\) として、\(Y=(y_1,\ldots,y_N)\) とおきます（隠れ変数）。
\(y_n\) は、普通に考えれば、単に1からKのスカラ値を持てばいい気がします。
ですが後の式変形の便宜上、ひとつの要素だけが1のK次元ベクトルである（\(y_n\in\{0,1\}^K\)）とします。
なお \(y_n\) の \(k\) 番目の要素を \(y_{nk}\) と表記することにします。

さて、\(x_n\) がクラスkから生成されたと分かっている場合である条件付分布は
\(p(x_n|y_{nk}=1,\theta)=N(x_n|\mu_k,S_k)\)
と書けます。\(\mu_k,S_k\) はクラスkの正規分布の平均と分散（共分散行列）で、全パラメータ \(\theta\) の一部です。

またクラスはランダムに選ばれると言いましたが一様ではなく、つまり
\(P(y_{nk}=1|\theta)=\phi_k, \ \sum_k\phi_k=1\) とおいて、\(\phi_k\) も \(\theta\) の一部とします。

結局、全パラメータは
\(\theta=(\phi_1,\ldots,\phi_K,\mu_1,\ldots,\mu_K,S_1,\ldots,S_K)\)
からなり、これを学習用の観測データ \(X=(x_1,\ldots,x_N)\) から推定することを考えます。
推定にはEMアルゴリズムを使います。

## 完全データの確率分布

EMを適用するには \(p(X,Y|\theta)\) がわかっていればOKです。

まず、
\(
p(x_n,y_{nk}=1|\theta)
=P(y_{nk}=1|\theta)p(x_n|y_{nk}=1,\theta)
=\phi_k N(x_n|\mu_k,S_k)
\)
です。
次に \(y_n\) はひとつの要素だけが1であることから、
\[
p(x_n,y_n|\theta)
=\prod_k p(x_n,y_{nk}=1|\theta)^{y_{nk}}
=\prod_k (\phi_k N(x_n|\mu_k,S_k))^{y_{nk}}
\]
と書けることが分かります。

各 \( (x_n,y_n) \) はi.i.d.なので、単に \(p(x_n,y_n|\theta)\) を全てのnについて掛ければ \(p(X,Y|\theta)\) になります：
\[
p(X,Y|\theta)=\prod_n\prod_k (\phi_k N(x_n|\mu_k,S_k))^{y_{nk}}
\]

## Eステップ

[EMアルゴリズム](EM)より
\[
Q(\theta, \theta^t) = \sum_{Y\in\Omega_Y} P(Y|X,\theta^t) \ln p(X,Y|\theta)
\]
を求めます。
\(
\ln p(X,Y|\theta) = \sum_n\sum_k y_{nk}(\ln\phi_k+\ln N(x_n|\mu_k,S_k))
\)
を代入し、\(\sum\) の順番を入れ替えて、
\[
Q(\theta, \theta^t) = \sum_n\sum_k \left[
(\ln\phi_k+\ln N(x_n|\mu_k,S_k))
  \left(
  \sum_{Y\in\Omega_Y} P(Y|X,\theta^t) y_{nk}
  \right)
\right]
\]
とします。

ここで、\(\sum_Y\) の項は、既知の \(X,\theta^t\) の元での \(y_{nk}\) の期待値なので、\(E(y_{nk})\) と置きます。
期待値計算なので、\(y_{nk}\) に関係する変数だけ考えればよいので、
\(
E(y_{nk})
=\sum_{y_{nk}=0,1} P(y_n|x_n,\theta^t)y_{nk}
=P(y_{nk}=1|x_n,\theta^t)
\)
となります。

ベイズの定理と周辺化で
\(
P(y_{nk}=1|x_n,\theta^t)
=p(x_n,y_{nk}=1|\theta^t)/p(x_n|\theta^t)
=p(x_n,y_{nk}=1|\theta^t)/\sum_i p(x_n,y_{ni}=1|\theta^t)
\)
が成り立つので、結局
$$
\begin{array}{ll}
Q(\theta, \theta^t) &= \sum_n\sum_k
(\ln\phi_k+\ln N(x_n|\mu_k,S_k))
E(y_{nk}) \\
E(y_{nk}) &=
{\phi_k N(x_n|\mu_k^t,S_k^t) \over \sum_i \phi_i^t N(x_n|\mu_i^t,S_i^t)}
\end{array}
$$
となります。

## Mステップ

[EMアルゴリズム](EM)と制約条件より
\[
\max_\theta Q(\theta, \theta^t)
\text{ under } \sum_k\phi_k=1
\]
を解いて \(\theta^{t+1}\) を決めます。
等式制約つき最適化問題なので、ラグランジュの未定乗数法を用います。
ラグランジアンは
\[
L(\theta,\lambda)=Q(\theta, \theta^t)+\lambda\left(1-\sum_k\phi_k\right)
\]
となります。

\(\phi_i\) についての極値条件から、
\[
{\partial\over\partial\phi_i}L=\sum_n\frac1{\phi_i}E(y_{ni})-\lambda=0
\ \rightarrow \,
\phi_i=\frac1{\lambda}\sum_nE(y_{ni})
\]
これを制約条件に代入しなおして \(\lambda\) を消去して、
\[
\sum_k\phi_k=1
\ \rightarrow \,
\lambda=\sum_n\sum_kE(y_{nk})=N
\ \rightarrow \,
\phi_i={\sum_nE(y_{ni}) \over N}
\]

\(\mu_i\) についての極値条件から、
\[
{\partial\over\partial\mu_i}L=\sum_nS_i^{-1}(x_n-\mu_i)E(y_{ni})=0
\ \rightarrow \,
\mu_i={\sum_n x_nE(y_{ni}) \over \sum_nE(y_{ni})}
\]

\(S_i^{-1}\) についての極値条件から、
\[
{\partial\over\partial S_i^{-1}}L=\sum_n \left(
\frac12 S_i -\frac12 (x_n-\mu_i)(x_n-\mu_i)^T
\right)E(y_{ni})=0
\ \rightarrow \,
S_i = { \sum_n (x_n-\mu_i)(x_n-\mu_i)^T E(y_{ni}) \over
        \sum_n E(y_{ni}) }
\]

ここで \(\ln N(x_n|\mu_k,S_k)\) のパラメータによる偏微分は、教科書みるなりググったりWikipedia先生に聞いてみたりしてください。

更新式をまとめると、
$$
\begin{array}{ll}
\phi_i^{t+1} &= {\sum_nE(y_{ni}) \over N}  \\
\mu_i^{t+1} &= {\sum_n x_nE(y_{ni}) \over \sum_nE(y_{ni})}  \\
S_i^{t+1} &= { \sum_n (x_n-\mu_i^{t+1})(x_n-\mu_i^{t+1})^T E(y_{ni}) \over \sum_n E(y_{ni}) }
\end{array}
$$
となります。

## 収束判定

\(\ln p(X|\theta)\) をチェックして、増分が適当に小さくなるまで更新を続けます。
$$
\begin{array}{ll}
\ln p(X|\theta)&=\ln \prod_n p(x_n|\theta) \\
&=\sum_n \ln \sum_k p(x_n,y_{nk}=1|\theta) \\
&=\sum_n \ln \sum_k p(y_{nk}=1|\theta) p(x_n|y_{nk}=1,\theta) \\
&=\sum_n \ln \sum_k \phi_k N(x_n|\mu_k, S_k) \\
\end{array}
$$
で計算できます。

## クラスタリング例

[GMMクラスタリング](GMMClustering) にて。
