use crate::space_object::SpaceObject;

pub fn GeForce(p1: &SpaceObject, p2: &SpaceObject) -> f64{
    let gravity = 6.674e-11;
    let r2 = (p1.position[0] - p2.position[0]).powf(2.0) + (p1.position[1] - p2.position[1]).powf(2.0);
    let force = gravity * ((p1.get_mass() * p2.get_mass()) / r2 as f64) ;
    return force;
}

pub fn acceleration(force: f64, mass: f64) -> f64 {
    force / mass
}

pub fn velocity(acceleration: f64, velocity: f64) -> f64 {
    acceleration + velocity
}
