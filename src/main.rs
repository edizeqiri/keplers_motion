mod space_object;
mod physics;
mod universe;
use std::time::{Instant, Duration};
use std::thread::sleep;
use space_object::SpaceObject;


fn main() {

    // set the time step to 100 milliseconds
    let dt = Duration::from_millis(100);

    // initialize the state of the simulation
    let mut milky_way = universe::Galaxy::new();

    milky_way.print();
    milky_way.update(Instant::now().elapsed());
    milky_way.print();

    loop {
        // get the start time
        let start = Instant::now();

        // update the simulation based on the time step
        milky_way.update(dt);
        milky_way.print();

        // sleep for the remaining duration, if any
        let elapsed = start.elapsed();
        if elapsed < dt {
            sleep(dt - elapsed);
        }
    }
}


// make a function that calculates the acceleration of the objects
// make a function that calculates the velocity of the objects
// make a function that calculates the position of the objects
// make a function that calculates the distance between two objects
