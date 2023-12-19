pub struct Interval<T> {
    pub min: T,
    pub max: T,
}

impl<T> Interval<T> {
    pub fn new(min: T, max: T) -> Self {
        Interval { min, max }
    }

    pub fn surrounds(&self, value: &T) -> bool
    where
        T: PartialOrd,
    {
        &self.min < value && value < &self.max
    }

    pub fn clamp(&self, value: &T) -> T
    where
        T: PartialOrd + Copy,
    {
        if value < &self.min {
            self.min
        } else if value > &self.max {
            self.max
        } else {
            *value
        }
    }
}
