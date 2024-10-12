use rustysynth::Synthesizer;

use crate::{
    midi::messages::ChannelMessage,
    midifile::{
        miditrack::{midievent::MidiEvent, MidiTrack, MidiTrackEvent},
        MidiFile,
    },
};

/// Turn MIDI files and soundfont into audio samples.
///
/// # Examples
///
/// ```
/// // Minimal example for getting samples out of a sequencer:
/// // Notice that the sequencer depends on RustySynth for generating audio.
///
/// use crustysynth::{midifile::MidiFile, sequencer::MidiSequencer};
/// use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
/// use std::{fs::File, sync::Arc};
///
/// let mut font_file = File::open("../../samples/Neo1MGM.sf2").unwrap();
/// let font = Arc::new(SoundFont::new(&mut font_file).unwrap());
/// let midi_file = File::open("../../samples/salsa.mid").unwrap();
/// let midi = MidiFile::try_from(midi_file).unwrap();
///
/// let settings = SynthesizerSettings::new(44100);
/// let synthesizer = Synthesizer::new(&font, &settings).unwrap();
/// let mut sequencer = MidiSequencer::new(synthesizer);
/// sequencer.play_midi_file(midi);
///
/// // The sequencer will return Some until it runs out.
/// let sample: [f32; 2] = sequencer.render().unwrap();
///
/// ```
pub struct MidiSequencer {
    midi_file: Option<MidiFile>,
    synthesizer: Synthesizer,
    /// Next event on each track.
    tracks: Vec<TrackSequencer>,
    /// Number of samples since last division.
    delta_samples: usize,
    bpm: f64,
}

impl MidiSequencer {
    /// # Arguments
    /// * `synthesizer` - A RustySynth [Synthesizer](https://docs.rs/rustysynth/1.3.2/rustysynth/struct.Synthesizer.html)
    ///
    pub fn new(synthesizer: Synthesizer) -> Self {
        Self {
            midi_file: None,
            synthesizer,
            tracks: vec![],
            delta_samples: 0,
            bpm: 120.0,
        }
    }

    pub fn play_midi_file(&mut self, midi_file: MidiFile) {
        self.tracks.clear();
        for track in midi_file.get_tracks() {
            self.tracks.push(TrackSequencer::new(track.clone()));
        }

        self.midi_file = Some(midi_file);

        self.synthesizer.reset();
    }

    pub fn render(&mut self) -> Option<[f32; 2]> {
        let Some(midi) = &self.midi_file else {
            return None;
        };
        let mut quit = true;
        for track in &self.tracks {
            quit &= track.is_finished();
        }
        if quit {
            return None;
        }

        let samplerate = self.synthesizer.get_sample_rate();
        let tick_duration = midi.get_division().get_tick_duration(self.bpm);

        let tick_samples = (tick_duration.as_secs_f64() * samplerate as f64) as usize;
        if self.delta_samples == tick_samples {
            self.delta_samples = 0;
            for i in 0..self.tracks.len() {
                let track = &mut self.tracks[i];
                let events = track.get_events();
                for event in events {
                    synthesize_event(&mut self.synthesizer, event);
                }
            }
        } else {
            self.delta_samples += 1;
        }

        let mut l = [0.0];
        let mut r = [0.0];
        self.synthesizer.render(&mut l, &mut r);

        Some([l[0], r[0]])
    }
}

struct TrackSequencer {
    track: MidiTrack,
    next_event_index: usize,
    ticks_since_last: usize,
}
impl TrackSequencer {
    pub fn new(track: MidiTrack) -> Self {
        Self {
            track,
            next_event_index: 0,
            ticks_since_last: 0,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.next_event_index == self.track.get_events().len()
    }

    /// Call this exacly once every division.
    pub fn get_events(&mut self) -> Vec<&MidiTrackEvent> {
        if self.next_event_index == self.track.get_events().len() {
            return vec![];
        }
        let mut event = &self.track.get_events()[self.next_event_index];

        if self.ticks_since_last < event.get_delta_time() {
            self.ticks_since_last += 1;
            return vec![];
        }

        if self.ticks_since_last == event.get_delta_time() {
            let mut events = vec![];

            while self.ticks_since_last == event.get_delta_time() {
                events.push(event);
                self.ticks_since_last = 0;

                self.next_event_index += 1;
                if self.next_event_index == self.track.get_events().len() {
                    break;
                }
                event = &self.track.get_events()[self.next_event_index];
            }

            return events;
        }

        panic!("Somehow we have passed an event.");
    }
}

fn synthesize_event(synthesizer: &mut Synthesizer, track_event: &MidiTrackEvent) {
    let event = track_event.get_event();
    match event {
        MidiEvent::Channel(channel_message) => {
            let command = event.get_command().into();
            let ch;
            let data1;
            let data2;
            match channel_message {
                ChannelMessage::NoteOff { channel, key, vel }
                | ChannelMessage::NoteOn { channel, key, vel } => {
                    ch = *channel as i32;
                    data1 = *key as i32;
                    data2 = *vel as i32;
                }
                ChannelMessage::AfterTouch {
                    channel,
                    key,
                    pressure,
                } => {
                    ch = *channel as i32;
                    data1 = *key as i32;
                    data2 = *pressure as i32;
                }
                ChannelMessage::ControlChange {
                    channel,
                    control,
                    value,
                } => {
                    ch = *channel as i32;
                    data1 = *control as i32;
                    data2 = *value as i32;
                }
                ChannelMessage::ProgramChange { channel, program } => {
                    ch = *channel as i32;
                    data1 = *program as i32;
                    data2 = 0;
                }
                ChannelMessage::ChannelPressure { channel, value } => {
                    ch = *channel as i32;
                    data1 = *value as i32;
                    data2 = 0;
                }
                ChannelMessage::PitchBend { channel, value } => {
                    ch = *channel as i32;
                    data1 = *value as i32;
                    data2 = 0;
                }
                ChannelMessage::ChannelMode {
                    channel,
                    control,
                    value,
                } => {
                    ch = *channel as i32;
                    data1 = *control as i32;
                    data2 = *value as i32;
                }
            }
            synthesizer.process_midi_message(ch, command, data1, data2);
        }
        MidiEvent::System(..) => (),
        MidiEvent::Meta { .. } => (),
    }
}
