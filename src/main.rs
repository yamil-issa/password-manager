mod cli;
use rpassword::prompt_password;
fn main() {
   let cli = cli::parse_args();

   // read the master password
   let master_password = prompt_password("Enter your master password: ").expect("Failed to read master password");

   match cli.command.as_str() {
    "add" => {
      println!("Adding a new password for {:?}", cli.service);
      println!("password added: {} ", master_password);
    },
    "get" => println!("Getting a password..."),
    "list" => println!("Listing all passwords..."),
    "delete" => println!("Deleting a password..."),
    _ => eprintln!("Invalid command"),
   }

}
