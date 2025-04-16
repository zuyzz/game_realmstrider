use realmstrider::game::combat::{Entity, Health, Modifier};

fn main() {
    let mut player = Entity::new("David");
    player.health.set(100);
    player.strength.set(30);
    player.magic.set(40);
    player.health.add_event_listener(Box::new(|curr_val| -> () {
        println!(" ___________ Player HP : {}", curr_val)
    }));
    player.health.add_event_listener(Box::new(|curr_val| -> () {
        if curr_val == 0 {
            println!("PLAYER LOSE");
        }
    }));

    let mut enemy = Entity::new("Goblin");
    enemy.health.set(100);
    enemy.strength.set(30);
    enemy.magic.set(40);
    enemy.health.add_event_listener(Box::new(|curr_val| -> () {
        println!(" ___________ Enemy HP : {}", curr_val)
    }));
    enemy.health.add_event_listener(Box::new(|curr_val| -> () {
        if curr_val == 0 {
            println!("PLAYER WIN ");
        }
    }));

    let mut turn = 1;
    let mut end: bool = false;

    while !end {
        let player_dmg = Modifier::<Health>::init(20);
        let enemy_dmg = Modifier::<Health>::init(10);

        if turn % 2 == 1 {
            player.combat(&mut enemy, player_dmg);
            end = enemy.health.get() <= 0;
        } else {
            enemy.combat(&mut player, enemy_dmg);
            end = player.health.get() <= 0;
        }
        turn += 1;
    }
}
