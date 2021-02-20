# 初期値と終了基準

初期値は
\\[
    x_0=
    \left[ \begin{matrix}
    \hat x_0 \\\\ \hat y_0 \\\\ \hat s_0 \\\\ \hat \tau_0
    \end{matrix} \right]
    =
    \left[ \begin{matrix}
    0 \\\\ 0 \\\\ 0 \\\\ 1
    \end{matrix} \right]
    ,\qquad
    y=0
\\]
とする。

[Pock/Chambolleの一次法](./pock_chambolle.md)の反復において、\\(x_k\\) は \\({\bf prox}^\tau\_{G}\\) による射影の後のため必ず
\\(x_k \in {\bf R}^n \times \mathcal{K}^\ast \times \mathcal{K} \times {\bf R}\_+\\) となっており、
主・双対の錐の制約条件を満たしている。
この状況のもと、終了基準は[参考文献](./reference.md)[1]と同等のものを適用する。

## \\(\hat \tau^k > \varepsilon_{\rm zero}\\) の場合

## \\(\hat \tau^k \not > \varepsilon_{\rm zero}\\) の場合
