# Data Stream Clustering Algorithms

This is an implementation of some well known Data Stream Clustering Algorithms for my engineering thesis @ PWR Algorithmic Computer Science. This project also includes sampling wrappers for aforementioned algorithms.

# Author

Maksymilian Neumann

# Requirements

1. Internet connection
2. [Rust toolchain](https://www.rust-lang.org/tools/install)
3. Python 3.12
4. Install python libraries
````bash
pip install -r requirements.txt
````
5. Generate data
````bash
python gen_all_data.py
````

# Usage

1. Run
````bash
cargo run -r
````
2. Choose any benchmarks or run all
3. Plot to see results in ./plots
````bash
python gen_all_plots.py
````
4. (Optional) generate demo results for k-means and DBSCAN as a comparison
````bash
python classic_demo.py
````

# Implementations

### Algorithms:

- [x] BIRCH
- [x] CluStream
- [x] DenStream

### Samplers:

- [x] Static
- [x] Dynamic

# Data Used
Real data used:

B. S. and R. Nagapadma. "RT-IoT2022 ," UCI Machine Learning Repository, 2023. [Online]. Available: https://doi.org/10.24432/C5P338.
