// This Rust source file is a multiple threaded implementation to perform an
// extremely fast Segmented Sieve of Zakiya (SSoZ) to find Twin Primes <= N.

// Inputs are single values N, or ranges N1 and N2, of 64-bits, 0 -- 2^64 - 1.
// Output is the number of twin primes <= N, or in range N1 to N2; the last
// twin prime value for the range; and the total time of execution.

// This code was developed on a System76 laptop with an Intel I7 6700HQ cpu,
// 2.6-3.5 GHz clock, with 8 threads, and 16GB of memory. Parameter tuning
// probably needed to optimize for other hardware systems (ARM, PowerPC, etc).

// Can compile as: $ cargo build --release
// or: $ RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release
// The later compilation creates faster runtime on my system.
// To reduce binary size in target/release/ do: $ strip twinprimes_ssoz
// Single val: $ echo val1 | ./twinprimes_ssoz
// Range vals: $ echo val1 val2 | ./twinprimes_ssoz

// Mathematical and technical basis for implementation are explained here:
// https://www.academia.edu/37952623/The_Use_of_Prime_Generators_to_Implement_Fast_
// Twin_Primes_Sieve_of_Zakiya_SoZ_Applications_to_Number_Theory_and_Implications_
// for_the_Riemann_Hypotheses
// https://www.academia.edu/7583194/The_Segmented_Sieve_of_Zakiya_SSoZ_
// https://www.academia.edu/19786419/PRIMES-UTILS_HANDBOOK

// This source code, and updates, can be found here:
// https://gist.github.com/jzakiya/b96b0b70cf377dfd8feb3f35eb437225

// Significant contributions provided from https://users.rust-lang.org/
// This code is provided free and subject to copyright and terms of the
// GNU General Public License Version 3, GPLv3, or greater.
// License copy/terms are here: http://www.gnu.org/licenses/

// Copyright (c) 2017-2021; Jabari Zakiya -- jzakiya at gmail dot com
// Last update: 2021/3/17

extern crate rayon;
extern crate num_cpus;
extern crate integer_sqrt;
use rayon::prelude::*;
use std::time::SystemTime;
use integer_sqrt::IntegerSquareRoot;
use std::sync::atomic::{self, AtomicUsize};

// A counter implemented using relaxed (unsynchronized) atomic operations.
struct RelaxedCounter(AtomicUsize);
impl RelaxedCounter {
  fn new() -> Self { RelaxedCounter(AtomicUsize::new(0)) }
  /// Increment and get the new value.
  fn increment(&self) -> usize {
    self.0.fetch_add(1, atomic::Ordering::Relaxed) + 1 }
}

fn print_time(title: &str, time: SystemTime) {
  print!("{} = ", title);
  println!("{} secs", {
    match time.elapsed() {
      Ok(e)  => { e.as_secs() as f64 + e.subsec_nanos() as f64 / 1_000_000_000f64 },
      Err(e) => { panic!("Timer error {:?}", e) },
    }
  });
}

// Customized gcd for prime generators; n > m; m odd
fn gcd(mut m: usize, mut n: usize) -> usize {
  while m|1 != 1 { let t = m; m = n % m; n = t }
  m
}

// Compute modular inverse a^-1 to base m, e.g. a*(a^-1) mod m = 1
fn modinv(a0: usize, m0: usize) -> usize {
  if m0 == 1 { return 1 }
  let (mut a, mut m) = (a0 as isize, m0 as isize);
  let (mut x0, mut inv) = (0, 1);
  while a > 1 {
    inv -= (a / m) * x0;
    a = a % m;
    std::mem::swap(&mut a, &mut m);
    std::mem::swap(&mut x0, &mut inv);
  }
  if inv < 0 { inv += m0 as isize }
  inv as usize
}

fn gen_pg_parameters(prime: usize) -> (usize, usize, usize, Vec<usize>, Vec<usize>) {
  // Create prime generator parameters for given Pn
  println!("using Prime Generator parameters for P{}", prime);
  let primes: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];
  let (mut modpg, mut res_0) = (1, 0);      // compute Pn's modulus and res_0 value
  for prm in primes { res_0 = prm; if prm > prime { break };  modpg *= prm }

  let mut restwins: Vec<usize> = vec![];    // save upper twinpair residues here
  let mut inverses = vec![0usize; modpg+2]; // save Pn's residues inverses here
  let (mut pc, mut inc, mut res) = (5,2,0); // use P3's PGS to generate pcs
  while pc < modpg / 2 {                    // find a residue, then complement|inverse
    if gcd(pc, modpg) == 1 {                // if pc a residue
      inverses[pc] = modinv(pc, modpg);     // save its inverse
      let mc = modpg - pc;                  // create its modular complement
      inverses[mc] = modinv(mc, modpg);     // save its inverse
      if res + 2 == pc { restwins.push(pc); restwins.push(mc + 2) } // save hi_tp residues
      res = pc;                             // save current found residue
    }
    pc += inc; inc ^= 0b110;                // create next P3 sequence pc: 5 7 11 13 17 19 ...
  }
  restwins.sort();         restwins.push(modpg + 1);        // last residue is last hi_tp
  inverses[modpg + 1] = 1; inverses[modpg - 1] = modpg - 1; // last 2 residues are self inverses
  (modpg, res_0, restwins.len(), restwins, inverses)
}

