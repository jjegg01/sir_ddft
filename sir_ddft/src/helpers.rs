// sir_ddft - A Rust implementation of the SIR-DDFT model
// Copyright (C) 2021 Julian Jeggle, Raphael Wittkowski

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// Crate internal helper functions:

use std::sync::Arc;
#[cfg(all(target_feature="sse2", target_arch = "x86_64"))]
use std::arch::x86_64::*;

use rustfft::Fft;
use num_complex::Complex64;

// Calculates ceil(x/y) if x > 0
#[allow(dead_code)]
pub(crate) fn ceil_div(x: usize, y: usize) -> usize {
    assert!(x > 0);
    1 + (x-1)/y
}

/// 1D symmetric Laplacian (same as second derivative in space)
#[inline(always)]
pub(crate) fn laplace_1d(y: &[f64], prev: usize, i: usize, next: usize, dx: f64) -> f64 {
    (y[prev] + y[next] - 2.*y[i]) / (dx * dx)
}

/// 1D forward gradient
#[inline(always)]
pub(crate) fn grad_1d_val(y_prev: f64, y_next: f64, dx: f64) -> f64 {
    (y_next - y_prev) / (dx * 2.)
}

/// 2D symmetric Laplacian (i.e. sum of second partial derivatives)
#[inline(always)]
pub(crate) fn laplace_2d(y: &[f64], 
    prev_x: usize, ix: usize, next_x: usize,
    prev_y: usize, iy: usize, next_y: usize,
    nx: usize, dx: f64, dy: f64) -> f64 
{
    // Second partial derivative in x
    (y[prev_x + nx*iy] + y[next_x + nx*iy] - 2.*y[ix + nx*iy]) / (dx * dx) +
    // Second partial derivative in y
    (y[ix + nx*prev_y] + y[ix + nx*next_y] - 2.*y[ix + nx*iy]) / (dy * dy)
}

/// Alternative for 2D symmetric Laplacian (i.e. sum of second partial derivatives)
/// Only valid for dx==dy!
#[inline(always)]
pub(crate) fn laplace_2d9(y: &[f64], 
    prev_x: usize, ix: usize, next_x: usize,
    prev_y: usize, iy: usize, next_y: usize,
    nx: usize, dx: f64, _dy: f64) -> f64 
{
    (
        y[prev_x + nx*prev_y] + y[ix + nx*prev_y]   + y[next_x + nx*prev_y] + 
        y[prev_x + nx*iy]     - 8. * y[ix + nx*iy]  + y[next_x + nx*iy] + 
        y[prev_x + nx*next_y] + y[ix + nx*next_y]   + y[next_x + nx*next_y]
    ) / (dx * dx)
}

// Transpose slice as if it was a row-major 2d matrix
pub(crate) fn transpose_2d<T>(v: &mut[T], n: usize) {
    for iy in 0..n {
        for ix in (iy+1)..n {
            let i1 = iy * n + ix;
            let i2 = ix * n + iy;
            v.swap(i1, i2);
        }
    }
}

// (Sort of) in-place convolution of data with a precalculated kernel given as
// its Fourier transform (with baked in normalization factors)
pub(crate) fn convolve_2d(data: &mut[Complex64], kernel_fft: &[Complex64],
    fft: Arc<dyn Fft<f64>>, ifft: Arc<dyn Fft<f64>>, scratch: &mut[Complex64]) 
{
    let n = fft.len();
    // Fourier transform data
    fft.process_with_scratch(data, scratch);
    transpose_2d(data, n);
    fft.process_with_scratch(data, scratch);
    // Multiply Fourier transforms
    for i in 0..(n*n) {
        data[i] = data[i] * kernel_fft[i];
    }
    // Reverse transform
    ifft.process_with_scratch(data, scratch);
    transpose_2d(data, n);
    ifft.process_with_scratch(data, scratch);
}

// Same as convolve_2d, but parallel
#[cfg(not(target_arch = "wasm32"))] 
pub(crate) fn convolve_2d_parallel(data: &mut[Complex64], kernel_fft: &[Complex64],
    fft: Arc<dyn Fft<f64>>, ifft: Arc<dyn Fft<f64>>, scratch: &mut[Complex64],
    thread_pool: &mut scoped_threadpool::Pool)
{
    // Call serial version if thread pool has only one thread
    let num_threads = thread_pool.thread_count();
    if num_threads < 2 {
        return convolve_2d(data, kernel_fft, fft, ifft, scratch);
    }
    // Fourier transform data
    let n = fft.len();
    let chunk_size = n * ceil_div(n, num_threads as usize);
    macro_rules! process_with_scratch_parallel {
        ($fft: expr, $buffer: expr, $scratch: expr) => {
            thread_pool.scoped(|s| {
                let buffer_chunks = $buffer.chunks_mut(chunk_size);
                let scratch_chunks = $scratch.chunks_mut(chunk_size);
                for (buffer,scratch) in buffer_chunks.zip(scratch_chunks) {
                    let fft = $fft.clone();
                    s.execute(move || {
                        fft.process_with_scratch(buffer, scratch);
                    })
                }
            });
        }
    };
    process_with_scratch_parallel!(fft, data, scratch);
    transpose_2d(data, n); // Parallelizing matrix transposition is icky
    process_with_scratch_parallel!(fft, data, scratch);
    // Multiply Fourier transforms
    for i in 0..(n*n) {
        data[i] = data[i] * kernel_fft[i];
    }
    // Reverse transform
    process_with_scratch_parallel!(ifft, data, scratch);
    transpose_2d(data, n);
    process_with_scratch_parallel!(ifft, data, scratch);
}

// Stub for WASM
#[cfg(target_arch = "wasm32")]
pub(crate) fn convolve_2d_parallel(data: &mut[Complex64], kernel_fft: &[Complex64],
    fft: Arc<dyn Fft<f64>>, ifft: Arc<dyn Fft<f64>>, scratch: &mut[Complex64],
    _: ())
{
    convolve_2d(data, kernel_fft, fft, ifft, scratch);
}

/// Calculate the two preceding and the two following indices to a given index `i`
/// in a vector of `n` elements (wrapping around for )
#[inline(always)]
pub(crate) fn calc_indices(i: i32, n: i32) -> [usize;4] {
    // This optimization is probably useless (performance impact < systematic noise)
    // It is a nice little demo of SIMD in Rust, however, so we'll keep it for now
    #[cfg(all(target_feature="sse2", target_arch = "x86_64"))] {
        unsafe {
            // Build index vector and offset vector
            let mut indices = _mm_set1_epi32(i);
            let offsets = _mm_set_epi32(n-2,n-1,1,2);
            // Do raw offsets
            indices = _mm_add_epi32(indices,offsets);
            // Wrapping (if index > n-1 then subtract n from index)
            let mut tmp = _mm_set1_epi32(n-1);
            let mask = _mm_cmpgt_epi32(indices,tmp);
            tmp = _mm_set1_epi32(n);
            tmp = _mm_and_si128(tmp,mask);
            indices = _mm_sub_epi32(indices,tmp);
            // Unpack index vector
            let indices = std::mem::transmute::<__m128i,[i32;4]>(indices);
            return [indices[0] as usize, indices[1] as usize, indices[2] as usize, indices[3] as usize];
        }
    }
    #[cfg(not(all(target_feature="sse2", target_arch = "x86_64")))] {
        let prevprev = (i + n - 2) % n; 
        let prev = (i + n - 1) % n;
        let next = (i + 1) % n;
        let nextnext = (i + 2) % n;
        return [prevprev as usize,prev as usize,next as usize,nextnext as usize];
    }
}