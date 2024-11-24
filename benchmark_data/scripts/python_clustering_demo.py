import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
from sklearn.cluster import KMeans, DBSCAN, AgglomerativeClustering
import argparse

def main():
    # Command-line argument parser
    parser = argparse.ArgumentParser(description="Cluster visualization script for 2D data.")
    parser.add_argument("file_path", type=str, help="Path to the CSV file containing the data.")
    parser.add_argument("--algorithm", type=str, choices=["kmeans", "dbscan", "agglomerative"], default="kmeans",
                        help="Clustering algorithm to use (default: kmeans).")
    parser.add_argument("--clusters", type=int, default=3,
                        help="Number of clusters (used by kmeans and agglomerative clustering). Default is 3.")
    parser.add_argument("--eps", type=float, default=3.0,
                        help="Epsilon parameter for DBSCAN (default: 3.0).")
    parser.add_argument("--min_samples", type=int, default=5,
                        help="Minimum samples for DBSCAN (default: 5).")
    args = parser.parse_args()

    # Load the dataset
    data = pd.read_csv(args.file_path)

    # Extract coordinates
    X = data[['x', 'y']].values

    # Initialize the clustering algorithm based on the user's choice
    if args.algorithm == "kmeans":
        algorithm = KMeans(n_clusters=args.clusters, random_state=42)
    elif args.algorithm == "dbscan":
        algorithm = DBSCAN(eps=args.eps, min_samples=args.min_samples)
    elif args.algorithm == "agglomerative":
        algorithm = AgglomerativeClustering(n_clusters=args.clusters)

    # Fit the clustering algorithm
    algorithm.fit(X)

    # Extract cluster labels
    if hasattr(algorithm, 'labels_'):
        labels = algorithm.labels_
    else:
        labels = algorithm.predict(X)

    # Add cluster labels to the dataframe
    data['cluster'] = labels

    # Plot clusters using Seaborn
    sns.set(style="whitegrid")
    plt.figure(figsize=(10, 8))
    sns.scatterplot(
        x='x', y='y', hue='cluster', palette='tab10', data=data, s=50, edgecolor="w", legend="full"
    )
    plt.title(f"Clustering Result - {args.algorithm.capitalize()}", fontsize=16)
    plt.xlabel("x", fontsize=12)
    plt.ylabel("y", fontsize=12)
    plt.legend(title="Cluster", loc='upper right', fontsize=10)
    plt.show()

if __name__ == "__main__":
    main()
