enum FlashMessage {
  Success, // A unit variant
  Warning{ category: i32, message: String }, // A struct variant
  Error(String) // A tuple variant
}

fn main() {
  let mut form_status = FlashMessage::Success;
  print_flash_message(form_status);

  form_status = FlashMessage::Warning {category: 2, message: String::from("Field X is required")};
  print_flash_message(form_status);

  form_status = FlashMessage::Error(String::from("Connection Error"));
  print_flash_message(form_status);
}

fn print_flash_message(m : FlashMessage) {
  // Pattern matching with enum
  match m {
    FlashMessage::Success =>
      println!("Form Submitted correctly"),
    FlashMessage::Warning {category, message} => // Destructure, should use same field names
      println!("Warning : {} - {}", category, message),
    FlashMessage::Error(msg) =>
      println!("Error : {}", msg)
  }
}