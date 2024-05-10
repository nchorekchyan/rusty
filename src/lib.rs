use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{EventLoop, ActiveEventLoop}, window::{Window, WindowId}};



    struct App {
        window: Option<Window>,
    }

    impl App {
        fn new() -> Self {
            Self {
                window: None
            }
        }
    }

    impl ApplicationHandler for App {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            let window = event_loop.create_window(Default::default()).unwrap();
            self.window = Some(window);
        }

        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _: WindowId,
            event: WindowEvent,
        ) {
            match event {
                WindowEvent::CloseRequested =>  {
                    let _ = self.window.take();
                    event_loop.exit();
                },
                _ => {},
            }
        }
    }

pub fn dummy_window() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}

