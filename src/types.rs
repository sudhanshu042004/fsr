#[derive(Default)]
pub struct Manifest {
    pub name: String,
    pub size: u64,
    pub blocks_count: u64,
    pub blocks: Vec<Blocks>,
}
pub struct Blocks {
    pub block_id: u64,
    pub block_size: u64,
}
