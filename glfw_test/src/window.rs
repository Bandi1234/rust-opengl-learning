use glfw::Context;
use gl33::global_loader as gl_loader;

pub struct WindowEvents {
    events : std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>
}

pub struct Window {
    glfw : glfw::Glfw,
    monitor : glfw::Monitor,
    window : glfw::Window,
    is_fullscreen : bool,
    width : u32,
    height : u32,
    old_width : u32,
    old_height : u32,
    old_x : i32,
    old_y : i32,
}

impl Window {
    pub fn new(title : &str, pref_width : i32, pref_height : i32) -> (Self, WindowEvents) {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let monitor: glfw::Monitor = glfw::Monitor::from_primary();
        let monitor_width = monitor.get_video_mode().unwrap().width;
        let monitor_height = monitor.get_video_mode().unwrap().height;

        let window_width = (pref_width as f32 * (monitor_width as f32 / 1920.0)) as u32;
        let window_height = (pref_height as f32 * (monitor_height as f32 / 1080.0)) as u32;
        let window_x = monitor_width / 2 - window_width / 2;
        let window_y = monitor_height / 2 - window_height / 2; 

        // Create window
        let (mut window, events) = glfw
        .create_window(window_width, window_height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

        // Set position, setup events
        window.set_pos((window_x) as i32, (window_y) as i32);
        window.make_current();
        window.set_key_polling(true);
        window.set_size_polling(true);

        (Window {
            glfw,
            monitor,
            window,
            is_fullscreen: false,
            width: window_width,
            height: window_height,
            old_width: 0,
            old_height: 0,
            old_x: 0,
            old_y: 0
        }, WindowEvents {
            events
        })
    }

    pub fn setup_gl(&mut self, major_version : u32, minor_version : u32, ) {
        unsafe {
            self.glfw.window_hint(glfw::WindowHint::ContextVersionMajor(major_version));
            self.glfw.window_hint(glfw::WindowHint::ContextVersionMinor(minor_version));
            self.glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

            gl_loader::load_global_gl(&|p| {
                let c_str = std::ffi::CStr::from_ptr(p as *const i8);
                let rust_str = c_str.to_str().unwrap();
                self.glfw.get_proc_address_raw(rust_str) as _
            });
            crate::renderer::enable_basic_blending();
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn close(&mut self) {
        self.window.set_should_close(true);
    }

    pub fn handle_events(&mut self, events : &WindowEvents) {
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events.events) {
            match event {
                glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
                    // Handle key events (this is a bad solution)
                    match (key, scancode, action, modifiers) {
                        (glfw::Key::F, _, glfw::Action::Press, _) => {
                            self.toggle_fullscreen();
                        }
                        (glfw::Key::Escape, _, glfw::Action::Press, _) => {
                            self.close();
                        }
                        _ => {

                        }
                    }
                }
                glfw::WindowEvent::Size(width, height) => {
                    self.width = width as u32;
                    self.height = height as u32;
                    crate::renderer::on_resize(width, height);
                }
                _ => {}
            }
        }
    }

    pub fn enter_fullscreen(&mut self) {
        let video_mode = self.monitor.get_video_mode().expect("No video mode?");
        (self.old_x, self.old_y) = self.window.get_pos();
        let (temp_w, temp_h) = self.window.get_size();
        self.old_width = temp_w as u32;
        self.old_height = temp_h as u32;
        self.window.set_monitor(
            glfw::WindowMode::FullScreen(&self.monitor),
            0,
            0,
            video_mode.width,
            video_mode.height,
            Some(video_mode.refresh_rate)
        );
        self.is_fullscreen = true;
    }

    pub fn exit_fullscreen(&mut self) {
        let video_mode = self.monitor.get_video_mode().expect("No video mode?");
        self.window.set_monitor(
            glfw::WindowMode::Windowed,
            self.old_x,
            self.old_y,
            self.width,
            self.height,
            Some(video_mode.refresh_rate)
        );
        self.is_fullscreen = false;
    }

    pub fn toggle_fullscreen(&mut self) {
        if self.is_fullscreen {
            self.exit_fullscreen();
        } else {
            self.enter_fullscreen();
        }
    }
}