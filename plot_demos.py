import matplotlib.pyplot as plt
import pandas as pd
from pathlib import Path

# Create DataFrame
results_dir = Path('./demo_results')
for result_path in results_dir.glob('*.csv'):
    splt_path = result_path.stem.split('_')
    demo_type = splt_path[0]
    algorithm = splt_path[-1]

    df = pd.read_csv(result_path)
    # Extract x and y coordinates from the 'center' column
    df[['x', 'y']] = df['center'].str.split(',', expand=True).astype(float)
    # Assign colors to clusters
    cluster_colors = {cluster: f"C{cluster}" for cluster in df['cluster'].unique()}

    # Plot clusters as circles
    plt.figure(figsize=(10, 8))
    for _, row in df.iterrows():
        circle = plt.Circle(
            (row['x'], row['y']),
            row['radius'],
            color=cluster_colors[row['cluster']],
            alpha=0.5,
            label=f"Cluster {row['cluster']}" if f"Cluster {row['cluster']}" not in plt.gca().get_legend_handles_labels()[1] else ""
        )
        plt.gca().add_patch(circle)

    # Formatting the plot
    plt.xlabel("X Coordinate")
    plt.ylabel("Y Coordinate")
    plt.title(f"{demo_type} - {algorithm} Clustering")
    plt.axis('equal')  # Ensures equal scaling of axes for proper circle representation
    plt.grid(True)
    plt.savefig(f'plots/{demo_type}_{algorithm}.png', dpi=300)
