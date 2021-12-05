use std::ops::{Index, IndexMut};
use crate::utils::coordinates::Vector;

#[derive(Debug)]
pub struct Table<T> {
    table: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T: Copy> Table<T> {
    pub fn get_row(&self, idx: usize) -> Vec<T> {
        let mut row = Vec::new();
        for column in 0..self.columns {
            row.push(self.table[self.compute_idx(idx, column)]);
        }
        row
    }

    pub fn get_rows(&self) -> Vec<Vec<T>> {
        let mut rows = Vec::new();
        for row in 0..self.rows {
            rows.push(self.get_row(row));
        }
        rows
    }

    pub fn get_column(&self, idx: usize) -> Vec<T> {
        let mut columns = Vec::new();
        for row in 0..self.rows {
            columns.push(self.table[self.compute_idx(row, idx)]);
        }
        columns
    }

    pub fn get_columns(&self) -> Vec<Vec<T>> {
        let mut columns = Vec::new();
        for column in 0..self.columns{
            columns.push(self.get_column(column));
        }
        columns
    }

    pub fn get_vector(&self) -> &Vec<T> {
        &self.table
    }

    pub fn get_vetor_mut(&mut self) -> &mut Vec<T> {
        &mut self.table
    }

    pub fn from_vec(vector: Vec<T>, dimension: (usize, usize)) -> Self {
        Table {
            table: vector,
            rows: dimension.0,
            columns: dimension.1,
        }
    }

    pub fn from_vecvec(vecvec: Vec<Vec<T>>) -> Self {
        let mut vector = Vec::new();
        for vec in &vecvec {
            vector.append(&mut vec.clone());
        }
        Self::from_vec(vector, (vecvec.len(), vecvec[0].len()))
    }

    fn compute_idx(&self, row: usize, column: usize) -> usize {
        row * self.rows + column
    }
}

impl<T: Copy> Index<(usize, usize)> for Table<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.table[self.compute_idx(idx.0, idx.1)]
    }
}

impl<T: Copy> IndexMut<(usize, usize)> for Table<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let idx = self.compute_idx(idx.0, idx.1);
        &mut self.table[idx]
    }
}

impl<T: Copy> Index<&Vector<usize>> for Table<T> {
    type Output = T;

    fn index(&self, idx: &Vector<usize>) -> &Self::Output {
        &self.table[self.compute_idx(idx['x'], idx['y'])]
    }
}

impl<T: Copy> IndexMut<&Vector<usize>> for Table<T> {
    fn index_mut(&mut self, idx: &Vector<usize>) -> &mut Self::Output {
        let idx = self.compute_idx(idx['x'], idx['y']);
        &mut self.table[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA_STR: &str = "1 2 3 4 5
    6 7 8 9 10
    11 12 13 14 15
    16 17 18 19 20
    21 22 23 24 25";

    const TESTDATA_ARR: [&str; 25] = [
        "1", "2", "3", "4", "5",
        "6", "7", "8", "9", "10",
        "11", "12", "13", "14", "15",
        "16", "17", "18", "19", "20",
        "21", "22", "23", "24", "25",
    ];


    #[test]
    fn indexing_yields_ref() {
        let test_table = Table::from_vec(TESTDATA_ARR.to_vec(), (5, 5));
        assert_eq!("1", test_table[(0, 0)]);
    }

    #[test]
    fn indexing_yields_ref_next_row() {
        let test_table = Table::from_vec(TESTDATA_ARR.to_vec(), (5, 5));
        assert_eq!("19", test_table[(3, 3)]);
    }

    #[test]
    fn indexing_mut_enables_mutability() {
        let mut test_table = Table::from_vec(TESTDATA_ARR.to_vec(), (5, 5));
        assert_eq!("1", test_table[(0, 0)]);
        test_table[(0, 0)] = "2";
        assert_eq!("2", test_table[(0, 0)]);
    }
}
