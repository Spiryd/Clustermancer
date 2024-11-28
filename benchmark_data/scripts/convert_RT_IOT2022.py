import pandas as pd

df = pd.read_csv('benchmark_data/real/RT_IOT2022.csv')
df = df.drop(df.columns[0], axis=1)
df = df.drop(df.columns[-1], axis=1)
df = df.astype(float)
df.reset_index(drop=True, inplace=True)
df.to_csv('benchmark_data/real/converted_RT_IOT2022.csv', index=False)
df = df.iloc[:, :10]
df.to_csv('benchmark_data/real/converted_RT_IOT2022_10.csv', index=False)