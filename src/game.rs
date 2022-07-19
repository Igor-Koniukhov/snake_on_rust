use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

use rand::{thread_rng, Rng};
use crate::draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color=[0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

pub struct Game{
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32
}

impl Game {
    pub  fn new(width: i32, height: i32)->Game{
        Game{
            food_exists: false,
            food_x: 0,
            food_y: 0,
            width,
            height
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d){
        if self.food_exists{
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height-1, self.width, 1,con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width-1, 0, 1, self.height, con, g);
    }
    pub fn update(&mut self, delta_time: f64){
        if !self.food_exists{
            self.add_food();
        }
    }
    fn add_food(&mut self){
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1, self.width-1);
        let mut new_y = rng.gen_range(1, self.height-1);

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

}