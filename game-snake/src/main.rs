use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ScrollUp, SetSize},
    ExecutableCommand, QueueableCommand,
};
use std::{io, thread, time};
use std::{
    io::{stdout, Result, Stdout, Write},
    os::linux::raw::stat,
};

use crossterm::event::{poll, read, Event};

use rand::Rng;

// low number = more speed
const GAME_SPEED: u64 = 200;

struct Location {
    x: u16,
    y: u16,
}
#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
struct Snake {
    direction: Direction,
    locations: Vec<Location>,
}

struct World {
    snake: Snake,
    maxX: u16,
    maxY: u16,
    play: bool,
}

fn draw(mut sc: &mut Stdout, world: &mut World) {
    sc.queue(Clear(crossterm::terminal::ClearType::All));

    // draw snake head body
    for i in 1..(world.snake.locations.len()) {
        sc.queue(MoveTo(
            world.snake.locations[i].x,
            world.snake.locations[i].y,
        ))
        .unwrap()
        .queue(Print("#"))
        .unwrap();
    }

    // draw snake head
    sc.queue(MoveTo(
        world.snake.locations[0].x,
        world.snake.locations[0].y,
    ))
    .unwrap()
    .queue(Print("O"))
    .unwrap();

    sc.flush().unwrap();
}

fn pysics(world: &mut World) {
    let newLocations: Vec<Location> = vec![];

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
}

fn main() {
    // init screen
    let mut sc: Stdout = stdout();
    let (maxX, maxY) = size().unwrap();

    enable_raw_mode().unwrap();
    sc.execute(Hide).unwrap();

    let mut world = World {
        snake: Snake {
            direction: (Direction::Left),
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
    sc.execute(Show).unwrap();
}
