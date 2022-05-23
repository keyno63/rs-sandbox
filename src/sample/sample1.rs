#[derive(Debug, PartialEq)]
pub struct Sample {
    pub name: String
}

impl Eq for Sample {}

impl Sample {
    pub fn new(name: String) -> Self {
        Self{name}
    }
}

#[cfg(test)]
mod tests {
    use sample::Sample;
    use crate::sample;

    #[test]
    fn test_eq() {
        let s1 = Sample::new(String::from("name"));
        let s2 = Sample::new(String::from("name"));
        assert_eq!(s1, s2);
    }
}
