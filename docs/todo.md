- io-handler
- complete procedural logic
- ui element for displaying mined tiles
- harpoon logic

1. Off-thread Chunk Generation (Highest Impact) Currently, generate_chunk is
   called during process, meaning the game waits for the math (Perlin noise,
   tile logic) to finish before rendering the frame.
   - The Potential: You can move the noise calculations for new chunks to a
     background thread.
   - Implementation: Use Godot's WorkerThreadPool to spawn tasks from Rust. The
     background thread generates the Chunk data, and once finished, it pushes it
     to your spawn_queue for the main thread to update the TileMapLayer.

2. Asynchronous File I/O Your save_chunk and load_chunk functions use
   FileAccess, which currently blocks the main thread while reading/writing to
   disk.

- The Potential: Disk I/O is notoriously slow compared to CPU logic. Loading a
  saved chunk should happen in the background so the player doesn't feel a
  "hitch" when moving into an area with many saved files.

3. Parallel Tile Generation (Internal Rust Optimization) If you decide to stick
   to synchronous generation but want it to be faster:

- The Potential: Each tile in a chunk is generated independently in a nested
  loop.
- Implementation: You could use the Rayon library in Rust to turn your for loops
  into parallel iterators (par_iter). This would utilize all CPU cores to fill a
  single chunk's data much faster.

Technical Challenges to Consider:

- Thread Safety: You'll need to wrap shared resources (like the chunk_hash_map)
  in thread-safe containers like Arc<RwLock<...>> or Arc<Mutex<...>> if multiple
  threads need access.
- Main Thread Constraints: Godot's TileMapLayer.set_cell must be called from the
  main thread. Your multithreaded architecture would look like:
  1.  Background: Calculate noise + Load from disk.
  2.  Main Thread: Take the finished data and call set_cell.

Recommended Strategy If you start noticing "micro-stutters" as you move:

1.  Add Rayon to Cargo.toml for easy parallel loops in generate_chunk.
2.  Refactor ChunkGenerator to use a "Job" system where chunks are requested and
    then delivered via a thread-safe channel (like crossbeam-channel) once
    ready.
