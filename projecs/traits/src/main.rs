#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cake {
    Chocolate,
    MapleBacon,
    Spice,
}

#[derive(Debug)]
pub struct Party {
    pub at_restaurant: bool,
    pub num_people: u8,
    pub cake: Cake,
}

fn main() {
    let cake = Cake::Spice;
    admire_cake(cake);

    match cake {
        Cake::Chocolate => println!("The name's Chocolate. Dark...Chocolate."),
        Cake::MapleBacon => println!("Dreams do come true!"),
        Cake::Spice => println!("Great, let's spice it up!"),
    }

    impl Default for Party {
        fn default() -> Self {
            Party {
                at_restaurant: true,
                num_people: 8,
                cake: Cake::Chocolate,
            }
        }
    }

    impl PartialEq for Party {
        fn eq(&self, other: &Self) -> bool {
            self.cake == other.cake
        }
    }

    impl From<&Party> for Cake {
        fn from(party: &Party) -> Self {
            party.cake
        }
    }

    println!("The default Party is\n{:#?}", Party::default());

    let party = Party {
        cake: Cake::MapleBacon,
        ..Default::default()
    };

    println!("Yes! My party has my favorite {:?} cake!", party.cake);

    let other_party = Party {
        at_restaurant: false,
        num_people: 235,
        cake: Cake::MapleBacon,
    };

    if party == other_party {
        println!("Your party is just like mine!");
    }

    smell_cake(&party);

    println!(
        "Yum! I'm eating this cake: {:?}. Oops, I dropped it on the floor.",
        party.cake
    );
    drop(cake);
}

pub fn admire_cake(cake: Cake) {
    println!("What a nice {:?} cake! 🎂", cake);
}

pub fn smell_cake<T: Into<Cake>>(something: T) {
    println!("Hmm...something smells like a {:?} cake!", something.into());
}
