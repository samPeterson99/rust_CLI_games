pub struct Row {
    pub one: char,
    pub two: char,
    pub three: char,
    pub four: char,
    pub five: char,
}

impl From<Vec<char>> for Row {
    fn from(vector: Vec<char>) -> Self {
        Self {
            one: vector[0],
            two: vector[1],
            three: vector[2],
            four: vector[3],
            five: vector[4],
        }
    }
}
