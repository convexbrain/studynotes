# Pock/Chambolleの一次法

先の[鞍点問題](./separable_problem.md#鞍点問題)の数値解法として、
[参考文献](./reference.md)[1]では以下（を含めた形）のアルゴリズムが提示されている。
\\[
    \begin{array}{l}
        x^{k+1}={\bf prox}^T\_G(x^k-TK^Ty^k) \\\\
        y^{k+1}={\bf prox}^\Sigma\_{F^\ast}(y^k+\Sigma K(2x^{k+1}-x^k))
    \end{array}
\\]

\\(T\in{\bf R}^{n\times n},\Sigma\in{\bf R}^{m\times m}\\) は対角要素がすべて正の対角行列であり（したがって正定値行列でもある）、
前処理行列としての役割をもつ。
\\[
    \begin{array}{l}
        T_{jj}=\frac1{\sum_{i=1}^m|K_{ij}|} & \quad(j=1,\ldots,n) \\\\
        \Sigma_{ii}=\frac1{\sum_{j=1}^n|K_{ij}|} & \quad(i=1,\ldots,m)
    \end{array}
\\]
と定めており、このとき
\\[
    \\|\Sigma^{\frac12} K T^{\frac12}\\|^2\le1
\\]
（左辺のノルムは作用素ノルム）が成立する。
また、この不等式が成立するとき上記アルゴリズムが収束する（解が存在すれば）ことが示されている。
**`Totsu`では、\\({\bf prox}\\) の計算を容易にするため対角要素にグルーピングを施すが、この不等式が成立するようにグルーピングしている。**

\\({\bf prox}^T\_G\\) は近接作用素であるが、\\(T^{-1}\\) による（標準でない）内積に誘導されたノルム
\\(\\|x\\|\_{T^{-1}}=\langle x,x\rangle\_{T^{-1}}^{\frac12}=\sqrt{x^TT^{-1}x}\\) を用いて定義される：
\\[
    {\bf prox}^T\_G(\hat x) = \arg\min_x G(x) + \frac12\\|x-\hat x\\|\_{T^{-1}}^2
\\]
なおMoreau分解により
\\[
    \hat x = {\bf prox}^\Sigma\_F(\hat x) + {\bf prox}^\Sigma\_{F^\ast}(\hat x)
\\]
が成り立つ。
**`Totsu`では、対角要素グルーピングにより標準でないノルムを考慮しなくてよく、また \\(G,F\\) を指示関数とするため近接作用素が単に射影となる。**
