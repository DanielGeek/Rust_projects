# This is a TCP server and client implementation in Rust

## Run the project

---------------------------------------------------------------------------------------------------------------------------------------------

docker-compose up --build # Build the Docker image and start the container

## Inside the container

docker exec -it tcp_rust_server bash # Open a shell inside the server container

docker exec -it tcp_rust_client bash # Open a shell inside the client container

ping -I tun0 192.168.0.2 # Test connectivity

nc 192.168.0.2 80 # Try to connect to port 80

tshark -i tun0 # Capture packets on tun0

---------------------------------------------------------------------------------------------------------------------------------------------

## Helpfull commands

## 1️⃣ Create a new Rust project

cargo new --bin TCP_Rust  # Creates a new Rust project with src/main.rs

## 2️⃣ Build the project in release mode

cargo build --release  # Binary located at target/release/TCP_Rust

## 3️⃣ Build the Docker image

docker-compose build  # Build image from Dockerfile with dependencies

## 4️⃣ Start the container in foreground

docker-compose up  # Creates tun0, assigns IP, activates it, runs Rust binary

## 5️⃣ Start the container in background (detached mode)

docker-compose up -d  # Run container in background

## 6️⃣ Stop the container

docker-compose down  # Stops container and network

## 7️⃣ Remove containers, images, volumes, and rebuild

docker-compose down --rmi all --volumes --remove-orphans  # Clean all
docker-compose build  # Rebuild image
docker-compose up  # Start container again

## 8️⃣ Open a shell inside the container for manual testing

docker-compose run --rm tcp_rust bash  # Interactive container shell

## 9️⃣ Check running containers

docker ps  # Shows active containers

## 🔟 Check container logs

docker logs tcp_rust_server  # View container output

## 11️⃣ List all network interfaces and IPs on macOS

ifconfig | grep inet  # Shows IP addresses

## 12️⃣ Get Wi-Fi interface IP

ipconfig getifaddr en0  # Get en0 (Wi-Fi) IP address

## 13️⃣ Get detailed Wi-Fi info

networksetup -getinfo Wi-Fi  # Shows network configuration

## 14️⃣ Assign IP to TUN interface manually if needed

sudo ifconfig utun0 inet 192.168.0.1 192.168.0.1 netmask 255.255.255.0 up  # Assign and activate

## 15️⃣ Activate TUN interface manually if needed

sudo ifconfig utun0 up  # Just bring interface up

## 16️⃣ Clone the repository

git clone [https://github.com/yourusername/TCP_Rust.git](https://github.com/yourusername/TCP_Rust.git)  # Clone repo
cd TCP_Rust  # Enter project folder

## 17️⃣ Pull latest changes

git pull origin main  # Update local repo

## 18️⃣ Check local changes

git status  # View modified/untracked files

## 19️⃣ Add and commit changes

git add .  # Stage changes
git commit -m "Your commit message"  # Commit changes
git push origin main  # Push to remote

## Docs

## Read RFC 793

[https://datatracker.ietf.org/doc/html/rfc793#page-15](https://datatracker.ietf.org/doc/html/rfc793#page-15)

[https://docs.rs/etherparse/0.8.0/etherparse/](https://docs.rs/etherparse/0.8.0/etherparse/)

[https://docs.rs/tun-tap/0.1.2/tun_tap/](https://docs.rs/tun-tap/0.1.2/tun_tap/)
