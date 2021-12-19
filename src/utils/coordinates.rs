use num;

#[derive(Clone, Debug, Hash)]
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

    pub fn rotate(&mut self, axis: char, degree: u32) {
        let degree = (degree as f64).to_radians();
        match axis {
            'x' => self.rotate_x(degree),
            'y' => self.rotate_y(degree),
            'z' => self.rotate_z(degree),
            _ => unimplemented!(),
        }
    }

    fn rotate_x(&mut self, degree: f64) {
        let new_y = degree.cos() * (self.dimensions[1] as f64) - degree.sin() * (self.dimensions[2] as f64);
        let new_z = degree.sin() * (self.dimensions[1] as f64) + degree.cos() * (self.dimensions[2] as f64);
        self.dimensions[1] = new_y.round() as isize;
        self.dimensions[2] = new_z.round() as isize;
    }

    fn rotate_y(&mut self, degree: f64) {
        let new_x = degree.cos() * (self.dimensions[0] as f64) + degree.sin() * (self.dimensions[2] as f64);
        let new_z = degree.cos() * (self.dimensions[2] as f64) - degree.sin() * (self.dimensions[0] as f64);
        self.dimensions[0] = new_x.round() as isize;
        self.dimensions[2] = new_z.round() as isize;
    }

    fn rotate_z(&mut self, degree: f64) {
        let new_x = degree.cos() * (self.dimensions[0] as f64) - degree.sin() * (self.dimensions[1] as f64);
        let new_y = degree.sin() * (self.dimensions[0] as f64) + degree.cos() * (self.dimensions[1] as f64);
        self.dimensions[0] = new_x.round() as isize;
        self.dimensions[1] = new_y.round() as isize;
    }

    pub fn manhattan_distance(&self) -> u64 {
        self.dimensions.iter()
            .fold(0, |distance, value| (distance + value.abs())) as u64
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

impl<T: Copy + std::ops::Sub<Output = T> + num::ToPrimitive> std::ops::Sub for Vector<T> {
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

impl<T: PartialEq + Copy> std::cmp::PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.dimensions.iter().zip(other.dimensions.iter()).all(|(&l, &r)| l == r)
    }
}

impl<T: Eq + Copy> std::cmp::Eq for Vector<T> {}
