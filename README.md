# P2P File Transfer Website
![Rust](https://img.shields.io/badge/Rust-1.72-orange?style=for-the-badge&logo=rust)
![React](https://img.shields.io/badge/React-18-blue?style=for-the-badge&logo=react)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Contributions Welcome](https://img.shields.io/badge/Contributions-Welcome-brightgreen?style=for-the-badge)

## Overview

This project is a peer-to-peer (P2P) file transfer application that allows users to upload and share files directly with others using a unique ticket system. The backend is built with **Rust** using the [Iroh](https://github.com/n0-computer/iroh) library for P2P functionality, while the frontend is developed with **React** to provide a user-friendly interface. The application enables efficient, decentralized file sharing without relying on centralized servers for storage.

---

## Features

- **File Upload & Sharing**: Users can upload files and generate a unique ticket to share with others.
- **File Download**: Users can input a ticket to download the corresponding file directly from the peer.
- **Decentralized Architecture**: Powered by Iroh, the application uses P2P technology for direct file transfers.
- **User-Friendly Interface**: A clean and simple React-based UI for seamless interaction.
- **Secure Transfers**: Files are transferred directly between peers, ensuring privacy and security.

---

## Technologies Used

### Backend
- **Rust**: For building a high-performance server.
- **Iroh**: A Rust library for P2P networking and content-addressed storage.
- **Axum**: A web framework for handling HTTP requests.
- **Tokio**: An asynchronous runtime for Rust.

### Frontend
- **React**: For building the user interface.
- **Axios**: For making HTTP requests to the backend.
- **Vite**: For fast development and build tooling.

---

## Installation and Setup

### Prerequisites
Ensure you have the following installed on your system:
1. [Rust](https://www.rust-lang.org/tools/install)
2. [Node.js](https://nodejs.org/) (with npm or yarn)

### Backend Setup
1. Navigate to the `backend` directory:
   ```bash
   cd backend
   ```
2. Install dependencies and build the project:
   ```bash
   cargo build
   ```
3. Run the backend server:
   ```bash
   cargo run
   ```
4. The backend will start on `http://localhost:3000`.

### Frontend Setup
1. Navigate to the `frontend` directory:
   ```bash
   cd frontend
   ```
2. Install dependencies:
   ```bash
   npm install
   ```
3. Start the development server:
   ```bash
   npm run dev
   ```
4. The frontend will start on `http://localhost:5173`.

---

## Usage Instructions

### Sending a File
1. Open the frontend in your browser (`http://localhost:5173`).
2. In the "Send File" section, select a file to upload.
3. Click "Upload" to generate a unique ticket.
4. Share this ticket with another user.

### Receiving a File
1. Obtain a ticket from the sender.
2. Open the frontend in your browser (`http://localhost:5173`).
3. In the "Receive File" section, enter the ticket in the input field.
4. Click "Download" to retrieve the file.

---

## Project Structure

```
p2p-file-transfer/
├── backend/                # Rust-based backend with Iroh integration
│   ├── Cargo.toml          # Backend dependencies and configuration
│   └── src/
│       └── main.rs         # Main entry point for the backend server
├── frontend/               # React-based frontend for user interaction
│   ├── package.json        # Frontend dependencies and configuration
│   ├── vite.config.js      # Vite configuration for development/build tooling
│   └── src/
│       ├── App.jsx         # Main React component for UI logic
│       └── main.jsx        # Entry point for React application
```

---

## Key Endpoints

### Backend API Endpoints

| Endpoint       | Method | Description                     |
|----------------|--------|---------------------------------|
| `/api/send`    | POST   | Uploads a file and generates a unique ticket |
| `/api/receive` | POST   | Downloads a file using a provided ticket |

---

## Future Improvements

Here are some potential enhancements that could be added to this project:
1. **File Encryption**: Add end-to-end encryption for secure transfers.
2. **Progress Indicators**: Display upload/download progress in real-time.
3. **WebRTC Integration**: Use WebRTC for better NAT traversal in P2P connections.
4. **Mobile Support**: Optimize UI for mobile devices.
5. **Persistent Storage**: Allow users to store files temporarily on nodes.

---

## Contributing

Contributions are welcome! If you'd like to contribute to this project:
1. Fork the repository.
2. Create a new branch (`git checkout -b feature-name`).
3. Make your changes and commit them (`git commit -m "Add feature"`).
4. Push your branch (`git push origin feature-name`).
5. Open a pull request.

---

## License

This project is licensed under the MIT License. You are free to use, modify, and distribute it as long as proper credit is given.

---

## Acknowledgments

This project was inspired by modern decentralized technologies and aims to provide an accessible way to share files without relying on centralized systems.

Happy coding! 🎉
