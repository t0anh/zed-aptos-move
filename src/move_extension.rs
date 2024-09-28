use zed_extension_api as zed;

struct AptosMoveExtension;

impl AptosMoveExtension {
    fn language_server_binary_path(
        &mut self,
        _lang_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        worktree
            .which("aptos-move-analyzer")
            .ok_or("aptos-move-analyzer executable not found".to_string())
    }
}

impl zed::Extension for AptosMoveExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        lang_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(lang_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(AptosMoveExtension);
