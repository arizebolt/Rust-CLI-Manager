mod password_manager;
mod auth;
mod backup;
mod db;
mod utils;

use password_manager::PasswordManager;

fn main() {
    // Create a new PasswordManager instance
    let mut pm = PasswordManager::new();

    // Authenticate the user
    if !auth::authenticate(&mut pm) {
        println!("Authentication failed. Exiting...");
        return;
    }

    // Load the password database
    if let Err(e) = db::load_database(&mut pm) {
        println!("Failed to load database: {}", e);
        return;
    }

    // Perform any necessary backup or restore operations
    backup::do_backup(&mut pm);

    // Process command-line arguments
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.get(0).map(|s| s.as_str()).unwrap_or("help");

    match cmd {
        "add" => password_manager::add_password(&mut pm, &args[1..]),
        "get" => password_manager::get_password(&mut pm, &args[1..]),
        "update" => password_manager::update_password(&mut pm, &args[1..]),
        "delete" => password_manager::delete_password(&mut pm, &args[1..]),
        "list" => password_manager::list_passwords(&mut pm),
        "backup" => backup::create_backup(&mut pm),
        "restore" => backup::restore_backup(&mut pm, &args[1..]),
        "help" | _ => password_manager::print_help(),
    }

    // Save changes to the password database
    if let Err(e) = db::save_database(&pm) {
        println!("Failed to save database: {}", e);
    }
}
