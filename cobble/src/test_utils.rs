#[cfg(test)]
pub mod tests {
    use bevy::app::App;
    use bevy::prelude::*;

    pub fn assert_state_fn<T: States>(app: &mut App, state: T) {
        app.world
            .resource_scope(|_w, current_state: Mut<State<T>>| {
                assert_eq!(*current_state, state);
            });
    }

    macro_rules! assert_state {
        ($left:expr, $right:expr) => {
            assert_state_fn($left, $right);
        };
    }
    pub(crate) use assert_state;

    pub fn set_next_state<T: States>(app: &mut App, state: T) {
        app.world
            .resource_scope(|_w, mut next_state: Mut<NextState<T>>| {
                next_state.set(state);
            });
    }
}
