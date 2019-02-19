extern crate quicksilver;
extern crate rand;

use quicksilver::{
    geom::{Vector, Shape, Circle, Rectangle, Transform},
    graphics::{Color, Font, FontStyle, Image, Background},
    lifecycle::{run, Asset, Settings, State, Window, Event},
    input::{ButtonState, Key},
    Future, Result,
};

use rand::{RngCore, thread_rng};

use std::cmp::{max, min};

fn main() {
    let settings = Settings {
        ..Default::default()
    };
    run::<Game>("Ping Pong", Vector::new(MAX_X, MAX_Y), settings);
}

struct Game {
    ball: (i32, i32),
    velocity: (i32, i32),
    score: (usize, usize),
    racket: (i32, i32),
    digits: Vec<Asset<Image>>
}

const MAX_X :i32 = 800;
const MAX_Y :i32 = 600;
const R: i32 = 10;
const RACKET_X: i32 = 20;
const RACKET_Y: i32 = 100;
const WIN_X: i32 = 20;
const ROCKET_SPEED:i32 = 40;
const FONT: &str = "mononoki-Regular.ttf";

impl Game {
    fn serve (side: u32, pos: i32) -> ((i32, i32), (i32, i32)){
        let mut yv = thread_rng().next_u32() as i32 % 5 + 1;
        if thread_rng().next_u32() % 2 == 0 {
            yv = -yv;
        }

        if side > 0 {
            ((MAX_X - WIN_X - RACKET_X - 1, pos), (-5, yv))
        } else {
            ((WIN_X + RACKET_X + R + 1, pos), (5, yv))
        }
    }
}

impl State for Game {
    /// Load the assets and initialise the game
    fn new() -> Result<Self> {
        let mut digits = Vec::with_capacity(10);

        for digit in 0 .. 10 {
            digits.push(Asset::new(Font::load(FONT).and_then(move |font| {
                font.render(format!("{}", digit).as_str(), &FontStyle::new(72.0, Color::RED))
            })));
        }

        let side = thread_rng().next_u32 ()  % 2;
        let (start, velocity) = Self::serve(side, MAX_Y/2);

        Ok(Game{ball: start, velocity, score: (0,0), racket: (MAX_Y/2 - RACKET_Y /2, MAX_Y/2 - RACKET_Y /2),
            digits})
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        if self.ball.0 <= WIN_X + RACKET_X + R && self.ball.1 >= self.racket.0 && self.ball.1 <= self.racket.0 + RACKET_Y {
            self.velocity.0 = i32::abs(self.velocity.0);
            self.velocity.1 += thread_rng().next_u32() as i32 % 2  - 1;
        }

        if self.ball.0 + R >= MAX_X - WIN_X - RACKET_X && self.ball.1 >= self.racket.1 && self.ball.1 <= self.racket.1 + RACKET_Y {
            self.velocity.0 = -i32::abs(self.velocity.0);
            self.velocity.1 += thread_rng().next_u32() as i32 % 2  - 1;
        }

        if self.ball.0 + R > MAX_X || self.ball.0 < R {
            self.velocity.0 = -self.velocity.0;
        }
        if self.ball.1 + R > MAX_Y || self.ball.1 < R {
            self.velocity.1 = -self.velocity.1;
        }

        self.ball.0 += self.velocity.0;
        self.ball.1 += self.velocity.1;

        if self.ball.0 <= R {
            self.score.1 += 1;
            let (start, velocity) = Self::serve(0, self.racket.0 + RACKET_Y/2);
            self.ball = start;
            self.velocity = velocity;
        }
        if self.ball.0 + R >= MAX_X {
            self.score.0 += 1;
            let (start, velocity) = Self::serve(1, self.racket.1 + RACKET_Y/2);
            self.ball = start;
            self.velocity = velocity;
        }

        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Key(Key::P, ButtonState::Pressed)
                =>  self.racket.1 = max(0, self.racket.1 - ROCKET_SPEED),
            Event::Key(Key::L, ButtonState::Pressed)
                =>  self.racket.1 = min(MAX_Y - RACKET_Y, self.racket.1 + ROCKET_SPEED),
            Event::Key(Key::Q, ButtonState::Pressed)
                =>  self.racket.0 = max(0, self.racket.0 - ROCKET_SPEED),
            Event::Key(Key::A, ButtonState::Pressed)
                =>  self.racket.0 = min(MAX_Y - RACKET_Y, self.racket.0 + ROCKET_SPEED),
            _ => {}
        }
        Ok(())
    }

    /// Draw stuff on the screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        self.digits[self.score.0 % 10].execute(|image| {
            window.draw(
                &image
                    .area()
                    .translate((MAX_X/2 - 30, MAX_Y - 60)),
                Background::Img(&image),
            );
            Ok(())
        })?;

        self.digits[self.score.1 % 10].execute(|image| {
            window.draw(
                &image
                    .area()
                    .translate((MAX_X/2 + 30, MAX_Y - 60)),
                Background::Img(&image),
            );
            Ok(())
        })?;


        window.draw_ex(&Circle::new((0, 0), R), Background::Col(Color::YELLOW),
            Transform::translate(self.ball.clone()), 0);

        window.draw_ex(&Rectangle::new((0,0), (RACKET_X, RACKET_Y)), Background::Col(Color::WHITE),
                       Transform::translate((WIN_X, self.racket.0)), 0);

        window.draw_ex(&Rectangle::new((0,0), (RACKET_X, RACKET_Y)), Background::Col(Color::WHITE),
                       Transform::translate((MAX_X - WIN_X - RACKET_X, self.racket.1)), 0);


        Ok(())
    }
}
