use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig, SizedSample, Sample};
use egui::Ui;
use num_traits::{FromPrimitive, ToPrimitive};
use projectm::core::{ProjectM, STEREO};
use rfd::FileDialog;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct Playback {
    pub queue: Vec<PathBuf>,
    pub current_track_index: Option<usize>,
    stream: Option<Stream>,
    audio_data: Arc<Mutex<Vec<f32>>>,
    audio_position: Arc<Mutex<usize>>,
    projectm: Arc<Mutex<ProjectM>>,
}

impl Playback {
    pub fn new(projectm: Arc<ProjectM>) -> Self {
        let audio_data = Arc::new(Mutex::new(Vec::new()));
        let audio_position = Arc::new(Mutex::new(0));

        let audio_data_clone = audio_data.clone();
        let audio_position_clone = audio_position.clone();
        let projectm_clone = Arc::new(Mutex::new((*projectm).clone()));

        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => Self::create_stream::<f32>(
                device,
                config.into(),
                audio_data_clone,
                audio_position_clone,
                projectm_clone,
            ),
            cpal::SampleFormat::I16 => Self::create_stream::<i16>(
                device,
                config.into(),
                audio_data_clone,
                audio_position_clone,
                projectm_clone,
            ),
            cpal::SampleFormat::U16 => Self::create_stream::<u16>(
                device,
                config.into(),
                audio_data_clone,
                audio_position_clone,
                projectm_clone,
            ),
            _ => panic!("Unsupported sample format"),
        };

        stream.play().unwrap();

        Self {
            queue: Vec::new(),
            current_track_index: None,
            stream: Some(stream),
            audio_data,
            audio_position,
            projectm: Arc::new(Mutex::new((*projectm).clone())),
        }
    }

    fn create_stream<T>(
        device: cpal::Device,
        config: StreamConfig,
        audio_data: Arc<Mutex<Vec<f32>>>,
        audio_position: Arc<Mutex<usize>>,
        projectm: Arc<Mutex<ProjectM>>,
    ) -> Stream
    where
        T: SizedSample + Sample + FromPrimitive + ToPrimitive,
    {
        device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    let mut audio_data = audio_data.lock().unwrap();
                    let mut audio_position = audio_position.lock().unwrap();

                    for sample in data.iter_mut() {
                        if *audio_position < audio_data.len() {
                            *sample = T::from_f32(audio_data[*audio_position]).unwrap();
                            *audio_position += 1;
                        } else {
                            *sample = T::from_f32(0.0f32).unwrap();
                        }
                    }

                    let pcm_data: Vec<f32> = data
                        .iter()
                        .map(|s| s.to_f32().unwrap())
                        .collect();
                    
                    for chunk in pcm_data.chunks(512) {
                        projectm.lock().unwrap().pcm_add_float(chunk, STEREO);
                    }
                },
                |err| eprintln!("an error occurred on stream: {}", err),
                None,
            )
            .unwrap()
    }

    pub fn add_files(&mut self, files: Vec<PathBuf>) {
        self.queue.extend(files);
        if self.current_track_index.is_none() && !self.queue.is_empty() {
            self.play_track(0);
        }
    }

    fn play_track(&mut self, index: usize) {
        if let Some(path) = self.queue.get(index) {
            let mut hint = Hint::new();
            hint.with_extension(path.extension().unwrap().to_str().unwrap());
            let source = Box::new(File::open(path).unwrap());
            let mss = MediaSourceStream::new(source, Default::default());
            let format_opts = FormatOptions {
                enable_gapless: true,
                ..Default::default()
            };
            let metadata_opts: MetadataOptions = Default::default();
            let decoder_opts: DecoderOptions = Default::default();
            let mut probed =
                symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts).unwrap();

            let track = probed.format.default_track().unwrap();
            let track_id = track.id;
            let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts).unwrap();

            let mut samples_f32: Vec<f32> = Vec::new();

            loop {
                let packet = match probed.format.next_packet() {
                    Ok(packet) => packet,
                    Err(Error::ResetRequired) => {
                        unimplemented!();
                    }
                    Err(Error::IoError(_)) => {
                        break;
                    }
                    Err(err) => {
                        panic!("{}", err);
                    }
                };

                if packet.track_id() != track_id {
                    continue;
                }

                match decoder.decode(&packet) {
                    Ok(decoded) => {
                        let mut sample_buf =
                            SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
                        sample_buf.copy_interleaved_ref(decoded);
                        samples_f32.extend_from_slice(sample_buf.samples());
                    }
                    Err(Error::IoError(_)) => {
                        break;
                    }
                    Err(Error::DecodeError(_)) => {
                        continue;
                    }
                    Err(err) => {
                        panic!("{}", err);
                    }
                }
            }

            let mut audio_data = self.audio_data.lock().unwrap();
            let mut audio_position = self.audio_position.lock().unwrap();

            *audio_data = samples_f32;
            *audio_position = 0;
            self.current_track_index = Some(index);
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("Playback");

        if ui.button("Add Files").clicked() {
            if let Some(files) = FileDialog::new()
                .add_filter("Audio", &["mp3", "wav", "flac", "ogg"])
                .pick_files()
            {
                self.add_files(files);
            }
        }

        ui.separator();

        if let Some(index) = self.current_track_index {
            if let Some(path) = self.queue.get(index) {
                ui.label(format!("Now Playing: {}", path.display()));
            }
        }
    }
}