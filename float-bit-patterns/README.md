# 🧠 Float Bit Patterns in Rust

This project explores how a 32-bit `f32` floating-point number is represented in memory using the IEEE 754 standard.

We break down the float into:

- The **sign bit**
- The **exponent** (biased and actual)
- The **mantissa**

---

## 📘 What You’ll Learn

- How to view a `f32` number's internal representation
- How to extract:
  - Sign bit (positive/negative)
  - Exponent (biased and unbiased)
  - Mantissa (fractional precision part)
- How to use bitwise operations (`>>`, `&`, and masks like `0xff`, `0x7FFFFF`)

---

## 🧪 Example Run Output

```zsh
cargo run
```
