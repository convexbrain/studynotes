# TV最小化アルゴリズム

## もう少し厳密な定義

$$u$$ のTotal Variationは正式には

$$
\int|\nabla u|dx = \max_{g^2\le1}\int u \ \nabla\cdot g \ dx
$$

となるそうです（ [TVについての補足](TVSupp) 参照）。
$$g=g(x)=(g_1(x_1,x_2),g_2(x_1,x_2))$$ は、2次元平面上に定義され、2次元縦ベクトルを値として持つ関数です。
$$g^2\le1$$ の制約があるので、半径1の円盤内のベクトルになります。

左辺に比べて、右辺は微分不可能な $$u$$ にも適用できるので、より一般的な定義を与えていることが分かります。
ただし、別に以下で $$u$$ が微分不可能でもOKじゃないと困るというよりも、目的関数に絶対値や偏微分が含まれていないのが扱いやすいだけかなと思います。

## 問題の再掲と変形

さて本題のTV最小化問題は、

$$
\min_u \left(
  \left( \max_{g^2\le1}\int u \ \nabla\cdot g \ dx \right) +
  \frac1{2\lambda}\int(u-u_0)^2dx
\right)
$$

となります。
最小化目的関数の第2項は $$g$$ に関係ないので、

$$
\min_u \max_{g^2\le1} \left(
   \int u \ \nabla\cdot g \ dx +
   \frac1{2\lambda}\int(u-u_0)^2dx
\right)
$$

としても一緒です。
さらに、ここで $$\min$$ と $$\max$$ を入れ替えてもいいらしいです。

$$
\max_{g^2\le1} \min_u \left(
  \int u \ \nabla\cdot g \ dx +
  \frac1{2\lambda}\int(u-u_0)^2dx
\right)
$$

おそらくミニマックス定理によるものと思われます。
目的関数は $$u$$ について凸関数かつ $$g$$ についてリニア（つまり凹関数）で、
$$g^2\le1$$ も凸集合なので、ミニマックス定理により鞍点を持ちます。
鞍点を持つとすなわち上の $$\min\max=\max\min$$ が保証されます。

大カッコ内の目的関数を $$E(u)$$ とおきます。
$$u$$ は $$x$$ の関数なので、$$E$$ は関数の関数、つまり汎関数であることに注意しましょう。
汎関数 $$E$$ は積分の形をしているので、その極値（停留値）問題を扱うのに変分法を使います。

$$u(x)\rightarrow u(x)+\delta u(x)$$ としたときの $$E$$ の第1変分（$$\delta u$$ の2次の項は無視です）は

$$
\delta E=\int\delta u(\nabla\cdot g + \frac1\lambda(u-u_0))dx
$$

となります。
したがって、任意の関数 $$\delta u$$ に対して恒等的に $$\delta E=0$$ となるためには、

$$
u=u_0 - \lambda\nabla\cdot g
$$

を満たす必要があります。
これを $$E$$ に代入して

$$
\min_u E=\int u_0 \ \nabla\cdot g \ dx
-\frac\lambda2\int(\nabla\cdot g)^2dx
$$

が求まります。
この $$\min_u E$$ を改めて $$-F(g)$$ とおき $$\max_{g^2\le1}(-F)=\min_{g^2\le1}F$$ を考えます。
（$$g$$ が求まれば $$u=u_0 - \lambda\nabla\cdot g$$ で問題の $$u$$ が出ます。）

## 最適性条件

制約条件 $$g(x)^2\le1$$ を後の計算の便宜上 $$\frac\lambda2(g(x)^2-1)\le0$$ とし、これに対するラグランジュ乗数を $$\alpha(x)$$（これも関数！）とします。

ラグランジアンは（$$x$$ が離散で有限個の値しかとらないなら）$$L=F+\sum_{\forall x}\alpha\frac\lambda2(g^2-1)$$ となりそうですが、今 $$x$$ は連続なので、和の代わりに積分にします：

