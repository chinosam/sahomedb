use super::*;

/// The ID of a vector record.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[derive(Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct VectorID(pub u32);

impl VectorID {
    /// True if this vector ID is valid.
    pub fn is_valid(&self) -> bool {
        self.0 != u32::MAX
    }
}

impl From<u32> for VectorID {
    fn from(id: u32) -> Self {
        VectorID(id)
    }
}

impl From<usize> for VectorID {
    fn from(id: usize) -> Self {
        VectorID(id as u32)
    }
}

/// The vector embedding of float numbers.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Vector(pub Vec<f32>);

impl Vector {
    /// Returns the dimension of the vector.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the Euclidean distance between two vectors.
    pub fn distance(&self, other: &Self) -> f32 {
        assert_eq!(self.0.len(), other.0.len());
        let iter = self.0.iter().zip(other.0.iter());
        iter.map(|(a, b)| (a - b).powi(2)).sum::<f32>().sqrt()
    }

    /// Generates a random vector for testing.
    /// * `dimension`: Vector dimension.
    pub fn random(dimension: usize) -> Self {
        let mut vec = vec![0.0; dimension];

        for float in vec.iter_mut() {
            *float = random::<f32>();
        }

        vec.into()
    }
}

impl Index<&VectorID> for [Vector] {
    type Output = Vector;
    fn index(&self, index: &VectorID) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl From<Vec<f32>> for Vector {
    fn from(vec: Vec<f32>) -> Self {
        Vector(vec)
    }
}

impl From<&Vec<f32>> for Vector {
    fn from(vec: &Vec<f32>) -> Self {
        Vector(vec.clone())
    }
}
