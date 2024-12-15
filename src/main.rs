use speedy2d::color::{Color};
use speedy2d::{window::{WindowHandler, WindowHelper}, Graphics2D, Window};

struct Vector {
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
            }
    pub fn add(&mut self, other: &Vector) -> &Self {
            self.x += other.x;
            self.y += other.y;
            self
        }

    pub fn set(&mut self, x: f32, y: f32) -> &Self {
            self.x = x;
            self.y = y;
            self
        }
    }

struct Pendulum {
    origin: Vector,
    position: Vector,
    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,
    r: f32,
    m: f32,
    g: f32,
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Self {
        Pendulum { 
            origin: Vector::new(x, y),
            position: Vector::new(0.0, 0.0),

            angle: 1.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,

            r,
            m: 1.0,
            g: 1.5,
        }
    }

    fn update(&mut self) {
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;
        
        self.angular_velocity += self.angular_acceleration;
        
        self.angle += self.angular_velocity;

        self.position.set(self.r * self.angle.sin(), self.r * self.angle.cos());
        
        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y), 
            (self.position.x, self.position.y), 
            3.0, 
            Color::RED
        );

        
        graphics.draw_circle(
            (self.position.x, self.position.y), 
            30.0, 
            Color::BLUE
        );
    }
}

struct MyWindowHandler {
    p: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.p.update();
        self.p.draw(graphics);

        // Draw the frame.
        helper.request_redraw();
    }
}

fn main() {
    let window = Window::new_centered("RNTrybusy - Pendulum", (800, 600))
    .expect("Erro ao criar a janela");

    let win = MyWindowHandler { p: Pendulum::new(400.0, 0.0, 200.0) };

    window.run_loop(win);
}
