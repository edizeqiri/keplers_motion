pub struct SpaceObject {
    name: String,
    is_planet: bool,
    pub position: [f64; 2],
    mass: f64,
    velocity: f64,
    acceleration: f64,
    force: f64,
    radius: i32
}

impl SpaceObject {

    pub fn new(name: String, is_planet: bool, position: [f64; 2], mass: f64, velocity: f64,
               acceleration: f64, force: f64, radius: i32) -> SpaceObject {
        SpaceObject {
            name,
            is_planet,
            position,
            mass,
            velocity,
            acceleration,
            force,
            radius
        }
    }

    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    pub fn get_radius(&self) -> i32 {
        self.radius
    }

    pub fn update(&mut self, force: f64, dt: f64) {
        self.force += force;
        self.acceleration = self.acceleration + self.force / self.mass as f64;
        self.velocity = self.velocity + self.acceleration * dt;

    }

    pub fn print(&self) {
        println!("{} is a planet: {}, their force is: {} and their velocity is: {} ", self.name,
                 self.is_planet, self.force, self.velocity);
    }

}