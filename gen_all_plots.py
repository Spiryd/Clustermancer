import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from pathlib import Path

def plot_quality():
    try:
        df = pd.read_csv('benchmark_results/real_quality.csv')
        sns.barplot(data=df, x='algorithm', y='SSQ')
        plt.title('SSQ on Real Data')
        plt.xlabel('Algorithm')
        plt.ylabel('SSQ')
        plt.savefig('plots/quality_real.png', dpi=300)
        plt.clf()
        print("Real quality ploted")
    except Exception as e:
        print(f"Real quality not ploted: {e}")
    
    try:
        df = pd.read_csv('benchmark_results/synthetic_quality.csv')
        sns.barplot(data=df, x='algorithm', y='SSQ')
        plt.title('SSQ on Synthetic Data')
        plt.xlabel('Algorithm')
        plt.ylabel('SSQ')
        plt.savefig('plots/quality_synth.png', dpi=300)
        plt.clf()
        print("Synthetic quality ploted")
    except Exception as e:
        print(f"Synthetic quality not ploted: {e}")

    try:
        df = pd.read_csv('benchmark_results/synthetic_quality_samplers.csv')
        df[['sampler', 'algorithm']] = df['algorithm'].str.split(',', expand=True)
        algorithms = df['algorithm'].unique()
        for algo in algorithms:
            algo_data = df[df['algorithm'] == algo]
            sns.barplot(data=algo_data, x='sampler', y='SSQ')
            plt.title(f'SSQ on Synthetic Data for {algo}')
            plt.xlabel('Algorithm')
            plt.ylabel('SSQ')
            plt.savefig(f'plots/quality_synth_samplers_{algo}.png', dpi=300)
            plt.clf()
        print("Synthetic samplers quality ploted")
    except Exception as e:
        print(f"Real samplers quality not ploted: {e}")

    try:
        df = pd.read_csv('benchmark_results/real_quality_samplers.csv')
        df[['sampler', 'algorithm']] = df['algorithm'].str.split(',', expand=True)
        algorithms = df['algorithm'].unique()
        for algo in algorithms:
            algo_data = df[df['algorithm'] == algo]
            sns.barplot(data=algo_data, x='sampler', y='SSQ')
            plt.title(f'SSQ on Real Data for {algo}')
            plt.xlabel('Algorithm')
            plt.ylabel('SSQ')
            plt.savefig(f'plots/quality_real_samplers_{algo}.png', dpi=300)
            plt.clf()
        print("Real samplers quality ploted")
    except Exception as e:
        print(f"Synthetic samplers quality not ploted: {e}")

def plot_benchmark():
    try:
        df = pd.read_csv('benchmark_results/processing_rate.csv')
        df['processing_rate'] = df.apply(
            lambda row: 0 if row['record_no'] == 0 else (10000 / row['interval']) * 1_000_000, axis=1
        )
        df = df[df['dimention'] == 4]
        # Group by 'algo' and 'record_no' and calculate the mean processing_rate
        avg_processing_rate = df.groupby(['algorithm', 'record_no'])['processing_rate'].mean().reset_index()
        # Rename the column for clarity
        avg_processing_rate.rename(columns={'processing_rate': 'avg_processing_rate'}, inplace=True)

        sns.scatterplot(data=avg_processing_rate, x='record_no', y='avg_processing_rate', hue="algorithm", style="algorithm")
        plt.title('Processing Rate Over Time')
        plt.xlabel('Number of records')
        plt.ylabel('Processing Rate (records per second)')
        plt.savefig('plots/processing_rate_over_time.png', dpi=300)
        plt.clf()
        print("Processing rate over time ploted")
    except Exception as e:
        print(f"Processing rate over time not ploted: {e}")

    try:
        df = pd.read_csv('benchmark_results/dimentionality_processing_time.csv')
        sns.lineplot(data=df, x='dimention', y='processing_time', hue='algorithm', style='algorithm')
        plt.title('Processing Time Over Dimentionality')
        plt.xlabel('Dimentionality')
        plt.ylabel('Processing Time (s)')
        plt.savefig('plots/processing_time_over_dimentionality.png', dpi=300)
        plt.clf()
        print("Processing time over dimentionality ploted")
    except Exception as e:
        print(f"Processing time over dimentionality not ploted: {e}")

    try:
        df = pd.read_csv('benchmark_results/processing_rate_samplers.csv')
        df['processing_rate'] = df.apply(
            lambda row: 0 if row['record_no'] == 0 else (10000 / row['interval']) * 1_000_000, axis=1
        )
        df = df[df['dimention'] == 4]
        # Split the 'algorithm' column into 'sampler' and 'algorithm'
        df[['sampler', 'algorithm']] = df['algorithm'].str.split(',', expand=True)
        # Group by 'sampler', 'algorithm', and 'record_no' and calculate the mean processing_rate
        avg_processing_rate = df.groupby(['sampler', 'algorithm', 'record_no'])['processing_rate'].mean().reset_index()
        # Rename the column for clarity
        avg_processing_rate.rename(columns={'processing_rate': 'avg_processing_rate'}, inplace=True)

        # Plot for each algorithm
        algorithms = avg_processing_rate['algorithm'].unique()
        for algo in algorithms:
            algo_data = avg_processing_rate[avg_processing_rate['algorithm'] == algo]
            sns.scatterplot(data=algo_data, x='record_no', y='avg_processing_rate', hue="sampler", style="sampler")
            plt.title(f'Processing Rate Over Time for {algo}')
            plt.xlabel('Number of records')
            plt.ylabel('Processing Rate (records per second)')
            plt.legend(title='Sampler')
            plt.savefig(f'plots/samplers_processing_rate_over_time_{algo}.png', dpi=300)
            plt.clf()
        print("Processing rate over time for samplers ploted")
    except Exception as e:
        print(f"Processing rate over time for samplers not ploted: {e}")

def plot_demos():
    results_dir = Path('./demo_results')
    for result_path in results_dir.glob('*.csv'):
        print(f"Processing {result_path}")
        splt_path = result_path.stem.split('_')
        demo_type = splt_path[0]
        algorithm = splt_path[-1]

        df = pd.read_csv(result_path)
        if not df.empty:
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
            plt.clf()
        plt.close()


if __name__ == "__main__":
    Path("./plots").mkdir(parents=True, exist_ok=True)
    plot_quality()
    plot_benchmark()
    plot_demos()
    
