use serde::Serialize;

#[derive(Serialize)]
pub struct StaticData {
    pub cpu: StaticCPUData,
}

#[derive(Serialize)]
pub struct NetworkInterfaceStats {
    pub name: String,
    pub tx: u64,
    pub rx: u64,
}

#[derive(Serialize)]
pub struct CPUStats {
    pub cpu_usage: f32,
    pub frequency: u64,
}

#[derive(Serialize)]
pub struct RAMStats {
    pub free_memory: u64,
    pub available_memory: u64,
    pub used_memory: u64,
    pub total_memory: u64,
}

#[derive(Serialize)]
pub struct GPUStats {
    pub brand: String,
    pub gpu_usage: u32,
    pub power_usage: u32,
    pub memory_free: u64,
    pub memory_used: u64,
    pub memory_total: u64,
}

#[derive(Serialize)]
pub struct DiskStats {
    pub name: String,
    pub mount: String,
    pub filesystem: String,
    pub disk_type: String,
    pub free: u64,
    pub total: u64,
    pub used: u64,
}

#[derive(Serialize)]
pub struct StaticCPUData {
    pub name: String,
    pub vendor_id: String,
    pub brand: String,
}