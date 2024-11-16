import numpy as np
import csv
import argparse

def generate_cluster_centers(k, min_distance, center_range):
    centers = []
    attempts = 0
    max_attempts = 1000 * k  # Prevent infinite loops

    while len(centers) < k and attempts < max_attempts:
        # Generate a random center within the specified range
        center = np.random.uniform(-center_range, center_range, size=2)
        # Check if this center is at least min_distance away from existing centers
        if all(np.linalg.norm(center - np.array(existing_center)) >= min_distance for existing_center in centers):
            centers.append(center)
        attempts += 1

    if len(centers) < k:
        raise ValueError(f"Could not place {k} cluster centers with min_distance {min_distance}. Try adjusting the parameters.")

    return np.array(centers)

def generate_clustered_data(n, k, output_file, cluster_std=1.0, min_distance=5.0, center_range=10.0, chunk_size=100000, order='sequential'):
    # Generate cluster centers that are at least min_distance apart
    cluster_centers = generate_cluster_centers(k, min_distance, center_range)

    # Calculate the number of points per cluster
    n_per_cluster = n // k
    remainder = n % k  # Handle cases where n is not divisible by k

    # Determine the number of points for each cluster
    cluster_sizes = [n_per_cluster + (1 if i < remainder else 0) for i in range(k)]

    if order == 'sequential':
        with open(output_file, 'w', newline='') as csvfile:
            csvwriter = csv.writer(csvfile)
            # Write the header
            csvwriter.writerow(['x', 'y', 'cluster'])

            for i in range(k):
                n_points = cluster_sizes[i]
                center = cluster_centers[i]

                # Process data in chunks to manage memory usage
                for start_idx in range(0, n_points, chunk_size):
                    end_idx = min(start_idx + chunk_size, n_points)
                    size = end_idx - start_idx

                    # Generate data points for this chunk
                    x_points = np.random.normal(loc=center[0], scale=cluster_std, size=size)
                    y_points = np.random.normal(loc=center[1], scale=cluster_std, size=size)
                    cluster_labels = np.full(size, i, dtype=int)

                    # Write the data points to the CSV file
                    for x, y, c in zip(x_points, y_points, cluster_labels):
                        csvwriter.writerow([x, y, c])

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
            csvwriter.writerow(['x', 'y', 'cluster'])

            # Process the shuffled labels in chunks to manage memory usage
            total_points = len(cluster_labels)
            for start_idx in range(0, total_points, chunk_size):
                end_idx = min(start_idx + chunk_size, total_points)
                labels_chunk = cluster_labels[start_idx:end_idx]

                # Generate data points for this chunk
                x_points = np.zeros(len(labels_chunk))
                y_points = np.zeros(len(labels_chunk))

                for idx, label in enumerate(labels_chunk):
                    center = cluster_centers[label]
                    x_points[idx] = np.random.normal(loc=center[0], scale=cluster_std)
                    y_points[idx] = np.random.normal(loc=center[1], scale=cluster_std)

                # Write the data points to the CSV file
                for x, y, c in zip(x_points, y_points, labels_chunk):
                    csvwriter.writerow([x, y, c])
    else:
        raise ValueError("Invalid order parameter. Use 'sequential' or 'random'.")

if __name__ == "__main__":
        parser = argparse.ArgumentParser(description='Generate a CSV file with clustered x, y coordinates.')
        parser.add_argument('--n', type=int, default=1000, help='Total number of data points to generate.')
        parser.add_argument('--k', type=int, default=5, help='Number of clusters.')
        parser.add_argument('--output', type=str, default='data.csv', help='Output CSV file name.')
        parser.add_argument('--cluster_std', type=float, default=1.0, help='Standard deviation of clusters.')
        parser.add_argument('--min_distance', type=float, default=5.0, help='Minimum distance between cluster centers.')
        parser.add_argument('--center_range', type=float, default=10.0, help='Range for generating cluster centers.')
        parser.add_argument('--chunk_size', type=int, default=100000, help='Number of data points to process at a time.')
        parser.add_argument('--order', type=str, default='random', choices=['sequential', 'random'], help="Order of data points in the output file: 'sequential' or 'random'.")

        args = parser.parse_args()

        generate_clustered_data(
            n=args.n,
            k=args.k,
            output_file=args.output,
            cluster_std=args.cluster_std,
            min_distance=args.min_distance,
            center_range=args.center_range,
            chunk_size=args.chunk_size,
            order=args.order
        )