import json
import matplotlib.pyplot as plt
import matplotlib.colors as colors
import numpy as np

x12 = 0.5 + np.arange(256)
y1 = []
y2 = []
dat3 = []
dat4 = [[0]]
with open("sekaiweights.json", "r") as fp:
  j = json.load(fp)
  y1 = j["hist_sat"]
  y2 = j["hist_vis"]
  dat3 = j["fft_vis"]
try:
  with open("../debug_fft_output.json", "r") as fp:
    dat4 = json.load(fp)
except FileNotFoundError:
  pass

fig, [[ax1, ax2], [ax3, ax4]] = plt.subplots(2, 2)

ax1.bar(x12, y1, width=1, edgecolor="white", linewidth=0.7)

ax1.set(
  xlim=(0, 256), xticks=np.arange(0, 17) * 16,
  ylim=(0, 6), yticks=np.arange(0, 7))

ax2.bar(x12, y2, width=1, edgecolor="white", linewidth=0.7)

ax2.set(
  xlim=(0, 256), xticks=np.arange(0, 17) * 16,
  ylim=(0, 6), yticks=np.arange(0, 7))

ax3.imshow(dat3, norm=colors.Normalize(0, 5))
ax4.imshow(dat4, norm=colors.Normalize(0, 5))

plt.show()