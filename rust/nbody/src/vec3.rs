#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_default_instances_to_0() {
        let zeros = Vec3::default();
        assert!(zeros[0] == 0.);
        assert!(zeros[1] == 0.);
        assert!(zeros[2] == 0.);
    }

    #[test]
    fn it_instantiates_with_args() {
        let some_vec = Vec3([1., 10., 100.]);
        assert!(some_vec[0] == 1.);
        assert!(some_vec[1] == 10.);
        assert!(some_vec[2] == 100.);
    }

    #[test]
    fn it_supports_equality() {
        assert!(Vec3([1., 10., 100.]) == Vec3([1., 10., 100.]))
    }

    #[test]
    fn it_can_be_accessed_like_3_array() {
        let some_vec = Vec3([1., 2., 3.]);
        assert!(some_vec[0] == 1.);
        assert!(some_vec[1] == 2.);
        assert!(some_vec[2] == 3.);
    }

    #[test]
    fn it_can_be_added_to_another_vec3() {
        let unit = Vec3([1., 1., 1.]);
        let one_two_three = Vec3([1., 2., 3.]);

        let added = unit + one_two_three;
        assert!(added[0] == 2.);
        assert!(added[1] == 3.);
        assert!(added[2] == 4.);
    }

    #[test]
    fn it_can_be_subtracted_from_another_vec3() {
        let big = Vec3([10., 100., 1000.]);
        let small = Vec3([9., 99., 999.]);

        assert!(big - small == Vec3([1., 1., 1.]));
        assert!(small - big == Vec3([-1., -1., -1.]));
    }

    #[test]
    fn it_can_be_negated() {
        let v1 = Vec3([1., 20., 300.]);

        assert_eq!(-v1, Vec3([-1., -20., -300.]));
    }

    #[test]
    fn it_can_be_multipled_by_an_f64() {
        let v1 = Vec3([1., 20., 300.]);

        assert!(v1.scale(10.) == Vec3([10., 200., 3000.]));
    }

    #[test]
    fn it_can_be_iterated_over() {
        let v1 = Vec3([1., 2., 3.]);
        let expected = [v1[0], v1[1], v1[2]];
        for (index, element) in v1.iter().enumerate() {
            assert_eq!(element, &expected[index]);
        }
    }

    #[test]
    fn it_can_be_dot_producted_with_another() {
        let v1 = Vec3([1., 2., 3.]);
        let v2 = Vec3([10., 20., 30.]);

        assert!(v1.dot(v2) == (1. * 10.) + (2. * 20.) + (3. * 30.));
        assert!(v2.dot(v1) == (1. * 10.) + (2. * 20.) + (3. * 30.));
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub [f64; 3]);

impl Default for Vec3 {
    fn default() -> Self {
        Vec3([0.0, 0.0, 0.0])
    }
}

impl Vec3 {
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3([x, y, z])
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn scale(self, scalar: f64) -> Vec3 {
        Vec3([scalar * self[0], scalar * self[1], scalar * self[2]])
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64>{
        self.0.iter_mut()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3([self[0] + other[0], self[1] + other[1], self[2] + other[2]])
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3([self[0] - other[0], self[1] - other[1], self[2] - other[2]])
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3([-self[0], -self[1], -self[2]])
    }
}

impl std::cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self[0] == other[0] && self[1] == other[1] && self[2] == other[2]
    }
}

impl Eq for Vec3 {}

impl std::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.0[0],
            1 => &self.0[1],
            2 => &self.0[2],
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
            0 => &mut self.0[0],
            1 => &mut self.0[1],
            2 => &mut self.0[2],
            index => panic!(
                "Unknown value {} for index found: must be in 0, 1, 2",
                index
            ),
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3([self[0] + other[0], self[1] + other[1], self[2] + other[2]])
    }
}
