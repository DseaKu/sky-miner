use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::path::PathBuf;
use super::{Chunk, ChunkCoord};

pub enum IOMessage {
    Save(ChunkCoord, Chunk),
    Load(ChunkCoord),
    Shutdown,
}

pub enum IOResponse {
    Loaded(ChunkCoord, Chunk),
    NotFound(ChunkCoord),
}

pub struct IOHandler {
    to_worker: Sender<IOMessage>,
    from_worker: Receiver<IOResponse>,
    worker_handle: Option<thread::JoinHandle<()>>,
}

impl IOHandler {
    pub fn new(base_path: PathBuf) -> Self {
        let (to_worker, worker_rx) = channel();
        let (worker_tx, from_worker) = channel();

        // Ensure directory exists
        let _ = std::fs::create_dir_all(&base_path);

        let handle = thread::spawn(move || {
            Self::worker_loop(worker_rx, worker_tx, base_path);
        });

        Self {
            to_worker,
            from_worker,
            worker_handle: Some(handle),
        }
    }

    fn worker_loop(
        rx: Receiver<IOMessage>,
        tx: Sender<IOResponse>,
        base_path: PathBuf,
    ) {
        while let Ok(msg) = rx.recv() {
            match msg {
                IOMessage::Save(coord, chunk) => {
                    let path = base_path.join(format!("chunk_{}_{}.bin", coord.x, coord.y));
                    if let Ok(encoded) = bincode::serialize(&chunk) {
                        let _ = std::fs::write(path, encoded);
                    }
                }
                IOMessage::Load(coord) => {
                    let path = base_path.join(format!("chunk_{}_{}.bin", coord.x, coord.y));
                    if path.exists() {
                        if let Ok(bytes) = std::fs::read(path) {
                            if let Ok(chunk) = bincode::deserialize::<Chunk>(&bytes) {
                                let _ = tx.send(IOResponse::Loaded(coord, chunk));
                                continue;
                            }
                        }
                    }
                    let _ = tx.send(IOResponse::NotFound(coord));
                }
                IOMessage::Shutdown => break,
            }
        }
    }

    pub fn save(&self, coord: ChunkCoord, chunk: Chunk) {
        let _ = self.to_worker.send(IOMessage::Save(coord, chunk));
    }

    pub fn load(&self, coord: ChunkCoord) {
        let _ = self.to_worker.send(IOMessage::Load(coord));
    }

    pub fn poll(&self) -> Option<IOResponse> {
        self.from_worker.try_recv().ok()
    }
}

impl Drop for IOHandler {
    fn drop(&mut self) {
        let _ = self.to_worker.send(IOMessage::Shutdown);
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }
}
