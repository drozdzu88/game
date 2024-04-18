mod lib;

use lib::character::Class;

fn main() {
    // let char_1 = create_new_character(String::from("Łukasz"), Class::Mage);
    let char_1 = lib::character::Character::create_new_character(String::from("Łukasz"), Class::Mage);

    char_1.describe();
}
