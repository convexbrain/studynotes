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
    #print(_theta)
    _prob = np.empty(0)
    for _theta_intvl in _theta:
        #print(_theta_intvl)
        _theta = [_theta_intvl[0], _theta_intvl[1]]
        _prob = np.append(_prob, normal_model_log_prob(_x[_theta_intvl[2]:_theta_intvl[3]], _theta))
    return _prob

def max_likelihood_est(_x, _k):
    _max_l = -float('inf')
    _n = _x.size
    #
    for _div in itertools.combinations(range(1, _n), _k - 1):
        _div_a = list(_div)
        _div_a.append(_n)

        _l = 0
        _theta = []
        _intvl_s = 0
        for _i in range(_k):
            _theta_intvl = normal_max_likelihood_est(_x[_intvl_s:_div_a[_i]])
            if _theta_intvl[1] < 0.1: # ???
                _l = -float('inf')
            else:
                _l += normal_log_likelihood(_x[_intvl_s:_div_a[_i]], _theta_intvl)
            _theta_intvl.append(_intvl_s)
            _theta_intvl.append(_div_a[_i])
            _theta.append(_theta_intvl)
            #
            _intvl_s = _div_a[_i]
        #
        if _l > _max_l:
            _max_l = _l
            _max_theta = _theta
    #print(_max_l)
    #print(_max_theta)

    return _max_theta

def bootstrap_residual(_x, _ids, _theta):
    _x_r = np.zeros_like(_x)
    for _i in enumerate(_ids):
        for _theta_intvl in _theta:
            if (_theta_intvl[2] <= _i[1]) and (_i[1] < _theta_intvl[3]):
                _mu_sample = _theta_intvl[0]
            if (_theta_intvl[2] <= _i[0]) and (_i[0] < _theta_intvl[3]):
                _mu_intvl = _theta_intvl[0]
        _x_r[_i[0]] = _x[_i[0]] + _mu_intvl - _mu_sample
    return _x_r


#-----

def log_likelihood(_x, _theta):
    _l = np.sum(model_log_prob(_x, _theta))
    return _l

def bootstrap_sample(_x, _theta):
    _n = _x.size
    _ids = np.random.randint(0, _n, _n)
    _x_ast = _x[_ids]

    _x_ast_r = bootstrap_residual(_x_ast, _ids, _theta)

    return _x_ast_r

def EIC_biasE(_x, _k, _B):
    _D_ast = np.zeros((_B, 2))
    _theta = max_likelihood_est(_x, _k)
    for i in range(_B):
        _x_ast = bootstrap_sample(_x, _theta)
        _theta_ast = max_likelihood_est(_x_ast, _k)
        _D_ast[i, 0] = log_likelihood(_x_ast, _theta_ast) - log_likelihood(_x_ast, _theta)
        _D_ast[i, 1] = log_likelihood(_x, _theta) - log_likelihood(_x, _theta_ast)

    _b_b = np.mean(_D_ast[:, 0] + _D_ast[:, 1])
    return _b_b

#-----

def main(T, n, k, c, B):
    t_l = np.zeros(T)
    t_bias = np.zeros(T)

    prv_ut = time.time()
    for t in range(T):
        ut = time.time()
        if ut - prv_ut > 5.0:
            prv_ut = ut
            print("---", t, file=sys.stderr)

        #-- samples from true distribution
        x = np.random.normal(0.0, 1.0, n // 2)
        x = np.append(x, np.random.normal(c, 1.0, n // 2))

        theta = max_likelihood_est(x, k)
        t_l[t] = log_likelihood(x, theta)

        t_bias[t] = EIC_biasE(x, k, B)

    print(k, # k
          c, # c
          np.mean(t_bias), # bias, mean
          np.mean(t_l), # likelihood, mean
          )


if __name__ == '__main__':

    T = 10

    n = 100

    B = 100

    for k in [1, 2, 3]:
        for c in [0, 0.5, 1, 2, 4, 8]:
            main(T, n, k, c, B)
