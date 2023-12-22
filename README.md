# Fast Reverse Geocoding Server

Welcome to our open-source project, a fast reverse geocoding server built with Rust. This project was born out of the need for a more efficient alternative to Nominatim, which we found to be too slow for our requirements. 

## Motivation

Our primary motivation was to create a reverse geocoding server that could handle high request volumes with low latency. We found that Nominatim, while a great tool, was not able to meet our performance needs. This led us to develop our own solution, optimized for speed and efficiency.

## Approach

Our approach to achieving this high performance was to cache everything in-memory, rather than relying on a database. This resulted in a significant performance increase, with our server able to handle over 70,000 requests per second on a MacBook M1 Pro machine, while only using around 100MB of memory.

```console
❯ wrk http://localhost:3000/query\?lat\=21.022894363180978\&lon\=105.80110064069345
Running 10s test @ http://localhost:3000/query?lat=21.022894363180978&lon=105.80110064069345
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   154.91us  233.58us  10.15ms   97.01%
    Req/Sec    35.61k     2.78k   38.49k    92.57%
  715604 requests in 10.10s, 121.48MB read
Requests/sec:  70853.45
Transfer/sec:     12.03MB
```

To further optimize memory usage, we stripped down the data from 64-bit floating point numbers (f64) to 32-bit floating point numbers (f32). This is because f32 provides sufficient accuracy for GPS latitude and longitude coordinates, and it uses less memory. 

## Current State

At its current state, our server can convert latitude and longitude coordinates to street names. This is the simplest form of reverse geocoding, but it serves as a solid foundation for future enhancements.

We are excited about the potential of this project and we welcome contributions from the open-source community. Whether you're a Rust enthusiast, a geocoding expert, or just someone interested in high-performance servers, we'd love to have you on board!

## Getting Started

To start using the Fast Reverse Geocoding Server, you have three options:

### 1. Starting from Source Code

To start from the source code, follow these steps:

1. Clone the repository: `git clone https://github.com/giangndm/fast-pbf-query-server.git`
2. Navigate to the project directory: `cd fast-pbf-query-server`
3. Build the project: `cargo build`
4. Run the server: `cargo run -- --path path_to.pbf --cache ./geo.index`

### 2. Starting from Docker

To start from Docker, follow these steps:

1. Pull the Docker image: `docker pull ghcr.io/giangndm/fast-pbf-query-server:main`
2. Run the Docker container: `docker run -p 3000:3000 ghcr.io/giangndm/fast-pbf-query-server:main --path path_to.pbf --cache ./geo.index`

### 3. Starting from Binary

To start from the binary, follow these steps:

1. Download the binary for your operating system from the [releases page](https://github.com/giangndm/fast-pbf-query-server/releases).
2. Make the binary executable: `chmod +x fast-pbf-server`
3. Run the server: `./fast-pbf-server --path path_to.pbf --cache ./geo.index`

Once the server is running, you can test it by making a request using cURL:

`curl http://localhost:3000/query?lat=LAT&lon=LON`

