use winit::{
    application::ApplicationHandler, 
    event::WindowEvent, 
    event_loop::{EventLoop, ControlFlow, ActiveEventLoop}, 
    window::{Window, WindowId}
};

    #[derive(Default)]
    struct App {
        window: Option<Window>,
    }

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested =>  {
                let _ = self.window.take(); // WORKAROUND TO SURVIVE PANIC DUE TO BUG in WINIT 0.30 MACOS
                event_loop.exit();
                println!("Boop");
            },
            WindowEvent::RedrawRequested => {
                if self.window.is_some() {  // WORKAROUND SINCE CLOSE STILL REQUESTS REDRAW, window stays open when returning to command prompt but closes on clean exit of game
                    self.window.as_ref().unwrap().request_redraw();
                }
            }
            _ => (),
        }
    }
}

pub fn dummy_window() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}