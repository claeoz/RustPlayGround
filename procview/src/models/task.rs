#[derive(Debug, Clone)]

pub struct Task
{
	pub Name: String,
	pub pid: u32,
	pub CPUPercentage: f32,
	pub RamPercentage: f32,
	pub RamBytes: u64,
	pub StorageUsagePercentage: f32,
	pub StorageUsageBytes: u64,
	pub NetworkPercentage: f32,
	pub NetworkBytes: u64,
}