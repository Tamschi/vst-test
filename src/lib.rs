#![doc(html_root_url = "https://docs.rs/vst-test/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]

use core::ffi::c_void;
use vst::{
	api::{Events, Supported},
	buffer::AudioBuffer,
	editor::Editor,
	plugin::{CanDo, Category, HostCallback, Info, Plugin},
	plugin_main,
};

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

#[derive(Default)]
struct MyPlugin;

impl Plugin for MyPlugin {
	fn get_info(&self) -> Info {
		#[allow(clippy::zero_prefixed_literal)]
		Info {
			name: "My terrible VST plugin".to_string(),
			vendor: "Tamschi".to_string(),
			// presets: 0,
			// parameters: 0,
			// inputs: 1,
			// outputs: 1,
			// version: 0001,
			// category: Category::Effect,
			// unique_id: 1_820_746,
			// initial_delay: 0,
			// f64_precision: true,
			// silent_when_stopped: true,
			..Info::default()
		}
	}

	fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
		Some(Box::new(MyEditor::new()))
	}

	fn can_do(&self, _can_do: CanDo) -> Supported {
		Supported::No
	}

	fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
		// For each input and output
		for (input, output) in buffer.zip() {
			// For each input sample and output sample in buffer
			for (in_frame, out_frame) in input.iter().zip(output.iter_mut()) {
				*out_frame = *in_frame;
			}
		}
	}

	fn process_f64(&mut self, buffer: &mut AudioBuffer<f64>) {
		// For each input and output
		for (input, output) in buffer.zip() {
			// For each input sample and output sample in buffer
			for (in_frame, out_frame) in input.iter().zip(output.iter_mut()) {
				*out_frame = *in_frame;
			}
		}
	}
}

plugin_main!(MyPlugin);

struct MyEditor {
	is_open: bool,
}
impl MyEditor {
	fn new() -> Self {
		Self { is_open: false }
	}
}
impl Editor for MyEditor {
	fn size(&self) -> (i32, i32) {
		(100, 100)
	}

	fn position(&self) -> (i32, i32) {
		(0, 0)
	}

	fn open(&mut self, parent: *mut c_void) -> bool {
		self.is_open = true;
		true
	}

	fn is_open(&mut self) -> bool {
		self.is_open
	}

	fn close(&mut self) {
		self.is_open = false;
	}
}
