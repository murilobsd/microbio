#[derive(Debug, Clone, Default)]
pub struct Options {
    name: String,
    address: String,
}

impl Options {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn name_mut(&mut self, value: String) {
        self.name = value;
    }

    pub fn address_mut(&mut self, value: String) {
        self.address = value;
    }
}

#[cfg(test)]
mod tests {
    use super::Options;

    #[test]
    fn default_options() {
        let mut options = Options::default();
        let name: String = String::from("name-server");
        let address: String = String::from("127.0.0.1:8080");

        options.name_mut(name.clone());
        options.address_mut(address.clone());

        assert_eq!(options.name(), &name);
        assert_eq!(options.address(), &address);
    }
}
