use bevy::prelude::*;

use crate::resources::{AsciiFrame, AsciiGrid};
use crate::upgrades::UpgradeSelectionState;

pub fn render_upgrade_selection(
    mut frame: ResMut<AsciiFrame>,
    grid: Res<AsciiGrid>,
    selection: Option<Res<UpgradeSelectionState>>,
) {
    let Some(selection) = selection else {
        return;
    };

    frame.clear();

    let center_x = grid.grid_size.x as i32 / 2;
    let grid_height = grid.grid_size.y as i32;

    let title = "CHOOSE AN UPGRADE";
    let title_x = center_x - title.len() as i32 / 2;
    frame.put_string(
        IVec2::new(title_x, 3),
        title,
        Color::linear_rgb(1.0, 1.0, 0.0),
        Color::NONE,
    );

    let divider: String = "-".repeat(40);
    let div_x = center_x - 20;
    frame.put_string(
        IVec2::new(div_x, 5),
        &divider,
        Color::linear_rgb(0.3, 0.3, 0.3),
        Color::NONE,
    );

    if selection.options.is_empty() {
        let msg = "No upgrades available!";
        let msg_x = center_x - msg.len() as i32 / 2;
        frame.put_string(IVec2::new(msg_x, 10), msg, Color::WHITE, Color::NONE);
        return;
    }

    let option_height = 4;
    let total_height = selection.options.len() as i32 * option_height;
    let start_y = (grid_height - total_height) / 2 - 1;

    for (i, upgrade) in selection.options.iter().enumerate() {
        let y = start_y + (i as i32 * option_height);
        let is_selected = i == selection.selected_index;

        let rarity_color = upgrade.rarity.color();

        let cursor = if is_selected { "> " } else { "  " };
        let name_color = if is_selected {
            Color::linear_rgb(1.0, 1.0, 0.0)
        } else {
            Color::WHITE
        };

        let name_line = format!("{}{}", cursor, upgrade.name);
        frame.put_string(IVec2::new(div_x, y), &name_line, name_color, Color::NONE);

        let rarity_str = match &upgrade.rarity {
            crate::upgrades::UpgradeRarity::Common => "Common",
            crate::upgrades::UpgradeRarity::Uncommon => "Uncommon",
            crate::upgrades::UpgradeRarity::Rare => "Rare",
            crate::upgrades::UpgradeRarity::Epic => "Epic",
        };

        let rarity_x = div_x + name_line.len() as i32 + 1;
        frame.put_string(
            IVec2::new(rarity_x, y),
            rarity_str,
            rarity_color,
            Color::NONE,
        );

        let desc_line = format!("    {}", upgrade.description);
        frame.put_string(
            IVec2::new(div_x, y + 1),
            &desc_line,
            Color::linear_rgb(0.7, 0.7, 0.7),
            Color::NONE,
        );

        if i < selection.options.len() - 1 {
            let sep: String = ".".repeat(40);
            frame.put_string(
                IVec2::new(div_x, y + 3),
                &sep,
                Color::linear_rgb(0.2, 0.2, 0.2),
                Color::NONE,
            );
        }
    }

    let hint = "W/S or Up/Down to navigate, Enter to select";
    let hint_x = center_x - hint.len() as i32 / 2;
    frame.put_string(
        IVec2::new(hint_x, grid_height - 3),
        hint,
        Color::linear_rgb(0.4, 0.4, 0.4),
        Color::NONE,
    );
}
