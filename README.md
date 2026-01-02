# ☄️ Planet Rusteroids

Welcome to **Rusteroids**! As a high-performance **TYPE A** planet, we prioritize structural integrity and strategic energy management. We don't just store energy; we weaponize it for defense.

---

## 🌍 Planet Properties

Our planetary core is built for stability and high-capacity storage:

| Property | Specification |
| --- | --- |
| **Planet Class** | TYPE A |
| **Basic Resource** | Carbon |
| **Energy Storage** | Vector-based (5 Energy Cells) |
| **Defense Capacity** | 1 Rocket (Active Defense) |

---

## 💡 Planet Logic

### Sunray Handling

Upon receiving a sunray, the planet recharges its available energy cells (up to our maximum capacity of 5).

### Automated Defense & Fabrication

Rusteroids maintains a "Ready-to-Launch" policy:

* **Fabrication:** If the planet has no rocket available and at least **one (1)** charged energy cell, it immediately consumes that cell to construct a new rocket.
* **Interception:** Once a rocket is active, it is automatically deployed to deflect incoming asteroids.

### Energy Reservation System

We provide a **customizable threshold variable** to allow users to balance defense and utility. This variable defines the number of energy cells reserved strictly for rocket production:

* **Resource Extraction:** Resources can only be extracted if the current energy level is **greater than** your defined threshold.
* **Example:** If you set a threshold of `3`, the planet will refuse resource requests unless it has 4 or 5 cells charged, ensuring your defense budget is never compromised.

---

## 🛠 Usage Instructions

To integrate the Rusteroids module into your galaxy simulation:

### Testing

```bash
# Verify planet logic and dependencies
cargo build
cargo test

```

### Implementation

*placeholedr_instruction*

---

## 🛰 Contact Us

Join our orbital command center for support and updates:

**[Join the Rusteroids Discord](https://discord.gg/wu96DjsA)**
