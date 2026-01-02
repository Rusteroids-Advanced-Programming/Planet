# ☄️ Planet Rusteroids

Welcome to **Rusteroids**! We are a high-performance **TYPE A** planet engineered for maximum durability and strategic energy management. While others focus on complexity, we focus on security and reliability.

---

## 🌍 Planet Properties

Rusteroids is built on a foundation of stability and defensive readiness:

* **Planet Class:** TYPE A
* **Fundamental Resource:** Carbon
* **Energy Storage:** Vector-based Array (5 Dedicated Cells)
* **Defensive Hardware:** Integrated Rocket Launchers (Supports 1 Rocket)

---

## 💡 Planet Logic

### Sunray Absorption

Our energy vector is designed for rapid intake. Whenever a sunray hits the surface, Rusteroids recharges its internal energy cells sequentially up to our 5-cell capacity.

### Rocket Fabrication & Defense

We maintain a "Safety First" protocol. If the planet’s orbital defense is empty (no rocket available), the system automatically consumes one charged energy cell to immediately construct a new rocket. This rocket is held in standby to deflect incoming asteroids.

### Strategic Energy Reservation

Rusteroids features a unique **Energy Threshold** variable. This allows commanders to decide exactly how much power is reserved exclusively for defense:

* **Defensive Priority:** You define the minimum number of charged cells required to remain in "Safety Mode."
* **Resource Extraction:** The planet will only allow resource gathering if the current energy level **exceeds** your custom threshold.
* **Example:** If your threshold is set to `3`, Rusteroids will block all resource requests until at least 4 cells are fully charged, ensuring you always have power left for rocket fabrication.

---

## 🛠 Usage Instructions

To integrate Rusteroids into your galaxy simulation:

### 1. Add Dependency

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rusteroid_planet = { git = "https://github.com/Rusteroids-Advanced-Programming/Planet.git" }

```

### 2. Initialize the Planet

Use the following snippet to bring your planet online:

```rust
// Initialize the planet and extract the core instance
let planet = Rusteroids::new(...).unwrap().planet;

```

### 3. Testing

```bash
# Compile the planetary modules
cargo build

# Execute the defense and energy logic tests
cargo test

```

---

## 🛰 Contact Us

Join our orbital command center to discuss strategies, report issues, or connect with the team:

**[Join the Rusteroids Discord](https://discord.gg/wu96DjsA)**
