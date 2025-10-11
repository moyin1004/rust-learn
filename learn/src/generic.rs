pub fn lastest<T: std::cmp::PartialOrd>(list: &mut [T]) -> &mut T {
    let mut latest_index = 0;

    for (i, item) in list.iter().enumerate() {
        if *item > (*list)[latest_index] {
            latest_index = i;
        }
    }

    &mut list[latest_index]
}

pub struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> &T {
        &self.x
    }
    pub fn y(&self) -> &T {
        &self.y
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}
