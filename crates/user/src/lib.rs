pub mod show_streams;

pub mod derive_macro {
    use std::sync::Arc;

    use macros::TouchUpdateEvent;
    pub struct TouchUpdate {
        pub x: f32,
        pub y: f32,
    }

    pub struct TouchEvent(pub Arc<TouchUpdate>);

    #[derive(TouchUpdateEvent)]
    pub struct TouchMove(pub Arc<TouchUpdate>);

    #[test]
    fn touch_event() {
        let update = Arc::new(TouchUpdate { x: 1.0, y: 2.0 });
        let evt = TouchEvent(update.clone());

        let move_evt = TouchMove::from(&evt);
        println!("coords = {}, {}", move_evt.x, move_evt.y);

        let cloned = move_evt.clone();
        println!("cloned ok: {}", Arc::ptr_eq(&cloned, &move_evt));
    }
}

pub mod declarative_macro {
    // Declarative macros can't be in the same crate as procedural macros.
    // See https://doc.rust-lang.org/reference/procedural-macros.html#
    // But you could make those in a different crate.
    #[macro_export]
    macro_rules! define_event_with_constructor {
        ($name:ident, $inner:ty, $custom_new:expr) => {
            pub struct $name(pub Arc<$inner>);

            impl $name {
                pub fn new(internal: Arc<$inner>) -> Self {
                    // Insert user-provided code block.
                    let internal = $custom_new(internal);
                    $name(internal)
                }
            }
            impl std::ops::Deref for $name {
                type Target = Arc<$inner>;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl Clone for $name {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }

            impl From<&TouchEvent> for $name {
                fn from(value: &TouchEvent) -> Self {
                    $name(value.0.clone())
                }
            }

            impl $name {
                pub fn from_unchecked(value: &TouchEvent) -> Self {
                    $name(value.0.clone())
                }
            }
            // Add your other impls or derives here.
        };
        // Case without custom constructor code (original behavior).
        ($name:ident, $inner:ty) => {
            $crate::define_event_with_constructor!($name, $inner, |internal| { internal });
        };
    }
    use std::sync::Arc;

    #[derive(Debug)]
    pub struct TouchUpdate {
        pub x: f32,
        pub y: f32,
    }

    pub struct TouchEvent(pub Arc<TouchUpdate>);

    define_event_with_constructor!(
        TouchMove,
        TouchUpdate,
        // Custom constructor logic can go here.
        |internal| {
            println!("Creating TouchMove event: {:?}", internal);
            internal
        }
    );

    #[test]
    fn touch_event() {
        let update = Arc::new(TouchUpdate { x: 1.0, y: 2.0 });
        let evt = TouchEvent(update.clone());

        let move_evt = TouchMove::from(&evt);
        println!("coords = {}, {}", move_evt.x, move_evt.y);

        let cloned = move_evt.clone();
        println!("cloned ok: {}", Arc::ptr_eq(&cloned, &move_evt));
    }
}
