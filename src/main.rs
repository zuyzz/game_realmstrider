struct Entity {
    pub name: &'static str,
    pub health: i32,
    pub strength: i32,
}

fn attack(source: &Entity, target: &mut Entity) {
    let pre_health = target.health;
    target.health -= source.strength;
    println!("{} deal {} damage to {} {}->{}", 
    source.name, 
    source.strength, 
    target.name,
    pre_health,
    target.health);
}

fn main() {
    let mut player: Entity = Entity { name: "David", health: 100, strength: 40 };
    let mut enemy: Entity = Entity { name: "Goblin", health: 100, strength: 30 };

    let mut turn = 1;

    loop {
        if player.health <= 0 {
            println!("You lose");
            break;
        }
        if enemy.health <= 0 {
            println!("You win");
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
