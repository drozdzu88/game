use character::new::Class;

mod character;

fn main() {
    let char_1 = character::new::Character::create_new_character(String::from("Łukasz"), Class::Mage);

    char_1.describe();
}
