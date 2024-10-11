use rustysynth::Synthesizer;

use crate::midifile::{
    miditrack::{MidiTrack, MidiTrackEvent},
    MidiFile,
};

/// Turn MIDI files and soundfont into audio samples.
///
/// # Examples
///
/// ```
/// // Notice that the sequencer depends on RustySynth for generating audio.
/// 
/// use crustysynth::{midifile::MidiFile, sequencer::MidiSequencer};
/// use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
/// use std::sync::Arc;
/// 
/// let soundfont: Arc<SoundFont> = todo!();
/// let midi_file: MidiFile = todo!();
///
/// let settings = SynthesizerSettings::new(44100);
/// let synthesizer = Synthesizer::new(&soundfont, &settings).expect("Synth creation failed!");
/// let mut sequencer = MidiSequencer::new(synthesizer);
///
/// sequencer.play_midi_file(midi_file);
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
    tempo: f64,
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
            tempo: 120.0,
        }
    }

    pub fn play_midi_file(&mut self, midi_file: MidiFile) {
        self.tracks.clear();
        for track in midi_file.get_tracks() {
            self.tracks.push(TrackSequencer::new(track.clone()));
        }

        self.midi_file = Some(midi_file);
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
        let tick_duration = midi.get_division().get_tick_duration(self.tempo);

        let tick_samples = (tick_duration.as_secs_f64() * samplerate as f64) as usize;
        if self.delta_samples == tick_samples {
            println!("tick: {tick_duration:?}");

            self.delta_samples = 0;
            for i in 0..self.tracks.len() {
                let track = &mut self.tracks[i];
                let events = track.get_events();
                for event in events {
                    println!("Track {i} - {event:?}");
                }
            }
        } else {
            self.delta_samples += 1;
        }

        Some([609.0, 420.0])
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

        //println!(
        //    "tick_delta: {}/{}, next_event: {}/{}",
        //    self.ticks_since_last,
        //    event.get_delta_time(),
        //    self.next_event_index,
        //    self.track.get_events().len()
        //);

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
