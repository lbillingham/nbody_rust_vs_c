#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_default_instances_to_0() {
        let zeros = Vec3::default();
        assert!(zeros.x == 0.);
        assert!(zeros.y == 0.);
        assert!(zeros.z == 0.);
    }

    #[test]
    fn it_instantiates_with_args() {
        let some_vec = Vec3 {
            x: 1.,
            y: 10.,
            z: 100.,
        };
        assert!(some_vec.x == 1.);
        assert!(some_vec.y == 10.);
        assert!(some_vec.z == 100.);
    }

    #[test]
    fn it_supports_equality() {
        assert!(
            Vec3 {
                x: 1.,
                y: 10.,
                z: 100.,
            } == Vec3 {
                x: 1.,
                y: 10.,
                z: 100.,
            }
        )
    }

    #[test]
    fn it_can_be_constructed_from_a_3_array() {
        let some_vec = Vec3::from_3_array([1., 2., 3.]);
        assert!(some_vec.x == 1.);
        assert!(some_vec.y == 2.);
        assert!(some_vec.z == 3.);
    }

    #[test]
    fn it_can_be_accessed_like_3_array() {
        let some_vec = Vec3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        assert!(some_vec[0] == 1.);
        assert!(some_vec[1] == 2.);
        assert!(some_vec[2] == 3.);
    }

    #[test]
    fn it_can_be_added_to_another_vec3() {
        let unit = Vec3 {
            x: 1.,
            y: 1.,
            z: 1.,
        };
        let one_two_three = Vec3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };

        let added = unit + one_two_three;
        assert!(added.x == 2.);
        assert!(added.y == 3.);
        assert!(added.z == 4.);
    }

    #[test]
    fn it_can_be_subtracted_from_another_vec3() {
        let big = Vec3 {
            x: 10.,
            y: 100.,
            z: 1000.,
        };
        let small = Vec3 {
            x: 9.,
            y: 99.,
            z: 999.,
        };

        assert!(
            big - small
                == Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 1.
                }
        );
        assert!(
            small - big
                == Vec3 {
                    x: -1.,
                    y: -1.,
                    z: -1.
                }
        );
    }

    #[test]
    fn it_can_be_negated() {
        let v1 = Vec3 {
            x: 1.,
            y: 20.,
            z: 300.,
        };

        assert!(
            -v1 == Vec3 {
                x: -1.,
                y: -20.,
                z: -300.
            }
        );
    }

    #[test]
    fn it_can_be_multipled_by_an_f64() {
        let v1 = Vec3 {
            x: 1.,
            y: 20.,
            z: 300.,
        };

        assert!(
            v1.scale(10.)
                == Vec3 {
                    x: 10.,
                    y: 200.,
                    z: 3000.
                }
        );
    }

    // #[test]
    // fn it_can_be_iterated_over() {
    //     let v1 = Vec3 {
    //         x: 1.,
    //         y: 2.,
    //         z: 3.,
    //     };
    //     let expected = [v1.x, v1.y, v1.z];
    //     for (idx, cmpt) in v1.iter().enumerate() {
    //         assert!(cmpt == expected[idx])
    //     }
    // }

    #[test]
    fn it_can_be_dot_producted_with_another() {
        let v1 = Vec3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let v2 = Vec3 {
            x: 10.,
            y: 20.,
            z: 30.,
        };

        assert!(v1.dot(v2) == (1. * 10.) + (2. * 20.) + (3. * 30.));
        assert!(v2.dot(v1) == (1. * 10.) + (2. * 20.) + (3. * 30.));
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vec3 {
    fn default() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vec3 {
    fn from_3_array(arr: [f64; 3]) -> Vec3 {
        Vec3 {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn scale(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vec3 {}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            index => panic!(
                "Unknown value {} for index found: must be in 0, 1, 2",
                index
            ),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            index => panic!(
                "Unknown value {} for index found: must be in 0, 1, 2",
                index
            ),
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}
