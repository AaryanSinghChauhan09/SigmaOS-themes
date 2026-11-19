use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ThemeVariant { Light, Dark, Auto }

#[derive(Debug, Clone)]
pub struct AccentColor { pub r: u8, pub g: u8, pub b: u8 }
impl AccentColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
    pub fn hex(&self) -> String { format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b) }
    pub fn sigma_blue() -> Self { Self::new(0x40, 0x7B, 0xFF) }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub variant: ThemeVariant,
    pub accent: AccentColor,
    pub font_family: String,
    pub icon_theme: String,
    pub cursor_theme: String,
}
impl Theme {
    pub fn sigma_default() -> Self {
        Self {
            name: "SigmaOS Default".to_string(),
            variant: ThemeVariant::Light,
            accent: AccentColor::sigma_blue(),
            font_family: "Inter".to_string(),
            icon_theme: "sigma-icons".to_string(),
            cursor_theme: "sigma-cursors".to_string(),
        }
    }
}

pub struct ThemeRegistry { themes: HashMap<String, Theme>, active: String }
impl ThemeRegistry {
    pub fn new() -> Self {
        let default = Theme::sigma_default();
        let name = default.name.clone();
        let mut reg = Self { themes: HashMap::new(), active: name.clone() };
        reg.themes.insert(name, default);
        reg
    }
    pub fn register(&mut self, theme: Theme) { self.themes.insert(theme.name.clone(), theme); }
    pub fn set_active(&mut self, name: &str) -> Result<(), String> {
        if self.themes.contains_key(name) { self.active = name.to_string(); Ok(()) }
        else { Err(format!("theme '{}' not found", name)) }
    }
    pub fn active_theme(&self) -> Option<&Theme> { self.themes.get(&self.active) }
    pub fn count(&self) -> usize { self.themes.len() }
}
impl Default for ThemeRegistry { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accent_color_hex() {
        let c = AccentColor::new(0x12, 0x34, 0x56);
        assert_eq!(c.hex(), "#123456");
    }

    #[test]
    fn test_sigma_blue() {
        let c = AccentColor::sigma_blue();
        assert_eq!(c.hex(), "#407BFF");
    }

    #[test]
    fn test_theme_variants() {
        let variants = vec![ThemeVariant::Light, ThemeVariant::Dark, ThemeVariant::Auto];
        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn test_registry_new_has_default() {
        let reg = ThemeRegistry::new();
        assert_eq!(reg.count(), 1);
        assert!(reg.active_theme().is_some());
    }

    #[test]
    fn test_registry_register_and_activate() {
        let mut reg = ThemeRegistry::new();
        let theme = Theme::sigma_default();
        let name = theme.name.clone();
        reg.register(theme);
        assert!(reg.set_active(&name).is_ok());
    }

    #[test]
    fn test_registry_activate_missing_fails() {
        let mut reg = ThemeRegistry::new();
        assert!(reg.set_active("nonexistent").is_err());
    }
}
