use cgmath::*;
use core::ops::Mul;
use gl::types::*;
use sdl2::video::GLProfile;
use std::ffi::{CStr, CString};
use std::fs;
use std::time::{Duration, Instant};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

// const vertices: [f32; 32] = [
//     0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.6, 0.4, // top right
//     0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.6, 0.6, // bottom right
//     -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.4, 0.6, // bottom left
//     -0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 0.4, 0.4, // top left
// ];

// Triangle
const VERTICES: [GLfloat; 12] = [
    0.5, 0.5, 0.0, // top right
    0.5, -0.5, 0.0, // bottom right
    -0.5, -0.5, 0.0, // bottom left
    -0.5, 0.5, 0.0, // top left
];

const INDICES: [GLuint; 6] = [0, 1, 3, 1, 2, 3];

pub fn start_opengl_test() {
    let vertex_source =
        fs::read_to_string("src/opengl_test/vertex.glsl").expect("Error reading file vertex.glsl");
    let fragment_source = fs::read_to_string("src/opengl_test/fragment.glsl")
        .expect("Error reading file fragment.glsl");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("opengl_test", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    // Shader creation
    // vertex shader
    let vertex_shader_source: CString =
        CString::new(vertex_source.to_string()).expect("CString::new failed");

    let fragment_shader_source: CString =
        CString::new(fragment_source.to_string()).expect("CString::new failed");

    let mut shader_program: GLuint = 0;
    unsafe {
        let mut vertex_shader: GLuint = 0;
        let mut fragment_shader: GLuint = 0;

        vertex_shader = shader_from_source(&vertex_shader_source, gl::VERTEX_SHADER).unwrap();
        fragment_shader = shader_from_source(&fragment_shader_source, gl::FRAGMENT_SHADER).unwrap();
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        // check for linking errors LATER
        // gl::GetProgramiv(shader_program, gl::LINK_STATUS, &success);
        // if (!success) {
        //     glGetProgramInfoLog(shaderProgram, 512, NULL, infoLog);
        //     std::cout << "ERROR::SHADER::PROGRAM::LINKING_FAILED\n" << infoLog << std::endl;
        // }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }
    // ===============

    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    let mut ebo: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTICES.len() * std::mem::size_of::<GLfloat>()) as gl::types::GLsizeiptr,
            std::mem::transmute(&VERTICES[0]),
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (INDICES.len() * std::mem::size_of::<GLuint>()) as gl::types::GLsizeiptr,
            std::mem::transmute(&INDICES[0]),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<GLfloat>()) as gl::types::GLint,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    let aspect_ratio = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;

    let model_name = CString::new("model").unwrap();
    let model_ptr = model_name.as_ptr() as *const GLchar;
    let view_name = CString::new("view").unwrap();
    let view_ptr = view_name.as_ptr() as *const GLchar;
    let proj_name = CString::new("projection").unwrap();
    let proj_ptr = proj_name.as_ptr() as *const GLchar;

    let now = Instant::now();

    'game: loop {
        let mut model: Matrix4<f32> = One::one();
        let rotation =
            Matrix4::from_angle_y(Rad::from(Deg(now.elapsed().as_millis() as f32 / 10.0)));
        model = model.mul(rotation);
        let model_: [[f32; 4]; 4] = model.into();
        let translation = Matrix4::from_translation(Vector3::new(0.0, 0.0, -3.0));
        let mut view: Matrix4<f32> = One::one();
        view = view.mul(translation);
        let view_: [[f32; 4]; 4] = view.into();
        let projection: [[f32; 4]; 4] =
            perspective(Rad::from(Deg(45.0)), aspect_ratio, 0.1, 100.0).into();
        unsafe {
            gl::ClearColor(1.0, 0.0, 0.21, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);

            let model_mat_ptr: *const f32 = std::mem::transmute(&model_);
            let model_loc = gl::GetUniformLocation(shader_program, model_ptr);
            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model_mat_ptr);

            let view_mat_ptr: *const f32 = std::mem::transmute(&view_);
            let view_loc = gl::GetUniformLocation(shader_program, view_ptr);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view_mat_ptr);

            let proj_mat_ptr: *const f32 = std::mem::transmute(&projection);
            let proj_loc = gl::GetUniformLocation(shader_program, proj_ptr);
            gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE, proj_mat_ptr);

            gl::BindVertexArray(vao);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }

        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'game,
                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(20));
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        buffer.extend([b' '].iter().cycle().take(len as usize));
        let error: CString = unsafe { CString::from_vec_unchecked(buffer) };
        [b' '].iter().cycle().take(len as usize);
    }

    Ok(id)
}
