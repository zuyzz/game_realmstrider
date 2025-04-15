struct Entity {
    profile: Profile,
    health: i32,
    strength: i32,
    magic: i32,
}

impl Entity {
    fn new(profile: Profile, health: i32, strength: i32, magic: i32) -> Self {
        Entity {
            profile,
            health,
            strength,
            magic,
        }
    }

    fn name(&self) -> &str {
        &self.profile.name
    }

    fn modify_health(&mut self, value: i32) {
        self.health += value
    }
}

struct Profile {
    pub name: String,
    pub age: u8,
    pub religion: String,
}

fn attack(source: &Entity, target: &mut Entity) {
    let pre_health = target.health;
    let dmg = source.strength + source.magic;
    target.health -= dmg;
    println!("[{}] deal {} damage to [{}] {}->{}", 
    source.name(), 
    dmg, 
    target.name(),
    pre_health,
    target.health);
}

fn main() {
    let mut player: Entity = Entity::new(
        Profile { 
            name: String::from("David"), 
            age: 0, 
            religion: String::from("")
        }, 
        100, 
        40, 
        20
    );
    let mut enemy: Entity = Entity::new( 
        Profile { 
            name: String::from("Goblin"), 
            age: 0, 
            religion: String::from("")
        }, 
        100, 
        30, 
        10
    );

    let mut turn = 1;

    loop {
        if player.health <= 0 {
            println!("[{}] LOSE", player.name());
            break;
        }
        if enemy.health <= 0 {
            println!("[{}] WIN", player.name());
            break;
        }

        if turn % 2 == 1{
            attack(&player, &mut enemy);
        } else {
            attack(&enemy, &mut player);
        }
        turn += 1;
    }
}
