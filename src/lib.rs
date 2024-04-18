pub mod character {
    use std::fmt;
    
    
    pub struct Character {
       pub name: String,
       pub class: Class
    }

    
    pub enum Class {
        Mage,
        Warrior,
        Palladin,
        Archer,
    }

    impl Character {

        pub fn create_new_character(name: String, class: Class) -> Character {
            let character = Character {
                name,
                class
            };
            character
        }

        pub fn describe(&self) {
            println!("Your character name is {}, and you're a {}", self.name, self.class);
        }
    }

    impl fmt::Display for Character {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}, {}", self.name, self.class)
        }
    }

    impl fmt::Display for Class {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let printable = match *self {
                Class::Archer => "Archer",
                Class::Mage => "Mage",
                Class::Palladin => "Palladin",
                Class::Warrior => "Warrior",
            };

            write!(f, "{}", printable)
        }
    }

}