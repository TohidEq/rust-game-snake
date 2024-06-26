use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::KeyCode,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear},
    QueueableCommand,
};
use std::io::{Stdout, Write};
use std::{thread, time};

use crossterm::event::{poll, read, Event};
use crossterm::style::Stylize;
use rand::Rng;

// low number = more speed⊕
const GAME_SPEED: u64 = 200;
const GOLD_MAX: u16 = 4;

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Location {
    x: u16,
    y: u16,
}
struct Snake {
    direction: Direction,
    locations: Vec<Location>,
    grow: bool,
}
struct Gold {
    exist: bool,
    location: Location,
}

struct World {
    snake: Snake,
    maxX: u16,
    maxY: u16,
    play: bool,
    golds: Vec<Gold>,
}

fn draw_f(
    x: u16,
    y: u16,
    text: crossterm::style::StyledContent<&str>,
    mut sc: &mut Stdout,
    world: &mut World,
) {
    sc.queue(MoveTo(x * 2, y))
        .unwrap()
        .queue(Print(text))
        .unwrap()
        .queue(MoveTo(x * 2 + 1, y))
        .unwrap()
        .queue(Print(text))
        .unwrap();
}

fn draw(mut sc: &mut Stdout, world: &mut World) {
    sc.queue(Clear(crossterm::terminal::ClearType::All));

    // draw golds
    for i in 0..world.golds.len() {
        draw_f(
            world.golds[i].location.x,
            world.golds[i].location.y,
            "⊕".green().on_green(),
            sc,
            world,
        );
        if world.golds[i].location.x == world.snake.locations[0].x
            && world.golds[i].location.y == world.snake.locations[0].y
        {
            world.golds[i].exist = false;
            world.snake.grow = true;
        }
    }

    // draw snake head body
    for i in 1..(world.snake.locations.len()) {
        let mut text = " ".black().on_red();
        if i % 2 != 0 {
            text = " ".red().on_black();
        }

        draw_f(
            world.snake.locations[i].x,
            world.snake.locations[i].y,
            text,
            sc,
            world,
        );
    }

    // draw snake head
    draw_f(
        world.snake.locations[0].x,
        world.snake.locations[0].y,
        "O".red().on_red(),
        sc,
        world,
    );

    sc.flush().unwrap();
}

fn pysics(world: &mut World) {
    let mut rng = rand::thread_rng();

    // add snake size
    if world.snake.grow {
        world.snake.grow = false;
        world.snake.locations.push(Location { x: 0, y: 0 });
    }

    // move snake (body)
    for i in (1..world.snake.locations.len()).rev() {
        world.snake.locations[i].x = world.snake.locations[i - 1].x;
        world.snake.locations[i].y = world.snake.locations[i - 1].y;
    }

    // move snake (head)
    match world.snake.direction {
        Direction::Left => {
            if world.snake.locations[0].x != 0 {
                world.snake.locations[0].x -= 1;
            } else {
                world.snake.locations[0].x = world.maxX;
            }
        }
        Direction::Right => {
            if world.snake.locations[0].x != world.maxX {
                world.snake.locations[0].x += 1;
            } else {
                world.snake.locations[0].x = 0;
            }
        }
        Direction::Up => {
            if world.snake.locations[0].y != 0 {
                world.snake.locations[0].y -= 1;
            } else {
                world.snake.locations[0].y = world.maxY;
            }
        }
        Direction::Down => {
            if world.snake.locations[0].y != world.maxY {
                world.snake.locations[0].y += 1;
            } else {
                world.snake.locations[0].y = 0;
            }
        }
    }

    // gold init
    if world.golds.len() < GOLD_MAX as usize {
        let new_x = rng.gen_range(0..world.maxX);
        let new_y = rng.gen_range(0..world.maxY);
        world.golds.push(Gold {
            exist: true,
            location: Location { x: new_x, y: new_y },
        });
    }

    for i in 0..world.golds.len() {
        if world.golds[i].exist == false {
            let new_x = rng.gen_range(0..world.maxX);
            let new_y = rng.gen_range(0..world.maxY);
            world.golds[i] = Gold {
                exist: true,
                location: Location { x: new_x, y: new_y },
            }
        }
    }
}

fn main() {
    // init screen
    let mut sc: Stdout = std::io::stdout();
    let (maxX_fake, maxY_fake) = size().unwrap();
    let maxX = maxX_fake / 2 - 1;
    let maxY = maxY_fake - 2;
    enable_raw_mode().unwrap();
    crossterm::ExecutableCommand::execute(&mut sc, Hide).unwrap();

    let mut world = World {
        snake: Snake {
            direction: (Direction::Left),
            grow: false,
            locations: vec![
                Location {
                    x: maxX / 2,
                    y: maxY / 2 - 1,
                },
                Location {
                    x: maxX / 2,
                    y: maxY / 2,
                },
                Location {
                    x: maxX / 2 + 1,
                    y: maxY / 2,
                },
                Location {
                    x: maxX / 2 + 2,
                    y: maxY / 2,
                },
                Location {
                    x: maxX / 2 + 3,
                    y: maxY / 2,
                },
                Location {
                    x: maxX / 2 + 3,
                    y: maxY / 2 + 1,
                },
            ],
        },
        maxX: maxX,
        maxY: maxY,
        golds: Vec::new(),
        play: true,
    };

    while world.play {
        // read and apply keyboard

        // `poll()` waits for an `Event` for a given time period
        if poll(time::Duration::from_millis(10)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            let key = read().unwrap();

            // clear the buffer
            while poll(time::Duration::from_millis(10)).unwrap() {
                let _ = read();
            }

            match key {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => {
                        world.play = false;
                    }

                    KeyCode::Char('w') => {
                        if world.snake.direction != Direction::Down {
                            world.snake.direction = Direction::Up
                        }
                    }
                    KeyCode::Char('a') => {
                        if world.snake.direction != Direction::Right {
                            world.snake.direction = Direction::Left
                        }
                    }
                    KeyCode::Char('s') => {
                        if world.snake.direction != Direction::Up {
                            world.snake.direction = Direction::Down
                        }
                    }
                    KeyCode::Char('d') => {
                        if world.snake.direction != Direction::Left {
                            world.snake.direction = Direction::Right
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }

        // draw
        draw(&mut sc, &mut world);

        // pysics
        pysics(&mut world);

        let millis = time::Duration::from_millis(GAME_SPEED);
        thread::sleep(millis);
    }

    disable_raw_mode().unwrap();
    crossterm::ExecutableCommand::execute(&mut sc, Show).unwrap();
}
