import numpy as np
import scipy.stats as sct
import time

#----- normal distribution

def model_prob(_x, _theta):
    #-- parameters
    _mu = _theta[0]
    _sigma_sq = _theta[1]
    #-- probability
    _p = np.exp(-(_x - _mu) ** 2 / (2 * _sigma_sq)) / np.sqrt(2 * np.pi * _sigma_sq)
    #print(_p)
    return _p

def max_likelihood_est(_x):
    #_mu = np.median(_x)
    #_sigma_sq = (np.median(np.abs(_x - _mu)) / sct.norm.ppf(0.75)) ** 2
    _mu = np.mean(_x)
    _sigma_sq = np.var(_x)
    _theta = [_mu, _sigma_sq]
    #print(_theta)
    return _theta

#-----

def likelihood(_x, _theta):
    _l = np.sum(np.log(model_prob(_x, _theta)))
    #print(_l)
    return _l

def bootstrap_sample(_x):
    _n = _x.size
    _ids = np.random.randint(0, _n, _n)
    #print(_ids)
    _x_ast = _x[_ids]
    #print(_x_ast)
    return _x_ast

def EIC_bias(_x, _B):
    _D_ast = np.zeros(_B)
    for i in range(_B):
        _x_ast = bootstrap_sample(_x)
        _theta_ast = max_likelihood_est(_x_ast)
        _D_ast[i] = likelihood(_x_ast, _theta_ast) - likelihood(_x, _theta_ast)
        #print(_D_ast[i])
    #print(_D_ast)

    _b_b = np.mean(_D_ast)
    #print(_b_b)
    return _b_b

def EIC_bias2(_x, _B):
    _D_ast = np.zeros(_B)
    for i in range(_B):
        _x_ast = bootstrap_sample(_x)
        _theta_ast = max_likelihood_est(_x_ast)
        _eic = EIC_bias(_x_ast, _B)
        _D_ast[i] = likelihood(_x_ast, _theta_ast) - _eic - likelihood(_x, _theta_ast)
    _b_2nd = np.mean(_D_ast)
    return _b_2nd

def EIC_biasE(_x, _B):
    _D_ast = np.zeros((_B, 3))
    _theta = max_likelihood_est(_x)
    for i in range(_B):
        _x_ast = bootstrap_sample(_x)
        _theta_ast = max_likelihood_est(_x_ast)
        _D_ast[i, 0] = likelihood(_x_ast, _theta_ast) - likelihood(_x_ast, _theta)
        _D_ast[i, 1] = likelihood(_x_ast, _theta) - likelihood(_x, _theta)
        _D_ast[i, 2] = likelihood(_x, _theta) - likelihood(_x, _theta_ast)

    _b_b012 = np.mean(_D_ast[:, 0] + _D_ast[:, 1] + _D_ast[:, 2])
    _b_b02 = np.mean(_D_ast[:, 0] + _D_ast[:, 2])
    _b_b0 = np.mean(_D_ast[:, 0])
    _b_b1 = np.mean(_D_ast[:, 1])
    _b_b2 = np.mean(_D_ast[:, 2])
    return [_b_b012, _b_b02, _b_b0, _b_b1, _b_b2]


#-----


if __name__ == '__main__':

    T = 100 #10000

    n = 25 #25, 100, 400
    B = 100 #100

    t_eic = np.zeros((T, 5))

    prv_ut = time.time()
    for t in range(T):
        ut = time.time()
        if ut - prv_ut > 5.0:
            prv_ut = ut
            print("---", t)
        
        #-- samples from true distribution
        x = np.random.normal(0.0, 1.0, n)
        #x = np.random.laplace(0.0, 1.0, n)
        #print(x)

        t_eic[t] = EIC_biasE(x, B)
        #print(t_eic[t])

    print("mean EIC bias:", np.mean(t_eic[:, 0]))
    print("mean EIC bias:", np.mean(t_eic[:, 1]))
    print("mean EIC bias:", np.mean(t_eic[:, 2]))
    print("mean EIC bias:", np.mean(t_eic[:, 3]))
    print("mean EIC bias:", np.mean(t_eic[:, 4]))
    print("variance EIC bias:", np.var(t_eic[:, 0]))
    print("variance EIC bias:", np.var(t_eic[:, 1]))
    print("variance EIC bias:", np.var(t_eic[:, 2]))
    print("variance EIC bias:", np.var(t_eic[:, 3]))
    print("variance EIC bias:", np.var(t_eic[:, 4]))

