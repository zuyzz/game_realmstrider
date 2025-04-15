#[derive(Default)]
struct Attribute {
    value: i32,
    listeners: Vec<Box<dyn Fn(i32) -> ()>>,
}

impl Attribute {
    fn init(value: i32) -> Self {
        Attribute {
            value, 
            listeners: vec![]
        }
    }

    fn get(&self) -> i32 {
        self.value
    }
    
    fn set(&mut self, value: i32) {
        if self.value == value { 
            return; 
        }
        if value < 0 {
            self.value = 0
        } else {
            self.value = value;
        }
        for listener in self.listeners.iter() {
            listener(self.value);
        }
    }

    fn add_event_listener(&mut self, listener: Box<dyn Fn(i32) -> ()>){
        self.listeners.push(listener);
    }
}

#[derive(Default)]
struct Entity {
    profile: Identity,
    health: Attribute,
    strength: Attribute,
    magic: Attribute,
}

impl Entity {
    fn new(profile: Identity) -> Self {
        Entity {
            profile,
            ..Default::default()
        }
    }

    fn name(&self) -> &str {
        &self.profile.name
    }

    fn build_stats(&mut self, health: i32, strength: i32, magic: i32) {
        self.health = Attribute::init(health);
        self.strength = Attribute::init(strength);
        self.magic = Attribute::init(magic);
    }
}

#[derive(Default)]
struct Identity {
    pub name: String,
    pub age: u8,
    pub religion: String,
}

fn attack_strength(source: &Entity, target: &mut Entity) {
    let dmg = source.strength.get();    
    let health_before = target.health.get();
    let health_after = health_before - dmg;
    println!("[{}] deal {} physical damage to [{}] {}->{}", 
        source.name(), 
        dmg, 
        target.name(),
        health_before,
        health_after
    );
    target.health.set(health_after);
}

fn attack_magic(source: &Entity, target: &mut Entity) {
    let dmg = source.magic.get()+1;    
    let health_before = target.health.get();
    let health_after = health_before - dmg;
    println!("[{}] deal {} magical damage to [{}] {}->{}", 
        source.name(), 
        dmg, 
        target.name(),
        health_before,
        health_after
    );
    target.health.set(health_after);
}


fn main() {
    let mut player = Entity::new(
        Identity { 
            name: String::from("David"), 
            age: 0, 
            religion: String::from("")
        }
    );
    player.build_stats(100, 30, 40);
    player.health.add_event_listener(Box::new(|curr_val| -> () {
        if curr_val == 0 {
            println!("LOSE");
        }
    }));

    let mut enemy = Entity::new( 
        Identity { 
            name: String::from("Goblin"), 
            age: 0, 
            religion: String::from("")
        }
    );
    enemy.build_stats(100, 20, 20);
    enemy.health.add_event_listener(Box::new(|curr_val| -> () {
        if curr_val == 0 {
            println!("WIN");
        }
    }));

    let mut turn = 1;
    let mut end: bool = false;

    while !end {
        if turn % 2 == 1 {
            attack_strength(&player, &mut enemy);
            end = enemy.health.get() <= 0;
        } else {
            attack_magic(&enemy, &mut player);
            end = player.health.get() <= 0;
        }
        turn += 1;
    }
}
