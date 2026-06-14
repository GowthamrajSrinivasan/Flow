#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

pub trait TextInjector {
    fn inject(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(target_os = "macos")]
pub fn get_injector() -> Result<Box<dyn TextInjector>, Box<dyn std::error::Error>> {
    Ok(Box::new(macos::MacOsTextInjector::new()?))
}

#[cfg(target_os = "windows")]
pub fn get_injector() -> Result<Box<dyn TextInjector>, Box<dyn std::error::Error>> {
    Ok(Box::new(windows::WindowsTextInjector::new()?))
}
