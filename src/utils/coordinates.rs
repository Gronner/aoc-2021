use num;

#[derive(Clone, Debug)]
pub struct Vector<T> {
    dimensions: Vec<T>
}

impl Vector<isize> {
    pub fn normalized(&self) -> Self {
        Vector {
            dimensions: self.dimensions.iter()
                .map(|&dimension| num::signum(dimension))
                .collect(),
        }
    }
}

impl Vector<usize> {
    pub fn normalized(&self) -> Self {
        Vector {
            dimensions: self.dimensions.iter()
                .map(|&dimension| if dimension > num::zero() { 1 } else { 0 })
                .collect(),
        }
    }
}

impl<T> From<(T, T)> for Vector<T> {
    fn from(input: (T, T)) -> Self {
        Vector {
            dimensions: vec![input.0, input.1],
        }
    }
}

impl<T> From<(T, T, T)> for Vector<T> {
    fn from(input: (T, T, T)) -> Self {
        Vector {
            dimensions: vec![input.0, input.1, input.2],
        }
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(input: Vec<T>) -> Self {
        Vector {
            dimensions: input,
        }
    }
}

impl<T: Copy + std::ops::Add<Output = T>> std::ops::Add for &Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            dimensions: self.dimensions.iter()
                .zip(rhs.dimensions.iter())
                .map(|(&l, &r)| l + r)
                .collect(),
        }
    }
}

impl std::ops::Add<&Vector<isize>> for &Vector<usize> {
    type Output = Vector<usize>;

    fn add(self, rhs: &Vector<isize>) -> Self::Output {
        Vector {
            dimensions: self.dimensions.iter()
                .zip(rhs.dimensions.iter())
                .map(|(&l, &r)| (l as isize + r) as usize)
                .collect(),
        }
    }
}

impl std::ops::Add<&Vector<usize>> for &Vector<isize> {
    type Output = Vector<isize>;

    fn add(self, rhs: &Vector<usize>) -> Self::Output {
        Vector {
            dimensions: self.dimensions.iter()
                .zip(rhs.dimensions.iter())
                .map(|(&l, &r)| l + r as isize)
                .collect(),
        }
    }
}

impl<T: Copy + std::ops::Sub<Output = T> + num::ToPrimitive> std::ops::Sub for &Vector<T> {
    type Output = Vector<isize>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            dimensions: self.dimensions.iter()
                .zip(rhs.dimensions.iter())
                .map(|(&l, &r)| -(l.to_isize().unwrap() -  r.to_isize().unwrap()))
                .collect::<Vec<isize>>(),
        }
    }
}

impl<T> std::ops::Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.dimensions[idx]
    }
}

impl<T> std::ops::IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.dimensions[idx]
    }
}

impl<T> std::ops::Index<char> for Vector<T> {
    type Output = T;

    fn index(&self, idx: char) -> &Self::Output {
        let idx = match idx {
            'x' => 0,
            'y' => 1,
            'z' => 2,
            _ => panic!("Character indexing is only supported for x, y and z"),
        };
        &self.dimensions[idx]
    }
}

impl<T> std::ops::IndexMut<char> for Vector<T> {
    fn index_mut(&mut self, idx: char) -> &mut Self::Output {
        let idx = match idx {
            'x' => 0,
            'y' => 1,
            'z' => 2,
            _ => panic!("Character indexing is only supported for x, y and z"),
        };
        &mut self.dimensions[idx]
    }
}

impl<T: PartialEq + Copy> std::cmp::PartialEq for &Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.dimensions.iter().zip(other.dimensions.iter()).all(|(&l, &r)| l == r)
    }
}
