pub struct Editor {
    // ... 其他字段 ...
    zen_mode: bool,
}

impl Editor {
    // ... 其他方法 ...
    
    pub fn toggle_zen_mode(&mut self) {
        self.zen_mode = !self.zen_mode;
        self.refresh_view();
    }
    
    pub fn is_zen_mode(&self) -> bool {
        self.zen_mode
    }
} 