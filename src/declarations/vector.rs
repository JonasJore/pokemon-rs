pub mod vector {
    pub trait VectorExtension {
        fn get_id(self, name: &str) -> usize;
    }

    impl VectorExtension for Vec<&str> {
        fn get_id(self, name: &str) -> usize {
            self.iter()
                .position(|&pokemon| pokemon.to_owned() == name)
                .unwrap()
                + 1
        }
    }
}
