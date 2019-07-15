<!-- $size: A4 -->
<!-- page_number: true -->
<!-- footer: ブートストラップ情報量基準 -->
---

ブートストラップ情報量基準
=
情報量基準をモンテカルロ法で数値計算
* 長所
  * 真の分布についての解析的評価や条件付けが不要
  * パラメータ推定も数値計算で解を求めることができればよい
* 短所
  * モンテカルロ近似誤差が生じる
  * モンテカルロ反復のため計算時間がかかる

---
復習
=
（絵）
* ***KL情報量*** （小さいほどモデルが近い）
  $$ I(g;f) = E_G\left[ \log{g(x)\over f(x)} \right]
            = \int \log{g(x)\over f(x)} dG $$
  * $dG$は$g$の分布を細切れにした事象$(\int dG=1)$
  * 連続モデルの場合
    * $g,f$は密度関数
    * $G(x)=\int_{-\infty}^x g(x')dx'$ つまり $dG=g(x)dx$ と考えれば
  $$ I(g;f) = \int_{-\infty}^\infty \left( \log{g(x)\over f(x)} \right) g(x)dx$$

---
復習
=
変形
$$ I(g;f) = \int \log{g(x)\over f(x)} dG $$
$$ \int \log g(x)dG - \int \log f(x)dG $$
第1項：真の分布$G$のみに依存する定数
第2項： ***平均対数尤度*** と呼ばれる
$$ \mathrm{const.} - \int \log f(x)dG $$

---
復習
=
ところで
* $f$はパラメータ$\theta$をもつパラメトリックモデルだった
* 今$\theta$は、$G$から取得したデータ$\mathbf{x}_n=(x_1, \ldots, x_n)$にもとづいて、
  最尤推定などで求まる値$\hat\theta$である
$$ \mathrm{const.} - \int \log f(x|\hat\theta)dG $$
ここで、未知の$G$→データにもとづく経験分布$G'$に置き換える
$$ \mathrm{const.} - \int \log f(x|\hat\theta)dG'
   = \mathrm{const.} - {1\over n} \sum_{\alpha=1}^n \log f(x_\alpha|\hat\theta) $$
第2項：平均対数尤度→データにもとづく ***対数尤度*** の$1/n$

---
復習
=
平均対数尤度を対数尤度に置き換えた結果、偏り（バイアス）が生じる
* バイアスを期待値評価し、補正する
  * バイアス＝平均対数尤度による真値−対数尤度による疑似値
    $$ \left( \mathrm{const.} - \int \log f(x|\hat\theta)dG \right) -
       \left( \mathrm{const.} - {1\over n} \sum_{\alpha=1}^n \log f(x_\alpha|\hat\theta) \right) $$
    $$ {1\over n} \left[
       \sum_{\alpha=1}^n \log f(x_\alpha|\hat\theta) - n\int \log f(x|\hat\theta)dG
       \right] $$
  * データ$\mathbf{x}_n=(x_1, \ldots, x_n)$を、確率変数$\mathbf{X}_n=(X_1, \ldots, X_n)$の実現値と考えて、期待値評価
    * $\hat\theta$も$\mathbf{x}_n$にもとづいて求まる値なので$\mathbf{X}_n$に依存

---
復習
=
$$ {1\over n} E_{G(\mathbf{X}_n)} \left[
   \sum_{\alpha=1}^n \log f(X_\alpha|\hat\theta(\mathbf{X}_n))
   - n\int \log f(x|\hat\theta(\mathbf{X}_n))dG
   \right] $$
$$ {1\over n} E_{G(\mathbf{X}_n)} \left[
   \log f(\mathbf{X}_n|\hat\theta(\mathbf{X}_n))
   - n\int \log f(x|\hat\theta(\mathbf{X}_n))dG
   \right] \equiv {1\over n} b(G)$$
バイアス$b(G)$

---
復習
=
KL情報量に戻る
* 対数尤度で置き換えたKL情報量に${1\over n} b(G)$を足して補正
$$ \mathrm{const.} - {1\over n} \sum_{\alpha=1}^n \log f(x_\alpha|\hat\theta) + {1\over n} b(G)$$
$$ \mathrm{const.} + {1\over 2n} \left(
   -2 \sum_{\alpha=1}^n \log f(x_\alpha|\hat\theta) + 2b(G) \right)$$
第2項大カッコ内：情報量基準の一般形
