#![allow(incomplete_features)]
#![feature(generic_const_exprs, inherent_associated_types)]

extern crate raylib;

use raylib::{prelude::{RaylibDraw, Color, RaylibShaderModeExt, Rectangle, Vector2, TraceLogLevel}, shaders::RaylibShader};
use transform4d::Mat5;

use crate::transform4d::Vec4;

mod transform4d;
mod objects;

use objects::ObjectsManager;

fn main() {
    //let c = std::f64::consts::PI / 180.0;
    //let angles = [90.0 * c, 90.0 * c, 90.0 * c, 90.0 * c, 90.0 * c, 90.0 * c];
    
    //let pos = nalgebra::vector![1.0, 0.0, 0.0, 0.0, 1.0];
    //println!("{}", transform * pos);

    raylib::core::logging::set_trace_log(TraceLogLevel::LOG_DEBUG);

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .resizable()
        .build();

    rl.set_target_fps(60);
    

    let origin = Vector2 { x: 0.0, y: 0.0 };
    let mut shader = rl.load_shader(&thread, Some("shaders/main.vert"), Some("shaders/main.frag")).unwrap();
    let time_loc = shader.get_shader_location("u_time");
    let screen_size_loc = shader.get_shader_location("u_screen_size");

    
    //gl::load_with(|name| rl.get_proc_address(s) as *const _);
    //let objects_manages = ObjectsManager::new();
    //let objects = default_objects();

    while !rl.window_should_close() {
        let width = rl.get_screen_width() as f32;
        let height = rl.get_screen_height() as f32;
        let rectangle = Rectangle { x: 0.0, y: 0.0, width, height };
        

        let time = rl.get_time();
        shader.set_shader_value(time_loc, time as f32);
        shader.set_shader_value(screen_size_loc, Vector2::new(width, height));

        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        let mut s = d.begin_shader_mode(&shader);
        s.draw_rectangle_pro(rectangle, origin, 0.0, Color::YELLOW);
    }
}
