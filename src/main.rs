#![allow(incomplete_features)]

extern crate glfw;
extern crate gl;
extern crate lazy_static;
extern crate nalgebra;
extern crate shader_loader;

mod mesh;
mod transform4d;
mod util;

use std::collections::HashMap;

use gl::types::GLenum;
use glfw::{Action, Context, Key, WindowEvent, Window, MouseButton, CursorMode};
use lazy_static::__Deref;

use mesh::Mesh;
use nalgebra::Vector3;
use shader_loader::{program::Program, preprocessor::FileLoader};
use transform4d::{Vec4, Vec6, Mat5};
use util::{current_time, uniform_mat5};

lazy_static::lazy_static! {
    /// Transforms opengl axes (+x = right, +y = up, -z = forward)
    /// to custom camera axes (+x = forward, +y = right, +z = up)
    static ref OPENGL_TO_CAMERA: Mat5 = nalgebra::matrix![
        0.0, 1.0, 0.0, 0.0, 0.0;
        0.0, 0.0, 1.0, 0.0, 0.0;
        -1.0, 0.0, 0.0, 0.0, 0.0;
        0.0, 0.0, 0.0, 1.0, 0.0;
        0.0, 0.0, 0.0, 0.0, 1.0;
    ];
}

fn main() {
    // ====== GLFW, OpenGL and window initialization
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "4D Viewer", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
    }

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
        //println!("Render took: {} ms", (current_time() - start) * 1000.0);
        
        last_frame = cur_frame;
    }
}

struct App {
    program: Program,
    meshes: Vec<(Mesh, Box<dyn Fn(f64) -> Mat5>)>,
    meshes_wireframe: Vec<(Mesh, Box<dyn Fn(f64) -> Mat5>)>,

    start_time_secs: f64,
    camera_pos: Vec4,
    camera_rot: Vec6,
    last_mouse_pos: Option<(f64, f64)>,

    keys_pressed: HashMap<glfw::Key, bool>,
    w_perspective: bool,
}

fn load_program(files: &[(&str, GLenum)]) -> Program {
    match Program::from_loader(&FileLoader::new(), files) {
        Ok(program) => program,
        Err(error) => {
            panic!("\n{}", error);
        }
    }
}

