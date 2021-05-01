#[derive(Debug, Clone)]
pub struct Options {
    id: String,
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

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name_mut(&mut self, value: String) {
        self.name = value;
    }

    pub fn address_mut(&mut self, value: String) {
        self.address = value;
    }

    pub fn id_mut(&mut self, value: String) {
        self.id = value;
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            name: "rs.microbio.server".to_string(),
            address: "127.0.0.1:8080".to_string(),
            id: "id".to_string(),
        }
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
        let id: String = String::from("id");

        options.name_mut(name.clone());
        options.address_mut(address.clone());
        options.id_mut(id.clone());

        assert_eq!(options.name(), &name);
        assert_eq!(options.address(), &address);
        assert_eq!(options.id(), &id);
    }
}
