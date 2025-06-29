# Ray Tracing in One Week — Rust Port Project Report

## 🔗 Project Repository

[https://github.com/starrywiki/RayTracing-in-One-Week-by-Rust](https://github.com/starrywiki/RayTracing-in-One-Week-by-Rust)

---

## 📸 Project Result Showcase

This project reproduces the final rendering output of *Ray Tracing in One Weekend* using Rust. The renderer supports diffuse, metal, and dielectric materials, with anti-aliasing and shadowing.

### 🌅 Rendered Image

| Image                         | Description                                    |
| ----------------------------- | ---------------------------------------------- |
| ![demo](./results/output.png) | Final rendered image with resolution 1200*675. |

> Render time: ~110 minutes 

---

## ⚙️ Implementation and Technology Stack

- **Original Language**: C++
- **Target Language**: Rust (1.77)
- **Build Tool**: Cargo
- **Image Output**: [`image`](https://crates.io/crates/image) crate
- **Multithreading**: [`rayon`](https://crates.io/crates/rayon) crate
- **Project Structure**:
  - `vec3.rs`: Vector operations
  - `ray.rs`: Ray structure
  - `hittable.rs`: Trait for hittable objects and sphere implementation
  - `material.rs`: Lambertian, Metal, Dielectric materials
  - `camera.rs`: Camera configuration
  - `main.rs`: Rendering logic

---

## 🧱 Challenges and Solutions

### 1. Trait Object Recursion Limitations in Rust

- **Problem**: Rust trait objects cannot hold self-referencing structures, causing issues in representing BVH or nested `Hittable` lists.
- **Solution**:
  - Wrapped trait objects using `Box<dyn Hittable>` and `Arc` to manage ownership and thread safety.
  - Built `HittableList` and `BVHNode` with boxed dynamic dispatch and shared pointers.

### 2. Ownership and Parallelism

- **Problem**: Rust’s ownership model makes data sharing across threads non-trivial, especially when using `rayon`.
- **Solution**:
  - Used `Arc` to wrap shared structures.
  - Ensured each thread worked on a separate portion of the output buffer to avoid data races.

### 3. Tedious Operator Overloading

- **Problem**: Rust requires explicit implementation of traits like `Add`, `Mul`, etc., unlike C++’s operator overloading.
- **Solution**:
  - Implemented traits like `Add`, `Sub`, `Mul` manually for `Vec3`.
  - Used Rust macros to reduce boilerplate where possible.

---

## 🌱 Takeaways and Reflections

### Advantages of Rust

- Ownership and compile-time checks eliminate memory bugs.
- Strong concurrency performance using `rayon`.

### Challenges

- Steep learning curve, especially around traits and lifetimes.
- Translation from OOP (C++) to Rust's trait system requires a paradigm shift.

### What I Learned

- Deepened understanding of ray tracing fundamentals.
- Gained hands-on experience with Rust's core language features (traits, lifetimes, concurrency).
- Learned how to design and organize a rendering engine idiomatically in Rust.

---

## 📄 Appendix

### Running the Project

```bash
git clone https://github.com/starrywiki/RayTracing-in-One-Week-by-Rust.git
cd RayTracing-in-One-Week-by-Rust
cargo run --release
