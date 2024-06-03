use std::{collections::BTreeMap, path::PathBuf};
use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    permission_granted: bool,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
        ]);
        subscribe(&[EventType::PermissionRequestResult]);

        if self.permission_granted {
            hide_self();
        }
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        let session_name = pipe_message.payload.unwrap().to_string();
        let collection: Vec<&str> = session_name.split("::").collect::<Vec<&str>>().clone();
        let session_name = collection[0];
        let cwd = Some(PathBuf::from(collection[1]));
        let layout_name = "default";
        let layout: LayoutInfo = LayoutInfo::File(layout_name.to_string());
        switch_session_with_layout(Some(&session_name), layout, cwd);
        true
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(permission) => {
                self.permission_granted = match permission {
                    PermissionStatus::Granted => true,
                    PermissionStatus::Denied => false,
                };
                if self.permission_granted {
                    hide_self();
                }
            }
            _ => {}
        }
        false
    }
}
