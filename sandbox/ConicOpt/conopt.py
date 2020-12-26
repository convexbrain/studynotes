import numpy as np
import scipy.linalg as spla

#

def proj_psd(y):
    out_y = np.copy(y)

    n = int((np.sqrt(8 * out_y.size + 1) - 1) / 2)
    assert (n * (n + 1) == 2 * out_y.size)

    a = np.zeros((n, n))
    i = 0
    for c in range(n):
        a[c: n, c] = out_y[i: i + n - c]
        i += n - c
    assert (i == out_y.size)

    w, v = spla.eigh(a)
    #print(a)
    #print(w)
    #print(v)
    w = np.clip(w, 0, None)
    a = np.dot(np.dot(v, np.diag(w)), v.T)
    #print(a)

    i = 0
    for c in range(n):
        out_y[i: i + n - c] = a[c: n, c]
        i += n - c
    assert (i == out_y.size)

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
    n = 1
    m = 1

    c = np.random.randn(n)
    A = np.random.randn(m, n)
    b = np.random.randn(m)
    c[0] = -1
    A[0, 0] = 1
    b[0] = 1
    #print(c)
    #print(A)
    #print(b)

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

    L_norm = np.amax(spla.svdvals(L))
    #print(L_norm)

    tau = 1 / L_norm
    sigma = 1 / L_norm
    max_iter = 1000

    eps_zero = 1e-12
    eps_pri = 1e-6
    eps_dual = 1e-6
    eps_gap = 1e-6
    eps_unbdd = 1e-6
    eps_infeas = 1e-6

    b_norm = spla.norm(b)
    c_norm = spla.norm(c)

    for i in range(max_iter):
        print("-----", i)

        x_tilde = x - tau * np.dot(L.T, y)
        #print(x_tilde)
        x_tilde[n: n + m] = proj_psd(x_tilde[n: n + m])
        x_tilde[n + m] = proj_pos(x_tilde[n + m])
        x_tilde[(n + m + 1): (n + m + 1) + n] = 0
        x_tilde[(n + m + 1) + n: (n + m + 1) + n + m] = proj_psd(x_tilde[(n + m + 1) + n: (n + m + 1) + n + m])
        x_tilde[(n + m + 1) + n + m] = proj_pos(x_tilde[(n + m + 1) + n + m])
        #print(x_tilde)

        y_tilde = y + sigma * np.dot(L, 2 * x_tilde - x)

        x = x_tilde
        y = y_tilde

        u_tau_k = x[n + m]
        assert u_tau_k > eps_zero
        u_k_x = x[0: n]
        v_k_s = x[(n + m + 1) + n: (n + m + 1) + n + m]
        u_k_y = x[n: n + m]
        x_k = u_k_x / u_tau_k
        s_k = v_k_s / u_tau_k
        y_k = u_k_y / u_tau_k
        p_k = np.dot(A, x_k) + s_k - b
        d_k = np.dot(A.T, y_k) + c
        g_k_x = np.dot(c.T, x_k)
        g_k_y = np.dot(b.T, y_k)
        g_k = g_k_x + g_k_y

        term_pri = ( spla.norm(p_k) <= eps_pri * (1 + b_norm) )
        term_dual = ( spla.norm(d_k) <= eps_dual * (1 + c_norm) )
        term_gap = ( np.abs(g_k) <= eps_gap * (1 + np.abs(g_k_x) + np.abs(g_k_y)) )

        p_unbdd = np.dot(A, u_k_x) + v_k_s
        p_infeas = np.dot(A.T, u_k_y)

        term_unbdd = ( spla.norm(p_unbdd) * c_norm <= -np.dot(c.T, u_k_x) * eps_unbdd )
        term_infeas = ( spla.norm(p_infeas) * b_norm <= -np.dot(b.T, u_k_y) * eps_infeas )

        print(term_pri, term_dual, term_gap, term_unbdd, term_infeas)
        print(x_k, s_k)

        if term_pri and term_dual and term_gap:
            pass
            break

        if term_unbdd:
            pass
            #break

        if term_infeas:
            pass
            #break


