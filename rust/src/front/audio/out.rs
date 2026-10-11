use std::collections::VecDeque;
use std::sync::{Arc, Mutex, PoisonError};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// Bounded queue depth — ~8 frames of 30 fps audio; a slow consumer
/// sheds the oldest samples rather than growing latency.
const CAP_FRAMES: u32 = 8;

/// A live cpal output stream plus the sample queue the game thread
/// fills. Dropping [`Out`] stops playback; [`Out::open`] yields `None`
/// when no output device/config is usable, and the caller just renders
/// into the void (the synth still runs — it stays cheap).
pub struct Out {
    /// The stream — its callback drains `sink` across `chans`.
    pub stream: cpal::Stream,
    /// Mono `f32` samples queued by the render side.
    sink: Arc<Mutex<VecDeque<f32>>>,
    /// Negotiated output rate — [`Audio::set_rate`][super::Audio::set_rate].
    pub rate: u32,
}

impl Out {
    /// Open the default output device in `f32`; `None` on any failure.
    #[must_use]
    pub fn open() -> Option<Self> {
        let dev = cpal::default_host().default_output_device()?;
        let cfg = dev.default_output_config().ok()?;
        Self::build(&dev, &cfg)
    }

    /// Build + play the stream; the callback duplicates each mono
    /// sample across `channels` and pads with silence on underrun.
    fn build(dev: &cpal::Device, cfg: &cpal::SupportedStreamConfig) -> Option<Self> {
        let sink = Arc::new(Mutex::new(VecDeque::new()));
        let chans = usize::from(cfg.channels());
        let q = Arc::clone(&sink);
        let stream = dev
            .build_output_stream(
                cfg.config(),
                move |out: &mut [f32], _| fill(&q, out, chans),
                |e| eprintln!("audio: {e}"),
                None,
            )
            .ok()?;
        stream.play().ok()?;
        Some(Self {
            stream,
            sink,
            rate: cfg.sample_rate(),
        })
    }

    /// Queue rendered mono `pcm`; the queue is capped so overproducing
    /// drops the oldest audio first.
    pub fn push(&self, pcm: &[f32]) {
        let mut q = self.sink.lock().unwrap_or_else(PoisonError::into_inner);
        q.extend(pcm.iter().copied());
        let cap = self.rate as usize * usize::try_from(CAP_FRAMES).unwrap_or(8) / 30;
        while q.len() > cap {
            q.pop_front();
        }
    }
}

/// The stream callback — pop one mono sample per output frame.
fn fill(sink: &Mutex<VecDeque<f32>>, out: &mut [f32], chans: usize) {
    let mut q = sink.lock().unwrap_or_else(PoisonError::into_inner);
    for frame in out.chunks_mut(chans.max(1)) {
        let s = q.pop_front().unwrap_or(0.0);
        for c in frame.iter_mut() {
            *c = s;
        }
    }
}
