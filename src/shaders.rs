use std::{
    fs::File,
    io::Read,
};

use gl::types::*;


fn compile_shader(filename: &str, shader_type: GLenum) -> GLuint {
    let mut src = Vec::with_capacity(512);
    File::open(filename)
        .unwrap_or_else(|e| {
            panic!("Could not open shader source `{}`: {}", filename, e)
        })
        .read_to_end(&mut src)
        .unwrap_or_else(|e| {
            panic!("Could not read shader source `{}`: {}", filename, e)
        });
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &(src.as_ptr() as *const GLchar), &(src.len() as GLsizei));
        gl::CompileShader(shader);
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE.into() {
            eprintln!("Could not compile shader `{}`. Log follows:", filename);
            let mut length = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);
            let mut log = Vec::with_capacity(length as usize);
            let mut str_len = 0;
            gl::GetShaderInfoLog(
                shader,
                length,
                &mut str_len,
                log.as_mut_ptr() as *mut GLchar
            );
            log.set_len(str_len as usize + 1);
            eprintln!("{}", String::from_utf8_lossy(&log));
            panic!("Shader compilation failed");
        }
        shader
    }
}

pub(crate) fn load_shaders() -> GLuint {
    let vert = compile_shader("./src/main.vert", gl::VERTEX_SHADER);
    let frag = compile_shader("./src/main.frag", gl::FRAGMENT_SHADER);
    
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vert);
        gl::AttachShader(program, frag);
        gl::LinkProgram(program);
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE.into() {
            panic!("Could not link shaders");
        }
        gl::DetachShader(program, vert);
        gl::DetachShader(program, frag);
        gl::DeleteShader(vert);
        gl::DeleteShader(frag);
        program
    }
}
