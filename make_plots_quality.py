import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.read_csv('benchmark_results/real_quality.csv')
sns.barplot(data=df, x='algorithm', y='SSQ')
plt.title('SSQ on Real Data')
plt.xlabel('Algorithm')
plt.ylabel('SSQ')
plt.savefig('plots/quality_real.png', dpi=300)
plt.clf()

df = pd.read_csv('benchmark_results/synthetic_quality.csv')
sns.barplot(data=df, x='algorithm', y='SSQ')
plt.title('SSQ on Synthetic Data')
plt.xlabel('Algorithm')
plt.ylabel('SSQ')
plt.savefig('plots/quality_synth.png', dpi=300)
plt.clf()

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

