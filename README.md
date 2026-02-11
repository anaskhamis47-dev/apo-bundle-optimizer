# 💊 Apo Bundle Optimizer

**Smart Pharmacy Inventory Management & Profit Recovery Tool**

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-brightgreen)
![Build](https://img.shields.io/badge/build-passing-success)

## 🌟 Overview
**Apo Bundle Optimizer** is a high-performance CLI tool developed in **Rust**. It is specifically designed for independent German pharmacies (*Apotheken*) to solve the problem of inventory waste. By utilizing intelligent algorithms, the tool identifies medications nearing expiry and suggests "Smart Bundles" (e.g., Immunity Kits, Travel Packs) with optimized pricing to recover revenue that would otherwise be lost.

## 🚀 Key Features
* **Intelligent Bundling:** Automatically pairs near-expiry items based on logical health categories.
* **Profit Recovery:** Calculates dynamic discounts to ensure stock clearance before expiration.
* **Cross-Platform Performance:** Native binaries for both **Windows** and **Linux**.
* **Data Privacy:** Runs locally on the pharmacy PC; no sensitive patient data is uploaded to the cloud.

## 🐧 Linux Compilation & CI/CD
This project is fully **Linux-compatible**. We utilize **GitHub Actions** for Continuous Integration (CI). Every push to the repository triggers a Linux build process on an Ubuntu environment.
* **Target Architecture:** x86_64-unknown-linux-gnu.
* **Build Status:** ✅ Verified through GitHub Actions workflow.

## 🛠️ Installation & Usage

### For Windows
1. Download the `apo_bundle_project.exe` from the repository.
2. Run it via the terminal or command prompt.

### For Linux Users
You can build the binary directly from the source:
```bash
git clone [https://github.com/anaskhamis47-dev/apo-bundle-optimizer.git](https://github.com/anaskhamis47-dev/apo-bundle-optimizer.git)
cd apo-bundle-optimizer
cargo build --release
