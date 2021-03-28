#![feature(test)]
#[allow(soft_unstable)]

/*
Output of cargo bench (on AMD 3900X):

running 6 tests
test bench_baseline_different                        ... bench:         300 ns/iter (+/- 2)
test bench_baseline_equal                            ... bench:       1,170 ns/iter (+/- 18)
test bench_manual_compare_non_shortcircuit_different ... bench:         942 ns/iter (+/- 10)
test bench_manual_compare_non_shortcircuit_equal     ... bench:       1,165 ns/iter (+/- 7)
test bench_u128_transmute_different                  ... bench:         475 ns/iter (+/- 3)
test bench_u128_transmute_equal                      ... bench:         470 ns/iter (+/- 6)



 */

extern crate test;
use test::Bencher;

#[derive(PartialEq,Clone,Copy)]
pub struct Blueprint {
    pub fuel_tank_size: u16,
    pub payload: u16,
    pub wheel_diameter: u32,
    pub wheel_width: u32,
    pub storage: u32,
}

pub fn manual_compare_non_shortcircuit(a:&Blueprint, b:&Blueprint) -> bool {
    (a.wheel_diameter == b.wheel_diameter) &
    (a.wheel_width == b.wheel_width) &
    (a.storage == b.storage) &
    (a.fuel_tank_size == b.fuel_tank_size) &
    (a.payload == b.payload)
}


pub fn u128_transmute_compare(a:&Blueprint, b:&Blueprint) -> bool {
    unsafe {
        let a: u128 = std::mem::transmute(*a);
        let b: u128 = std::mem::transmute(*b);
        a == b
    }
}

#[bench]
fn bench_baseline_equal(b:&mut Bencher) {
    let obj = Blueprint {
        fuel_tank_size: 0,
        payload: 0,
        wheel_diameter: 0,
        wheel_width: 0,
        storage: 0
    };
    let obj1s = vec![obj;1000];
    let obj2s = vec![obj;1000];

    b.iter(||{
        obj1s.iter().zip(obj2s.iter()).all(|(a,b)| a == b)
    })
}

#[bench]
fn bench_manual_compare_non_shortcircuit_equal(b:&mut Bencher) {
    let obj = Blueprint {
        fuel_tank_size: 0,
        payload: 0,
        wheel_diameter: 0,
        wheel_width: 0,
        storage: 0
    };
    let obj1s = vec![obj;1000];
    let obj2s = vec![obj;1000];

    b.iter(||{
        obj1s.iter().zip(obj2s.iter()).all(|(a,b)| manual_compare_non_shortcircuit(a, b))
    })
}


#[bench]
fn bench_u128_transmute_equal(b:&mut Bencher) {
    let obj = Blueprint {
        fuel_tank_size: 0,
        payload: 0,
        wheel_diameter: 0,
        wheel_width: 0,
        storage: 0
    };
    let obj1s = vec![obj;1000];
    let obj2s = vec![obj;1000];

    b.iter(||{
        obj1s.iter().zip(obj2s.iter()).all(|(a,b)| u128_transmute_compare(a, b))
    })
}

#[bench]
fn bench_baseline_different(b:&mut Bencher) {
    let mut obj = Blueprint {
        fuel_tank_size: 0,
        payload: 0,
        wheel_diameter: 0,
        wheel_width: 0,
        storage: 0
    };
    let obj1s = vec![obj;1000];
    obj.fuel_tank_size=1;
    let obj2s = vec![obj;1000];

    b.iter(||{
        obj1s.iter().zip(obj2s.iter()).any(|(a,b)| a == b)
    })
}

#[bench]
fn bench_manual_compare_non_shortcircuit_different(b:&mut Bencher) {
    let mut obj = Blueprint {
        fuel_tank_size: 0,
        payload: 0,
        wheel_diameter: 0,
        wheel_width: 0,
        storage: 0
    };
    let obj1s = vec![obj;1000];
    obj.fuel_tank_size=1;
    let obj2s = vec![obj;1000];

    b.iter(||{
        obj1s.iter().zip(obj2s.iter()).any(|(a,b)| manual_compare_non_shortcircuit(a, b))
    })
}


#[bench]
fn bench_u128_transmute_different(b:&mut Bencher) {
    let mut obj = Blueprint {
        fuel_tank_size: 0,
        payload: 0,
        wheel_diameter: 0,
        wheel_width: 0,
        storage: 0
    };
    let obj1s = vec![obj;1000];
    obj.fuel_tank_size=1;
    let obj2s = vec![obj;1000];

    b.iter(||{
        obj1s.iter().zip(obj2s.iter()).any(|(a,b)| u128_transmute_compare(a, b))
    })
}



fn main() {
}

