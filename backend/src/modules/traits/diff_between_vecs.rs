pub trait GetDiff<T> {

    /// # get the diffference between two vectors
    /// get the difference between 2 vectors by iterating over them.
    /// this is not a performant option with a big O of n^2
    ///
    /// ## Arguments
    /// * `other` - The vector to compare to
    ///
    /// ## Returns
    /// * 'Vec<T>' - The difference between the two vectors
    fn diff(&self, other: &Vec<T>) -> Vec<T>;
}

impl<T: PartialEq + Clone > GetDiff<T> for Vec<T> {
    fn diff(&self, other: &Vec<T>) -> Vec<T> {
        let mut difference = Vec::new();
        for x in self {
            if !other.contains(x) {
                difference.push(x.to_owned());
            }
        }

        difference.to_vec()
    }
}
