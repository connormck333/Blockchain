# Blockchain
A simple blockchain implemented from scratch in Rust inspired by Bitcoin.
This project is a learning exercise to understand the basic concepts of blockchain technology, including blocks, transactions, and consensus mechanisms.

## Features
- Proof of Work (PoW) consensus mechanism
- Peer to peer communication
- Peer discovery
- Fork detection and resolution
- Block creation and validation
- Transaction validation
- Mempool implementation
- Postgres database
- Kubernetes & Helm deployment
- Client wallet GUI

## Requirements
- Rust (latest stable)
- Cargo
- PostgreSQL
- Optional: Docker, Kubernetes & Helm for dockerized deployment

## Usage

### 1. Clone the repository
```bash
git clone https://github.com/connormck333/blockchain.git
cd blockchain
```

### 2. Set up the database
- Ensure PostgreSQL is running and accessible.
- Create a .env file and add the following variables:
```env
POSTGRES_USERNAME=your_username
POSTGRES_PASSWORD=your_password
POSTGRES_HOST=localhost
```

### 3. Build the project
```bash
cd node
cargo build --release
```

### 4. Run the opening node
- Host: the address that the node will listen on. E.g: http://localhost
- Port: the port that the node will listen on. E.g: 8080
```bash
cargo run full open <host>:<port>
```

### 5. Run miner nodes
- Host: the address that the node will listen on. E.g: http://localhost
- Port: the port that the node will listen on. E.g: 8081
- Existing Node Host: the address of an existing node to join. E.g: http://127.0.0.1
- Existing Node Port: the port of an existing node to join. E.g: 8080
```bash
cargo run miner join <host>:<port> <existing_node_host>:<existing_node_port>
```

### 6. Build and run the wallet GUI
```bash
cd ../wallet
cargo build --release
cargo run --release
```
### Alternative: Dockerized Deployment
If you prefer to run the blockchain in a Dockerized environment, you can use the provided Docker and Kubernetes configurations.
I have provided a simple deployment script that uses Helm to deploy the blockchain nodes:
```bash
./deploy.sh
```
And to uninstall the deployment:
```bash
./stop.sh
```

## Future Work
- Consider security requirements and implement necessary measures.
- Add more features like smart contracts, token standards, etc.
- Improve the wallet GUI with ability to view transactions and balances.
- Implement more advanced consensus mechanisms like Proof of Stake (PoS).