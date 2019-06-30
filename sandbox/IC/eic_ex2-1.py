import numpy as np
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


#-----


if __name__ == '__main__':

    T = 100 #10000

    n = 25 #25, 100, 400
    B = 100 #1000

    t_eic = np.zeros(T)
    t_eic_2nd = np.zeros(T)

    prv_ut = time.time()
    for t in range(T):
        ut = time.time()
        if ut - prv_ut > 5.0:
            prv_ut = ut
            print("---", t)
        
        #-- samples from true distribution
        x = np.random.normal(0.0, 1.0, n)
        #print(x)

        t_eic[t] = EIC_bias(x, B)
        t_eic_2nd[t] = EIC_bias2(x, B) + t_eic[t]
        #print(t_eic[t])
        #print(t_eic_2nd[t])

    print("mean EIC bias:", np.mean(t_eic))
    print("mean EIC_2nd bias:", np.mean(t_eic_2nd))

