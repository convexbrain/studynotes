import numpy as np

n = 6
g = np.random.randn(n, n)
g = (g + g.T) / 2

p = np.copy(g)
v = np.eye(n)

#print(g)
#print(p)
#print(v)

eps = 1e-16
TOL1 = eps * eps * 4
TOL2 = eps * eps
conv = False

while not conv:
    conv = True
    for i in range(n):
        for j in range(i + 1, n):
            #print(i, j)
            a = p[i, i]
            b = p[j, j]
            d = p[i, j]
            #print(a, b, d)

            if d * d > TOL1 * a * b and d * d > TOL2:
                conv = False

            zeta = (b - a) / (2. * d)
            #print(zeta)
            if zeta > 0:
                t = 1 / (zeta + np.sqrt(1 + zeta * zeta))
            else:
                t = -1 / (-zeta + np.sqrt(1 + zeta * zeta))
            #print(t)
            c = 1 / np.sqrt(1 + t * t)
            s = c * t
            #print(c, s)
            #print(c * c + s * s)
            #
            #print(p)
            pi = np.copy(p[:, i])
            pj = np.copy(p[:, j])
            p[:, i] = c * pi - s * pj
            p[:, j] = s * pi + c * pj
            #print(p)
            pi = np.copy(p[i, :])
            pj = np.copy(p[j, :])
            p[i, :] = c * pi - s * pj
            p[j, :] = s * pi + c * pj
            #print(p)
            vi = np.copy(v[:, i])
            vj = np.copy(v[:, j])
            v[:, i] = c * vi - s * vj
            v[:, j] = s * vi + c * vj
            #print(v)

l = np.diag(p)
gg = v @ np.diag(l) @ v.T

#print(g)
#print(p)
print(l)
#print(np.diag(l))
print(v)
#print(gg)
assert np.allclose(g, gg)
