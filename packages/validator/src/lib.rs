pub struct Validator;

impl Validator {
    pub fn string(value: String) -> StringValidator {
        StringValidator::new(value)
    }
}

pub trait TypeValidator {
    fn validate(&self) -> bool;

    fn get_error_message(&self) -> String;
}

#[derive(Clone, PartialEq)]
pub struct StringValidator {
    value: String,
    validate: bool,
    error_message: String,
}

impl StringValidator {
    pub fn new(value: String) -> Self {
        Self { value, validate: true, error_message: String::new() }
    }

    pub fn min_length(mut self, len: usize, error_message: &'static str) -> Self {
        self.validate = self.value.len() >= len;
        self.handle_error_message(error_message)
    }

    pub fn required(mut self, error_message: &'static str) -> Self {
        self.validate = !self.value.is_empty();
        self.handle_error_message(error_message)
    }

    fn handle_error_message(mut self, error_message: &'static str) -> Self {
        if !self.validate {
            self.error_message = error_message.to_string();
        }
        self
    }
}

impl TypeValidator for StringValidator {
    fn validate(&self) -> bool {
        self.validate
    }

    fn get_error_message(&self) -> String {
        self.error_message.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Validator;
    use crate::TypeValidator;

    #[test]
    fn string_min_length() {
        let values = vec![("foobar", true), ("foo", false)];

        for (value, expected) in values {
            let error_message: &'static str = "foobar must be at least 6 chars long"; 
            let validator = Validator::string(value.to_string()).min_length(6, error_message);

            assert_eq!(validator.validate(), expected);

            if !validator.validate() {
                assert_eq!(validator.error_message, error_message);
            }
        }
    }

    #[test]
    fn string_required() {
        let values = vec![("", false), ("foo", true)];

        for (value, expected) in values {
            let error_message: &'static str = "foo is required"; 
            let validator = Validator::string(value.to_string()).required(error_message);

            assert_eq!(validator.validate(), expected);

            if !validator.validate() {
                assert_eq!(validator.error_message, error_message);
            }
        }
    }
}