fn set_sieve_parameters(start_num: usize, end_num: usize) ->
  (usize, usize, usize, usize, usize, usize, usize, Vec<usize>, Vec<usize>) {
  // Select at runtime best PG and segment size parameters for input values.
  // These are good estimates derived from PG data profiling. Can be improved.
  let nrange = end_num - start_num;
  let bn: usize; let pg: usize;
  if end_num < 49 {
    bn = 1; pg = 3;
  } else if nrange < 10_000_000 {
    bn = 16; pg = 5;
  } else if nrange <  1_100_000_000 {
    bn = 32; pg = 7;
  } else if nrange < 35_500_000_000 {
    bn = 64; pg = 11;
  } else if nrange < 15_000_000_000_000 {
    pg = 13;
    if      nrange > 7_000_000_000_000 { bn = 384; }
    else if nrange > 2_500_000_000_000 { bn = 320; }
    else if nrange >   250_000_000_000 { bn = 196; }
    else { bn = 128; }
  } else {
    bn = 384; pg = 17;
  }
  let (modpg, res_0, pairscnt, restwins, resinvrs) = gen_pg_parameters(pg);
  let kmin = (start_num-2) / modpg + 1; // number of resgroups to start_num
  let kmax = (end_num - 2) / modpg + 1; // number of resgroups to end_num
  let krange = kmax - kmin + 1;         // number of resgroups in range, at least 1
  let n = if krange < 37_500_000_000_000 { 4 } else if krange < 975_000_000_000_000 { 6 } else { 8 };
  let b = bn * 1024 * n;                // set seg size to optimize for selected PG
  let kb = if krange < b { krange } else { b }; // segments resgroups size

  println!("segment size = {} resgroups; seg array is [1 x {}] 64-bits", kb, ((kb-1) >> 6) + 1);
  let maxpairs = krange * pairscnt;     // maximum number of twinprime pcs
  println!("twinprime candidates = {}; resgroups = {}", maxpairs, krange);
  (modpg, res_0, kb, kmin, kmax, krange, pairscnt, restwins, resinvrs)
}

fn sozpg(val: usize, res_0: usize) -> Vec<usize> {
  // Compute the primes r0..sqrt(input_num) and store in 'primes' array.
  // Any algorithm (fast|small) is usable. Here the SoZ for P5 is used.
  let (md, rscnt) = (30, 8);        // P5's modulus and residues count
  static RES: [usize; 8] = [7,11,13,17,19,23,29,31];
  static POSN: [usize; 30] = [0,0,0,0,0,0,0,0,0,1,0,2,0,0,0,3,0,4,0,0,0,5,0,0,0,0,0,6,0,7];

  let kmax = (val - 7) / md + 1;    // number of resgroups upto input value
  let mut prms = vec![0u8; kmax];   // byte array of prime candidates, init '0'
  let sqrt_n = val.integer_sqrt();  // compute integer sqrt of val
  let (mut modk, mut r, mut k) = (0, 0, 0 ); // initialize residue parameters

  loop {                            // for r0..sqrtN primes mark their multiples
    if r == rscnt { r = 0; modk += md; k += 1 }
    if (prms[k] & (1 << r)) != 0 { r += 1; continue } // skip pc if not prime
    let prm_r = RES[r];             // if prime save its residue value
    let prime = modk + prm_r;       // numerate the prime value
    if  prime > sqrt_n { break }    // we're finished when it's > sqrtN
    for ri in &RES {                // mark prime's multiples in prms
      let prod = prm_r * ri - 2;    // compute cross-product for prm_r|ri pair
      let bit_r = 1 << POSN[prod % md];           // bit mask for prod's residue
      let mut kpm = k * (prime + ri) + prod / md; // 1st resgroup for prime mult
      while kpm < kmax { prms[kpm] |= bit_r; kpm += prime };
    }
    r += 1;
  }
  // prms now contains the nonprime positions for the prime candidates r0..N
  // extract primes into array 'primes'
  let mut primes = vec![];          // create empty dynamic array for primes
  for (k, resgroup) in prms.iter().enumerate() { // for each kth residue group
    for (i, r_i) in RES.iter().enumerate() { // check for each ith residue
      if resgroup & (1 << i) == 0 { // if bit location a prime
        let prime = md * k + r_i;   // numerate its value, store if in range
        if prime >= res_0 && prime <= val { primes.push(prime); }
  } } }
  primes
}

