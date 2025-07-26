use crate::prelude::*;
#[cfg(target_arch = "wasm32")]
use crate::save_manager::{LocalStorageReader, LocalStorageWriter};

pub const FILE_SETTING_SAVE: &str = "setting.ron";

pub struct SettingSaveManagerPlugin;

impl Plugin for SettingSaveManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(save_on::<SettingSaveEvent>)
            .add_observer(load_on_default_event)
            .add_systems(Startup, loading_settings)
            .add_systems(Update, save_settings);
    }
}

struct SettingSaveEvent {
    pub path: String,
    #[cfg(target_arch = "wasm32")]
    pub stream: LocalStorageWriter,
}

impl SingleEvent for SettingSaveEvent {}

impl SaveEvent for SettingSaveEvent {
    type SaveFilter = ();

    fn filter_entity(&self, _: Entity) -> bool {
        false
    }

    fn before_serialize(&mut self, _: EntityWorldMut) {
        println!("before_serialize");
    }

    fn after_serialize(&mut self, _: EntityWorldMut) {
        println!("after_serialize");
    }

    fn component_filter(&self) -> SceneFilter {
        println!("component_filter");
        SceneFilter::default()
    }

    fn resource_filter(&self) -> SceneFilter {
        SceneFilter::deny_all().allow::<GlobalVolume>()
    }

    fn output(self) -> SaveOutput {
        #[cfg(target_arch = "wasm32")]
        {
            SaveOutput::stream(self.stream)
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            SaveOutput::file(self.path)
        }
    }
}

fn save_settings(
    mut save_event: EventReader<SaveSettingEvent>,
    mut commands: Commands,
    global_volume: Res<GlobalVolume>,
) {
    for ev in save_event.read() {
        println!("save event: {:?} global volume {:?}", ev, global_volume.0);
        #[cfg(target_arch = "wasm32")]
        let writer = LocalStorageWriter {
            key: FILE_SETTING_SAVE.to_string(),
            buffer: Vec::new(),
        };
        commands.trigger_save(SettingSaveEvent {
            #[cfg(target_arch = "wasm32")]
            stream: writer,
            path: FILE_SETTING_SAVE.to_string(),
        });
    }
}

fn loading_settings(mut commands: Commands) {
    #[cfg(target_arch = "wasm32")]
    {
        let reader = LocalStorageReader::new(FILE_SETTING_SAVE.to_string());
        if reader.data.is_empty() {
            return;
        }
        commands.trigger_load(LoadWorld::default_from_stream(reader));
        return;
    }
    commands.trigger_load(LoadWorld::default_from_file(FILE_SETTING_SAVE));
}
