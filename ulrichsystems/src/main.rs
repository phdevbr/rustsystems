use rand::Rng;
use std::io::Write;

trait Entity<T> {
    fn new() -> Self;
    fn attack(&self, ammount: u8, attacked: &mut T);
    fn gethealth(&self) -> u8;
}

#[derive(Debug)]
struct Player {
    health: u8,
}

#[derive(Debug)]
struct Enemy {
    health: u8,
}

impl Entity<Enemy> for Player {
    fn new() -> Self {
        Self { health: 100 }
    }
    fn attack(&self, ammount: u8, attacked: &mut Enemy) {
        attacked.health -= ammount;
    }
    fn gethealth(&self) -> u8 {
        self.health
    }
}

impl Entity<Player> for Enemy {
    fn new() -> Self {
        Self { health: 100 }
    }
    fn attack(&self, ammount: u8, attacked: &mut Player) {
        attacked.health -= ammount;
    }
    fn gethealth(&self) -> u8 {
        self.health
    }
}

fn main() {
    let mut player = Player::new();
    let mut enemy = Enemy::new();
    loop {
        print!(">");
        let mut line: String = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        std::io::stdout().flush().unwrap();
        if line == "attack\n" {
            let mut rng = rand::thread_rng();
            let random = rng.gen::<u8>() / 100;
            if random > 0 {
                enemy.attack((random / 12) * 5, &mut player);
            }
            player.attack(&mut enemy);
        } else if line.starts_with("health") {
            if line == "health\n" || line == "health me\n" {
                println!("{}", player.health);
            } else if line == "health enemy\n" {
                println!("{}", enemy.health);
            }
        } else if line == "exit\n" {
            println!("Process exited sucessfully");
            return;
        }
        enemy.attack(&mut player);
    }
}
