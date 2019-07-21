import matplotlib
import matplotlib.pyplot as plt
import numpy as np


a = np.loadtxt('slide/eic_ex2-2.log')
fname = 'slide/eic_ex2-3.png'
labels = a[:, 0].astype('int')
samples = a[:, 3]
xtitle = 'n'
ytitle = 'D-variance mean'
yformat = '{:.2f}'
fsz = 20

#-----

x = np.arange(len(labels))  # the label locations
width = 0.3  # the width of the bars

fig, ax = plt.subplots()
rects = ax.bar(x, samples, width)

# Add some text for labels, title and custom x-axis tick labels, etc.
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

#plt.show()
plt.savefig(fname)
