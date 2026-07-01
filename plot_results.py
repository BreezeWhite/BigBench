#!/usr/bin/env python
import json
import matplotlib.pyplot as plt
import numpy as np

# Load hyperfine results
with open("results.json", encoding="utf-8") as f:
    data = json.load(f)

results = data["results"]

command_to_label = {
    "raw-bbp": "raw f64",
    "rs-decimal-bbp": "rust-decimal",
    "big-decimal-bbp": "bigdecimal",
    "rug-bbp": "rug",
    "dashu-bbp": "dashu",
    "big-float-bbp": "num-bigfloat",
    "astro-float-bbp": "astro-float",
    "fastnum-bbp": "fastnum",
    "decimal-rs-bbp": "decimal-rs",
    "prim-fpdec-bbp": "prim-fpdec",
    "malachite-bbp": "malachite",
    "fixed-num-bbp": "fixed-num",
    "python main.py C 1000": "Python C",
    "python main.py PY 1000": "Python PY"
}

correctness_map = {
    "raw f64": 16,
    "rust-decimal": 28,
    "fixed-num": 19,
    "bigdecimal": 1000,
    "rug": 1000,
    "dashu": 1000,
    "num-bigfloat": 39,
    "astro-float": 1000,
    "fastnum": 307,
    "decimal-rs": 37,
    "prim-fpdec": 35,
    "malachite": 1000,
    "Python C": 1000,
    "Python PY": 1000
}

plot_data = []
for r in results:
    cmd = r["command"]
    label = cmd
    for k, v in command_to_label.items():
        if k in cmd:
            label = v
            break
    median_ms = r["median"] * 1000.0
    corr = correctness_map.get(label, 0)
    plot_data.append({
        "label": label,
        "median_ms": median_ms,
        "correctness": corr
    })

# Sort by median time (fastest to slowest)
plot_data.sort(key=lambda x: x["median_ms"])

labels = [x["label"] for x in plot_data]
medians = [x["median_ms"] for x in plot_data]
correctness_vals = [x["correctness"] for x in plot_data]

# Styling
plt.style.use('seaborn-v0_8-whitegrid' if 'seaborn-v0_8-whitegrid' in plt.style.available else 'default')

fig, ax1 = plt.subplots(figsize=(16, 10), dpi=150)

x = np.arange(len(labels))  # the label locations
width = 0.35  # the width of the bars

# Colors
color_time = '#2b5c8f'
color_correct = '#d95f02'

# Primary axis: Time (ms)
rects1 = ax1.bar(x - width/2, medians, width, label='Runtime (ms)', color=color_time, edgecolor='none')
ax1.set_ylabel('Median Runtime (ms)', color=color_time, fontsize=16, fontweight='bold')
ax1.tick_params(axis='y', labelcolor=color_time, labelsize=12)

# Secondary axis: Correctness (digits)
ax2 = ax1.twinx()
ax2.grid(False)  # Turn off grid for the secondary y-axis to prevent clutter
rects2 = ax2.bar(x + width/2, correctness_vals, width, label='Correctness (digits)', color=color_correct, edgecolor='none')
ax2.set_ylabel('Correctness (digits)', color=color_correct, fontsize=16, fontweight='bold')
ax2.tick_params(axis='y', labelcolor=color_correct, labelsize=12)

ax1.set_title("BBP Pi Calculation Benchmark", fontsize=22, fontweight='bold', pad=25)
ax1.set_xticks(x)
ax1.set_xticklabels(labels, rotation=45, ha='right', fontsize=13, fontweight='bold')

# Add values on top of the bars with size 12 font, rotated 90 degrees
max_median = max(medians)
padding_y1 = max_median * 0.02
for rect in rects1:
    height = rect.get_height()
    if height >= 1000:
        label_text = f"{height/1000:.2f} s"
    else:
        label_text = f"{height:.1f} ms"
    ax1.text(
        rect.get_x() + rect.get_width()/2.0,
        height + padding_y1,
        label_text,
        ha='center',
        va='bottom',
        fontsize=12,
        fontweight='bold',
        color=color_time,
        rotation=90
    )

max_correct = max(correctness_vals)
padding_y2 = max_correct * 0.02
for rect in rects2:
    height = rect.get_height()
    label_text = f"{int(height)}"
    ax2.text(
        rect.get_x() + rect.get_width()/2.0,
        height + padding_y2,
        label_text,
        ha='center',
        va='bottom',
        fontsize=12,
        fontweight='bold',
        color=color_correct,
        rotation=90
    )

# Legend
lines1, labels1 = ax1.get_legend_handles_labels()
lines2, labels2 = ax2.get_legend_handles_labels()
ax1.legend(lines1 + lines2, labels1 + labels2, loc='upper left', fontsize=12)

# Set ylimits with extra space for vertical labels
ax1.set_ylim(ymin=0.0, ymax=max_median * 1.35)
ax2.set_ylim(ymin=0.0, ymax=max_correct * 1.35)

plt.tight_layout()
plt.savefig("results.png", bbox_inches='tight')
print("Plot saved successfully to results.png")
