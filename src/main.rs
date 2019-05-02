#[macro_use]
extern crate ecs_rs;

use ecs_rs::types::*;
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
        // println!("-- Begin NoneSystem ---------------------");
        // println!("-- End NoneSystem ---------------------");
    }
});

impl_system_for! (PrintSystem {
    fn run(point: MutCD<Point>) {
        for point in point.dispatch() {
            point.x += 1.0;
            point.y += -1.0;
        }
        // println!("{:?}", point);
    }
});

impl_system_for! (Test2 {
    fn run(point: MutCD<Point>, gravity: GR<Gravity>) {
        for point in point.dispatch() {
            point.x += gravity.x;
            point.y += gravity.y;
        }
        // println!("{:?}", point);
    }
});

impl_system_for! (Test0 {
    fn run(point: MutCD<Point>) {
        for point in point.dispatch() {
            point.x += 1.0;
            point.y += -1.0;
        }
        // println!("{:?}", point);
    }
});

impl_system_for! (ChangeGravity {
    fn run(gravity: MutGR<Gravity>){
        gravity.x += 1.0;
    }
});

impl_system_for! (Test1 {
    fn run(gravity: MutGR<Gravity>){
        gravity.x += 1.0;
    }
});

// impl_system_for! (PrintSystem2 {
//     fn run(point: CD<Point>, me: CD<Me>) {
//         println!("-- Begin PrintSystem2 -------------------");
//         for (point, me) in (point, me).dispatch() {
//             println!("(point, me): ({:?}, {:?})", point, me);
//         }
//         println!("-- End PrintSystem2 -------------------");
//     }
// });
//
impl_system_for! (OperateGravity {
    fn run(point: MutCD<Point>, gravity: GR<Gravity>) {
        // println!("-- Begin PrintSystem2 ------------------");
        for point in point.dispatch() {
            point.x += gravity.x;
            point.y += gravity.y;
            println!("{:?}", point);
        }
        // println!("-- End PrintSystem2 -------------------\n");
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

    world.push_global_resource(Gravity { x: 0.0, y: -9.8 });

    world.entry_entity().push(Point { x: 2.0, y: 4.0 });

    // world.register_system::<NoneSystem>();
    // world.register_system::<PrintSystem>();
    // world.register_system::<PrintSystem2>();
    // world.register_system::<PrintGravity>();
    world.register_system::<OperateGravity>();
    world.register_system::<ChangeGravity>();
    world.register_system::<Test0>();
    world.register_system::<Test1>();

    world.run();
}
