#![allow(incomplete_features)]

extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use mesh::Mesh;
use shader::{Shader, Program};

mod shader;
mod mesh;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "4D Viewer", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let program = Program::from_files_auto("shaders/base").unwrap();
    let mut mesh = Mesh::new();
    mesh.bind_vertex_attribs([
        (0, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<f32>() as i32, 0 as *const _),
        (1, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<f32>() as i32, (3 * std::mem::size_of::<f32>()) as *const _)
    ]);

    let vertices: Vec<(f32, f32, f32, f32, f32, f32)> = vec![
        (-0.5, -0.5, 0.0, 1.0, 0.0, 0.0),
        (0.0, 0.5, 0.0, 0.0, 1.0, 0.0),
        (0.5, -0.5, 0.0, 0.0, 0.0, 1.0),
    ];
    mesh.set_vertex_data(&vertices, gl::STATIC_DRAW);
    mesh.set_indices_u32_tuples(&[(0, 1, 2)], gl::STATIC_DRAW);

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

        program.use_program();
        mesh.bind();
        mesh.draw();
    }
}