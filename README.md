# dande-ingest

i got tired of python pandas throwing out-of-memory errors every time i tried to ingest a 50gb log file. 

this is a zero-allocation, multithreaded csv parser written in rust. it uses `rayon` to parallelize chunk processing. it maps the byte records directly and doesn't leak memory. 

don't use this for tiny 10mb files, the thread overhead isn't worth it. use this when your dataset is massive and your standard i/o is bottlenecking.

**build:**
`cargo build --release`

**run:**
`./target/release/dande_ingest <path_to_massive_file.csv>`
