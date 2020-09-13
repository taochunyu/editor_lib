use druid::{Data, LocalizedString, MenuDesc, MenuItem, RawMods};
use druid::commands;

pub fn macos_application_menu<T: Data>() -> MenuDesc<T> {
    MenuDesc::new(LocalizedString::new("macos-menu-application-menu"))
        .append(MenuItem::new(
            LocalizedString::new("macos-menu-about-app"),
            commands::SHOW_ABOUT,
        ))
        .append_separator()
        .append(
            MenuItem::new(
                LocalizedString::new("macos-menu-preferences"),
                commands::SHOW_PREFERENCES,
            )
            .hotkey(RawMods::Meta, ",")
            .disabled(),
        )
        .append_separator()
        .append(MenuDesc::new(LocalizedString::new("macos-menu-service")))
        .append(
            MenuItem::new(
                LocalizedString::new("macos-menu-hide-app"),
                commands::HIDE_APPLICATION,
            )
            .hotkey(RawMods::Meta, "h"),
        )
        .append(
            MenuItem::new(
                LocalizedString::new("macos-menu-hide-others"),
                commands::HIDE_OTHERS,
            )
            .hotkey(RawMods::AltMeta, "h"),
        )
        .append(
            MenuItem::new(
                LocalizedString::new("macos-menu-show-all"),
                commands::SHOW_ALL,
            )
            .disabled(),
        )
        .append_separator()
        .append(
            MenuItem::new(
                LocalizedString::new("macos-menu-quit-app"),
                commands::QUIT_APP,
            )
            .hotkey(RawMods::Meta, "q"),
        )
}
