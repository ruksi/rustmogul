use bevy::prelude::*;

pub fn menu() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Percent(35.0),
            height: Val::Percent(18.0),
            min_width: Val::Px(140.0),
            min_height: Val::Px(32.0),
            margin: UiRect::bottom(Val::Px(4.0)),
            ..super::basic().style
        },
        ..super::basic()
    }
}
