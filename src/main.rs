#[macro_use]
extern crate ecs_rs;

use ecs_rs::{Dispatcher, MutCD, World};

impl_system_for!(ShiftF32 {
    fn run(point: MutCD<Point<f32>>) {
        for point in point.dispatch() {
            point.x += 1.0;
            point.y += 1.0;
        }
    }
});

impl_system_for!(ShiftU16 {
    fn run(point: MutCD<Point<u32>>) {
        for point in point.dispatch() {
            point.x += 1;
            point.y += 1;
        }
    }
});

struct Point<F> {
    x: F,
    y: F,
}

fn main() {
    let mut world = World::new();

    world.register_component::<Point<f32>>();
    world.register_component::<Point<u32>>();
    world
        .entry_entity()
        .push(Point::<f32> { x: 1.0, y: 2.0 })
        .push(Point::<u32> { x: 1, y: 2 });

    world.register_system::<ShiftF32>();
    world.register_system::<ShiftU16>();

    world.run();
}
