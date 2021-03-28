#![feature(test)]
#[allow(soft_unstable)]

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
fn bench_baseline(b:&mut Bencher) {
    let obj = Blueprint {
        fuel_tank_size: 0,
        payload: 0,
        wheel_diameter: 0,
        wheel_width: 0,
        storage: 0
    };
    let obj1s = vec![obj;1000];
    let obj2s = vec![obj;1000];
    b.iter(||obj1s==obj2s)
}

#[bench]
fn bench_manual_compare_non_shortcircuit(b:&mut Bencher) {
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
fn bench_u128_transmute(b:&mut Bencher) {
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

fn main() {
}

