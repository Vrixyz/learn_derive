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
