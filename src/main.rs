#![allow(incomplete_features)]

extern crate glfw;
extern crate gl;

use std::collections::HashMap;

use glfw::{Action, Context, Key, WindowEvent, Window};
use mesh::Mesh;
use objects::{GpuObjectsHandle, Object};
use shader_loader::{program::Program, preprocessor::FileLoader};
use transform4d::{Vec4, Vec6};
use util::OpenglBuffer;

mod mesh;
mod transform4d;
mod objects;
mod util;

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
    let mut last_frame = current_time();
    while !window.should_close() && !app.should_exit() {
        let cur_frame = current_time();
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

        app.update(cur_frame - last_frame);

        let start = current_time();
        app.render(&mut window);
        unsafe { gl::Finish(); }
        println!("Render took: {} ms", (current_time() - start) * 1000.0);
        
        last_frame = cur_frame;
    }
}

struct App {
    program: Program,
    mesh: Mesh,
    _objects: GpuObjectsHandle,

    start_time_secs: f64,
    camera_buf: OpenglBuffer,
    camera_pos: Vec4,
    camera_rot: Vec6,

    keys_pressed: HashMap<glfw::Key, bool>,
}

fn load_program(vert: &str, frag: &str) -> Program {
    match Program::from_loader(&FileLoader::new(), &[
        (frag, gl::FRAGMENT_SHADER),
        (vert, gl::VERTEX_SHADER)
    ]) {
        Ok(program) => program,
        Err(error) => {
            panic!("\n{}", error);
        }
    }
}

impl App {
    pub fn new() -> Self {
        let obj_handle = GpuObjectsHandle::new();
        obj_handle.bind_buffer_base(0);
        
        let program = load_program("shaders/base.vert", "shaders/base.frag");

        App {
            program,
            mesh: square_mesh(),
            _objects: obj_handle,

            start_time_secs: current_time(),
            camera_buf: OpenglBuffer::new(gl::SHADER_STORAGE_BUFFER),
            camera_pos: nalgebra::vector![0.0, 0.0, 0.0, 0.0],
            camera_rot: nalgebra::vector![0.0, 0.0, 0.0, 0.0, 0.0, 0.0],

            keys_pressed: HashMap::new(),
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.is_key_pressed(Key::Space) {
            self.camera_pos[2] += 1.0 * dt;
        }
        if self.is_key_pressed(Key::LeftShift) {
            self.camera_pos[2] -= 1.0 * dt;
        }

        if self.is_key_pressed(Key::W) {
            self.camera_pos[0] += 1.0 * dt;
        }
        if self.is_key_pressed(Key::S) {
            self.camera_pos[0] -= 1.0 * dt;
        }
        
        if self.is_key_pressed(Key::D) {
            self.camera_pos[1] += 1.0 * dt;
        }
        if self.is_key_pressed(Key::A) {
            self.camera_pos[1] -= 1.0 * dt;
        }

        if self.is_key_pressed(Key::R) {
            self.camera_pos[3] += 1.0 * dt;
        }
        if self.is_key_pressed(Key::F) {
            self.camera_pos[3] -= 1.0 * dt;
        }

        let camera_matrix = transform4d::full_transform(
            self.camera_pos, 
            nalgebra::vector![1.0, 1.0, 1.0, 1.0], 
            self.camera_rot
        );
        let camera_arr = transform4d::matrix_to_array(camera_matrix);

        self.camera_buf.write_data(&camera_arr, std::mem::size_of::<[f32;25]>(), gl::STATIC_READ);
        self.camera_buf.bind_buffer_base(1);
        //println!("Camera pos: {:?}", self.camera_pos);

        
        let objects = default_objects();
        self._objects.write_objects(&objects);
        self.program.uniform("u_objects_count", objects.len() as u32);
        // Nothing here yet
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        match self.keys_pressed.get(&key) {
            Some(true) => true,
            _ => false
        }
    }

    pub fn render(&mut self, window: &mut Window) {
        let win_size = window.get_size();
        let aspect_ratio = win_size.0 as f32 / win_size.1 as f32;
        self.program.uniform("u_aspect_ratio", aspect_ratio);
        self.program.uniform("u_time", self.time_elapsed() as f32);

        self.program.use_program();
        self.mesh.bind();

        unsafe {
            gl::ClearColor(0.1, 0.2, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.program.uniform("u_object_id", 0i32);
        self.mesh.draw();
        
        self.program.uniform("u_object_id", 1i32);
        self.mesh.draw();
    }

    pub fn handle_input(&mut self, event: WindowEvent, window: &mut Window) {
        println!("{:?}", event);
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            },

            glfw::WindowEvent::Key(key, _, Action::Press, _) => {
                self.keys_pressed.insert(key, true);
            } 
            
            glfw::WindowEvent::Key(key, _, Action::Release, _) => {
                self.keys_pressed.insert(key, false);
            }
            
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
    let time = current_time() / 2.0;

    vec![
        Object {
            obj_type: objects::ObjectType::Cube,
            transform: transform4d::shift(nalgebra::vector![2.0, -1.0, 0.0, 0.0]) *
                transform4d::rotation_full(nalgebra::vector![1.0, 1.0, 1.0, 1.0, 0.0, 0.0] * time),
        },
        
        Object {
            obj_type: objects::ObjectType::CubeFrame,
            transform: transform4d::shift(nalgebra::vector![2.0, 1.0, 0.0, 0.0]) *
                transform4d::rotation_full(nalgebra::vector![1.0, 1.0, 1.0, 1.0, 0.0, 0.0] * time),
        },
        
        /*Object {
            obj_type: objects::ObjectType::Sphere,
            transform: transform4d::shift(nalgebra::vector![1.0, 1.0, 2.0, 0.0]),
        },*/
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
