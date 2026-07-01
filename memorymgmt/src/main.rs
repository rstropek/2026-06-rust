use std::{cell::RefCell, rc::Rc};

//#[derive(Clone, Copy)]
struct MyPreciousRing {
    engraving: &'static str
}

impl Drop for MyPreciousRing {
    fn drop(&mut self) {
        println!("The ring is destroyed!");
    }
}

impl MyPreciousRing {
    fn forge() -> Self {
        MyPreciousRing {
            engraving: "One Ring to rule them all, One Ring to find them, One Ring to bring them all and in the darkness bind them."
        }
    }

    fn heat(&mut self) {
        self.engraving = "Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.";
    }
}

struct Character {
    name: &'static str,
    power: u32,
}

fn battle<'a>(c1: &'a Character, c2: &'a Character) -> &'a Character {
    if c1.power > c2.power {
        c1
    } else {
        c2
    }
}

struct Team {
    number_of_members: u32,
    members: Vec<Rc<Character>>,
    leader: Rc<Character>
}

fn main() {
    let characters = vec![
        Character {
            name: "Frodo",
            power: 10,
        },
        Character {
            name: "Samwise",
            power: 20,
        },
        Character {
            name: "Gollum",
            power: 30,
        },
    ];
    let team: Vec<Rc<Character>> = characters.into_iter().map(|c| Rc::new(c)).collect();
    let t = Team {
        number_of_members: team.len() as u32,
        leader: Rc::clone(&team[2]),
        members: team,
    };
    println!("The leader of the team is {}", t.leader.name);

    let gandalf = Character {
        name: "Gandalf",
        power: 1001,
    };

    let balrog = Character {
        name: "Balrog",
        power: 1000,
    };

    let winner = battle(&gandalf, &balrog);
    //drop(balrog);
    println!("The winner is {}", winner.name);

    let saurons_ring = MyPreciousRing::forge();
    println!("Sauron's ring engraving: {}", saurons_ring.engraving);

    let gollumns_ring = saurons_ring;
    //println!("Sauron's ring engraving: {}", saurons_ring.engraving);
    println!("Gollumn's ring engraving: {}", gollumns_ring.engraving);

    let bilbos_ring = gollumns_ring;
    //println!("Gollumn's ring engraving: {}", gollumns_ring.engraving);
    println!("Bilbo's ring engraving: {}", bilbos_ring.engraving);

    let mut bilbos_ring = bilbos_ring; // Shaddowing
    bilbos_ring.heat();
    //let bilbos_ring = bilbos_ring;
    //bilbos_ring.heat();

    drop(bilbos_ring);

    let saurons_ring = Rc::new(MyPreciousRing::forge());
    println!("Sauron's ring engraving: {}", saurons_ring.engraving);
    println!("Counter: {}", Rc::strong_count(&saurons_ring));
    let frodos_ring = Rc::clone(&saurons_ring);
    println!("Counter: {}", Rc::strong_count(&saurons_ring));
    let samwise_ring = frodos_ring.clone(); // NO cloning of the ring, just cloning the reference to it
    println!("Counter: {}", Rc::strong_count(&saurons_ring));
    drop(samwise_ring);
    println!("Counter: {}", Rc::strong_count(&saurons_ring));
    drop(frodos_ring);

    let saurons_ring = Rc::new(RefCell::new(MyPreciousRing::forge()));
    // RefCell implements borrow checker AT RUNTIME!!!!!!
    let mut saurons_ring_mut = saurons_ring.borrow_mut();
    saurons_ring_mut.heat();
    drop(saurons_ring_mut);
    println!("Sauron's ring engraving: {}", saurons_ring.borrow().engraving);

}
