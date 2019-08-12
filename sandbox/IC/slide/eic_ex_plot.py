import matplotlib
import matplotlib.pyplot as plt
import numpy as np


a = np.loadtxt('slide/eic_ex4-1.log')
fname = 'slide/eic_ex4-2.png'
labels = a[:, 0].astype(int)
samples = a[:, 2]
title = 'EIC1'
xtitle = 'k'
#ytitle = 'bias mean'
ytitle = 'log-likelihood mean'
yformat = '{:.2f}'
fsz = 14
#ylim = [0, 36]
ylim = [-150, -125]

#-----

x = np.arange(len(labels))  # the label locations
width = 0.3  # the width of the bars

fig, ax = plt.subplots()
rects = ax.bar(x, samples, width)

# Add some text for labels, title and custom x-axis tick labels, etc.
plt.title(title, fontsize=fsz)
ax.set_xlabel(xtitle, fontsize=fsz)
ax.set_ylabel(ytitle, fontsize=fsz)
ax.set_xticks(x)
ax.set_xticklabels(labels)
plt.tick_params(labelsize=fsz)

def autolabel(rects):
    """Attach a text label above each bar in *rects*, displaying its height."""
    for rect in rects:
        height = rect.get_height()
        ax.annotate(yformat.format(height),
                    xy=(rect.get_x() + rect.get_width() / 2, height),
                    xytext=(0, 0),
                    textcoords="offset points",
                    ha='center', va='bottom', fontsize=fsz)


autolabel(rects)

fig.tight_layout()

if 'ylim' in globals():
    plt.ylim(ylim)

#plt.show()
plt.savefig(fname)
