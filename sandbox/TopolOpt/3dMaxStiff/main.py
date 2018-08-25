import numpy as np
import scipy as sp
from scipy.sparse import lil_matrix
from scipy.sparse.linalg import spsolve
import matplotlib.pyplot as plt
import matplotlib.cm as cm
import itertools as itt

# x +:right, -:left
# y +:back, -:front
# z +:top, -:bottom

class FeMat:
    #
    def __init__(self, nz, ny, nx):
        self.nz = nz
        self.ny = ny
        self.nx = nx
        nq = nz * ny * nx * 3
        self.m = lil_matrix((nq, nq))
    def submat(self, nnodes):
        q_n = np.empty(0)
        for node in nnodes:
            serpos = (node[0] * self.ny + node[1]) * self.nx + node[2]
            for dim in range(3):
                q_n = np.append(q_n,  serpos * 3 + dim)
        return self.m[np.ix_(q_n, q_n)]
#
class FeVec:
    #
    def __init__(self, nz, ny, nx):
        self.nz = nz
        self.ny = ny
        self.nx = nx
        nq = nz * ny * nx * 3
        self.m = lil_matrix((nq, 1))
#
class TopolOpt3D:
    #
    @classmethod
    def neibor_nodes(cls, elem):
        nodes = np.empty(0)
        for ofst in np.ndindex(2, 2, 2):
            nodes = np.append(nodes, np.array(elem) + np.array(ofst))
        return nodes.reshape((2 ** 3, 3))
    #
    def solve(self, (nz, ny, nx), slen, vratio):
        # Configuration
        self.nz = nz
        self.nx = nx
        self.ny = ny
        self.slen = slen
        max_i = 10
        # Initialization
        rho = vratio * np.ones((self.nz, self.ny, self.nx))
        # Iteration
        i = 0
        while i < max_i: # TODO
            u, l = self.ana_fe(rho)
            dldr = self.calc_sens(u, rho)
            rho_new = self.update_oc(rho, dldr)
            #
            print('{0:5}: l:{1}'.format(i, l))
            # TODO
            rho = rho_new
            i = i + 1
        #
        return rho
    #
    def ana_fe(self, rho):
        K = FeMat(self.nz + 1, self.ny + 1, self.nx + 1)
        #
        for elem in np.ndindex(self.nz, self.ny, self.nx):
            nnodes = TopolOpt3D.neibor_nodes(elem)
            r_n = nnodes * self.slen
            K_n = K.submat(nnodes)
            K_n += self.stiff(rho[elem], r_n)
            # TODO
        #
        f = FeVec(self.nx + 1, self.ny + 1, self.nz + 1)
        #
        u = FeVec(self.nx + 1, self.ny + 1, self.nz + 1)
        #
        l = 0.0
        #
        return u, l
    #
    def calc_sens(self, u, rho):
        dldr = np.zeros((self.nx + 1, self.ny + 1, self.nz + 1))
        # TODO
        return dldr
    #
    def update_oc(self, rho, dldr):
        rho_new = np.zeros_like(rho)
        # TODO
        return rho_new
    #
    def stiff(self, rho_e, r_n):
        # TODO
        return np.zeros((r_n.size, r_n.size))
#
#####
#
if __name__ == '__main__':
    print('numpy ver: ' + np.version.full_version)
    print('scipy ver: ' + sp.version.full_version)
    #
    t = TopolOpt3D()
    t.solve((30, 20, 10), 1, 0.3)
#
