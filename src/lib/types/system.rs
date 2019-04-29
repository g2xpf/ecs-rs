use super::ComponentData;

pub trait System {
    fn new<'a>(component_data: &ComponentData) -> Self
    where
        Self: Sized;
    fn run(&self);
}

#[macro_export]
macro_rules! impl_system_for {
    ($system:ident {
        fn run() $body:block
    }) => {
        pub struct $system {
            pub name: &'static str,
        }

        impl $crate::types::System for $system {
            #[inline]
            fn new(component_data: &$crate::types::ComponentData) -> Self {
                $system {
                    name: stringify!($system),
                }
            }

            #[inline]
            fn run(&self) {
                $body
            }
        }
    };

    ($system:ident {
        fn run($values0:ident: $container0:ident<$args0:ty>) $body:block
    }) => {
        pub struct $system {
            pub name: &'static str,
            pub cache: impl_system_for!(expand; $args0),
        }

        impl $crate::types::System for $system {
            #[inline]
            fn new<'a>(component_data: &$crate::types::ComponentData) -> $system {
                $system {
                    name: stringify!($system),
                    cache: component_data.clone_component_data(&<$container0<'a, $args0> as $crate::types::InnerType<'a>>::to_inner_type())
                }
            }

            #[inline]
            fn run(&self) {
                let $values0 = &self.cache;
                let $values0: $container0<'_, $args0> = $crate::types::ResourceContainer::<'_, $container0<'_, $args0>>::to_container($values0);
                $body
            }
        }
    };

    ($system:ident {
        fn run( $values0:ident: $container0:ident<$args0:ty>, $($values:ident: $container:ident<$args:ty>),+ ) $body:block
    }) => {
        pub struct $system {
            pub name: &'static str,
            pub cache: impl_system_for!(expand; $args0, $($args),+),
        }

        impl $crate::types::System for $system {
            #[inline]
            fn new<'a>(component_data: &$crate::types::ComponentData) -> $system {
                $system {
                    name: stringify!($system),
                    cache: (component_data.clone_component_data(&<$container0<'a, $args0> as $crate::types::InnerType<'a>>::to_inner_type()),
                          $(component_data.clone_component_data(&<$container <'a, $args > as $crate::types::InnerType<'a>>::to_inner_type())),+),
                }
            }

            #[inline]
            fn run(&self) {
                let ($values0, $($values),+) = &self.cache;
                let ($values0, $($values),+): ($container0<'_, $args0>, $($container<'_, $args>),+) =
                                                ($crate::types::ResourceContainer::<'_, $container0<'_, $args0>>::to_container($values0),
                                                $($crate::types::ResourceContainer::<'_, $container<'_, $args>>::to_container($values)),+);
                $body
            }
        }
    };

    (expand; $head:ty, $($tail:ty),+) => {
        (std::rc::Rc<std::cell::RefCell<Box<$crate::data_types::Resource>>>, impl_system_for!(expand; $($tail),+))
    };

    (expand; $head:ty) => {
        std::rc::Rc<std::cell::RefCell<Box<$crate::data_types::Resource>>>
    };
}
