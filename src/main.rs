use speedy2d::color::{Color};
use speedy2d::{window::{WindowHandler, WindowHelper}, Graphics2D, Window};

struct Vector {
        pub x: f32,
        pub y: f32,
    }

    // Implementation of the Vector struct
    impl Vector {
        // Function to create a new Vector with given x and y values
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
            }

        // Function to add another Vector to this one, modifying it in-place
    pub fn add(&mut self, other: &Vector) -> &Self {
            self.x += other.x;
            self.y += other.y;

            // Return a reference to the modified Vector
            self
        }

        // Function to set the x and y values of this Vector, modifying it in-place
    pub fn set(&mut self, x: f32, y: f32) -> &Self {
            self.x = x;
            self.y = y;

            // Return a reference to the modified Vector
            self
        }
    }

struct Pendulum {
	// This vector is the position of the Pendulum.
    origin: Vector,
    
    // This vector is the position of the ball.
    position: Vector,
    
    // This is the angle of the pendulum.
    angle: f32,
    
    angular_velocity: f32,
    angular_acceleration: f32,
    
    r: f32, // The lenght of the pendulum.
    m: f32, // The mass of the ball.
    g: f32, // The gravity.
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Self {
        Pendulum {
	        // We need to set the origin of the pendulum.
            origin: Vector::new(x, y),
            // For now we'll set it to a default value.
            position: Vector::new(0.0, 0.0),

            angle: 1.0, // set the angle to 1.0 radian.
            angular_velocity: 0.0, // The pendulum is not moving in the beginning.
            angular_acceleration: 0.0, // The pendulum is not accelerating in the beginning.

            r,
            m: 1.0, // The mas of the ball is 1.0 for this example.
			g: 1.5, // The gravity is 0.5 for this example, but play with it.
        }
    }

    fn update(&mut self) {
	    // We use the pendulum equation to calculate the angular acceleration.
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

		// The angular velocity is the angular velocity plus the angular acceleration.
        self.angular_velocity += self.angular_acceleration;
        
		// The angle is the angle plus the angular velocity.
        self.angle += self.angular_velocity;

		// The position is the polar coodinates translated to cartesian coordinates.
        self.position.set(self.r * self.angle.sin(), self.r * self.angle.cos());

        // The final position of the ball in the canvas.
        // Pendulum plus the position vector.
        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        // We need to draw the line of the pendulum first.
        // It takes the start and end position, the width and the color of the line.
        graphics.draw_line(
            (self.origin.x, self.origin.y), 
            (self.position.x, self.position.y), 
            3.0, 
            Color::RED
        );

        // We need to draw the ball of the pendulum.
        // It takes the position of the ball, the radius and the color.
        graphics.draw_circle(
            (self.position.x, self.position.y), 
            30.0, 
            Color::BLUE
        );
    }
}

// This is the window handler.
// It handles the window events and have the objects that we want to draw and the logic.
struct MyWindowHandler {
    p: Pendulum,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
		// We need to clear the screen every frame.
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.p.update();
        self.p.draw(graphics);

        // Draw the frame.
        helper.request_redraw();
    }
}

fn main() {
	// We need this window object to create a window.
    let window = Window::new_centered("RNTrybusy - Pendulum", (800, 600))
    .expect("Erro ao criar a janela");

    let win = MyWindowHandler { p: Pendulum::new(400.0, 0.0, 200.0) };

    window.run_loop(win);
}
