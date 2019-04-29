#[macro_use]
extern crate ecs_rs;

use ecs_rs::types::{ResourceContainer, CD};
use ecs_rs::World;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Me {
    coord: Point,
    name: &'static str,
}

#[derive(Debug)]
struct Gravity {
    x: f32,
    y: f32,
}

impl_system_for! (NoneSystem {
    fn run() {
        println!("-- Begin NoneSystem ---------------------");
        println!("-- End NoneSystem ---------------------");
    }
});

impl_system_for! (PrintSystem {
    fn run(point: CD<Point>) {
        println!("-- Begin PrintSystem --------------------");
        for point in point.iter() {
            println!("point: {:?}", point);
        }
        println!("-- End PrintSystem --------------------");
    }
});

impl_system_for! (PrintSystem2 {
    fn run(point: CD<Point>, me: CD<Me>) {
        println!("-- Begin PrintSystem2 -------------------");
        for (point, me) in point.iter().zip(me.iter()) {
            println!("(point, me): ({:?}, {:?})", point, me);
        }
        println!("-- End PrintSystem2 -------------------");
    }
});

fn main() {
    let mut world = World::new();
    world.register_component::<Point>();
    world.register_component::<Me>();

    world
        .entry_entity()
        .push(Point { x: 1.0, y: 2.0 })
        .push(Me {
            coord: Point { x: 3.0, y: 2.0 },
            name: "baba",
        });

    world.push_global_resource(Gravity { x: 0.0, y: 9.8 });

    world.entry_entity().push(Point { x: 2.0, y: 4.0 });

    world.register_system::<NoneSystem>();
    world.register_system::<PrintSystem>();
    world.register_system::<PrintSystem2>();

    println!("{:?}", world.get_type_map());
    println!("{:?}", world.get_entity_ref());
    println!("{:?}", world.get_component_data_ref::<Point>());
    println!("{:?}", world.get_component_data_mut::<Me>());
    println!("{:?}", world.get_global_resource_ref::<Gravity>());

    world.run();
}
