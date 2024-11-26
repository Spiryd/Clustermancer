import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from sklearn.cluster import KMeans, DBSCAN, AgglomerativeClustering
import time

def cluster_and_time(data, algorithm_name, algorithm):
    """
    Cluster the data using the given algorithm and measure its execution time.
    """
    start_time = time.time()
    labels = algorithm.fit_predict(data)
    end_time = time.time()
    duration = end_time - start_time
    return labels, duration

def plot_clusters(data, labels, title, algorithm_name):
    """
    Plot clustered data.
    """
    sns.set(style="whitegrid")
    plt.figure(figsize=(8, 6))
    sns.scatterplot(
        x=data[:, 0],
        y=data[:, 1],
        hue=labels,
        palette="tab10",
        s=50,
        edgecolor="w",
        legend="full"
    )
    plt.title(title, fontsize=16)
    plt.xlabel("x", fontsize=12)
    plt.ylabel("y", fontsize=12)
    plt.legend(title="Cluster", loc="best", fontsize=10)
    plt.grid(True)
    plt.savefig(f"plots/circles_{algorithm_name}.png")

def main():
    # Load the data
    input_file = "demos/circles_demo.csv"  # Replace with your generated file path
    df = pd.read_csv(input_file)
    data = df[['x', 'y']].values

    # Algorithms to use
    algorithms = {
        "K-Means (k=2)": KMeans(n_clusters=2, random_state=42, max_iter=10000),
        "DBSCAN (eps=3)": DBSCAN(eps=3, min_samples=5),
    }

    # Perform clustering and time each algorithm
    for algorithm_name, algorithm in algorithms.items():
        print(f"Clustering with {algorithm_name}...")
        labels, duration = cluster_and_time(data, algorithm_name, algorithm)
        print(f"Execution time for {algorithm_name}: {duration:.4f} seconds")
        plot_clusters(data, labels, f"{algorithm_name} (Time: {duration:.4f}s)", algorithm_name)

if __name__ == "__main__":
    main()