fn nextp_init(rhi: usize, kmin: usize, modpg: usize,
  primes: &[usize], resinvrs: &[usize]) -> Vec<usize> {
  // Initialize 'nextp' array for twinpair upper residue rhi in 'restwins'.
  // Compute 1st prime multiple resgroups for each prime r0..sqrt(N) and
  // store consecutively as lo_tp|hi_tp pairs for their restracks.
  let mut nextp = vec![0usize; primes.len() * 2]; // 1st mults array for twinpair
  let (r_hi, r_lo) = (rhi, rhi - 2);     // upper|lower twinpair residue values
  for (j, prime) in primes.iter().enumerate() { // for each prime r0..sqrt(N)
    let k = (prime - 2) / modpg;         // find the resgroup it's in
    let r = (prime - 2) % modpg + 2;     // and its residue value
    let r_inv = resinvrs[r];             // and residue inverse
    let mut ri = (r_lo * r_inv - 2) % modpg + 2;  // compute r's ri for r_lo
    let mut ki = k * (prime + ri) + (r * ri - 2) / modpg; // and 1st mult
    if ki < kmin { ki = (kmin - ki) % prime; if ki > 0 { ki = prime - ki } }
    else { ki = ki - kmin };
    nextp[j * 2] = ki;      // prime's 1st mult resgroup val in range for lo_tp
    ri = (r_hi * r_inv - 2) % modpg + 2;          // compute r's ri for r_hi
    ki = k * (prime + ri) + (r * ri - 2) / modpg; // and 1st mult resgroup
    if ki < kmin { ki = (kmin - ki) % prime; if ki > 0 { ki = prime - ki } }
    else { ki = ki - kmin };
    nextp[j * 2 | 1] = ki;  // prime's 1st mult resgroup val in range for hi_tp
  nextp
}

fn twins_sieve(r_hi: usize, kmin: usize, kmax: usize, kb: usize, start_num: usize,
  end_num: usize, modpg: usize, primes: &[usize], resinvrs: &[usize]) -> (usize, usize) {
  // Perform in thread the ssoz for given twinpair residues for Kmax resgroups.
  // First create|init 'nextp' array of 1st prime mults for given twinpair,
  // stored consequtively in 'nextp', and init seg array for kb resgroups.
  // For sieve, mark resgroup bits to '1' if either twinpair restrack is nonprime
  // for primes mults resgroups, and update 'nextp' restrack slices acccordingly.
  // Return the last twinprime|sum for the range for this twinpair residues.
  // For speed, disable runtime seg array bounds checking; using 64-bit elem seg array
  unsafe {                                               // allow fast array indexing
    type MWord = u64;                                    // mem size for 64-bit cpus
    const S: usize = 6;                                  // shift value for 64 bits
    const BMASK: usize = (1 << S) - 1;                   // bitmask val for 64 bits
    let (mut sum, mut ki, mut kn) = (0usize, kmin-1, kb);// init these parameters
    let (mut hi_tp, mut k_max) = (0usize, kmax);         // max twinprime|resgroup val
    let mut seg = vec![0 as MWord; ((kb - 1) >> S) + 1]; // seg array for kb resgroups
    if ((ki * modpg) + r_hi - 2)  < start_num { ki += 1; }    // ensure lo tp in range
    if ((k_max - 1) * modpg + r_hi) > end_num { k_max -= 1; } // ensure hi tp in range
    let mut nextp = nextp_init(r_hi, ki, modpg, primes, resinvrs); // init nextp array
    while ki < k_max {                                 // for kb size slices upto kmax
      if kb > (k_max - ki) { kn = k_max - ki }         // adjust kb size for last seg
      for (j, prime) in primes.iter().enumerate() {    // for each prime r0..sqrt(N)
                                                       // for lower twinpair residue track
        let mut k = *nextp.get_unchecked(j * 2);  // starting from this resgroup in seg
        while k < kn  {                           // mark primenth resgrouup bits prime mults
          *seg.get_unchecked_mut(k >> S) |= 1 << (k & BMASK);
          k += prime; }                           // set resgroup for prime's next multiple
        *nextp.get_unchecked_mut(j * 2) = k - kn; // save 1st resgroup in next eligible seg
                                                  // for upper twinpair residue track
        k = *nextp.get_unchecked_mut(j * 2 | 1);  // starting from this resgroup in seg
        while k < kn  {                           // mark primenth resgroup bits prime mults
          *seg.get_unchecked_mut(k >> S) |= 1 << (k & BMASK);
          k += prime; }                           // set resgroup for prime's next multiple
        *nextp.get_unchecked_mut(j * 2 | 1) = k - kn; // save 1st resgroup in next eligible seg
      }                                           // need to set as nonprime unused bits in last
                                                  // mem of last seg; so fast, do for every seg
      *seg.get_unchecked_mut((kn - 1) >> S) |= (!0u64 << ((kn - 1) & BMASK)) << 1;
      let mut cnt = 0usize;                       // initialize segment twinprimes count
                                                  // then count the twinprimes in segment
      for &m in &seg[0..=(kn - 1) >> S] { cnt += m.count_zeros() as usize; }
      if cnt > 0 {              // if segment has twinprimes
        sum += cnt;             // add segment count to total range count
        let mut upk = kn - 1;   // from end of seg, count back to largest tp
        while *seg.get_unchecked(upk >> S) & (1 << (upk & BMASK)) != 0 { upk -= 1 }
        hi_tp = ki + upk;       // set resgroup value for largest tp in seg
      }
      ki += kb;                 // set 1st resgroup val of next seg slice
      if ki < k_max { seg.iter_mut().for_each(|m| *m = 0); } // set seg to all primes
    }                           // when sieve done, numerate largest twinprime in range
                                // for small ranges w/o twins, set largest to 1
    hi_tp = if r_hi > end_num || sum == 0 { 1 } else { hi_tp * modpg + r_hi };
    (hi_tp, sum)
  }
}

fn main() {
  let mut val = String::new();    // Inputs are 1 or 2 range values < 2**64
  std::io::stdin().read_line (&mut val).expect("Failed to read line");
  let mut substr_iter = val.split_whitespace();
  let mut next_or_default = |def| -> usize {
      substr_iter.next().unwrap_or(def).parse().expect("Input is not a number")
  };
  let mut end_num = std::cmp::max(next_or_default("3"), 3);  // min vals 3
  let mut start_num = std::cmp::max(next_or_default("3"), 3);
  if start_num > end_num { std::mem::swap(&mut end_num, &mut start_num) }

  println!("threads = {}", num_cpus::get());
  let ts = SystemTime::now();      // start timing sieve setup execution

  start_num |= 1;                  // if start_num even increase by 1
  end_num = (end_num - 1) | 1;     // if end_num even decrease by 1
                                   // select Pn, set sieving params for inputs
  let (modpg, res_0, kb, kmin, kmax, krange,
       pairscnt, restwins, resinvrs) = set_sieve_parameters(start_num, end_num);

  let primes: Vec<usize> = if end_num < 49 { vec![5] } // gen sieve primes
      else { sozpg(end_num.integer_sqrt(), res_0) };   // <= sqrt(end_num)

  println!("each of {} threads has nextp[2 x {}] array", pairscnt, primes.len());

  let mut twinscnt = 0usize;       // init twinprimes range count
  let lo_range = restwins[0] - 3;  // lo_range = lo_tp - 1
  for tp in &[3, 5, 11, 17] {      // excluded low tp values PGs used
    if end_num == 3 { break };     // if 3 end of range, no twinprimes
    if tp >= &start_num && tp <= &lo_range { twinscnt += 1 }; // cnt any small tps
  }

  print_time("setup time", ts);    // sieve setup time
  println!("perform twinprimes ssoz sieve");
  let t1 = SystemTime::now();      // start timing ssoz sieve execution
                                   // sieve each twinpair restracks in parallel
  let (lastwins, cnts): (Vec<_>, Vec<_>) = { // store outputs in these arrays
    let counter = RelaxedCounter::new();
    restwins.par_iter().map( |r_hi| {
      let out = twins_sieve(*r_hi, kmin, kmax, kb, start_num, end_num, modpg,
                            &primes.to_vec(), &resinvrs);
      print!("\r{} of {} twinpairs done", counter.increment(), pairscnt);
      out
    }).unzip()
  };
  let mut last_twin = 0usize;      // find largest twinprime|cnts in range
  for (i, cnt_i) in cnts.iter().enumerate() {
    twinscnt += cnt_i;
    if last_twin < lastwins[i] { last_twin = lastwins[i]; }
  }
  if end_num == 5 && twinscnt == 1 { last_twin = 5; }
  let mut kn = krange % kb;        // set number of resgroups in last slice
  if kn == 0 { kn = kb };          // if multiple of seg size set to seg size

  print_time("\nsieve time", t1);  // ssoz sieve time
  print_time("total time", ts);    // setup + sieve time
  println!("last segment = {} resgroups; segment slices = {}", kn, (krange - 1)/kb + 1);
  println!("total twins = {}; last twin = {}+/-1", twinscnt, last_twin - 1);
}