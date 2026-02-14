pub mod data_output_rate;
pub use data_output_rate::DataOutputRate;
pub mod measurement_mode;
pub use measurement_mode::MeasurementMode;
pub mod measurement_sample_output;
pub use measurement_sample_output::MeasurementSampleRate;
pub mod register_a;
mod register_b;
mod gain_config;
mod mode_register;
mod mode_register_config;

pub use register_a::RegisterA;