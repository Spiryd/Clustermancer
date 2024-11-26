import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

df = pd.read_csv('benchmark_results/processing_rate.csv')
df['processing_rate'] = df.apply(
    lambda row: 0 if row['record_no'] == 0 else (10000 / row['interval']) * 1000, axis=1
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

df = pd.read_csv('benchmark_results/dimentionality_processing_time.csv')
sns.lineplot(data=df, x='dimention', y='processing_time', hue='algorithm', style='algorithm')
plt.title('Processing Time Over Dimentionality')
plt.xlabel('Dimentionality')
plt.ylabel('Processing Time (s)')

plt.savefig('plots/processing_time_over_dimentionality.png', dpi=300)
