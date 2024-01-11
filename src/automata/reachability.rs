use std::collections::HashSet;

use ndarray::Array2;

pub struct Reachability {
    _matrix: Array2<usize>,
    pub as_outcoming: Vec<Vec<usize>>,
    pub as_incoming: Vec<HashSet<usize>>,
}

impl Reachability {
    pub fn from_automata(a: &super::AutomataImpl) -> Self {
        let _matrix = Self::get_matrix(a);
        let as_outcoming = Self::get_outcoming(&_matrix);
        let as_incoming = Self::get_incoming(&as_outcoming);

        Self {
            _matrix,
            as_outcoming,
            as_incoming,
        }
    }

    fn get_matrix(a: &super::AutomataImpl) -> Array2<usize> {
        let mut adjacency_vec = vec![0; a.size * a.size];
        for (i, row) in a.transitions.iter().enumerate() {
            for (j, letter_opt) in row.iter().enumerate() {
                if letter_opt.is_some() {
                    adjacency_vec[i * a.size + j] = 1;
                }
            }
        }

        let adjacency_matrix = Array2::from_shape_vec((a.size, a.size), adjacency_vec).unwrap();

        let mut reachability_matrix = adjacency_matrix.clone();
        let mut composition_matrix = adjacency_matrix.clone();
        for _ in 0..a.size - 1 {
            composition_matrix = composition_matrix.dot(&adjacency_matrix);
            reachability_matrix += &composition_matrix;
        }

        reachability_matrix
    }

    fn get_outcoming(matrix: &Array2<usize>) -> Vec<Vec<usize>> {
        let mut outcoming = vec![Vec::<usize>::new(); matrix.dim().0];

        for ((i, j), paths_count) in matrix.indexed_iter() {
            if paths_count.gt(&0) {
                outcoming[i].push(j);
            }
        }

        outcoming
    }

    fn get_incoming(outcoming: &Vec<Vec<usize>>) -> Vec<HashSet<usize>> {
        let mut incoming = vec![HashSet::<usize>::new(); outcoming.len()];

        for (i, row) in outcoming.iter().enumerate() {
            for j in row.iter() {
                incoming[*j].insert(i);
            }
        }

        incoming
    }
}
