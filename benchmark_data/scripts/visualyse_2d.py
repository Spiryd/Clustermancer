import pandas as pd
import matplotlib.pyplot as plt
import argparse

def visualize_data(input_file):
    # Read the CSV file
    data = pd.read_csv(input_file)

    # Create a scatter plot, color-coded by cluster
    plt.figure(figsize=(10, 8))
    scatter = plt.scatter(data['x'], data['y'], c=data['cluster'], s=10, cmap='tab10', alpha=0.7)

    plt.title('Scatter Plot of Non-Overlapping Clusters')
    plt.xlabel('X Coordinate')
    plt.ylabel('Y Coordinate')
    plt.grid(True)

    # Add a legend
    legend1 = plt.legend(*scatter.legend_elements(), title="Clusters")
    plt.gca().add_artist(legend1)

    plt.show()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Visualize x, y coordinates from a CSV file.')
    parser.add_argument('--input', type=str, default='data.csv', help='Input CSV file name.')

    args = parser.parse_args()

    visualize_data(args.input)
