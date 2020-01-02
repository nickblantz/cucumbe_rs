pub trait IntoToken {
    fn get_symbol(&self) -> String;
    fn get_value(&self) -> &String;
}

impl std::fmt::Debug for dyn IntoToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{{ {} '{}' }}", self.get_symbol(), *self.get_value())
    }
}
