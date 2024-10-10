## Introduction
In a world with an ever increasing data volume and velocity classic offline processing is becoming impractical due to the requirement of a massive storage capacity as well as the delay in the analysis.

## Data Streams
A data stream is a possibly infinite ordered sequence of data instances. Data streams are described by the following notation: $S = \{ x_1, x_2, x_3,\dots, x_i, \dots, x_N\}$ where $x_i$ is the $i^{th}$
instance. Each instance($x_i$) is a $d$-dimensional vector. Length $N$ of the stream goes to infinity.

## Data Clustering
Data clustering is a process in data mining and machine learning that involves grouping a set of objects or data points in such a way that items in the same group (known as a cluster) are more similar to each other than to those in other clusters. The primary goal of clustering is to discover patterns, trends, or structures within a dataset without requiring prior knowledge of the group labels. This unsupervised learning technique helps identify inherent structures in the data, making it useful for various tasks, including data compression, outlier detection, and exploratory data analysis.

## Online Approach vs Classical Approach

|                   | Online                         | Classical              |
| ----------------- | ------------------------------ | ---------------------- |
| Resaults          | Approximate Results ecceptable | Exact results expected |
| Data Velocity     | Fast                           | Stationaty             |
| Data Avaliability | Only one pass can be perfored  | Unbound access to data |
| Storage           | Only a synopsis is stored      | All of the raw data    |

## Data Stream Clustering Challenges and Solutions
- [[Concept Drift]]
- [[Time Window]]
- [[Outlier Detection]]
- [[Data Structures]]
- [[Offline Refinement Strategy]]\

