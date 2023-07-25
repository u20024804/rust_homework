use std::ops::{Add, Sub};

// #[derive(Debug, Copy, Clone, PartialEq)]
trait Negative {
    fn negative(&self) -> bool;
}


trait XAdd<'a, Rhs: Add<&'a Rhs, Output=Rhs> + 'a>: Add<&'a Rhs, Output=Rhs> + 'a
    where &'a Rhs: Add<&'a Rhs, Output=Rhs> + 'a, &'a Self: Add<&'a Rhs, Output=Rhs> + 'a,
          <&'a Self as Add<&'a Rhs>>::Output: Add<&'a Rhs, Output=Rhs> + 'a {
    fn xadd(&'a self, rhs1: &'a Rhs, rhs2: &'a Rhs) -> Rhs {
        self + rhs1 + rhs2
    }
}

trait XSub<'a, Rhs: Sub<&'a Rhs, Output=Rhs> + 'a> : Sub<&'a Rhs, Output=Rhs>
    where &'a Rhs: Sub<&'a Rhs, Output=Rhs>, &'a Self: Sub<&'a Rhs, Output=Rhs> + 'a,
          <&'a Self as Sub<&'a Rhs>>::Output: Sub<&'a Rhs, Output=Rhs> {
    fn xsub(&'a self, rhs1: &'a Rhs, rhs2: &'a Rhs) -> Rhs {
        self - rhs1 - rhs2
    }
}

trait AddSub<'a, Rhs: Add<&'a Rhs, Output=Rhs> + Sub<&'a Rhs, Output=Rhs> + Negative + 'a> : Add<&'a Rhs, Output=Rhs> + Sub<&'a Rhs, Output=Rhs>
    where &'a Rhs: Add<&'a Rhs, Output = Rhs> + Sub<&'a Rhs, Output = Rhs>,
          &'a Self: Sub<&'a Rhs, Output=Rhs> + 'a, &'a Self: Add<&'a Rhs, Output=Rhs> + 'a,
          <&'a Self as Add<&'a Rhs>>::Output: Add<&'a Rhs, Output=Rhs>,
          <&'a Self as Sub<&'a Rhs>>::Output: Sub<&'a Rhs, Output=Rhs> {
    fn add_sub(&'a self, rhs1: &'a Rhs, rhs2: &'a Rhs) -> Rhs {
        self + rhs1 - rhs2
    }
}

