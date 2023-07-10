#![allow(incomplete_features)]

extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use shader::Shader;

mod shader;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "4D Viewer", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader_vert = Shader::from_file("shaders/base.vert".into(), gl::VERTEX_SHADER).unwrap();
    let shader_frag = Shader::from_file("shaders/base.frag".into(), gl::FRAGMENT_SHADER).unwrap();

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}