#[derive(Debug, Clone)]

pub struct Computer
{
	//os
	pub OS: String,
	pub OSVersion: String,
	pub OSKernelVersion: String,

	//cpu
	pub CPUName: String,
	pub CPUCores: u8,
	pub CPUThreads: u8,
	pub CPUUsage: f32,

	//ram
	pub RAMTotal: u64,
	pub RamUsedPercent: f32,
	pub RamUsedBytes: u64,

	//gpu
	pub GPUName: String,
	pub GPUDriverVersion: String,
	pub VRAM: u64,
	pub GPUUsedPercent: f32,
	pub GPUUsedByte: u64,

	//Memory
	pub Memory: u64,
}

impl Computer
{
	pub fn new() -> Self
	{
		Self
		{
			OS: "crappy os".to_string(),
			OSKernelVersion: "v1.1".to_string(),
			OSVersion: "1.1.1".to_string(),
			CPUName: "spymaster 69".to_string(),
			CPUCores: 4,
			CPUThreads: 4,
			CPUUsage: 68.0,
			RAMTotal: 56,
			RamUsedPercent: 32.0,
			RamUsedBytes: 12,
			GPUName: "overpriced crypto miner".to_string(),
			GPUDriverVersion: "v1.2.3".to_string(),
			VRAM: 34,
			GPUUsedPercent: 12.0,
			GPUUsedByte: 56,
			Memory: 58
		}
	}
}