trait XAddSub<'a, Rhs: Add<&'a Rhs, Output=Rhs> + Sub<&'a Rhs, Output=Rhs> + Negative + 'a> : XAdd<'a, Rhs> + XSub<'a, Rhs>
    where &'a Rhs: Add<&'a Rhs, Output = Rhs> + Sub<&'a Rhs, Output = Rhs>,
          <&'a Self as Add<&'a Rhs>>::Output: Add<&'a Rhs, Output = Rhs>,
          <&'a Self as Sub<&'a Rhs>>::Output: Sub<&'a Rhs, Output = Rhs>,
          &'a Self: Add<&'a Rhs, Output = Rhs> + Sub<&'a Rhs, Output = Rhs> + 'a {
    fn add_sub_add(&'a self, rhs1: &'a Rhs, rhs2: &'a Rhs, rhs3: &'a Rhs) -> Rhs {
        self + rhs1 -  rhs2 + rhs3
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct XY {
    pub x : i32,
    pub y : i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct AB {
    pub a : i32,
    pub b : i32,
}


impl<'a> Add<&'a XY> for XY {
    type Output = XY;

    fn add(self, rhs: &'a XY) -> XY {
        XY { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl<'a> Add<&'a XY> for &'a XY {
    type Output = XY;

    fn add(self, rhs: &'a XY) -> XY {
        XY { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<'a> XAdd<'a, XY> for XY {
}

impl<'a> Sub<&'a XY> for XY {
    type Output = XY;

    fn sub(self, rhs: &XY) -> XY {
        XY{ x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl<'a> Sub<&'a XY> for &'a XY {
    type Output = XY;

    fn sub(self, rhs: &XY) -> XY {
        XY{ x: self.x - rhs.x, y: self.y - rhs.y }
    }
}


impl<'a> XSub<'a, XY> for XY {}

impl<'a> XAddSub<'a, XY> for XY {}

impl Add<AB> for AB {
    type Output = AB;

    fn add(self, rhs: AB) -> AB {
        AB{ a: self.a + rhs.a, b: self.b + rhs.b }
    }
}

impl<'a> Add<&'a AB> for AB {
    type Output = AB;

    fn add(self, rhs: &'a AB) -> AB {
        AB{ a: self.a + rhs.a, b: self.b + rhs.b }
    }
}

impl<'a> Add<&'a AB> for &'a AB {
    type Output = AB;

    fn add(self, rhs: &'a AB) -> AB {
        AB{ a: self.a + rhs.a, b: self.b + rhs.b }
    }
}

impl<'a> XAdd<'a, AB> for AB {}

impl<'a> Sub<&'a AB> for AB {
    type Output = AB;

    fn sub(self, rhs: &'a AB) -> AB {
        AB{ a: self.a - rhs.a, b: self.b - rhs.b }
    }
}

impl<'a> Sub<&'a AB> for &'a AB {
    type Output = AB;

    fn sub(self, rhs: &'a AB) -> AB {
        AB{ a: self.a - rhs.a, b: self.b - rhs.b }
    }
}

impl Sub<AB> for AB {
    type Output = AB;

    fn sub(self, rhs: AB) -> AB {
        AB{ a: self.a - rhs.a, b: self.b - rhs.b }
    }
}

impl<'a> XSub<'a, AB> for AB {}


impl<'a> XAddSub<'a, AB> for AB {

}

// impl<'a> Add<&'a (dyn Negative + 'a)> for &'a &'a AB {
//     type Output = dyn Negative + 'a;
//
//     fn add(self, rhs: &'a (dyn Negative + 'a)) -> Self::Output {
//         rhs
//     }
// }
//
// impl<'a> XAdd<'a, dyn Negative + 'a + ?Sized> for &'a AB {}
//
//
// impl<'a> Add<&'a (dyn Negative + 'a)> for (dyn Negative + 'a) {
//     type Output = dyn Negative + 'a;
//
//     fn add(self, rhs: &'a (dyn Negative + 'a)) -> Self::Output {
//         rhs
//     }
// }
//
// impl<'a> Add<&'a (dyn Negative + 'a)> for &'a (dyn Negative + 'a) {
//     type Output = dyn Negative + 'a;
//
//     fn add(self, rhs: &'a (dyn Negative + 'a)) -> Self::Output {
//         rhs
//     }
// }
//
// impl<'a> XAdd<'a, dyn Negative + 'a> for &'a (dyn Negative + 'a) {}
// impl<'a> XAdd<'a, dyn Negative + 'a> for AB {}
//
// impl<'a> Add<&'a (dyn Negative + 'a)> for AB {
//     type Output = dyn Negative + 'a;
//
//     fn add(self, rhs: &'a (dyn Negative + 'a)) -> Self::Output {
//         rhs
//     }
// }
//
// impl<'a> XSub<'a, dyn Negative + 'a> for AB {}
//
// impl<'a> Sub<&'a (dyn Negative + 'a)> for AB {
//     type Output = dyn Negative + 'a;
//
//     fn sub(self, rhs: &'a (dyn Negative + 'a)) -> Self::Output {
//         rhs
//     }
// }
//
// impl<'a> XAddSub<'_, dyn Negative + 'a> for AB {
//
// }

impl Negative for XY {
    fn negative(&self) -> bool {
        self.x < 0 && self.y < 0
    }
}

impl Negative for AB {
    fn negative(&self) -> bool {
        self.a < 0 && self.b > 0
    }
}

fn test_neg(neg: &dyn Negative) -> bool {
    neg.negative()
}

pub fn do_q5() {
    let a = AB{a: 1, b: 2};
    let b = AB{a: 3, b: 7};
    let c = AB{a: 9, b: 17};
    println!("{:?}", a + &b);
    println!("{:?}", a - b);
    println!("{:?}", a.xadd(&b, &c));

    let x = XY {x: 3, y: 8};
    let y = XY {x: 11, y: 10};
    let z = XY {x: 33, y:64};

    let ab = AB {a: 123, b: 456};
    let cd = AB {a: 78, b: 91011};
    let ef = XY {x: 1, y: 234567};
    let jk = AB {a: 1, b: 234567};

    let m : &dyn XAddSub<AB> = &ab;
    let n : &dyn XAddSub<AB> = &cd;
    let p : &dyn XAddSub<XY> = &ef;
    let q : &dyn XAddSub<AB> = &jk;

    test_neg(&x);
    test_neg(&ab);
    // test_neg(m);

    println!("{:?}", ab);
    println!("{:?}", cd);
    println!("{:?}", ef);

    println!("{:?}", ab.add(&cd));
    println!("{:?}", ab.xadd(&cd, &jk));

    // p = &cd;
    // let n : dyn XAddSub<&dyn Negative, Output=dyn Negative> = AB {a: 111, b: 22};
    // let p : dyn XAddSub<&dyn Negative, Output=dyn Negative> = XY {x: 3, y: 8};

    // println!("{:?}", m + n);
    // println!("{:?}", m.xadd(n, p));
}
