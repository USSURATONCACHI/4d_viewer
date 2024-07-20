# 4D Camera Renderer in Rust

This project is a Rust implementation of a full 4D camera system with 3D perspective rendering, projected into 3D space using OpenGL. It demonstrates how to visualize and interact with four-dimensional objects by projecting them into three-dimensional space.

## Features

- **4D Camera**: Implemented with a 3D perspective.
- **OpenGL Rendering**: High-performance graphics rendering.
- **Dynamic Projection**: Projecting 4D objects into a 3D space.

## Screenshots

Here are two visual examples of the project's capabilities:

### Tesseract Rotation (`main` branch)

![Tesseract Rotation](./tesseract_rotates.gif)

### 4D Perspective Projection (`to_clipping` branch, WIP)

![4D Perspective](./4d_perspective.gif)

## Building and Running

To build and run this project, follow these steps:

1. **Build the project**:

   ```bash
   cargo build --release
   ```

2. **Run the project**:

   ```bash
   cargo run --release
   ```

## Dependencies

- Rust and Cargo
- OpenGL (Basically every pc has it now)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Rust programming language
- OpenGL for graphics rendering

For more information and detailed documentation, please refer to the project's Wiki or visit the official website.
