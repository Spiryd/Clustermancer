# Abstract

A fundamentally different philosophy for data stream clustering which is guided by application-centered requirements. The idea is divide the clustering process into an online component which periodically stores detailed summary statistics and an offline component which uses only this summary statistics. The offline component is utilized by the analyst who can use a wide variety of inputs (such as time horizon or number of clusters) in order to provide a quick understanding of the broad clusters in the data stream

# Description
## Micro-cluster
A micro-cluster is a temporal extension of the [[BIRCH#Clustering Feature |Clustering Feature]]. A micro-cluster for a set of d-dimensional points $X_{i_1}, \dots, X_{i_n}$ with time stamps $T_{i_1}, \dots, T_{i_n}$ is defined as (2*d + 3) tuple $(\overrightarrow{CF2^x}, \overrightarrow{CF1^x}, CF2^t, CF1^t, n)$
 $$\overrightarrow{CF2^x} = \sum^{n}_{j=1}{(\overrightarrow{X_j})^2}$$
$$\overrightarrow{CF1^x} = \sum^{n}_{j=1}{\overrightarrow{X_j}}$$
$$CF2^t = \sum^{n}_{j=1}(T_j)^2$$
$$CF1^t = \sum^{n}_{j=1}T_j$$
The micro-clusters are also stored at particular moments in the stream which are referred to as snapshots.
## Pyramidal Timeframe

In this technique, the snapshots are stored at differing levels of granularity depending upon the recency. Snapshots are classified into different orders which can vary from $1$ to $log(T)$,
