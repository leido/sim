use crate::car_dynamics::{EgoControl, EgoState};
use bevy::prelude::*;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SoundState>()
            .add_systems(Startup, load_sound_asset)
            .add_systems(OnEnter(SoundState::THROTTLE), trigger_throttle_sound)
            .add_systems(OnEnter(SoundState::BRAKE), trigger_brake_sound)
            .add_systems(Update, change_sound_state);
    }
}

#[derive(Resource)]
struct SoundTrack {
    brake: Handle<AudioSource>,
    throttle: Handle<AudioSource>,
}

#[derive(States, Clone, Debug, Hash, Default, PartialEq, Eq)]
enum SoundState {
    #[default]
    NORMAL,
    THROTTLE,
    BRAKE,
}

const BRAKE_SOUND_SPEED: f32 = 5.0;

fn load_sound_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundTrack {
        brake: asset_server.load("brake.mp3"),
        throttle: asset_server.load("throttle.mp3"),
    })
}

fn change_sound_state(
    ego_control: Single<&EgoControl>,
    ego_state: Single<&EgoState>,
    mut next_sound_state: ResMut<NextState<SoundState>>,
) {
    if ego_control.throttle > 0.5 {
        next_sound_state.set(SoundState::THROTTLE);
    } else if ego_control.brake > 0.5 && ego_state.v > BRAKE_SOUND_SPEED {
        next_sound_state.set(SoundState::BRAKE);
    } else {
        next_sound_state.set(SoundState::NORMAL);
    }
}

fn trigger_throttle_sound(
    mut commands: Commands,
    sound_track: Res<SoundTrack>,
    current_playing: Query<&AudioSink>,
) {
    for sound in current_playing {
        sound.stop();
    }
    commands.spawn((
        AudioPlayer(sound_track.throttle.clone()),
        PlaybackSettings::DESPAWN,
    ));
}
fn trigger_brake_sound(
    mut commands: Commands,
    sound_track: Res<SoundTrack>,
    current_playing: Query<&AudioSink>,
) {
    for sound in current_playing {
        sound.stop();
    }
    commands.spawn((
        AudioPlayer(sound_track.brake.clone()),
        PlaybackSettings::DESPAWN,
    ));
}
