use std::{
    mem::{size_of, size_of_val},
    rc::Rc,
    sync::atomic::AtomicBool,
};

use servo::gl;

use crate::{Error, Result};

const FRAG_SRC: &[u8] = include_bytes!("shader/fragment.glsl");
const VERT_SRC: &[u8] = include_bytes!("shader/vertex.glsl");

/// Painter struct to handle gl bindings and rendering.
pub struct Painter {
    gl: Rc<dyn gl::Gl>,
    program: gl::GLuint,
    vao: gl::GLuint,
    destroyed: AtomicBool,
}

impl Painter {
    /// Create a new painter instance.
    pub fn new(gl: Rc<dyn gl::Gl>) -> Result<Painter> {
        let program = gl.create_program();
        let v_shader = load_shader(&gl, gl::VERTEX_SHADER, VERT_SRC)?;
        let f_shader = load_shader(&gl, gl::FRAGMENT_SHADER, FRAG_SRC)?;
        gl.attach_shader(program, v_shader);
        gl.attach_shader(program, f_shader);
        gl.link_program(program);
        gl.use_program(program);
        let vao = create_vao(&gl);
        Ok(Painter {
            gl,
            program,
            vao,
            destroyed: AtomicBool::new(false),
        })
    }
}

fn load_shader(gl: &Rc<dyn gl::Gl>, shader_type: gl::GLenum, source: &[u8]) -> Result<gl::GLuint> {
    let shader = gl.create_shader(shader_type);
    if shader == 0 {
        return Err(Error::CompileShader);
    }
    gl.shader_source(shader, &[source]);
    gl.compile_shader(shader);
    let mut compiled = [0];
    unsafe {
        gl.get_shader_iv(shader, gl::COMPILE_STATUS, &mut compiled);
    }
    if compiled[0] == 0 {
        let log = gl.get_shader_info_log(shader);
        log::error!("Failed to compile shader in Verso painer: {log}");
        gl.delete_shader(shader);
        return Err(Error::CompileShader);
    }
    Ok(shader)
}

type Vertex = [f32; 3];
const VERTICES: [Vertex; 6] = [
    [-1.0, -1.0, 0.0],
    [1.0, -1.0, 0.0],
    [1.0, 1.0, 0.0],
    [-1.0, 1.0, 0.0],
    [-1.0, -1.0, 0.0],
    [1.0, 1.0, 0.0],
];

fn create_vao(gl: &Rc<dyn gl::Gl>) -> gl::GLuint {
    let vao = gl.gen_vertex_arrays(1)[0];
    gl.bind_vertex_array(vao);

    let vbo = gl.gen_buffers(1)[0];
    gl.bind_buffer(gl::ARRAY_BUFFER, vbo);
    gl.buffer_data_untyped(
        gl::ARRAY_BUFFER,
        size_of_val(&VERTICES) as isize,
        VERTICES.as_ptr().cast(),
        gl::STATIC_DRAW,
    );

    gl.vertex_attrib_pointer(
        0,
        3,
        gl::FLOAT,
        false,
        size_of::<Vertex>() as gl::GLsizei,
        0,
    );
    gl.enable_vertex_attrib_array(0);
    gl.bind_vertex_array(0);
    vao
}
