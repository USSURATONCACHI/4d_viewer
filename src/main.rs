#![allow(incomplete_features)]

extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use mesh::Mesh;
use shader::{Program};

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
        (0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null() as *const _)
    ]);

    let vertices: Vec<(f32, f32, f32)> = vec![
        (-0.5, -0.5, 0.0),
        (0.0, 0.5, 0.0),
        (0.5, -0.5, 0.0),
    ];
    mesh.set_vertex_data(&vertices, gl::STATIC_DRAW);

    //let indices = vec![(0, 1, 2)];
    //mesh.set_indices(&indices, 3, gl::STATIC_DRAW, gl::UNSIGNED_INT);

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
        unsafe {
            //gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo());
            //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo());       
            gl::DrawArrays(gl::TRIANGLES, 0, 3); 
        }
        //mesh.draw();
    }
}