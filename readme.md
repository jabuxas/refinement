# refinement

hii, this is my personal site built with [rocket](https://rocket.rs) and rust. it's dockerized for easy deployment.

## quick start

### running locally

1. clone the repo:

   ```bash
   git clone https://github.com/jabuxas/refinement.git
   cd refinement
   ```

2. run the site:

   ```bash
   cargo run
   ```

3. open your browser at http://127.0.0.1:8000

### running with docker

- this setup maps container port 8000 to host port 8880.

- build the image:

  ```bash
  docker build -t refinement .
  ```

- run the container:

      docker run -p 8880:8000 refinement

- or use docker-compose:

  ```bash
  docker-compose up -d
  ```

- open your browser at http://localhost:8880
