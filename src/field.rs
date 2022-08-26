pub const FIELD_ROW_SIZE: usize = 41;
pub const FIELD_COLUMN_SIZE: usize = 63;
pub const FIELD_CHIPS: usize = FIELD_ROW_SIZE * FIELD_COLUMN_SIZE;

#[derive(Eq, PartialEq, Debug)]
struct Field<T>([[T; FIELD_COLUMN_SIZE]; FIELD_ROW_SIZE]);

impl<T> Field<T> {
    fn set(&mut self, x: usize, y: usize, entity: T) {
        self.0[x][y] = entity
    }
}

impl<T: Default + Copy> Default for Field<T> {
    fn default() -> Self {
        Field([[T::default(); FIELD_COLUMN_SIZE]; FIELD_ROW_SIZE])
    }
}

impl<T: Default> Field<T> {
    fn init(&mut self) {
        for x in self.iter_mut() {
            *x = T::default()
        }
    }
    fn iter<'a>(&self) -> FieldIter<'_, T> {
        FieldIter { i: 0, field: &self }
    }
    fn iter_mut(&mut self) -> FieldIterMut<'_, T> {
        FieldIterMut { i: 0, field: self }
    }
}

struct FieldIter<'a, T> {
    i: usize,
    field: &'a Field<T>,
}

impl<'a, T> Iterator for FieldIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        if i >= FIELD_CHIPS {
            None
        } else {
            let next = &self.field.0[i / FIELD_COLUMN_SIZE][i % FIELD_COLUMN_SIZE];
            self.i += 1;
            Some(next)
        }
    }
}

struct FieldIterMut<'a, T> {
    i: usize,
    field: &'a mut Field<T>,
}

impl<'a, T> Iterator for FieldIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        if i >= FIELD_CHIPS {
            None
        } else {
            self.i += 1;
            unsafe {
                Some(
                    &mut *self.field.0[i / FIELD_COLUMN_SIZE]
                        .as_mut_ptr()
                        .add(i % FIELD_COLUMN_SIZE),
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::field::{Field, FIELD_COLUMN_SIZE, FIELD_ROW_SIZE};

    #[test]
    fn iter_test() {
        let mut x = Field([[128usize; FIELD_COLUMN_SIZE]; FIELD_ROW_SIZE]);
        assert!(x.iter().all(|t| t.eq(&128)));
        x.init();
        assert_eq!(x, Field::default());
    }

    fn set_test() {
        let mut x = Field::default();
        x.set(2, 3, 64usize);
        assert_eq!(
            x.iter().position(|t| t.eq(&64)),
            Some(&2 * FIELD_COLUMN_SIZE + 3)
        )
    }
}
