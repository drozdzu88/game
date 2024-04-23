pub mod character {
    use std::fmt;
    
    
    pub struct Character {
       pub name: String,
       pub class: Class,
       pub strength: u32, 
       pub dexterity: u32, 
       pub constitution: u32,
       intelligence: u32,
       wisdom: u32,
       charisma: u32,
       skills: Vec<String>,
       equipment: Vec<String>,
    }

    #[derive(Clone, Copy)]
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
                class,
                strength: match class {
                    Class::Archer => 10,
                    Class::Mage => 8,
                    Class::Palladin => 15,
                    Class::Warrior => 12,
                },         
                dexterity: match class {
                    Class::Archer => 15,
                    Class::Mage => 10,
                    Class::Palladin => 8,
                    Class::Warrior => 12,
                },        
                constitution: match class {
                    Class::Archer => 10,
                    Class::Mage => 8,
                    Class::Palladin => 12,
                    Class::Warrior => 15,
                },     
                intelligence: match class {
                    Class::Archer => 12,
                    Class::Mage => 15,
                    Class::Palladin => 10,
                    Class::Warrior => 8,
                },     
                wisdom: match class {
                    Class::Archer => 10,
                    Class::Mage => 15,
                    Class::Palladin => 12,
                    Class::Warrior => 8,
                },           
                charisma: match class {
                    Class::Archer => 10,
                    Class::Mage => 12,
                    Class::Palladin => 15,
                    Class::Warrior => 8,
                },         
                skills: Vec::new(),   
                equipment: Vec::new(),

            };
            character
        }

        pub fn describe(&self) {
            println!("Your character name is {}, and you're a {}\nyour basic strength is {}\nyour basic dexterity is {}\nyour basic constitution is {}\nyour basic intelligence is {}\nyour basic wisdom is {}\nyour basic charisma is {}\nyour skills: {:?}\nyour equipment: {:?}", self.name, self.class, self.strength, self.dexterity, self.constitution, self.intelligence, self.wisdom, self.charisma, self.skills, self.equipment);
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