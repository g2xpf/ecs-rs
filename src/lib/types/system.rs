use super::{ComponentData, Entity, TypeMap};

pub trait System {
    fn new<'a, 'b: 'a>(component_data: &ComponentData, type_map: &TypeMap) -> Self
    where
        Self: Sized;
    fn run(&self, entity: &Entity);
}

#[macro_export]
macro_rules! impl_system_for {
    ($system:ident {
        fn run() $body:block
    }) => {
        pub struct $system {
            name: &'static str,
        }

        impl $crate::types::System for $system {
            fn new<'a, 'b: 'a>(component_data: &$crate::types::ComponentData, type_map: &$crate::types::TypeMap) -> Self {
                $system {
                    name: stringify!($system),
                }
            }

            fn run(&self, entity: &$crate::types::Entity) {
                $body
            }
        }
    };

    ($system:ident {
        fn run($values0:ident: $container0:ident<$args0:ty>) $body:block
    }) => {
        pub struct $system {
            name: &'static str,
            cache: impl_system_for!(expand; $args0),
        }

        impl $crate::types::System for $system {
            fn new<'a, 'b: 'a>(component_data: &$crate::types::ComponentData, type_map: &$crate::types::TypeMap) -> $system {
                $system {
                    name: stringify!($system),
                    cache: (component_data.clone_component_data(&<$container0<'a, 'b, $args0> as $crate::types::InnerType<'a, 'b>>::to_inner_type()), *type_map.get(&std::any::TypeId::of::<$args0>()).unwrap()),
                }
            }

            fn run(&self, entity: &$crate::types::Entity) {
                let $values0 = &self.cache;
                let $values0: $container0<'_, '_, $args0> = $crate::types::ResourceContainer::<'_, '_, $container0<'_, '_, $args0>>::to_container($values0, entity);
                $body
            }
        }
    };

    ($system:ident {
        fn run( $values0:ident: $container0:ident<$args0:ty>, $($values:ident: $container:ident<$args:ty>),+ ) $body:block
    }) => {
        pub struct $system {
            name: &'static str,
            cache: impl_system_for!(expand; $args0, $($args),+),
        }

        impl $crate::types::System for $system {
            fn new<'a, 'b: 'a>(component_data: &$crate::types::ComponentData, type_map: &$crate::types::TypeMap) -> $system {
                $system {
                    name: stringify!($system),
                    cache: ((component_data.clone_component_data(&<$container0<'a, 'b, $args0> as $crate::types::InnerType<'a, 'b>>::to_inner_type()), *type_map.get(&std::any::TypeId::of::<$args0>()).unwrap()),
                          $((component_data.clone_component_data(&<$container <'a, 'b, $args > as $crate::types::InnerType<'a, 'b>>::to_inner_type()), *type_map.get(&std::any::TypeId::of::<$args >()).unwrap())),+),
                }
            }

            fn run(&self, entity: &$crate::types::Entity) {
                let ($values0, $($values),+) = &self.cache;
                let ($values0, $($values),+): ($container0<'_, '_, $args0>, $($container<'_, '_, $args>),+) =
                                                ($crate::types::ResourceContainer::<'_, '_, $container0<'_, '_, $args0>>::to_container($values0, entity),
                                                $($crate::types::ResourceContainer::<'_, '_, $container<'_, '_, $args >>::to_container($values , entity)),+);
                $body
            }
        }
    };

    (expand; $head:ty, $($tail:ty),+) => {
        (impl_system_for!(expand; $head), impl_system_for!(expand; $($tail),+))
    };

    (expand; $head:ty) => {
        (std::rc::Rc<std::cell::RefCell<Box<$crate::data_types::Resource>>>, $crate::types::ComponentMask)
    };

    (deps; $type_map:ident; $head:ty) => {{
        *$type_map.get(&std::any::TypeId::of::<$head>()).unwrap()
    }};

    (deps; $type_map:ident; $head:ty, $($tail:ty),+) => {{
        *$type_map.get(&std::any::TypeId::of::<$head>()).unwrap() | impl_system_for!(deps; $type_map; $($tail),+)
    }};
}
