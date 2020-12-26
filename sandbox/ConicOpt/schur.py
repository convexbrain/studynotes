import numpy as np
from scipy import linalg
import matplotlib.pyplot as plt

def gen_data(n):
    x = np.random.randn(n, 1)
    #print(x)
    c = np.random.randn(1, 1) * n**2
    #print(c)
    P_gen = np.random.randn(n, n)
    P_gen = (P_gen + np.transpose(P_gen)) / 2
    #print(P_gen)

    w, v = linalg.eigh(P_gen)
    #print(w)
    #print(v)
    P_rec = np.dot(np.dot(v, np.diag(w)), np.transpose(v))
    assert(np.allclose(P_gen, P_rec))

    ## make it positive-semidefinite
    ##
    w = np.where(w > 0, w, 0)
    #print(w)
    P = np.dot(np.dot(v, np.diag(w)), np.transpose(v))
    #print(P)
    assert(np.allclose(P, np.transpose(P)))

    return x, c, P


def make_matrix(x, c, P):
    Px = np.dot(P, x)
    #print(Px)
    F1 = np.hstack((P, Px))
    #print(F1)
    F2 = np.hstack((np.transpose(Px), c))
    #print(F2)
    F = np.vstack((F1, F2))
    #print(F)

    return F


def min_eig(F):
    eigF = linalg.eigvalsh(F)
    #print(eigF)
    min_eigF = np.amin(eigF)
    #print(min_eigF)

    return min_eigF


def schur_comp(x, c, P):
    Px = np.dot(P, x)
    #print(Px)

    val = c - np.dot(np.transpose(x), Px)
    val = val[0, 0]

    return val


if __name__ == "__main__":

    val = np.zeros((2, 1000))
    for i in range(val.shape[1]):
        n = np.random.randint(2, 50)

        x, c, P = gen_data(n)

        F = make_matrix(x, c, P)

        val[0, i] = min_eig(F)
        val[1, i] = schur_comp(x, c, P)
        #print(val[:, i])
    
    plt.plot(val[0], val[1], '.')
    plt.hlines([0], np.min(val[0]), np.max(val[0]), linestyle=":", lw=1)
    plt.vlines([0], np.min(val[1]), np.max(val[1]), linestyle=":", lw=1)
    plt.xlabel('left eval: minimum eigenvalue')
    plt.ylabel('right eval: schur complement value')
    plt.show()

    pass