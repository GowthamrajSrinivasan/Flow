use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};

pub struct VoiceFlowHotKeyManager {
    manager: GlobalHotKeyManager,
    main_hotkey: HotKey,
    cancel_hotkey: HotKey,
}

impl VoiceFlowHotKeyManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manager = GlobalHotKeyManager::new()?;
        
        // Main hotkey: Alt + Space
        let main_hotkey = HotKey::new(Some(Modifiers::ALT), Code::Space);
        manager.register(main_hotkey)?;

        // Cancel hotkey: Escape (registered/unregistered dynamically)
        let cancel_hotkey = HotKey::new(None, Code::Escape);

        Ok(Self {
            manager,
            main_hotkey,
            cancel_hotkey,
        })
    }

    pub fn main_hotkey_id(&self) -> u32 {
        self.main_hotkey.id()
    }

    pub fn cancel_hotkey_id(&self) -> u32 {
        self.cancel_hotkey.id()
    }

    pub fn register_cancel(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.manager.register(self.cancel_hotkey)?;
        Ok(())
    }

    pub fn unregister_cancel(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.manager.unregister(self.cancel_hotkey)?;
        Ok(())
    }

    pub fn unregister_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.manager.unregister(self.main_hotkey);
        let _ = self.manager.unregister(self.cancel_hotkey);
        Ok(())
    }
}

