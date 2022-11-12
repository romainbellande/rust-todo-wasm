pub struct Validator;

impl Validator {
    pub fn string(value: String) -> StringValidator {
        StringValidator::new(value)
    }
}

pub struct StringValidator {
    value: String,
    valid: bool,
}

impl StringValidator {
    pub fn new(value: String) -> Self {
        Self { value, valid: true }
    }

    pub fn min_length(mut self, len: usize) -> Self {
        self.valid = self.value.len() >= len;
        self
    }

    pub fn required(mut self) -> Self {
        self.valid = !self.value.is_empty();
        self
    }

    pub fn valid(&self) -> bool {
        self.valid
    }
}

#[cfg(test)]
mod tests {
    use crate::Validator;

    #[test]
    fn string_min_length() {
        let values = vec![("foobar", true), ("foo", false)];

        for (value, expected) in values {
            let result = Validator::string(value.to_string()).min_length(6).valid();

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn string_required() {
        let values = vec![("", false), ("foo", true)];

        for (value, expected) in values {
            let result = Validator::string(value.to_string()).required().valid();

            assert_eq!(result, expected);
        }
    }
}
