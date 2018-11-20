struct Poly {
    coeff: f32,
    exp: u32,
    rest: Option<Box<Poly>>,
}

impl Poly {
    fn _length(&self) -> i32 {
        match &self.rest {
            None => 1,
            Some(r) => 1 + r._length(),
        }
    }

    fn to_string(&self) -> String {
        let s = match &self.rest {
            None => String::new(),
            Some(r) => format!(" + {}", r.to_string()),
        };
        format!{"{}x^{}{}",
        &self.coeff.to_string(), &self.exp.to_string(), s}
    }

    fn evaluate(&self, x: f32) -> f32 {
        &self.coeff * f32::powi(x, self.exp as i32) + match &self.rest {
            None => 0.0,
            Some(r) => r.evaluate(x),
        }
    }

    fn differentiate(&self) -> Poly {
        if self.exp > 0 {
            Poly {
                coeff: &self.coeff * (self.exp as f32),
                exp: &self.exp - 1,
                rest: match &self.rest {
                    None => None,
                    Some(r) => Some(Box::new(r.differentiate())),
                },
            }
        } else if let Some(r) = &self.rest {
            r.differentiate()
        } else {
            Poly {
                coeff: 0.,
                exp: 0,
                rest: None,
            }
        }
    }

    fn newton(&self, x: f32, precision: f32) -> f32 {
        let precision = precision.abs();
        let deriv = self.differentiate();
        let x_new = x - self.evaluate(x) / deriv.evaluate(x);
        let delta = (x - x_new).abs();
        if delta < precision {
            (x + x_new) / 2.0
        } else {
            self.newton(x_new, precision)
        }
    }
    // add code here
}

fn main() {
    fn showpoly(p: &Poly, s: f32) {
        println!(
            "{} at {} converges to {}",
            p.to_string(),
            s,
            p.newton(s, 0.000001)
        );
    }

    //  x^3 2x^2 11x + 12 = 0
    let p1 = Poly {
        coeff: 1.0,
        exp: 3,
        rest: Some(Box::new(Poly {
            coeff: -2.0,
            exp: 2,
            rest: Some(Box::new(Poly {
                coeff: -11.0,
                exp: 1,
                rest: Some(Box::new(Poly {
                    coeff: 12.0,
                    exp: 0,
                    rest: None,
                })),
            })),
        })),
    };

    showpoly(&p1, -4.0);
    showpoly(&p1, 0.0);
    showpoly(&p1, 2.35287527);
    println!();

    //  x^3 - 2x^2 - 5x  +  6  = 0
    let p2 = Poly {
        coeff: 1.0,
        exp: 3,
        rest: Some(Box::new(Poly {
            coeff: -2.0,
            exp: 2,
            rest: Some(Box::new(Poly {
                coeff: -5.0,
                exp: 1,
                rest: Some(Box::new(Poly {
                    coeff: 6.0,
                    exp: 0,
                    rest: None,
                })),
            })),
        })),
    };

    showpoly(&p2, -3.0);
    showpoly(&p2, 0.0);
    showpoly(&p2, 4.0);
    println!();

    //2x^4 + 7x^3 + 6x^2 + 8x + 12 = 0
    let p3 = Poly {
        coeff: 2.0,
        exp: 4,
        rest: Some(Box::new(Poly {
            coeff: 7.0,
            exp: 3,
            rest: Some(Box::new(Poly {
                coeff: 6.0,
                exp: 2,
                rest: Some(Box::new(Poly {
                    coeff: 8.0,
                    exp: 1,
                    rest: Some(Box::new(Poly {
                        coeff: 12.0,
                        exp: 0,
                        rest: None,
                    })),
                })),
            })),
        })),
    };
    showpoly(&p3, 0.0);
    showpoly(&p3, -2.5);
}
