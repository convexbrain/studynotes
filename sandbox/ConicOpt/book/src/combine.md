# 二手法の組合せ

錐線形計画問題を[Pock/Chambolleの一次法](./pock_chambolle.md)で解くことを考える。

まず[Homogeneous Self-dual Embedding](./selfdual_embed.md)を施した最適性条件を、少し変数名と形を変えて、以下に再掲する：
\\[
    \left[ \begin{matrix}
    0 & A^T & 0 & c\\\\
    -A & 0 & -I & b \\\\
    -c^T & -b^T & 0 & 0
    \end{matrix} \right]
    \left[ \begin{matrix}
    x_x \\\\ x_y \\\\ x_s \\\\ x_\tau
    \end{matrix} \right]
    \in
    \left[ \begin{matrix}
    \lbrace0\rbrace^n \\\\ \lbrace0\rbrace^m \\\\ {\bf R}\_+
    \end{matrix} \right]
    , \qquad
    x_x \in {\bf R}^n, \quad
    x_y \in \mathcal{K}^*, \quad
    x_s \in \mathcal{K}, \quad
    x_\tau \in {\bf R}\_+
\\]
そして
\\[
    K=\left[ \begin{matrix}
    0 & A^T & 0 & c\\\\
    -A & 0 & -I & b \\\\
    -c^T & -b^T & 0 & 0
    \end{matrix} \right], \qquad
    x=\left[ \begin{matrix}
    x_x \\\\ x_y \\\\ x_s \\\\ x_\tau
    \end{matrix} \right]
\\]
とおく。

\\(G\\) を \\(x_x \in {\bf R}^n,\ x_y \in \mathcal{K}^\ast,\ x_s \in \mathcal{K},\ x_\tau \in {\bf R}\_+\\) の指示関数とする：
\\[
    G(x)=
    I_{{\bf R}^n \times \mathcal{K}^\ast \times \mathcal{K} \times {\bf R}\_+}(x)=
    \left\lbrace \begin{array}{l}
    0 & ({\rm if}\ x \in {\bf R}^n \times \mathcal{K}^\ast \times \mathcal{K} \times {\bf R}\_+) \\\\
    \infty & ({\rm otherwise})
    \end{array} \right.
\\]
同様に \\(F\\) も指示関数
\\[
    F(y)=
    I_{\lbrace0\rbrace^{n+m} \times {\bf R}\_+}(y)=
    \left\lbrace \begin{array}{l}
    0 & ({\rm if}\ y \in \lbrace0\rbrace^{n+m} \times {\bf R}\_+) \\\\
    \infty & ({\rm otherwise})
    \end{array} \right.
\\]
とすることで、元の式を
\\[
    \begin{array}{l}
    {\rm minimize} & G(x) + F(Kx)
    \end{array}
\\]
と表すことができる。
これに[Pock/Chambolleの一次法](./pock_chambolle.md)を適用すればよい。

## 近接作用素と前処理行列

まず
\\[
    \begin{array}{l}
    {\bf prox}^\sigma\_{F^\ast}(\tilde y)
    &=& \tilde y - {\bf prox}^\sigma\_F(\tilde y) \\\\
    &=& \tilde y - \arg\min_y \left( F(y) + \frac12\\|y-\tilde y\\|\_{{\bf diag}(\sigma)^{-1}}^2 \right)
    \end{array}
\\]
ここで \\(F\\) は指示関数なので \\(\arg\min\\) はその集合への射影となるが、
\\({\bf diag}(\sigma)^{-1}\\) によってスケールされた距離にもとづく射影である。
しかし \\(\lbrace0\rbrace^{n+m} \times {\bf R}\_+\\) への射影は各要素で互いに独立に射影してよく、
結局 \\(\sigma\\) に依存せずに
\\[
    \begin{array}{l}
    {\bf prox}^\sigma\_{F^\ast}(\tilde y)
    &=& \tilde y - \Pi_{\lbrace0\rbrace^{n+m} \times {\bf R}\_+} (\tilde y) \\\\
    &=& \tilde y - \left[ \begin{matrix}
                   \lbrace0\rbrace^{n+m} \\\\ \max(\tilde y\_{n+m+1}, 0)
                   \end{matrix} \right] \\\\
    &=& \left[ \begin{matrix}
        \tilde y_1 \\\\ \vdots \\\\ \tilde y_{n+m} \\\\ \min(\tilde y_{n+m+1}, 0)
        \end{matrix} \right] \\\\
    \end{array}
\\]
となる。

次に
\\[
    \begin{array}{l}
    {\bf prox}^\tau\_{G}(\tilde x)
    &=& \arg\min_x \left( G(x) + \frac12\\|x-\tilde x\\|\_{{\bf diag}(\tau)^{-1}}^2 \right)
    \end{array}
\\]
は \\({\bf R}^n \times \mathcal{K}^\ast \times \mathcal{K} \times {\bf R}\_+\\) への射影であり、
\\({\bf R}^n\\) への射影（これは何もしなくてよいということ）、\\({\bf R}\_+\\) への射影はやはり \\(\tau\\) に依存せず行うことができる。
また、仮に \\(\mathcal{K} = \mathcal{K}_1 \times \cdots \times \mathcal{K}_k\\) とした場合、
\\(\mathcal{K}^\ast = \mathcal{K}^\ast_1 \times \cdots \times \mathcal{K}^\ast_k\\) となり、
各 \\(\mathcal{K}_i,\ \mathcal{K}^\ast_i\\\) への射影どうしは独立しているが、
各々の射影ひとつひとつは一般には \\(\tau\\) によるスケーリングに依存してしまう。

ここで、\\(\mathcal{K}_i\\)（または \\(\mathcal{K}^\ast_i\\) ）に対応する \\(\tau\\) の成分を
\\(\tau\_{i\_1},\ldots,\tau\_{i\_t}\\) と置く。
これらをすべて等しい値 \\(\tau_i\\) に置き換えれば、等方スケールとなるので
\\[
    {\bf prox}^\tau\_{G}(\tilde x) =
    \Pi\_{{\bf R}^n \times \mathcal{K}^\ast \times \mathcal{K} \times {\bf R}\_+} (\tilde x)
\\]

