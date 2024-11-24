import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from sklearn.datasets import make_moons, make_circles, make_blobs
import argparse

def generate_spiral(n_points, noise=0.2):
    """
    Generate a spiral shape with noise.
    """
    n = np.sqrt(np.random.rand(n_points)) * 2 * np.pi  # Random radii
    x = n * np.cos(n) + np.random.normal(0, noise, n_points)
    y = n * np.sin(n) + np.random.normal(0, noise, n_points)
    return np.column_stack((x, y))

def generate_grid(n_points, noise=0.2):
    """
    Generate a grid-like structure with noise.
    """
    n_per_side = int(np.sqrt(n_points))
    x, y = np.meshgrid(np.linspace(0, 1, n_per_side), np.linspace(0, 1, n_per_side))
    x = x.flatten() + np.random.normal(0, noise, x.size)
    y = y.flatten() + np.random.normal(0, noise, y.size)
    return np.column_stack((x, y))

def scale_to_canvas(data, min_val=0, max_val=100):
    """
    Scale the data to fit within the specified canvas range [min_val, max_val].
    """
    data_min = np.min(data, axis=0)
    data_max = np.max(data, axis=0)
    scaled_data = (data - data_min) / (data_max - data_min)  # Normalize to [0, 1]
    scaled_data = scaled_data * (max_val - min_val) + min_val  # Scale to [min_val, max_val]
    return scaled_data

def main():
    parser = argparse.ArgumentParser(description="Generate interesting shapes for clustering.")
    parser.add_argument("--shape", type=str, choices=["moons", "circles", "spiral", "grid", "blobs"], default="moons",
                        help="Shape of the dataset (default: moons).")
    parser.add_argument("--points", type=int, default=500, help="Number of points to generate (default: 500).")
    parser.add_argument("--output", type=str, default="interesting_shapes.csv",
                        help="Output CSV file name (default: interesting_shapes.csv).")
    parser.add_argument("--noise", type=float, default=0.1, help="Noise level for the dataset (default: 0.1).")
    parser.add_argument("--clusters", type=int, default=3, help="Number of clusters for blobs (default: 3).")
    parser.add_argument("--blob_std", type=float, default=1.0, help="Standard deviation for blobs (default: 10.0).")
    args = parser.parse_args()

    # Generate the chosen shape
    if args.shape == "moons":
        X, _ = make_moons(n_samples=args.points, noise=args.noise)
    elif args.shape == "circles":
        X, _ = make_circles(n_samples=args.points, noise=args.noise, factor=0.5)
    elif args.shape == "spiral":
        X = generate_spiral(args.points, noise=args.noise)
    elif args.shape == "grid":
        X = generate_grid(args.points, noise=args.noise)
    elif args.shape == "blobs":
        X, _ = make_blobs(n_samples=args.points, centers=args.clusters, cluster_std=args.blob_std)

    # Scale data to fit within the 0-100 canvas
    X = scale_to_canvas(X, min_val=0, max_val=100)

    # Save to CSV
    df = pd.DataFrame(X, columns=["x", "y"])
    df.to_csv(args.output, index=False)

    # Plot the data
    plt.figure(figsize=(8, 6))
    plt.scatter(df["x"], df["y"], s=10, color="blue")
    plt.title(f"Generated Shape: {args.shape.capitalize()}", fontsize=16)
    plt.xlabel("x", fontsize=12)
    plt.ylabel("y", fontsize=12)
    plt.xlim(0, 100)
    plt.ylim(0, 100)
    plt.grid(True)
    plt.show()

    print(f"Data saved to {args.output}")

if __name__ == "__main__":
    main()
