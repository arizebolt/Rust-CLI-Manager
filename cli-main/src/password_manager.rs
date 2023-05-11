use std::collections::HashMap;

pub struct PasswordManager {
    master_password: Option<String>,
    password_database: HashMap<String, String>,
}
fn main() {
    let mut password_manager = PasswordManager::new();

    // Add a password to the database
    password_manager.add_password("google.com", "my-google-password");

    // Retrieve a password from the database
    let password = password_manager.get_password("google.com");

    // Authenticate the user with the master password
    let is_authenticated = password_manager.authenticate("my-master-password");

    if is_authenticated {
        println!("Authenticated successfully!");
        if let Some(password) = password {
            println!("The password for google.com is: {}", password);
        } else {
            println!("There is no password stored for google.com");
        }
    } else {
        println!("Authentication failed!");
    }
}



impl PasswordManager {
    pub fn new() -> Self {
        Self {
            master_password: None,
            password_database: HashMap::new(),
        }
    }

    pub fn set_master_password(&mut self, password: &str) {
        self.master_password = Some(password.to_owned());
    }
    pub fn authenticate(&self, password: &str) -> bool {
        self.master_password.as_ref().map_or(false, |p| p == password)
    }
    
    pub fn add_password(&mut self, args: &[String]) {
        if args.len() < 2 {
            println!("Error: not enough arguments");
            return;
        }
        let key = args[0].clone();
        let value = args[1].clone();
        self.password_database.insert(key, value);
    }

    pub fn get_password(&self, args: &[String]) {
        if args.len() < 1 {
            println!("Error: not enough arguments");
            return;
        }
        let key = args[0].clone();
        match self.password_database.get(&key) {
            Some(value) => println!("Password: {}", value),
            None => println!("Error: no entry found for '{}'", key),
        }
    }

    pub fn update_password(&mut self, args: &[String]) {
        if args.len() < 2 {
            println!("Error: not enough arguments");
            return;
        }
        let key = args[0].clone();
        let value = args[1].clone();
        if self.password_database.contains_key(&key) {
            self.password_database.insert(key, value);
        } else {
            println!("Error: no password found");
        }
    }

    pub fn delete_password(& mut self, args: &[String]) {
        if args.len() < 1 {
            println!("Error: not enough arguemnts");
            return;
        }
        let key = args[0].clone();
        if self.password_database.remove(&key).is_none() {
            println!("No password found!");

        }
    }

    pub fn list_passwords(&self) {
        for (key, value) in &self.password_database {
            println!("{}: {}", key, value);
        }
    }

    pub fn print_help() {
        println!("Usage: pw_manager [add|get|update|delete|list|backup|restore|help]");
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_password() {
        let mut pm = PasswordManager::new();
        let args = vec!["add".to_string(), "example.com".to_string(), "username".to_string(), "password".to_string()];
        let result = add_password(&mut pm, &args);
        assert_eq!(result, Ok(()));
        assert_eq!(pm.password_database.len(), 1);
    }

    #[test]
    fn test_get_password() {
        let mut pm = PasswordManager::new();
        let args_add = vec!["add".to_string(), "example.com".to_string(), "username".to_string(), "password".to_string()];
        let result_add = add_password(&mut pm, &args_add);
        assert_eq!(result_add, Ok(()));
        let args_get = vec!["get".to_string(), "example.com".to_string()];
        let result_get = get_password(&mut pm, &args_get);
        assert_eq!(result_get, Ok(()));
    }
}
