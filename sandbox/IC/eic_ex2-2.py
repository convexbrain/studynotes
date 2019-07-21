import numpy as np
import scipy.stats as sct
import time
import itertools
import sys

#----- normal distribution

def normal_model_log_prob(_x, _theta):
    #-- parameters
    _mu = _theta[0]
    _sigma_sq = _theta[1]
    #-- log probability
    _p = -(_x - _mu) ** 2 / (2 * _sigma_sq) - np.log(2 * np.pi * _sigma_sq) / 2
    return _p

def normal_max_likelihood_est(_x):
    _mu = np.mean(_x)
    _sigma_sq = np.var(_x)
    _theta = [_mu, _sigma_sq]
    return _theta

def normal_log_likelihood(_x, _theta):
    _l = np.sum(normal_model_log_prob(_x, _theta))
    return _l

#-----

def model_log_prob(_x, _theta):
    return normal_model_log_prob(_x, _theta)

def max_likelihood_est(_x):
    #_mu = np.median(_x)
    #_sigma_sq = (np.median(np.abs(_x - _mu)) / sct.norm.ppf(0.75)) ** 2
    return normal_max_likelihood_est(_x)

#-----

def log_likelihood(_x, _theta):
    _l = np.sum(model_log_prob(_x, _theta))
    return _l

def bootstrap_sample(_x):
    _n = _x.size
    _ids = np.random.randint(0, _n, _n)
    _x_ast = _x[_ids]
    return _x_ast

def EIC_biasE(_x, _B):
    _D_ast = np.zeros((_B, 3))
    _theta = max_likelihood_est(_x)
    for i in range(_B):
        _x_ast = bootstrap_sample(_x)
        _theta_ast = max_likelihood_est(_x_ast)
        _D_ast[i, 0] = log_likelihood(_x_ast, _theta_ast) - log_likelihood(_x_ast, _theta)
        #_D_ast[i, 1] = log_likelihood(_x_ast, _theta) - log_likelihood(_x, _theta)
        _D_ast[i, 2] = log_likelihood(_x, _theta) - log_likelihood(_x, _theta_ast)

    _b_b = np.mean(_D_ast[:, 0] + _D_ast[:, 2])
    _Dvar = np.var(_D_ast[:, 0] + _D_ast[:, 2])
    #_b_b012 = np.mean(_D_ast[:, 0] + _D_ast[:, 1] + _D_ast[:, 2])
    #_b_b0 = np.mean(_D_ast[:, 0])
    #_b_b1 = np.mean(_D_ast[:, 1])
    #_b_b2 = np.mean(_D_ast[:, 2])
    return [_b_b, _Dvar]

#-----

def main(T, n, B):
    t_bias = np.zeros(T)
    t_Dvar = np.zeros(T)

    prv_ut = time.time()
    for t in range(T):
        ut = time.time()
        if ut - prv_ut > 5.0:
            prv_ut = ut
            print("---", t, file=sys.stderr)
        
        #-- samples from true distribution
        x = np.random.normal(0.0, 1.0, n)
        #x = np.random.laplace(0.0, 1.0, n)

        t_bias[t], t_Dvar[t] = EIC_biasE(x, B)

    print(n, # n
          np.mean(t_bias), # bias, mean
          np.var(t_bias), # bias, variance
          np.mean(t_Dvar), # D variance, mean
          )


if __name__ == '__main__':

    T = 10000

    B = 100

    for n in [25, 100, 400, 1600]:
        main(T, n, B)
