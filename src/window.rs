use std::sync::mpsc::Receiver;
use std::mem;

use gl;
use glfw::{self, Glfw, Context, Key, Action, Window as GlfwWindow, WindowEvent};

use lang::{ObjectPar, RasterFloat, TimeSec};
use input::{MouseEvent, MouseButtonEvent, KeyEvent, InputEvent, InputControl};
use timing::Timing;

type Events = Receiver<(f64, WindowEvent)>;

pub struct Window {
    pub controls: Vec<ObjectPar<InputControl>>,
    pub timing: Timing,
    glfw: Glfw,
    window: GlfwWindow,
    events: Option<Events>,
    last_mouse_pos: Option<(RasterFloat, RasterFloat)>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        // ------------------------------
        // glfw: initialize and configure
        // ------------------------------
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
        #[cfg(target_os = "macos")]
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // --------------------
        // glfw window creation
        // --------------------
        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_framebuffer_size_polling(true);

        // -------------------------------------
        // gl: load all OpenGL function pointers
        // -------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window {
            controls: vec![],
            timing: Timing::default(),
            glfw,
            window,
            events: Some(events),
            last_mouse_pos: None,
        }
    }

    pub fn events_loop<F: FnMut(&mut Window) -> ()>(&mut self, mut render: Option<F>) {
        let events = mem::replace(&mut self.events, None);

        while !self.window.should_close() {
            self.timing();

            // ## events
            if let Some(ref events) = events {
                self.process_events(events);
            }

            // ## process input
            self.process_input();

            // ## render
            if let Some(ref mut render) = render {
                render(self);
            } else {
                self.render();
            }
            self.window.swap_buffers();

            // ## glfw: poll IO events (keys pressed/released, mouse moved etc.)
            self.glfw.poll_events();
        }
    }

    /// per-frame time logic
    fn timing(&mut self) {
        let current_frame = self.glfw.get_time() as TimeSec;
        self.timing.delta_time = current_frame - self.timing.last_frame;
        self.timing.last_frame = current_frame;
    }

    fn process_events(&mut self, events: &Events) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                },
                WindowEvent::CursorPos(x_pos, y_pos) => {
                    let (x_pos, y_pos) = (x_pos as RasterFloat, y_pos as RasterFloat);

                    if self.last_mouse_pos.is_none() {
                        self.last_mouse_pos = Some((x_pos, y_pos));
                    }

                    let (x_last, y_last) = self.last_mouse_pos.unwrap();
                    let x_offset = x_pos - x_last;
                    let y_offset = y_last - y_pos; // reversed since y-coordinates go from bottom to top

                    self.last_mouse_pos = Some((x_pos, y_pos));

                    self.mouse_event(MouseEvent {
                        x_pos,
                        y_pos,
                        x_offset,
                        y_offset,
                        is_scroll: false,
                        button_event: None,
                    });
                },
                WindowEvent::Scroll(x_offset, y_offset) => {
                    let (x_pos, y_pos) = self.last_mouse_pos.unwrap_or((0.0, 0.0));
                    self.mouse_event(MouseEvent {
                        x_pos,
                        y_pos,
                        x_offset: x_offset as RasterFloat,
                        y_offset: y_offset as RasterFloat,
                        is_scroll: true,
                        button_event: None,
                    });
                },
                // This is not work (why?), use GlfwWindow::get_mouse_button in process_input instead
                WindowEvent::MouseButton(button, action, modifiers) => {
                    let (x_pos, y_pos) = self.last_mouse_pos.unwrap_or((0.0, 0.0));
                    self.mouse_event(MouseEvent {
                        x_pos,
                        y_pos,
                        x_offset: 0.0,
                        y_offset: 0.0,
                        is_scroll: false,
                        button_event: Some(MouseButtonEvent(button, action, modifiers)),
                    });
                },
                WindowEvent::Key(key, code, action, modifiers) => {
                    self.keyboard_event(KeyEvent(key, code, action, modifiers))
                },
                _ => {}
            }
        }
    }

    fn process_input(&mut self) {
        for control in self.controls.iter() {
            if let Ok(mut control) = control.lock() {
                control.on_input(self.glfw_window(), self.timing.delta_time);
            }
        }
    }

    pub fn glfw_window(&self) -> &GlfwWindow {
        &self.window
    }

    pub fn glfw_window_mut(&mut self) -> &mut GlfwWindow {
        &mut self.window
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

impl InputEvent for Window {
    fn mouse_event(&mut self, event: MouseEvent) {
        for control in self.controls.iter() {
            if let Ok(mut control) = control.lock() {
                control.on_mouse(event.clone(), self.timing.delta_time);
            }
        }
    }

    fn keyboard_event(&mut self, event: KeyEvent) {
        match event {
            KeyEvent(Key::Escape, _, Action::Press, _) => self.window.set_should_close(true),
            _ => ()
        }

        for control in self.controls.iter() {
            if let Ok(mut control) = control.lock() {
                control.on_keyboard(event.clone(), self.timing.delta_time);
            }
        }
    }
}