#[macro_use]
extern crate glium;
extern crate raytracer;
extern crate threadpool;

extern crate image;

use glium::glutin;
use glium::DisplayBuild;
use glium::Surface;
use glium::glutin::Event;
use glium::texture::texture2d::Texture2d;
use glium::index::PrimitiveType;

fn main() {
    let display =
        glutin::WindowBuilder::new()
        .with_dimensions(640, 640)
        .with_title(format!("Ray Tracer"))
        .with_vsync()
        .build_glium()
        .unwrap();

    let (width, height) = display.get_framebuffer_dimensions();

    let target_width = width;
    let target_height = (target_width as f64 * (height as f64 / width as f64)) as u32;

    let tex = Texture2d::new(&display, display_pixels(target_width, target_height)).unwrap();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vert {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vert, position, tex_coords);

        glium::VertexBuffer::new(
            &display,
            &[
                Vert { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vert { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
                Vert { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                Vert { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] },
                ]
                ).unwrap()
    };

    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip,
                                               &[1 as u16, 2, 0, 3]).unwrap();

    let program = mk_program(&display);

    let mut closed = false;
    while !closed {

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
                ],
            tex: &tex,
        };
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                Event::Closed => closed = true,
                _ => ()
            }
        }

        if closed {
            let raw_image: glium::texture::RawImage2d<_> = display.read_front_buffer();
            let image_buf: image::ImageBuffer<image::Rgb<u8>, _> = image::ImageBuffer::from_raw(raw_image.width, raw_image.height, raw_image.data).unwrap();
            image_buf.save("output.png").unwrap();
        }
    }
}

use raytracer::tracer::*;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;

struct PixResult {
    x: u32,
    y: u32,
    col: (f32, f32, f32),
}

unsafe impl Send for PixResult {}

fn display_pixels(width: u32, height: u32) -> Vec<Vec<(f32, f32, f32)>> {
    let tasks = 100;

    let wf = width as f64;
    let hf = height as f64;

    let sf = wf.min(hf);

    let aac = 3;

    let aa_step = 1.0 / aac as f64;
    let aa_start = aa_step / 2.0;

    let env = raytracer::env::default_env();
    let shared_env: Arc<Environment> = Arc::new(env);

    let pool = ThreadPool::new(tasks);
    let (tx, rx) = channel();

    for y in 0..height {
        let child_tx = tx.clone();
        let local_env = shared_env.clone();

        pool.execute(move || {
            for x in 0..width {
                let yf = y as f64;
                let xf = x as f64;

                let mut illum = v3(0.0, 0.0, 0.0);

                for ax in 0..aac {
                    for ay in 0..aac {
                        let xx = xf + aa_start + aa_step * ax as f64;
                        let yy = yf + aa_start + aa_step * ay as f64;

                        let xp = xx / sf;
                        let yp = yy / sf;

                        illum = illum.add(&local_env.get_point_col(xp, yp));
                    }
                }

                illum = illum.mul(1.0 / (aac * aac) as f64);

                let _ = child_tx.send(
                    PixResult {
                        x: x,
                        y: y,
                        col: (illum.x as f32, illum.y as f32, illum.z as f32),
                    });
            }
        })
    }

    let mut result = Vec::with_capacity(height as usize);

    for _ in 0..height {
        result.push(vec![(0.0, 0.0, 0.0); width as usize]);
    }

    for _ in 0..height {
        for _ in 0..width {
            let pix = rx.recv().unwrap();

            result[pix.y as usize][pix.x as usize] = pix.col;
        }
    }

    result
}

fn mk_program<F>(facade: &F) -> glium::Program where F: glium::backend::Facade {
    program!(facade,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 f_color;
                void main() {
                    f_color = texture(tex, v_tex_coords);
                }
            "
        },

        110 => {
            vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec2 position;
                attribute vec2 tex_coords;
                varying vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 110
                uniform sampler2D tex;
                varying vec2 v_tex_coords;
                void main() {
                    gl_FragColor = texture2D(tex, v_tex_coords);
                }
            ",
        },

        100 => {
            vertex: "
                #version 100
                uniform lowp mat4 matrix;
                attribute lowp vec2 position;
                attribute lowp vec2 tex_coords;
                varying lowp vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 100
                uniform lowp sampler2D tex;
                varying lowp vec2 v_tex_coords;
                void main() {
                    gl_FragColor = texture2D(tex, v_tex_coords);
                }
            ",
        },
    ).unwrap()
}
