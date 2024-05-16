use anyhow::{Context, Result};
use hyprland::{
    config::binds::{Key, Mod},
    data::Binds,
    shared::HyprData,
};

use crate::utils::wlr::*;

pub mod wlr {
    #![allow(dead_code)]
    //! wlr stuff because i refuse to import the crate just for this
    pub const WLR_MODIFIER_SHIFT: u16 = 1 << 0;
    pub const WLR_MODIFIER_CAPS: u16 = 1 << 1;
    pub const WLR_MODIFIER_CTRL: u16 = 1 << 2;
    pub const WLR_MODIFIER_ALT: u16 = 1 << 3;
    pub const WLR_MODIFIER_MOD2: u16 = 1 << 4;
    pub const WLR_MODIFIER_MOD3: u16 = 1 << 5;
    pub const WLR_MODIFIER_LOGO: u16 = 1 << 6;
    pub const WLR_MODIFIER_MOD5: u16 = 1 << 7;
}

/// Prepend the prefix hyprrdrop- to workspace names
pub fn prepend_workspace_prefix(name: &str) -> String {
    const WORKSPACE_PREFIX: &str = "hyprrdrop";
    format!("{WORKSPACE_PREFIX}-{name}")
}

/// Parse a string keybind in the form <Mod>,<Key>
///
/// # Returns a tuple of Vec<Mod>, mod_mask and Key
pub fn parse_keybind(keybind: &str) -> Result<(Vec<Mod>, u16, Key<'_>)> {
    let (mods, key) = keybind
        .split_once(',')
        .context("splitting string keybind into mods and key")?;

    // uppercase and only check once
    let mods = mods.to_uppercase();
    let key = key.trim();

    let mut mods_vec = vec![];
    // hyprland uses u32 instead
    let mut mod_mask: u16 = 0;

    // this follows hyprland's processing flow from 121d3a72137d4780602cf245704615f63357ea22
    if mods.contains("SHIFT") {
        mods_vec.push(Mod::SHIFT);
        mod_mask |= WLR_MODIFIER_SHIFT;
    }
    // TODO: handle CAPS when supported by hyprland-rs
    if mods.contains("CTRL") || mods.contains("CONTROL") {
        mods_vec.push(Mod::CTRL);
        mod_mask |= WLR_MODIFIER_CTRL;
    }
    if mods.contains("ALT") || mods.contains("MOD1") {
        mods_vec.push(Mod::ALT);
        mod_mask |= WLR_MODIFIER_ALT;
    }
    if mods.contains("SUPER")
        || mods.contains("MOD4")
        || mods.contains("WIN")
        || mods.contains("LOGO")
    {
        mods_vec.push(Mod::SUPER);
        mod_mask |= WLR_MODIFIER_LOGO;
    }
    // TODO: handle other modkeys when supported by hyprland-rs
    let key = Key::Key(key);

    Ok((mods_vec, mod_mask, key))
}

/// Check hyprland for any keybinds matching query
pub fn check_if_bound(keybind: &str) -> Result<bool> {
    let (_, check_mod_mask, check_key) = parse_keybind(keybind)?;
    Ok(Binds::get()?
        .into_iter()
        .any(|e| e.modmask == check_mod_mask && e.key == check_key.to_string()))
}

mod tests {

    #[test]
    fn test_parse_keybinds() {
        use super::*;

        let keybinds = [
            "SUPERCTRLSHIFTALT, 3",
            "SUPERSHIFT, H",
            "ALT + CTRL, 3",
            "SUPERALT + CONTROL, L",
            "SHIFT,2",
        ];
        let results = [
            (
                vec![Mod::SHIFT, Mod::CTRL, Mod::ALT, Mod::SUPER],
                77_u16,
                Key::Key("3"),
            ),
            (vec![Mod::SHIFT, Mod::SUPER], 65_u16, Key::Key("H")),
            (vec![Mod::CTRL, Mod::ALT], 12_u16, Key::Key("3")),
            (vec![Mod::CTRL, Mod::ALT, Mod::SUPER], 76, Key::Key("L")),
            (vec![Mod::SHIFT], 1, Key::Key("2")),
        ];

        for (index, keybind) in keybinds.iter().enumerate() {
            let parsed = parse_keybind(keybind).unwrap();
            let result = &results[index];

            assert_eq!(&parsed, result);
        }
    }
}
