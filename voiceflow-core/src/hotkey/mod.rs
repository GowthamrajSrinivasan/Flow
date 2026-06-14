use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};

pub struct VoiceFlowHotKeyManager {
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
}

impl VoiceFlowHotKeyManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manager = GlobalHotKeyManager::new()?;
        
        // Default hotkey: Ctrl + Shift + Space
        let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Space);
        manager.register(hotkey)?;

        Ok(Self { manager, hotkey })
    }

    pub fn unregister(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.manager.unregister(self.hotkey)?;
        Ok(())
    }
}
