use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct SetVisibilitySoon {
    #[deref]
    pub timer: Timer,
    pub visibility: Visibility,
}

pub fn tick_set_visibility_timers(
    time: Res<Time>,
    mut query: Query<(&mut SetVisibilitySoon, &mut Visibility)>,
) {
    for (mut soon, mut visibility) in &mut query {
        if soon.timer.tick(time.delta()).just_finished() {
            *visibility = soon.visibility;
        }
    }
}
