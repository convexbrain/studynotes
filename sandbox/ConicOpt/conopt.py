import numpy as np
import scipy.linalg as spla

#

def mat_to_vec(m):
    #print(m)
    assert m.shape[0] == m.shape[1]
    n = m.shape[0]
    l = int(n * (n + 1) / 2)

    v = np.zeros(l)

    i = 0
    for c in range(n):
        # lower triangular elements of symmetric matrix vectorized in column-wise
        v[i: i + n - c] = m[c: n, c]
        v[i + 1: i + n - c] *= np.sqrt(2)
        i += n - c
    assert (i == l)

    #print(v)
    return v

#

def vec_to_mat(v):
    #print(v)
    l = v.size
    n = int((np.sqrt(8 * l + 1) - 1) / 2)
    assert (n * (n + 1) / 2 == l)

    m = np.zeros((n, n))

    i = 0
    for c in range(n):
        # lower triangular elements of symmetric matrix vectorized in column-wise
        m[c: n, c] = v[i: i + n - c]
        m[c + 1: n, c] /= np.sqrt(2)
        i += n - c
    assert (i == l)

    #print(m)
    return m

#

def proj_psd(y):
    out_y = np.copy(y)

    a = vec_to_mat(y)

    w, v = spla.eigh(a)
    #print(a)
    #print(w)
    #print(v)
    w = np.clip(w, 0, None)
    a = np.dot(np.dot(v, np.diag(w)), v.T)
    #print(a)

    out_y = mat_to_vec(a)

    return out_y

#

def proj_pos(t):
    out_t = np.copy(t)

    #print(out_t)
    out_t = np.clip(out_t, 0, None)
    #print(out_t)

    return out_t

#

if __name__ == "__main__":
    max_iter = 5000 #None

    n = 1
    m = 3

    seed = np.random.randint(65535)
    np.random.seed(seed)
    c = np.random.randn(n)
    A = np.random.randn(m, n)
    b = np.random.randn(m)
    #print(c)
    #print(A)
    #print(b)

    ### n=1,m=3  (x+2)(x-5)<=0
    #c[0] = 1
    #A[:, 0] = mat_to_vec(np.array([ [0, 0], [-1, -3] ]))
    #b[:] = mat_to_vec(np.array([ [1, 0], [0, 10] ]))

    ### n=1,m=3  constraint qualification not satisfied
    #c[0] = 1
    #A[:, 0] = np.array([0.5, -1, 1])
    #b[:] = np.array([0, 0, 0])

    L = np.zeros((n + m + 1, (n + m + 1) * 2))
    # Q
    L[0: n, n: n + m] = A.T
    L[0: n, n + m] = c
    L[n: n + m, 0: n] = -A
    L[n: n + m, n + m] = b
    L[n + m, 0: n] = -c.T
    L[n + m, n: n + m] = -b.T
    L[:, n + m + 1: (n + m + 1) * 2] = np.diag(-np.ones(n + m + 1))
    #print(L)

    x = np.zeros((n + m + 1) * 2) # u, v
    y = np.zeros(n + m + 1)

    x[n + m] = 1 # u_tau
    x[(n + m + 1) * 2 - 1] = 1 # v_kappa

    L_norm = np.amax(spla.svdvals(L)) ###TODO
    #print(L_norm)

    tau = 1 / L_norm
    sigma = 1 / L_norm

    def proj_cone(s):
        #return proj_pos(s)
        #return np.zeros_like(s)
        return proj_psd(s)

    def proj_cone_conj(y):
        #return proj_pos(y)
        #return np.copy(y)
        return proj_psd(y)

    eps_zero = 1e-12
    eps_pri = 1e-6
    eps_dual = 1e-6
    eps_gap = 1e-6
    eps_unbdd = 1e-6
    eps_infeas = 1e-6

    b_norm = spla.norm(b)
    c_norm = spla.norm(c)

    i = 0
    while True:
        print("-----", i)

        if True:
            # Alg 3.1

            x_tilde = x - tau * np.dot(L.T, y)
            #print(x_tilde)
            x_tilde[n: n + m] = proj_cone_conj(x_tilde[n: n + m]) # u_y
            x_tilde[n + m] = proj_pos(x_tilde[n + m]) # u_tau
            x_tilde[(n + m + 1): (n + m + 1) + n] = 0 # v_r
            x_tilde[(n + m + 1) + n: (n + m + 1) + n + m] = proj_cone(x_tilde[(n + m + 1) + n: (n + m + 1) + n + m]) # v_s
            x_tilde[(n + m + 1) + n + m] = proj_pos(x_tilde[(n + m + 1) + n + m]) # v_kappa

            y_tilde = y + sigma * np.dot(L, 2 * x_tilde - x)

        else:
            # Alg 3.2
            
            y_tilde = y + sigma * np.dot(L, x)

            x_tilde = x - tau * np.dot(L.T, 2 * y_tilde - y)
            #print(x_tilde)
            x_tilde[n: n + m] = proj_cone_conj(x_tilde[n: n + m]) # u_y
            x_tilde[n + m] = proj_pos(x_tilde[n + m]) # u_tau
            x_tilde[(n + m + 1): (n + m + 1) + n] = 0 # v_r
            x_tilde[(n + m + 1) + n: (n + m + 1) + n + m] = proj_cone(x_tilde[(n + m + 1) + n: (n + m + 1) + n + m]) # v_s
            x_tilde[(n + m + 1) + n + m] = proj_pos(x_tilde[(n + m + 1) + n + m]) # v_kappa

        x = x_tilde
        y = y_tilde
        #print(x)
        #print(y)

        u_k_x = x[0: n]
        v_k_s = x[(n + m + 1) + n: (n + m + 1) + n + m]
        u_k_y = x[n: n + m]
        u_tau_k = x[n + m]

        if u_tau_k > eps_zero:
            x_k = u_k_x / u_tau_k
            s_k = v_k_s / u_tau_k
            y_k = u_k_y / u_tau_k
            p_k = np.dot(A, x_k) + s_k - b
            d_k = np.dot(A.T, y_k) + c
            g_k_x = np.dot(c.T, x_k)
            g_k_y = np.dot(b.T, y_k)
            g_k = g_k_x + g_k_y

            #print(x_k)
            #print(s_k)
            #print(y_k)
            
            term_pri = ( spla.norm(p_k) <= eps_pri * (1 + b_norm) )
            term_dual = ( spla.norm(d_k) <= eps_dual * (1 + c_norm) )
            term_gap = ( np.abs(g_k) <= eps_gap * (1 + np.abs(g_k_x) + np.abs(g_k_y)) )

            print(term_pri, term_dual, term_gap)

            if term_pri and term_dual and term_gap:
                print("converged")
                print(x_k)
                break
        
        else:
            p_unbdd = np.dot(A, u_k_x) + v_k_s
            p_infeas = np.dot(A.T, u_k_y)
            m_cx = -np.dot(c.T, u_k_x)
            m_by = -np.dot(b.T, u_k_y)

            term_unbdd = (m_cx > eps_zero) and (
                spla.norm(p_unbdd) * c_norm <= eps_unbdd * m_cx
            )

            term_infeas = (m_by > eps_zero) and (
                spla.norm(p_infeas) * b_norm <= eps_infeas * m_by
            )

            print(term_unbdd, term_infeas)

            if term_unbdd:
                print("unbounded")
                break

            if term_infeas:
                print("infeasible")
                break

        i += 1
        if (max_iter is not None) and (i >= max_iter):
            print("timeover")
            break
        pass
    
    print(seed)
    print(c)
    print(A)
    print(b)
    pass
