extern crate vulkano;
extern crate winit;

use winit::{Event, EventsLoop, WindowBuilder, WindowEvent, dpi::LogicalSize};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct OrcaApplication {
    events_loop: EventsLoop,
}

impl OrcaApplication {
    pub fn initialize() -> Self {
        let events_loop = Self::init_window();

        Self { events_loop }
    }

    fn init_window() -> EventsLoop {
        let events_loop = EventsLoop::new();
        let _window = WindowBuilder::new()
            .with_title("Orca")
            .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build(&events_loop);
        events_loop
    }

    fn main_loop(&mut self) {
        loop {
            let mut done = false;
            self.events_loop.poll_events(|ev| {
                if let Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } = ev
                {
                    done = true
                }
            });
            if done {
                return;
            }
        }
    }
}

fn main() {
    let source_file = "./app.argon";
    let source_code = std::fs::read_to_string(source_file).unwrap();

    println!("{}", source_code);

    let mut app = OrcaApplication::initialize();
    app.main_loop();
}
