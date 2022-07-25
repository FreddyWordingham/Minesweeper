use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{game::GameState, loading::FontAssets};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Menu)
                .with_system(Self::click_play_button)
                .into(),
        )
        .add_enter_system(GameState::Menu, Self::spawn_menu)
        .add_exit_system(GameState::Menu, Self::despawn_menu);
    }
}

pub struct MenuData {
    pub play_button: Entity,
}

impl MenuPlugin {
    pub fn spawn_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
        let play_button = commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: bevy::prelude::UiColor(Color::GRAY),
                ..default()
            })
            .insert(Name::new("Menu"))
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Start".to_string(),
                            style: TextStyle {
                                font: font_assets.raleway.clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..default()
                });
            })
            .id();
        commands.insert_resource(MenuData { play_button });
    }

    pub fn despawn_menu(mut commands: Commands, menu_data: Res<MenuData>) {
        commands.entity(menu_data.play_button).despawn_recursive();
    }

    fn click_play_button(
        mut commands: Commands,
        mut interaction_query: Query<
            (Entity, &Interaction, &mut UiColor),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (button, interaction, mut color) in interaction_query.iter_mut() {
            match *interaction {
                Interaction::Clicked => commands.insert_resource(NextState(GameState::Playing)),
                Interaction::Hovered => {
                    *color = bevy::prelude::UiColor(Color::PINK);
                }
                Interaction::None => {
                    *color = bevy::prelude::UiColor(Color::BLUE);
                }
            }
        }
    }
}
