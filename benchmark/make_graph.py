import json
import matplotlib.pyplot as plt
import numpy as np

inputs = [6, 9, 12, 16, 20, 25, 30, 36, 42, 49, 56, 64, 72]

with open("report.json") as f:
    results = json.load(f)["results"]

data = [test["mean"] for test in results]
x = np.array(inputs)
width = 1

fig, ax = plt.subplots(layout = "constrained")
fig.set_figheight(5)
fig.set_figwidth(10)

for i, command in enumerate(inputs):
    offset = width
    rects = ax.bar(x[i] + offset, data[i], width, label=command, color="steelblue")

ax.set_xticks(x + 1, inputs)
ax.set_yscale('log')
ax.set_yticks([0.01, 0.1, 1, 10], labels=['10ms', '100ms', '1s', '10s'])
ax.grid(visible = True, axis = "y")

plt.title("Runtime growth with seat count at worst-case input")
plt.xlabel("Seats Count")
plt.ylabel("Time")

plt.savefig("graph.png")