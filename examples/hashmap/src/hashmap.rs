// hacspec implementation of creusot's hashmap
// [here](https://github.com/xldenis/creusot/blob/master/creusot/tests/should_succeed/hashmap.rs)
// (commit `bb201f5` on branch `master`).

use hacspec_lib::*;

pub type V = u64;
pub type K = V;
pub const HASH_SIZE: usize = 256;
pub const MAX_COLLISIONS: usize = 16;
// TODO allow HASH_SIZE and MAX_COLLISIONS to be parameters?
pub const MAX_SIZE: usize = HASH_SIZE * MAX_COLLISIONS;
array!(BucketsKeys, MAX_SIZE, K);
array!(BucketsVals, MAX_SIZE, V);
array!(BucketLen, HASH_SIZE, usize);
pub struct MyHashMap(BucketsKeys, BucketsVals, BucketLen, usize);

fn i1to2(i: usize) -> (usize, usize) {
  (i / MAX_COLLISIONS, i % MAX_COLLISIONS)
}

fn i2to1(i: usize, j: usize) -> usize {
  i * MAX_COLLISIONS + j
}

pub fn add(hm: MyHashMap, key: K, val: V) -> MyHashMap {
  let MyHashMap(mut keys, mut values, mut len, t) = hm;
  let index = (key % (t as K)) as usize;
  let bucketLen = len[index];

  let mut return_early = false;
  for i in 0..bucketLen {
    if !return_early {
      let j = i2to1(index, i);
      if keys[j] == key {
        values[j] = val;
        return_early = true;
      }
    }
  }

  // TODO resize on exceed MAX_COLLISIONS?
  if !return_early && bucketLen < MAX_COLLISIONS{
    let j = i2to1(index, bucketLen);
    keys[j] = key;
    values[j] = val;
    len[index] = bucketLen + 1;
  }
  MyHashMap(keys, values, len, t)
}

pub fn get(hm: MyHashMap, key: K) -> Option::<V> {
  let MyHashMap(keys, values, len, t) = hm;
  let index = (key % (t as K)) as usize;
  let bucketLen = len[index];

  let mut result = Option::<V>::None;
  let mut early_break = false;
  for i in 0..bucketLen {
    if !early_break {
      let j = i2to1(index, i);
      if keys[j] == key {
        result = Option::<V>::Some(values[j]);
        early_break = true;
      }
    }
  }
  result
}

// pub fn resize(hmFrom: MyHashMap, hmTo:MyHashMap) -> MyHashMap {
  
// }

#[test]
fn main() {
  let mut h1: MyHashMap = MyHashMap(
    BucketKeys([0; MAX_SIZE]),
    BucketVals([0; MAX_SIZE]),
    BucketLen([0; HASH_SIZE]),
    17,
  );
  let mut h2: MyHashMap = MyHashMap(
    BucketKeys([0; MAX_SIZE]),
    BucketVals([0; MAX_SIZE]),
    BucketLen([0; HASH_SIZE]),
    42,
  );
  let mut _x = get(h1, 1);
  let mut _y = get(h1, 2);
  let mut _z = get(h2, 1);
  let mut _t = get(h2, 2);
  assert!(_x == Option::<V>::None);
  assert!(_y == Option::<V>::None);
  assert!(_z == Option::<V>::None);
  assert!(_t == Option::<V>::None);

  h1 = add(h1, 1, 17);
  _x = get(h1, 1);
  _y = get(h1, 2);
  _z = get(h2, 1);
  _t = get(h2, 2);
  assert!(_x == Option::<V>::Some(17));
  assert!(_y == Option::<V>::None);
  assert!(_z == Option::<V>::None);
  assert!(_t == Option::<V>::None);

  h2 = add(h2, 1, 42);
  _x = get(h1, 1);
  _y = get(h1, 2);
  _z = get(h2, 1);
  _t = get(h2, 2);
  assert!(_x == Option::<V>::Some(17));
  assert!(_y == Option::<V>::None);
  assert!(_z == Option::<V>::Some(42));
  assert!(_t == Option::<V>::None);
}
// lines 217 - 247

// #[quickcheck]
// fn good_bucket(hm: MyHashMap, h: K) -> bool {
  // for all keys `k`
  // for all arbitrary value `v`
  // hm.get(k) == Some(v) implies k % length == h
  // thus h is an existential property of the predicate, it doesnt apply for all (not a quickcheck)
// }
// predicates
// fn no_double_binding // line 38
// fn good_bucket // line 200
// fn hashmap_inv // line 209