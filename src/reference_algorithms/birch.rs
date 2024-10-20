use std::fmt::Debug;

#[derive(Debug, Clone)]
struct ClusteringFeature {
    n: usize,
    ls: Vec<f64>,
    ss: f64,
}

impl ClusteringFeature {
    fn new(element: Vec<f64>) -> Self {
        ClusteringFeature {
            n: 1,
            ls: element.clone(),
            ss: element.iter().map(|&sub_element| sub_element * sub_element).sum(),
        }
    }

    fn centroid(&self) -> Vec<f64> {
        self.ls.iter().map(|&l| l / self.n as f64).collect()
    }

    fn radius(&self) -> f64 {
        (self.ss / self.n as f64) - (self.centroid().iter().map(|&c| c * c).sum::<f64>().sqrt())
    }

    /// Euclidean distance
    fn distance_0(&self, other: &Self) -> f64 {
        let a: Vec<f64> = self.centroid();
        let b: Vec<f64> = other.centroid();
        a.iter().zip(b.iter()).map(|(&a, &b)| (a - b).powi(2)).sum::<f64>().sqrt()
    }

    #[allow(dead_code)]
    /// Manhattan distance
    fn distance_1(&self, other: &Self) -> f64 {
        let a: Vec<f64> = self.centroid();
        let b: Vec<f64> = other.centroid();
        a.iter().zip(b.iter()).map(|(&a, &b)| (a - b).abs()).sum()
    }
}

impl std::ops::Add for ClusteringFeature {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let n = self.n + other.n;
        let ls = self.ls.iter().zip(other.ls.iter()).map(|(&a, &b)| a + b).collect();
        let ss = self.ss + other.ss;
        ClusteringFeature { n, ls, ss }
    }
}

#[derive(Clone, Debug)]
enum CFNode {
    Leaf{
        id: usize,
        parent_id: Option<usize>,
        features: Vec<ClusteringFeature>,
        prev: Option<usize>,
        next: Option<usize>,
    },
    NonLeaf {
        id: usize,
        parent_id: Option<usize>,
        features: Vec<(ClusteringFeature, usize)>,
    },
}

#[derive(Debug)]
struct CFTree {
    arena: Vec<CFNode>,
    next_id: usize,
    threshold: f64,
    branching_factor: usize,
}

impl CFTree {
    fn new(threshold: f64, branching_factor: usize) -> Self {
        CFTree {
            arena: Vec::new(),
            next_id: 0,
            threshold,
            branching_factor,
        }
    }

    fn insert(&mut self, instance: Vec<f64>) {
        let entry =  ClusteringFeature::new(instance);

        match self.arena.get_mut(0) {
            Some(root) => {
                let mut current_node = root;
                loop {
                    match current_node {
                        CFNode::Leaf { id, features, prev, next, parent_id } => {
                            let closest_feature_idx = features.iter().enumerate().min_by(|(_, a), (_, b)| {
                                a.distance_0(&entry).partial_cmp(&b.distance_0(&entry)).unwrap()
                            }).unwrap().0;
                            if (features[closest_feature_idx].clone() + entry.clone()).radius() < self.threshold {
                                features[closest_feature_idx] = features[closest_feature_idx].clone() + entry;
                            } else if features.len() < self.branching_factor {
                                features.push(entry);
                            } else {
                                let (seed_0, seed_1, _) = features.iter().enumerate().flat_map(|(i, f)| {
                                    features.iter().enumerate().map(move |(j, g)| (i, j, f.distance_0(g)))
                                }).max_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap()).unwrap();
                                let (group_0, group_1): (Vec<_>, Vec<_>) = features.iter().cloned().partition(|cf| cf.distance_0(&features[seed_0]) < cf.distance_0(&features[seed_1]));
                                let new_leaf_0 = CFNode::Leaf {
                                    id: self.next_id,
                                    features: group_0,
                                    prev: *prev,
                                    next: Some(self.next_id + 1),
                                    parent_id: Some(*id),
                                };
                                self.next_id += 1;
                                let new_leaf_1 = CFNode::Leaf {
                                    id: self.next_id,
                                    features: group_1,
                                    prev: Some(self.next_id - 1),
                                    next: *next,
                                    parent_id: Some(*id),
                                };
                                self.next_id += 1;
                                self.arena.push(new_leaf_0);
                                self.arena.push(new_leaf_1);
                            }
                            break;
                        },
                        CFNode::NonLeaf { features, .. } => {
                            let closest_child_id = features.iter().min_by(|(a, _), (b, _)| {
                                a.distance_0(&entry).partial_cmp(&b.distance_0(&entry)).unwrap()
                            }).unwrap().1;
                            current_node = &mut self.arena[closest_child_id];
                        },
                    }
                }
            },
            None => {
                let mut features = Vec::with_capacity(self.branching_factor);
                features.push(entry);
                let leaf = CFNode::Leaf {
                    id: self.next_id,
                    features,
                    prev: None,
                    next: None,
                    parent_id: None,
                };
                self.arena.push(leaf);
                self.next_id += 1;
            },
        }
    }
}

#[derive(Debug)]
pub struct Birch {
    tree: CFTree,
}

impl Birch {
    pub fn new(threshold: f64, branching_factor: usize) -> Self {
        Birch {
            tree: CFTree::new(threshold, branching_factor),
        }
    }

    pub fn insert(&mut self, instance: Vec<f64>) {
        self.tree.insert(instance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clustering_feature() {
        let cf1 = ClusteringFeature::new(vec![2.]);
        let cf2 = ClusteringFeature::new(vec![3.]);
        let cf3 = cf1 + cf2;

        assert_eq!(cf3.centroid(), vec![2.5]);
        assert_eq!(cf3.radius(), 4.);

    }
}
