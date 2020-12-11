use std::fmt;
use std::fmt::Display;

pub struct Grid<T> {
    pub vec: Vec<Vec<T>>,
}
impl<T> Grid<T> {
    pub fn size(&self) -> (usize, usize) {
        if self.vec.len() == 0 {
            (0, 0)
        } else {
            (self.vec.len(), self.vec[0].len())
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        Grid { vec: v }
    }
}

impl<T> AsRef<Vec<Vec<T>>> for Grid<T> {
    fn as_ref(&self) -> &Vec<Vec<T>> {
        self.vec.as_ref()
    }
}

impl<T> AsMut<Vec<Vec<T>>> for Grid<T> {
    fn as_mut(&mut self) -> &mut Vec<Vec<T>> {
        self.vec.as_mut()
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ref line in self.as_ref().into_iter() {
            for ref x in line.into_iter() {
                write!(f, "{}", x).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}
