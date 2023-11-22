use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

// This pointer event is used for _both_ UI and game world interaction.

#[allow(dead_code)]
#[rustfmt::skip]
#[derive(Debug, Event)]
pub enum PointerEvent {
    OnOver { target: Entity },
    OnOut { target: Entity },
    OnDown { target: Entity, button: PointerButton },
    OnUp { target: Entity, button: PointerButton },
    OnClick { target: Entity, button: PointerButton },
    OnMove { target: Entity, delta: Vec2 },
    OnDragStart { target: Entity },
    OnDrag { target: Entity, delta: Vec2, distance: Vec2 },
    OnDragEnd { target: Entity },
    OnDragEnter { target: Entity, dragged: Entity },
    OnDragLeave { target: Entity, dragged: Entity },
    OnDragOver { target: Entity, dragged: Entity },
    OnDrop { target: Entity, dropped: Entity },
}

impl From<ListenerInput<Pointer<Over>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        PointerEvent::OnOver { target: event.target }
    }
}

impl From<ListenerInput<Pointer<Out>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        PointerEvent::OnOut { target: event.target }
    }
}

impl From<ListenerInput<Pointer<Down>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        PointerEvent::OnDown { target: event.target, button: event.button }
    }
}

impl From<ListenerInput<Pointer<Up>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<Up>>) -> Self {
        PointerEvent::OnUp { target: event.target, button: event.button }
    }
}

impl From<ListenerInput<Pointer<Click>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        PointerEvent::OnClick { target: event.target, button: event.button }
    }
}

impl From<ListenerInput<Pointer<Move>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<Move>>) -> Self {
        PointerEvent::OnMove { target: event.target, delta: event.delta }
    }
}

impl From<ListenerInput<Pointer<DragStart>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<DragStart>>) -> Self {
        PointerEvent::OnDragStart { target: event.target }
    }
}

impl From<ListenerInput<Pointer<Drag>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<Drag>>) -> Self {
        PointerEvent::OnDrag {
            target: event.target,
            delta: event.delta,
            distance: event.distance,
        }
    }
}

impl From<ListenerInput<Pointer<DragEnd>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<DragEnd>>) -> Self {
        PointerEvent::OnDragEnd { target: event.target }
    }
}

impl From<ListenerInput<Pointer<DragEnter>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<DragEnter>>) -> Self {
        PointerEvent::OnDragEnter { target: event.target, dragged: event.dragged }
    }
}

impl From<ListenerInput<Pointer<DragOver>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<DragOver>>) -> Self {
        PointerEvent::OnDragOver { target: event.target, dragged: event.dragged }
    }
}

impl From<ListenerInput<Pointer<DragLeave>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<DragLeave>>) -> Self {
        PointerEvent::OnDragLeave { target: event.target, dragged: event.dragged }
    }
}

impl From<ListenerInput<Pointer<Drop>>> for PointerEvent {
    fn from(event: ListenerInput<Pointer<Drop>>) -> Self {
        PointerEvent::OnDrop { target: event.target, dropped: event.dropped }
    }
}

/// Usage:
///
/// ```
/// app.add_systems(
///     Update,
///     (pointer_event::debug_pointer_event).run_if(on_event::<PointerEvent>()),
/// );
/// ```
#[allow(dead_code)]
pub fn debug_pointer_event(mut events: EventReader<PointerEvent>) {
    for event in events.iter() {
        eprintln!("PointerEvent::{:?}", event);
    }
}