$$
L(g,\alpha)=F(g)+\int\alpha\frac\lambda2(g^2-1)dx
$$

KKT条件から、$$\min_{g^2\le1}F$$ の最適性条件は以下のようになります：

* $$g$$ の変分 $$\delta g$$ に対する $$L$$ の変分 $$\delta L=0$$
* $$g^2\le1,\ \alpha \ge0$$　
* $$\alpha(g^2-1)=0$$　

では $$\delta L$$ を計算しましょう。

$$
\delta L=-\int u_0\nabla\cdot\delta g \ dx
+\lambda\int(\nabla\cdot g)(\nabla\cdot\delta g)dx
+\lambda\int\alpha \ \delta g\cdot g \ dx
$$

ここで、[TVについての補足](TVSupp) から

$$
\int\phi \ \nabla\cdot\delta g \ dx = -\int \delta g\cdot\nabla\phi dx
$$

なので、

$$
\delta L=\int \delta g\cdot(
\nabla u_0-\lambda\nabla(\nabla\cdot g)+\lambda\alpha g
)dx
$$

となります。

$$\delta L=0$$ より、
$$H_\lambda(g)=\nabla(\nabla\cdot g - u_0/\lambda)$$ とおくと、
$$H_\lambda(g)=\alpha g$$ です。
両辺絶対値とってKKT条件に注意すると、

$$
|H_\lambda(g)|=\alpha|g| = \left\{
\begin{array}{ll}
0 \ (g^2<1) \\
\alpha \ (g^2=1)
\end{array}
\right.
$$

したがっていずれにせよ $$\alpha=|H_\lambda(g)|$$ とできます。
結局最適性条件は

$$
\begin{array}{ll}
H_\lambda(g)-|H_\lambda(g)|g=0   \\
g^2\le1
\end{array}
$$

とまとめられます。

## 半陰（semi-implicit）最急降下法

上記最適性条件を満たす $$g$$ を求めるために、ラグランジアンに対する最急降下法を踏まえ、汎関数 $$L(g,\alpha)$$ に対する最急降下法を考えます。

$$g(x)$$ がある点 $$x_o$$ だけにおいて変化すると考えます。
すると変分 $$\delta g$$ は、ディラックのデルタ関数を用いて
$$\delta g(x) = \delta(x-x_o)$$
と表すことができます。
このとき $$ \delta L=(-\lambda H_\lambda(g)+\lambda\alpha g)|_{x=x_o} $$ となるので、点 $$x_o$$ における最急降下法は

$$
g(x_o) \leftarrow g(x_o) - \tau(-\lambda H_\lambda(g)+\lambda\alpha g)|_{x=x_o}
$$

と書けます（ $$\tau$$ はステップパラメータ）。

改めて $$\tau\lambda$$ を $$\tau$$ とし、$$x_o$$ を $$x$$ と書き直します。
また $$\alpha$$ は最終的に $$|H_\lambda(g)|$$ にならなければならないので、
降下進行中もその値に固定してしまいましょう：

$$
g \leftarrow g + \tau(H_\lambda(g)-|H_\lambda(g)|g) \ \forall x
$$

さてここで普通なら右辺の $$g$$ を $$g^t$$、左辺の $$g$$ を $$g^{t+1}$$ として反復計算するところですが、Chambolleさんは

$$
g^{t+1} \leftarrow g^t + \tau(H_\lambda(g^t)-|H_\lambda(g^t)|g^{t+1})
$$

を考えました。こういう $$g$$ のまぜこぜな割り当て方を半陰と言うようです。
つまり

$$
g^{t+1} = { g^t + \tau H_\lambda(g^t) \over 1+\tau|H_\lambda(g^t)| }
$$

であり、$$\tau\le1/8$$ の条件で収束することも証明なさっています（実用上は $$\tau\le1/4$$ でいいらしい）。

このアルゴリズムで $$g$$ が決まり、追って $$u=u_0 - \lambda\nabla\cdot g$$ により $$u$$ が求まります。
