#![cfg_attr(target_arch = "riscv32", no_std, no_main)]
#[cfg(target_arch = "riscv32")]
use nexus_rt::println;
#[cfg(not(target_arch = "riscv32"))]
use std::println;

#[cfg(not(target_arch = "riscv32"))]
fn public_input_native() -> Result<(u32, u32, u32), String> {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line1 = lines
        .next()
        .ok_or("No first input provided")?
        .map_err(|e| format!("Failed to read first line: {}", e))?;

    let n = line1
        .trim()
        .parse()
        .map_err(|e| format!("Failed to parse first input as u32: {}", e))?;

    let init_a = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(1),
        _ => 1,
    };

    let init_b = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(1),
        _ => 1,
    };

    Ok((n, init_a, init_b))
}

#[nexus_rt::main]
#[cfg_attr(target_arch = "riscv32", nexus_rt::public_input(n, init_a, init_b))]
#[cfg_attr(
    not(target_arch = "riscv32"),
    nexus_rt::custom_input((n, init_a, init_b), public_input_native)
)]
fn main(n: u32, init_a: u32, init_b: u32) {
    // Simple Fibonacci calculation
    // let mut prev: u32 = init_a;
    // let mut curr: u32 = init_b;
    
    // for _ in 0..n {
    //     let next = prev.wrapping_add(curr);
    //     prev = curr;
    //     curr = next;
    // }
    let mut curr = f2(n, init_a, init_b);
    println!("{:?}", curr);
} 

fn f2(n: u32, init_a: u32, init_b: u32) -> u32 {
    // 矩阵快速幂实现
    #[derive(Copy, Clone)]
    struct Matrix {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }

    impl Matrix {
        // 单位矩阵
        fn identity() -> Self {
            Matrix { a: 1, b: 0, c: 0, d: 1 }
        }

        // 基础转移矩阵 (Fibonacci 递推矩阵)
        fn base() -> Self {
            Matrix { a: 1, b: 1, c: 1, d: 0 }
        }

        // 矩阵乘法 (使用 wrapping 运算)
        fn multiply(self, rhs: Matrix) -> Matrix {
            Matrix {
                a: self.a.wrapping_mul(rhs.a).wrapping_add(self.b.wrapping_mul(rhs.c)),
                b: self.a.wrapping_mul(rhs.b).wrapping_add(self.b.wrapping_mul(rhs.d)),
                c: self.c.wrapping_mul(rhs.a).wrapping_add(self.d.wrapping_mul(rhs.c)),
                d: self.c.wrapping_mul(rhs.b).wrapping_add(self.d.wrapping_mul(rhs.d)),
            }
        }

        // 快速幂算法
        fn power(self, mut exp: u32) -> Matrix {
            let mut base = self;
            let mut result = Matrix::identity();
            while exp > 0 {
                if exp % 2 == 1 {
                    result = result.multiply(base);
                }
                base = base.multiply(base);
                exp /= 2;
            }
            result
        }
    }
    // 计算 M^n (递推矩阵的 n 次幂)
    let mat_pow = Matrix::base().power(n);

    // 计算 F_{n+1} = M^n * [F1, F0]^T
    let result = mat_pow.a.wrapping_mul(init_b).wrapping_add(mat_pow.b.wrapping_mul(init_a));
    result
}

