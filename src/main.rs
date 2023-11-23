// https://stackoverflow.com/questions/60120532/detect-keydown
// https://stackoverflow.com/questions/34842526/update-console-without-flickering-c
use colored::Colorize;
use console::Term;
use std::io::stdout;

#[derive(Copy, Clone)]
struct Tile<'a> {
    icon: char,
    color: &'a str,
    visible: bool
}

struct Entity<'a> {
    tile: Tile<'a>,
    health: i8,
    x_pos: usize,
    y_pos: usize
}

impl Entity<'_> {
    fn move_entity(&mut self, x: usize, y: usize) {
        if (x >= 0 && x < 30) {
            self.x_pos = x;
        }
        if (y >= 0 && y < 20) {
            self.y_pos = y;
        }
    }
}

struct Game_Board<'a> {
    board: [[Tile<'a>; 30]; 20],
    
}

fn main() {
    let mut game_board: [[Tile; 30]; 20] = [[Tile {icon: ',', color: "green", visible: true}; 30]; 20];

    let mut player = Entity {
        tile: Tile {icon: '@', color: "white", visible: true},
        health: 10,
        x_pos: 7,
        y_pos: 7
    };

    game_board[player.x_pos][player.y_pos] = player.tile;
    
    for a in game_board {
        println!();
        for b in a {
            print!(" {} ", String::from(b.icon).color(b.color));
        }
    }

    let stdout = Term::buffered_stdout();
    'game_loop: loop {
        if let Ok(character) = stdout.read_char() {
            game_board[player.y_pos][player.x_pos] = Tile {icon: ',', color: "green", visible: true};
            match character {
                'w' => player.move_entity(player.x_pos, (player.y_pos - 1)),
                'a' => player.move_entity((player.x_pos - 1), player.y_pos),
                's' => player.move_entity(player.x_pos, (player.y_pos + 1)),
                'd' => player.move_entity((player.x_pos + 1), player.y_pos),
                _ => break 'game_loop,
            }
            game_board[player.y_pos][player.x_pos] = player.tile;

            std::process::Command::new("clear").status().unwrap();
            for x in game_board {
                println!();
                for y in x {
                    print!(" {} ", String::from(y.icon).color(y.color));
                }
            }
            println!("\n-= PLAYER DATA   Health: {}\t Gold: TODO\t =-", player.health);
        }
    }
}