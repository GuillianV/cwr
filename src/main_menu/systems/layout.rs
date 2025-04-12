use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::{styles::*, systems::interactions::*};

pub fn spawn_layout(commands: Commands, ) {//asset_server: Res<AssetServer>
    build_layout(commands,);// &asset_server
}

pub fn despawn_layout(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_layout(mut commands: Commands, ) {//asset_server: &Res<AssetServer>
    let container_node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };

    let button_node = Node {
        width: Val::Px(150.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(2.0)),
        padding: UiRect {
            left: Val::Px(30.0),
            right: Val::Px(30.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_node = Text::new("Play");
    let button_text_color = TextColor(Color::srgb(0.9, 0.9, 0.9));
    let button_text_font = TextFont {
        font_size: 30.0,
        ..default()
    };

    let container = commands
        .spawn((container_node, BackgroundColor(NORMAL_BUTTON), MainMenu {}))
        .id();
    let button = commands
        .spawn((
            button_node,
            BorderColor(Color::BLACK),
            BorderRadius::all(Val::Px(10.0)),
            BoxShadow {
                spread_radius: Val::Px(2.0),
                blur_radius: Val::Px(2.0),
                color: Color::BLACK,
                x_offset: Val::Px(1.0),
                y_offset: Val::Px(1.0),
            },
            BackgroundColor(NORMAL_BUTTON),
            PlayButton {},
        ))
        .observe(on_click)
        .observe(on_hover)
        .observe(on_hover_out)
        .id();

    let button_text = commands
        .spawn((button_text_node, button_text_color, button_text_font))
        .id();
    commands.entity(button).add_children(&[button_text]);
    commands.entity(container).add_children(&[button]);
}
