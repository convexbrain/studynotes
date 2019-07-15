import numpy as np
import scipy.stats as sct
import time
import itertools

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
    _beta = _theta[0]
    _sigma_sq = _theta[1]
    _X = _x[:, 0:-1]
    _Y = _x[:, -1]
    _z = _Y - _X @ _beta
    _prob = normal_model_log_prob(_z, [0, _sigma_sq])
    return _prob

def max_likelihood_est(_x, _k):
    _max_l = -float('inf')
    _K = _x.shape[1] - 1
    _Y = _x[:, -1]
    for _i in itertools.combinations(range(_K), _k):
        _ids = np.array(_i)
        _beta = np.zeros(_K)
        if _ids.size != 0:
            _X = _x[:, _ids]
            _Beta = np.linalg.solve(_X.T @ _X, _X.T @ _Y)
            _beta[_ids] = _Beta
        _sigma_sq = np.mean((_Y - _x[:, 0:-1] @ _beta) ** 2)
        _theta = [_beta, _sigma_sq]
        _l = log_likelihood(_x, _theta)
        if _l > _max_l:
            _max_l = _l
            _max_theta = _theta
    return _max_theta

def bootstrap_residual(_x, _ids, _theta):
    return _x


#-----

def log_likelihood(_x, _theta):
    _l = np.sum(model_log_prob(_x, _theta))
    return _l

def bootstrap_sample(_x, _theta):
    _n = _x.shape[0]
    _ids = np.random.randint(0, _n, _n)
    _x_ast = _x[_ids, :]

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


if __name__ == '__main__':

    T = 10 #1000

    n = 100 #100
    K = 20

    k = 2 #0, 1, ..., 20

    B = 100 #100

    t_l = np.zeros(T)
    t_eic = np.zeros(T)

    prv_ut = time.time()
    for t in range(T):
        ut = time.time()
        if ut - prv_ut > 5.0:
            prv_ut = ut
            print("---", t)
        
        #-- samples from true distribution
        #X = np.random.uniform(-1.0, 1.0, (n, K))
        X = np.random.normal(0.0, 1.0, (n, K))
        Y = np.random.normal(0.0, 1.0, (n, 1))
        x = np.hstack((X, Y))

        theta = max_likelihood_est(x, k)
        t_l[t] = log_likelihood(x, theta)

        t_eic[t] = EIC_biasE(x, k, B)

    print("mean EIC bias:", np.mean(t_eic))
    print("variance EIC bias:", np.var(t_eic))

    print("mean likelihood:", np.mean(t_l))
    print("variance likelihood:", np.var(t_l))
