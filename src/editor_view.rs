impl EditorView {
    fn render(&mut self) {
        // ... 现有代码 ...
        
        if self.editor.is_zen_mode() {
            // 隐藏行号
            self.show_line_numbers = false;
            
            // 设置文本居中
            let padding = (self.width - text_width) / 2;
            self.content_offset.x = padding;
            
            // 隐藏UI元素
            self.show_status_bar = false;
            self.show_gutter = false;
            self.show_minimap = false;
        }
    }
} 