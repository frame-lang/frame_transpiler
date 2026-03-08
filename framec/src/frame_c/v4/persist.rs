// Frame v4 Persistence Module
// Handles @@persist annotation for automatic state persistence

use crate::frame_c::visitors::TargetLanguage;

pub struct PersistenceGenerator;

impl PersistenceGenerator {
    /// Generate persistence code for the given target language
    pub fn generate_save_code(lang: TargetLanguage, class_name: &str) -> String {
        match lang {
            TargetLanguage::Python3 => {
                format!(r#"
    def save_state(self, filename: str = "{}_state.pkl"):
        """Save the current state to a file."""
        import pickle
        with open(filename, 'wb') as f:
            pickle.dump(self.__dict__, f)
    
    def load_state(self, filename: str = "{}_state.pkl"):
        """Load state from a file."""
        import pickle
        with open(filename, 'rb') as f:
            self.__dict__.update(pickle.load(f))
"#, class_name.to_lowercase(), class_name.to_lowercase())
            }
            TargetLanguage::TypeScript => {
                format!(r#"
    saveState(filename: string = "{}_state.json"): void {{
        const fs = require('fs');
        fs.writeFileSync(filename, JSON.stringify(this));
    }}
    
    loadState(filename: string = "{}_state.json"): void {{
        const fs = require('fs');
        const data = JSON.parse(fs.readFileSync(filename, 'utf8'));
        Object.assign(this, data);
    }}
"#, class_name.to_lowercase(), class_name.to_lowercase())
            }
            _ => {
                // Other languages not yet implemented
                String::from("// @@persist not yet implemented for this language\n")
            }
        }
    }
    
    /// Check if a module has @@persist annotation
    pub fn has_persist_annotation(source: &[u8]) -> bool {
        // Simple check for @@persist
        let source_str = String::from_utf8_lossy(source);
        source_str.contains("@@persist")
    }
}