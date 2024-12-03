#[derive(Default)]
pub enum TwoIter<T> {
    #[default]
    Zero,
    One(T),
    Two(T, T),
}

impl<T> Iterator for TwoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let this = std::mem::take(self);
        match this {
            Self::Zero => None,
            Self::One(value) => {
                *self = Self::Zero;
                Some(value)
            }
            Self::Two(first, second) => {
                *self = Self::One(second);
                Some(first)
            }
        }
    }
}
