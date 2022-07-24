use bevy::prelude::*;

use crate::{
    game::GameState,
    loading::FontAssets,
    menu::{ButtonColours, UiCamera},
    settings,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColours>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(Self::setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::Menu).with_system(Self::click_play_button),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Menu).with_system(Self::despawn_menu_camera),
            );
    }
}

impl MenuPlugin {
    fn setup_menu(
        mut commands: Commands,
        font_assets: Res<FontAssets>,
        button_colors: Res<ButtonColours>,
    ) {
        let ui_camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();
        commands.insert_resource(UiCamera {
            entity: ui_camera_entity,
        });

        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: button_colors.normal,
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Start".to_string(),
                            style: TextStyle {
                                font: font_assets.raleway.clone(),
                                font_size: 40.0,
                                color: Color::hex(settings::BUTTON_TEXT_COL.split_at(1).1).unwrap(),
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..default()
                });
            });
    }

    fn click_play_button(
        mut commands: Commands,
        button_colors: Res<ButtonColours>,
        mut state: ResMut<State<GameState>>,
        mut interaction_query: Query<
            (Entity, &Interaction, &mut UiColor),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (button, interaction, mut col) in interaction_query.iter_mut() {
            match *interaction {
                Interaction::Clicked => {
                    commands.entity(button).despawn_recursive();
                    state.set(GameState::Playing).unwrap();
                }
                Interaction::Hovered => {
                    *col = button_colors.hovered;
                }
                Interaction::None => {
                    *col = button_colors.normal;
                }
            }
        }
    }

    fn despawn_menu_camera(mut commands: Commands, ui_camera: Res<UiCamera>) {
        commands.entity(ui_camera.entity).despawn_recursive();
    }
}
