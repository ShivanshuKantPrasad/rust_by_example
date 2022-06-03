#[derive(Debug)]
struct Player {
    name: String,
    health: i32,
    ammo: i32
}

impl Player {
    fn kill(&mut self){
        self.health = 0;
    }
    fn add_title(&mut self, title: &String){
        self.name.push_str(title);
    }
}

fn reload_player (player: &mut Player) {
    player.ammo = 10;
}

fn main() {
    let mut player = Player{
        name: "Shivanshu".to_string(),
        health: 100,
        ammo: 0
    };
    println!("{:?}", player);

    player.add_title(&" 1337 H4X0R".to_string());
    player.kill();
    reload_player(&mut player);
    println!("{:?}", player);
    
    
}
