import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from sklearn.datasets import make_moons, make_circles, make_blobs
from pathlib import Path
import csv


def scale_to_canvas(data, min_val=0, max_val=100):
    """
    Scale the data to fit within the specified canvas range [min_val, max_val].
    """
    data_min = np.min(data, axis=0)
    data_max = np.max(data, axis=0)
    scaled_data = (data - data_min) / (data_max - data_min)  # Normalize to [0, 1]
    scaled_data = scaled_data * (max_val - min_val) + min_val  # Scale to [min_val, max_val]
    return scaled_data

def generate_cluster_centers(k, min_distance, center_range, dimensions):
    centers = []
    attempts = 0
    max_attempts = 1000 * k  # Prevent infinite loops

    while len(centers) < k and attempts < max_attempts:
        # Generate a random center within the specified range for all dimensions
        center = np.random.uniform(-center_range, center_range, size=dimensions)
        # Check if this center is at least min_distance away from existing centers
        if all(np.linalg.norm(center - existing_center) >= min_distance for existing_center in centers):
            centers.append(center)
        attempts += 1

    if len(centers) < k:
        raise ValueError(f"Could not place {k} cluster centers with min_distance {min_distance}. Try adjusting the parameters.")

    return np.array(centers)

def generate_clustered_data(n, k, output_file, dimensions=2, cluster_std=1.0, min_distance=5.0, center_range=10.0, chunk_size=100000, order='sequential'):
    # Generate cluster centers that are at least min_distance apart
    cluster_centers = generate_cluster_centers(k, min_distance, center_range, dimensions)

    # Calculate the number of points per cluster
    n_per_cluster = n // k
    remainder = n % k  # Handle cases where n is not divisible by k

    # Determine the number of points for each cluster
    cluster_sizes = [n_per_cluster + (1 if i < remainder else 0) for i in range(k)]

    # Prepare the header for the CSV file
    header = [f'dim_{i+1}' for i in range(dimensions)] + ['cluster']

    if order == 'sequential':
        with open(output_file, 'w', newline='') as csvfile:
            csvwriter = csv.writer(csvfile)
            # Write the header
            csvwriter.writerow(header)

            for i in range(k):
                n_points = cluster_sizes[i]
                center = cluster_centers[i]

                # Process data in chunks to manage memory usage
                for start_idx in range(0, n_points, chunk_size):
                    end_idx = min(start_idx + chunk_size, n_points)
                    size = end_idx - start_idx

                    # Generate data points for this chunk
                    data_points = np.random.normal(loc=center, scale=cluster_std, size=(size, dimensions))
                    cluster_labels = np.full(size, i, dtype=int)

                    # Write the data points to the CSV file
                    for point, c in zip(data_points, cluster_labels):
                        csvwriter.writerow(list(point) + [c])

    elif order == 'random':
        # Create an array of cluster labels according to cluster sizes
        cluster_labels = np.concatenate([
            np.full(size, i, dtype=int) for i, size in enumerate(cluster_sizes)
        ])

        # Shuffle the cluster labels to mix clusters
        np.random.shuffle(cluster_labels)

        with open(output_file, 'w', newline='') as csvfile:
            csvwriter = csv.writer(csvfile)
            # Write the header
            csvwriter.writerow(header)

            # Process the shuffled labels in chunks to manage memory usage
            total_points = len(cluster_labels)
            for start_idx in range(0, total_points, chunk_size):
                end_idx = min(start_idx + chunk_size, total_points)
                labels_chunk = cluster_labels[start_idx:end_idx]

                # Generate data points for this chunk
                data_points = np.zeros((len(labels_chunk), dimensions))

                for idx, label in enumerate(labels_chunk):
                    center = cluster_centers[label]
                    data_points[idx] = np.random.normal(loc=center, scale=cluster_std)

                # Write the data points to the CSV file
                for point, c in zip(data_points, labels_chunk):
                    csvwriter.writerow(list(point) + [c])
    else:
        raise ValueError("Invalid order parameter. Use 'sequential' or 'random'.")


def gen_all_demos():
    n = 100_000
    root = Path("./demos")
    X, _ = make_moons(n_samples=n, noise=0.1)
    X = scale_to_canvas(X, min_val=0, max_val=100)
    df = pd.DataFrame(X, columns=["x", "y"])
    df.to_csv(root / "moon_demo.csv", index=False)

    X, _ = make_circles(n_samples=n, noise=0.1, factor=0.5)
    X = scale_to_canvas(X, min_val=0, max_val=100)
    df = pd.DataFrame(X, columns=["x", "y"])
    df.to_csv(root / "circles_demo.csv", index=False)

    X, _ = make_blobs(n_samples=n, centers=3, cluster_std=1.)
    X = scale_to_canvas(X, min_val=0, max_val=100)
    df = pd.DataFrame(X, columns=["x", "y"])
    df.to_csv(root / "blobs_demo.csv", index=False)
    
def gen_matrix():
    for dimentionality in [2, 4, 5, 10, 20, 40, 60, 80]:
        for clusters in [5]:
            print(f'Generating data for {clusters} clusters and {dimentionality} dimentionality')
            generate_clustered_data(2_000_000, clusters, f'benchmark_data/synthetic/random_{clusters}k_{dimentionality}d.csv', dimensions=dimentionality, cluster_std=8.0, min_distance=15.0, center_range=100.0, order='random')

if __name__ == '__main__':
    Path("./demos").mkdir(parents=True, exist_ok=True)
    Path("./benchmark_data/synthetic").mkdir(parents=True, exist_ok=True)
    gen_all_demos()
    gen_matrix()
