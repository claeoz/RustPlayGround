#[derive(Debug, Clone)]

pub struct Computer
{
	//os
	OS: string,
	OSVersion: string,
	OSKernelVersion: string,

	//cpu
	CPUName: string,
	CPUCores: u8,
	CPUThreads: u8,
	CPUUsage: f32,

	//ram
	RAMTotal: u64,
	RamUsedPercent: f32,
	RamUsedBytes: u64,

	//gpu
	GPUName: string,
	GPUDriverVersion: string,
	VRAM: u64,
	Memory: u64,
	GPUUsedPercent: f32,
	GPUUsedByte: u64,
}