impl App {
    pub fn new() -> Self {
        let program = load_program(&[
            ("shaders/clipping.vert", gl::VERTEX_SHADER),
            ("shaders/clipping.frag", gl::FRAGMENT_SHADER)
        ]);

        let pi = std::f64::consts::PI;
        App {
            program,
            meshes: vec![
                (
                    cube_mesh(), Box::new(
                        |time|  transform4d::shift(nalgebra::vector![0.0, 0.0, -1.0, 0.0]) *
                        transform4d::rotation_full(nalgebra::vector![time, 0.0, time, 0.0, 0.0, 0.0])
                    )
                ),
                (
                    triangle_mesh_4d(), Box::new(
                        move |time|  transform4d::shift(nalgebra::vector![1.0, 0.0, 0.0, 0.0]) *
                        transform4d::rotation_full(nalgebra::vector![time.sin(), 90.0 * pi / 180.0, 0.0, time, 0.0, 0.0])
                    )
                ),
                (
                    triangle_mesh_4d(), Box::new(
                        move |time|  transform4d::shift(nalgebra::vector![0.0, 1.0, 0.0, 0.0]) *
                        transform4d::rotation_full(nalgebra::vector![time, 0.0, 0.0, 90.0 * pi / 180.0, 0.0, 0.0])
                    )
                ),
            ],
            meshes_wireframe: vec![
                (
                    cube_mesh_4d(),
                    Box::new(|time| transform4d::shift(nalgebra::vector![3.0, 0.0, 0.0, 0.0]) *
                        transform4d::rotation_full(nalgebra::vector![0.0, 0.0, time, 0.0, 0.0, 0.0]))
                ),
                (
                    cube_mesh_4d(),
                    Box::new(|time| transform4d::shift(nalgebra::vector![6.0, 0.0, 0.0, 0.0]) *
                        transform4d::rotation_full(nalgebra::vector![0.0, 0.0, time, 0.0, time * 1.4142, time * 0.9072]))
                ),
                (
                    cube_mesh(),
                    Box::new(|_time| transform4d::scale(nalgebra::vector![2.0, 2.0, 2.0, 2.0]))
                )
            ],

            start_time_secs: current_time(),
            camera_pos: nalgebra::vector![0.0, 0.0, 0.0, 0.0],
            camera_rot: nalgebra::vector![0.0, 0.0, 0.0, 0.0, 0.0, 0.0],

            keys_pressed: HashMap::new(),
            last_mouse_pos: None,
            w_perspective: false,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let mut offset = nalgebra::vector![0.0, 0.0, 0.0, 0.0, 0.0];

        if self.is_key_pressed(Key::W) { offset[0] += 1.0 * dt; }
        if self.is_key_pressed(Key::S) { offset[0] -= 1.0 * dt; }
        
        if self.is_key_pressed(Key::D) { offset[1] += 1.0 * dt; }
        if self.is_key_pressed(Key::A) { offset[1] -= 1.0 * dt; }
        

        if self.is_key_pressed(Key::R) { offset[3] += 1.0 * dt; }
        if self.is_key_pressed(Key::F) { offset[3] -= 1.0 * dt; }
        
        let mut offset = self.camera_matrix() * offset;

        if self.is_key_pressed(Key::Space)       { offset[2] += 1.0 * dt; }
        if self.is_key_pressed(Key::LeftControl) { offset[2] -= 1.0 * dt; }

        if self.is_key_pressed(Key::LeftShift) { offset *= 5.0; }

        self.camera_pos += nalgebra::vector![offset[0], offset[1], offset[2], offset[3]];
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
        let aspect_ratio_f64 = win_size.0 as f64 / win_size.1 as f64;
        
        self.program.uniform("u_aspect_ratio", aspect_ratio);
        self.program.uniform("u_time", self.time_elapsed() as f32);

        // CAMERA
        let camera: Mat5 = // Quite ugly tbh
            transform4d::perspective_matrix(90.0, aspect_ratio_f64 , 0.01, 100.0, if self.w_perspective { 1.0 } else { 0.0 }) * 
            OPENGL_TO_CAMERA.deref() * 
            self.camera_matrix().try_inverse().unwrap();

        let projection_loc = self.program.location("u_projection.data");
        uniform_mat5(projection_loc, &camera);

        // DRAWING
        unsafe {
            gl::ClearColor(0.1, 0.2, 0.4, 1.0);
            gl::ClearDepth(1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.program.use_program();
        let model_loc = self.program.location("u_model.data");

        for (mesh, pos_fn) in &self.meshes {
            mesh.bind();
            uniform_mat5(model_loc, &pos_fn(self.time_elapsed()));
            mesh.draw();
        }
        
        // Draw unit cube
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
        for (mesh, pos_fn) in &self.meshes_wireframe {
            mesh.bind();
            uniform_mat5(model_loc, &pos_fn(self.time_elapsed()));
            mesh.draw();
        }
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL); }
    }

    pub fn handle_input(&mut self, event: WindowEvent, window: &mut Window) {
        //println!("{:?}", event);
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_cursor_mode(glfw::CursorMode::Normal);
            },

            glfw::WindowEvent::Key(key, _, Action::Press, _) => {
                self.keys_pressed.insert(key, true);

                if key == Key::T {
                    self.w_perspective = !self.w_perspective;
                }
            } 
            
            glfw::WindowEvent::Key(key, _, Action::Release, _) => {
                self.keys_pressed.insert(key, false);
            }

            glfw::WindowEvent::MouseButton(button, action, _) => {
                println!("{:?} {:?}", button, action);
                if button == MouseButton::Button1 && action == Action::Press {
                    window.set_cursor_mode(glfw::CursorMode::Disabled);
                }
            }

            glfw::WindowEvent::CursorPos(x, y) => {
                if self.last_mouse_pos.is_none() {
                    self.last_mouse_pos = Some((x, y));
                }

                let old = self.last_mouse_pos.clone().unwrap();
                self.last_mouse_pos = Some((x, y));

                if window.get_cursor_mode() != CursorMode::Disabled {
                    return;
                }

                let diff = (x - old.0, y - old.1);
                self.camera_rot[0] += diff.0 / 300.0;
                self.camera_rot[1] += -diff.1 / 300.0;
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

    pub fn camera_matrix(&self) -> Mat5 { 
        transform4d::shift(self.camera_pos) *
        transform4d::rotation_full_inv(self.camera_rot)
    }
}

fn default_empty_mesh() -> Mesh {
    let mut mesh = Mesh::new();
    
    let float_size = std::mem::size_of::<f32>();
    mesh.bind_consecutive_attribs(0, &[
        (4, float_size, gl::FLOAT),
        (3, float_size, gl::FLOAT),
    ]);

    mesh
}

#[allow(dead_code)]
fn square_mesh() -> Mesh {
    let mut mesh = default_empty_mesh();

    let vertices: Vec<((f32, f32, f32, f32), (f32, f32, f32))> = vec![
        ((-1.0, -1.0,  0.0,  0.0), (1.0, 0.0, 0.0)),
        (( 1.0, -1.0,  0.0,  0.0), (0.0, 1.0, 0.0)),
        (( 1.0,  1.0,  0.0,  0.0), (0.0, 0.0, 1.0)),
        ((-1.0,  1.0,  0.0,  0.0), (1.0, 1.0, 0.0)),
    ];
    let indices = [(0, 1, 2), (0, 3, 2)];

    mesh.set_vertex_data(&vertices, gl::STATIC_DRAW);
    mesh.set_indices_u32_tuples(&indices, gl::STATIC_DRAW);

    mesh
}


#[allow(dead_code)]
fn triangle_mesh() -> Mesh {
    let mut mesh = default_empty_mesh();

    let vertices: Vec<((f32, f32, f32, f32), (f32, f32, f32))> = vec![
        ((-0.5, -0.5, 0.0, 0.0), (1.0, 0.0, 0.0)),
        (( 0.0,  0.5, 0.0, 0.0), (0.0, 1.0, 0.0)),
        (( 0.5, -0.5, 0.0, 0.0), (0.0, 0.0, 1.0)),
    ];
    mesh.set_vertex_data(&vertices, gl::STATIC_DRAW);
    mesh.set_indices_u32_tuples(&[(0, 1, 2)], gl::STATIC_DRAW);

    mesh
}


#[allow(dead_code)]
fn triangle_mesh_4d() -> Mesh {
    let mut mesh = default_empty_mesh();

    let vertices: Vec<((f32, f32, f32, f32), (f32, f32, f32))> = vec![
        ((-0.5, -0.5, 0.0, 0.0), (1.0, 0.0, 0.0)),
        (( 0.0,  0.5, 0.0, 0.0), (0.0, 1.0, 0.0)),
        (( 0.5, -0.5, 0.0, 0.0), (0.0, 0.0, 1.0)),
    ];
    mesh.set_vertex_data(&vertices, gl::STATIC_DRAW);
    mesh.set_indices_u32_tuples(&[(0, 1, 2)], gl::STATIC_DRAW);

    mesh
}

#[allow(dead_code)]
fn cube_mesh() -> Mesh {
    let mut mesh = default_empty_mesh();

    let vertices: Vec<((f32, f32, f32, f32), (f32, f32, f32))> = vec![
        ((-0.5, -0.5, -0.5, 0.0), (1.0, 0.0, 0.0)),
        ((-0.5, -0.5,  0.5, 0.0), (0.0, 1.0, 0.0)),
        ((-0.5,  0.5, -0.5, 0.0), (0.0, 0.0, 1.0)),
        ((-0.5,  0.5,  0.5, 0.0), (1.0, 1.0, 0.0)),
        (( 0.5, -0.5, -0.5, 0.0), (1.0, 0.0, 1.0)),
        (( 0.5, -0.5,  0.5, 0.0), (0.0, 1.0, 1.0)),
        (( 0.5,  0.5, -0.5, 0.0), (1.0, 1.0, 1.0)),
        (( 0.5,  0.5,  0.5, 0.0), (0.0, 0.0, 0.0)),
    ];
    mesh.set_vertex_data(&vertices, gl::STATIC_DRAW);
    mesh.set_indices_u32_tuples(&[
        (0, 1, 2), (2, 1, 3),
        (4, 5, 6), (6, 5, 7),
        
        (0, 2, 4), (2, 4, 6),
        (1, 3, 5), (3, 5, 7),

        (0, 1, 4), (1, 4, 5),
        (2, 3, 6), (3, 6, 7),
    ], gl::STATIC_DRAW);

    mesh
}

fn cube_mesh_4d() -> Mesh {
    let mut mesh = default_empty_mesh();

    let cube_vertices = [
        nalgebra::vector![-0.5, -0.5, -0.5, 0.0, 1.0],
        nalgebra::vector![-0.5, -0.5,  0.5, 0.0, 1.0],
        nalgebra::vector![-0.5,  0.5, -0.5, 0.0, 1.0],
        nalgebra::vector![-0.5,  0.5,  0.5, 0.0, 1.0],
        nalgebra::vector![ 0.5, -0.5, -0.5, 0.0, 1.0],
        nalgebra::vector![ 0.5, -0.5,  0.5, 0.0, 1.0],
        nalgebra::vector![ 0.5,  0.5, -0.5, 0.0, 1.0],
        nalgebra::vector![ 0.5,  0.5,  0.5, 0.0, 1.0],
    ];
    let cube_indices = [
        (0u32, 1u32, 2u32), (2, 1, 3),
        (4, 5, 6), (6, 5, 7),
        
        (0, 2, 4), (2, 4, 6),
        (1, 3, 5), (3, 5, 7),

        (0, 1, 4), (1, 4, 5),
        (2, 3, 6), (3, 6, 7),
    ];

    const X: usize = 0;
    const Y: usize = 1;
    const Z: usize = 2;
    const W: usize = 3;

    let new_vertices = |transform: &Mat5, color: Vector3<f32>| 
        cube_vertices.iter()
            .map(|v| transform * v)
            .map(|v| nalgebra::vector![v[0] as f32, v[1] as f32, v[2] as f32, v[3] as f32])
            .map(|v| ((v[0], v[1], v[2], v[3]),(color[0], color[1], color[2])))
            .collect::<Vec<_>>();

    let transforms = [
        transform4d::shift(nalgebra::vector![ 0.5, 0.0, 0.0, 0.0]) * transform4d::rotation_single(90.0 * std::f64::consts::PI / 180.0, X, W),
        transform4d::shift(nalgebra::vector![-0.5, 0.0, 0.0, 0.0]) * transform4d::rotation_single(90.0 * std::f64::consts::PI / 180.0, X, W),
        transform4d::shift(nalgebra::vector![0.0,  0.5, 0.0, 0.0]) * transform4d::rotation_single(90.0 * std::f64::consts::PI / 180.0, Y, W),
        transform4d::shift(nalgebra::vector![0.0, -0.5, 0.0, 0.0]) * transform4d::rotation_single(90.0 * std::f64::consts::PI / 180.0, Y, W),
        transform4d::shift(nalgebra::vector![0.0, 0.0,  0.5, 0.0]) * transform4d::rotation_single(90.0 * std::f64::consts::PI / 180.0, Z, W),
        transform4d::shift(nalgebra::vector![0.0, 0.0, -0.5, 0.0]) * transform4d::rotation_single(90.0 * std::f64::consts::PI / 180.0, Z, W),
        transform4d::shift(nalgebra::vector![0.0, 0.0, 0.0, -0.5]),
        transform4d::shift(nalgebra::vector![0.0, 0.0, 0.0,  0.5]),
    ];

    let colors = [
        nalgebra::vector![1.0_f32, 0.0, 0.0],
        nalgebra::vector![0.0, 1.0, 0.0],
        nalgebra::vector![0.0, 0.0, 1.0],
        nalgebra::vector![1.0, 1.0, 0.0],
        nalgebra::vector![1.0, 0.0, 1.0],
        nalgebra::vector![0.0, 1.0, 1.0],
        nalgebra::vector![1.0, 1.0, 1.0],
        nalgebra::vector![0.8, 0.6, 0.2],
    ];

    let mut vertices_4d = vec![];
    let mut indices_4d = vec![];
    let mut index_offset = 0;
    for (transform, color) in transforms.iter().zip(colors) {
        let to_add = new_vertices(transform, color);
        let count = to_add.len() as u32;
        vertices_4d.extend(to_add);

        indices_4d.extend(cube_indices.map(
            |index| (index.0 + index_offset, index.1 + index_offset, index.2 + index_offset)
        ));

        index_offset += count;
    }


    mesh.set_vertex_data(&vertices_4d, gl::STATIC_DRAW);
    mesh.set_indices_u32_tuples(&indices_4d, gl::STATIC_DRAW);

    mesh
}