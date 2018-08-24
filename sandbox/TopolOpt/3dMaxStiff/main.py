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

class DofMat:
    NDIM = 3
    #
    def __init__(self, nx, ny, nz):
        self.nx = nx
        self.ny = ny
        self.nz = nz
        nvars = DofMat.NDIM * nx * nx * nz
        self.mat = lil_matrix((nvars, nvars))
    def neibors(self, x, y, z):
        nx = self.nx
        ny = self.ny
        nodes = np.empty(0)
        for oz in range(2):
            for oy in range(2):
                for ox in range(2):
                    for dim in range(DofMat.NDIM):
                        nodes = np.append(nodes, dim + DofMat.NDIM * ((x + ox) + nx * ((y + oy) + ny * (z + oz))))
        return self.mat[np.ix_(nodes, nodes)]
#
class DofVec:
    #
    def __init__(self, nx, ny, nz):
        self.nx = nx
        self.ny = ny
        self.nz = nz
        nvars = 3 * nx * nx * nz
        self.mat = lil_matrix((nvars, 1))
#
class TopolOpt3D:
    #
    @classmethod
    def idx_neibors(cls, x, y, z):
        indices = np.empty(0)
        for oz in range(2):
            for oy in range(2):
                for ox in range(2):
                    indices = np.append(indices, np.array([x + ox, y + oy, z + oz]))
        return indices.reshape((2 ** 3, 3))
    #
    def solve(self, (nx, ny, nz), slen, vratio):
        # Configuration
        self.nx = nx
        self.ny = ny
        self.nz = nz
        self.slen = slen
        max_i = 10
        # Initialization
        rho = vratio * np.ones((self.nx, self.ny, self.nz))
        # Iteration
        i = 0
        while i < max_i: # TODO
            u, l = self.fe_ana(rho)
            dldr = self.fe_sens(u, rho)
            rho_new = self.update_oc(rho, dldr)
            #
            print('{0:5}: l:{1}'.format(i, l))
            # TODO
            rho = rho_new
            i = i + 1
        #
        return rho
    #
    def fe_ana(self, rho):
        K = DofMat(self.nx + 1, self.ny + 1, self.nz + 1)
        #
        for x, y, z in itt.product(range(self.nx), range(self.ny), range(self.nz)):
            r_n = TopolOpt3D.idx_neibors(x, y, z) * self.slen
            K_n = K.neibors(x, y, z)
            K_n += el_stiff(rho[x, y, z], r_n)
            # TODO
        #
        f = DofVec(self.nx + 1, self.ny + 1, self.nz + 1)
        #
        u = DofVec(self.nx + 1, self.ny + 1, self.nz + 1)
        #
        l = 0.0
        #
        return u, l
    #
    def fe_sens(self, u, rho):
        dldr = np.zeros((self.nx + 1, self.ny + 1, self.nz + 1))
        # TODO
        return dldr
    #
    def update_oc(self, rho, dldr):
        rho_new = np.zeros((self.nx, self.ny, self.nz))
        # TODO
        return rho_new
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
