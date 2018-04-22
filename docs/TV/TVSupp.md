---
layout: ext
---
# TVについての補足

## 式変形

関数 $$\phi(x)$$ の、厳密な定義によるTotal Variation
$$
\max_{g^2\le1}\int\phi(x) \ \nabla\cdot g(x) \ dx
$$
の中の積分を変形したいと思います。

そこで $$\nabla\cdot(\phi g)$$ について考えます。
ベクトル成分を書き下すと
$$
\nabla\cdot(\phi g) =
  {\partial\over\partial x_1}(\phi g_1) +
  {\partial\over\partial x_2}(\phi g_2)
$$
であり、積分してグリーンの公式を適用すると
$$
\begin{array}{ll}
\int\int\left({\partial\over\partial x_1}(\phi g_1)
             +{\partial\over\partial x_2}(\phi g_2)\right)dx_1dx_2
&=\oint(\phi g_1dx_2-\phi g_2dx_1) \\
&=\oint\phi g\cdot dn
\end{array}
$$
となります。
周回積分は、$$x$$ 平面上で関数（画像）$$\phi$$ が定義されている領域の境界線上で取ります。
$$dn=(dx_2,-dx_1)$$ はその境界線の線素を直角に回転させたものです。

簡単のため周回積分をゼロにしたいので、$$g$$ は境界線上で境界線に対して平行（ $$g\cdot dn=0$$ ）と仮定します。 
したがって、結局
$$
\int\nabla\cdot(\phi g)dx=0
$$
です。
一方で積の微分を考えると
$$
\nabla\cdot(\phi g) = \phi \ \nabla\cdot g+g\cdot\nabla\phi
$$
なので、
$$
\int\phi(x) \ \nabla\cdot g(x) \ dx = -\int g(x)\cdot\nabla\phi(x) dx
$$
が成り立ちます。

## 式評価

先の式変形により、
$$
  \max_{g^2\le1}\int\phi \ \nabla\cdot g \ dx
= \max_{g^2\le1} \ -\int g\cdot\nabla\phi dx
$$
となります。
ここでベクトル $$g$$ は半径1の円盤上にあるので $$g \leftrightarrow -g$$ としても一緒です：
$$
  \max_{g^2\le1}\int\phi \ \nabla\cdot g \ dx
= \max_{g^2\le1}\int g\cdot\nabla\phi dx
$$

さて最大値をとるとき、幾何学的に考えれば、ベクトル $$g$$ が全ての点 $$x$$ においてベクトル $$\nabla\phi$$ と平行で、大きさが1であればいいと思われます。
（もちろんこれは厳密な考え方ではありません。
たとえばそのような $$g$$ が微分可能で連続な関数として存在するかどうか？など無視しています。）
このとき $$g\cdot\nabla\phi=|\nabla\phi|$$ なので、
$$
  \max_{g^2\le1}\int\phi \ \nabla\cdot g \ dx
= \int|\nabla\phi|dx
$$
となり、[Total Variation](TV) の最初の定義に戻りました。

厳密な証明はどうやら難しそうなので、まだ見ることすらしていません！
