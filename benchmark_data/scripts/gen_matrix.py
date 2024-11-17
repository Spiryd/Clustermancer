from gen_normal_centers import generate_clustered_data

for dimentionality in [2, 4, 5, 10, 20, 40, 60, 80]:
    for clusters in [2, 5, 10, 15, 20]:
        generate_clustered_data(1_000_000, clusters, f'benchmark_data/synthetic/random_{clusters}k_{dimentionality}d.csv', dimensions=dimentionality, cluster_std=8.0, min_distance=15.0, center_range=100.0, order='random')