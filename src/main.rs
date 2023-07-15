#![allow(incomplete_features)]

extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key, WindowEvent, Window};
use mesh::Mesh;
use objects::{GpuObjectsHandle, Object};
use shader::Program;

mod shader;
mod mesh;
mod transform4d;
mod objects;

fn main() {
    // ====== GLFW, OpenGL and window initialization
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "4D Viewer", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // ====== Program initialization
    let mut app = App::new();
    let mut last_size = window.get_size();

    // ====== Main loop
    while !window.should_close() && !app.should_exit() {
        window.swap_buffers();

        let cur_window_size = window.get_size();
        if cur_window_size != last_size {
            last_size = cur_window_size;
            
            unsafe {
                gl::Viewport(0, 0, cur_window_size.0, cur_window_size.1);
            }
        }

        glfw.poll_events();
        glfw::flush_messages(&events)
            .for_each(|(_, event)| app.handle_input(event, &mut window));

        app.update(0.0);
        app.render(&mut window);
    }
}

struct App {
    program: Program,
    mesh: Mesh,
    _objects: GpuObjectsHandle,

    start_time_secs: f64,
}

impl App {
    pub fn new() -> Self {
        let mut obj_handle = GpuObjectsHandle::new();
        let objects = default_objects();
        obj_handle.write_objects(&objects);
        obj_handle.bind_buffer_base(0);
        
        let program = Program::from_files_auto("shaders/base").unwrap();
        program.uniform("u_objects_count", objects.len() as u32);

        App {
            program,
            mesh: square_mesh(),
            _objects: obj_handle,

            start_time_secs: current_time(),
        }
    }

    pub fn update(&mut self, _dt: f64) {
        // Nothing here yet
    }

    pub fn render(&mut self, window: &mut Window) {
        let win_size = window.get_size();
        let aspect_ratio = win_size.0 as f32 / win_size.1 as f32;
        self.program.uniform("u_aspect_ratio", aspect_ratio);
        self.program.uniform("u_time", self.time_elapsed() as f32);

        self.program.use_program();
        self.mesh.bind();
        self.mesh.draw();
    }

    pub fn handle_input(&mut self, event: WindowEvent, window: &mut Window) {
        println!("{:?}", event);
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            },
            _ => {},
        }
    }

    pub fn should_exit(&self) -> bool {
        false
    }

    pub fn time_elapsed(&self) -> f64 {
        current_time() - self.start_time_secs
    }
}


fn current_time() -> f64 {
    (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as f64) / 1000.0
}

fn default_objects() -> Vec<Object> {
    vec![
        Object {
            obj_type: objects::ObjectType::Cube,
            transform: transform4d::shift(nalgebra::vector![1.0, 1.0, 1.0, 0.0]),
        },
        
        Object {
            obj_type: objects::ObjectType::Sphere,
            transform: transform4d::shift(nalgebra::vector![1.0, 1.0, 2.0, 0.0]),
        },
    ]
}

/*

fn default_objects() -> Vec<Object> {
    vec![
        Object {
            obj_type: Cube,
            transform: transform4d::full_transform(
                nalgebra::vector![0.0, 0.0, 0.0, 0.0],
                nalgebra::vector![2.0, 1.0, 1.0, 1.0],
                nalgebra::vector![30.0 * std::f64::consts::PI / 180.0, 0.0, 0.0, 0.0, 0.0, 0.0]
            )
        },

        Object {
            obj_type: Sphere,
            transform: transform4d::shift(nalgebra::vector![0.0, 0.0, 1.0, 0.0])
        }
    ]
}
*/

fn square_mesh() -> Mesh {
    let mut mesh = Mesh::new();

    mesh.bind_consecutive_attribs(0, &[
        (3, std::mem::size_of::<f32>(), gl::FLOAT),
    ]);

    let vertices: Vec<(f32, f32, f32)> = vec![
        (-1.0, -1.0, 0.0),
        (1.0, -1.0, 0.0),
        (1.0, 1.0, 0.0),
        (-1.0, 1.0, 0.0),
    ];
    let indices = [(0, 1, 2), (0, 3, 2)];

    mesh.set_vertex_data(&vertices, gl::STATIC_DRAW);
    mesh.set_indices_u32_tuples(&indices, gl::STATIC_DRAW);

    mesh
}

#[allow(dead_code)]
fn triangle_mesh() -> Mesh {
    let mut mesh = Mesh::new();
    
    let float_size = std::mem::size_of::<f32>();
    mesh.bind_consecutive_attribs(0, &[
        (3, float_size, gl::FLOAT),
        (3, float_size, gl::FLOAT),
    ]);

    let vertices: Vec<(f32, f32, f32, f32, f32, f32)> = vec![
        (-0.5, -0.5, 0.0, 1.0, 0.0, 0.0),
        (0.0, 0.5, 0.0, 0.0, 1.0, 0.0),
        (0.5, -0.5, 0.0, 0.0, 0.0, 1.0),
    ];
    mesh.set_vertex_data(&vertices, gl::STATIC_DRAW);
    mesh.set_indices_u32_tuples(&[(0, 1, 2)], gl::STATIC_DRAW);

    mesh
}
