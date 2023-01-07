pub struct Helpers {}

impl Helpers {
    pub fn get_difference_between_vectors<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
        let mut difference = Vec::new();
        for x in a {
            if !b.contains(x) {
                difference.push(x.to_owned());
            }
        }

        difference.to_vec()
    }
}