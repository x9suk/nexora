use zed_extension_api::{self as zed, Result};

struct NexoraExtension;

impl zed::Extension for NexoraExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _config: zed::LanguageServer,
    ) -> Result<zed::Command> {
        Err("No language server configured".into())
    }
}

zed::register_extension!(NexoraExtension);
