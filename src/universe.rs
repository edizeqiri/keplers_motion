use std::time::Duration;
use crate::space_object::SpaceObject;
use crate::physics::GeForce;

pub struct Galaxy {
    entities: Vec<SpaceObject>,
}

impl Galaxy {
    pub fn new() -> Galaxy {
        let sun = SpaceObject::new("Sun".to_string(), false,
                                   [0.0, 0.0], 1.989e30, 0.0, 0.0, 0.0, 1);
        let earth = SpaceObject::new("Earth".to_string(), true,
                                     [152e3, 152e3], 5.972e24, 0.0, 0.0, 2.0, 1);
        let pluto = SpaceObject::new("Pluto".to_string(), true,
                                     [5.9e4, 5.9e4], 1.30900e22, 0.0, 0.0, 10.0, 1);

        let space_things: Vec<SpaceObject> = vec![sun, earth, pluto];

        Galaxy {
            entities: space_things
        }
    }

    pub fn update(&mut self, dt: Duration) {
        let dt = dt.as_secs_f64();
        for i in 0..self.entities.len() {
            for j in 0..self.entities.len() {
                if i != j {
                    let force = GeForce(&self.entities[i], &self.entities[j]);
                    self.entities[i].update(force, dt);
                    self.entities[j].update(-force, dt);
                }
            }
        }
    }

    pub fn get_size(&self) -> usize {
        self.entities.len()
    }

    pub fn print(&self) {
        for i in 0..self.get_size() {
            self.entities[i].print();
        }
    }
}