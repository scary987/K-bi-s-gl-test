#[macro_use] extern crate c_str_macro;

use glutin::{
    self,
    dpi,
    event_loop::ControlFlow,
    event::{
        Event,
        DeviceEvent,
        WindowEvent,
    },
};

use gl::types::*;

use std::{
    mem,
};

mod shaders;


fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello GL world!")
        .with_resizable(false)
        .with_inner_size(dpi::LogicalSize::new(800.0, 450.0));
    let windowed_context = glutin::ContextBuilder::new()
        .with_gl_robustness(glutin::Robustness::TryRobustLoseContextOnReset)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe {
        windowed_context.make_current().expect("failed to make context current")
    };
    gl::load_with(|s| windowed_context.get_proc_address(s));
    setup_gl();
    windowed_context.swap_buffers().expect("failed to swap buffers");
    el.run(move |ev, _, flow| {
        *flow = glutin::event_loop::ControlFlow::Wait;
        match ev {
            | Event::MainEventsCleared
            | Event::RedrawEventsCleared
            | Event::NewEvents { .. }
            | Event::DeviceEvent { event: DeviceEvent::Motion { .. }, .. }
            | Event::WindowEvent { event: WindowEvent::AxisMotion { .. }, .. }
            => {}
            | Event::WindowEvent { event: WindowEvent::CloseRequested, .. }
            | Event::WindowEvent { event: WindowEvent::Destroyed, .. }
            => *flow = ControlFlow::Exit,
            e => eprintln!("{:?}", e),
        }
    });
}

fn setup_gl() {
    const VERTICES: [[f32; 2]; 3] = [
        [-0.8, -0.8],
        [0.8, -0.8],
        [0.0, 0.8],
    ];
    let program = shaders::load_shaders();
    unsafe {
        gl::UseProgram(program);
        let loc_pos = gl::GetAttribLocation(
            program, c_str!("position").as_ptr()) as GLuint;
        
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        
        let mut buffer = 0;
        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of_val(&VERTICES) as GLsizeiptr,
            &VERTICES as *const _ as *const GLvoid,
            gl::STATIC_DRAW
        );
        gl::EnableVertexAttribArray(loc_pos);
        gl::VertexAttribPointer(
            loc_pos, 2, gl::FLOAT, gl::FALSE, 0, 0 as *const _);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::DrawArrays(gl::TRIANGLES, 0, VERTICES.len() as GLsizei);
    }
}


