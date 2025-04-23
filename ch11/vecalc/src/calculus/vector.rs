use crate::calculus::matrix::Matrix;

#[derive(Clone)]
pub struct Vector {
    components: Vec<f64>,
    name: String,
}

#[macro_export]
macro_rules! V {
        ($($x:expr), *) => {
            {
                let mut v: Vector = Vector{name: "nothing".to_string(), components: Vec::new()};
                $(
                    v.components.push($x);
                )*
                v
            }
        };
}

impl Vector {
    pub fn new(name: &String, v: Vec<f64>) -> Vector {
        Vector {
            components: v,
            name: name.clone(),
        }
    }

    pub fn zero(dimension: usize) -> Vector {
        Vector {
            name: "nothing".to_string(),
            components: vec![0.0; dimension],
        }
    }

    pub fn ones(dimension: usize) -> Vector {
        Vector {
            name: "nothing".to_string(),
            components: vec![1.0; dimension],
        }
    }

    pub fn eye(one_index: usize, dimension: usize) -> Option<Vector> {
        if one_index > dimension {
            return None;
        }
        let mut v = Vector::zero(dimension);
        v.components[one_index - 1] = 1.0;
        Some(v)
    }

    pub fn plus_cv(&self, v: &Vector, c: f64) -> Option<Vector> {
        let n = self.components.len();
        if n != v.components.len() {
            return None;
        }
        let mut u: Vector = Vector::zero(n);
        for i in 0..n {
            u.components[i] = self.components[i] + c * v.components[i];
        }
        u.name = format!(
            "{}{}{}",
            self.name,
            if c != 1.0 {
                if c != -1.0 {
                    String::from(" + ") + &c.to_string()
                } else {
                    String::from(" - ")
                }
            } else {
                String::from(" + ")
            },
            v.name
        );
        Some(u)
    }

    pub fn add(&self, v: &Vector) -> Option<Vector> {
        self.plus_cv(v, 1.0)
    }

    pub fn sub(&self, v: &Vector) -> Option<Vector> {
        self.plus_cv(v, -1.0)
    }

    pub fn dot(&self, v: &Vector) -> Option<f64> {
        let n = self.components.len();
        if n != v.components.len() {
            return None;
        }
        let mut u: f64 = 0.0;
        for i in 0..n {
            u += self.components[i] * v.components[i];
        }
        Some(u)
    }

    pub fn multiply(&self, v: &Vector) -> Option<Vector> {
        let n = self.components.len();
        if n != v.components.len() {
            return None;
        }
        let mut u: Vector = Vector::zero(n);
        for i in 0..n {
            u.components[i] = self.components[i] * v.components[i];
        }
        u.name = format!("{} * 1{}", self.name, v.name);
        Some(u)
    }

    pub fn cross(&self, v: &Vector) -> Matrix {
        let mut outer_product: Matrix =
            Matrix::new(&format!("{} x {}", self.name, v.name), Vec::new());
        for x in &self.components {
            outer_product.extend(&v.map(*x, 0.0))
        }
        outer_product
    }

    pub fn map(&self, multiply_by: f64, increment_by: f64) -> Vector {
        let mut u: Vector = Vector::zero(self.components.len());
        u.name = format!(
            "{}{}{}",
            if multiply_by != 1.0 {
                if multiply_by != -1.0 {
                    multiply_by.to_string()
                } else {
                    String::from("-")
                }
            } else {
                String::from("")
            },
            self.name,
            if increment_by != 0.0 {
                format!("+ {}", increment_by)
            } else {
                String::from("")
            }
        );
        for (i, &xi) in self.components.iter().enumerate() {
            u.components[i] = multiply_by * xi + increment_by;
        }
        u
    }

    pub fn update(&mut self, new_components: Vec<f64>) {
        self.components = new_components;
    }

    pub fn as_row(&self) -> String {
        let mut representation: String = String::new();

        for xi in &self.components {
            representation = format!(
                "{}{}{}",
                representation,
                if representation.is_empty() { "" } else { ", " },
                if xi.fract() != 0.0 {
                    xi.to_string()
                } else {
                    (xi.trunc() as i64).to_string()
                }
            )
        }
        representation
    }

    pub fn to_string(&self) -> String {
        format!("({})", self.as_row())
    }

    pub fn definition_string(&self) -> String {
        format!("{} = {}", self.name, self.to_string())
    }

    pub fn clone(&self) -> Self {
        Vector::new(&format!("{}_copy", self.name), self.components.clone())
    }

    pub fn at(&self, index: usize) -> f64 {
        if index >= self.components.len() {
            panic!("Invalid vector index!")
        }
        self.components[index]
    }

    pub fn size(&self) -> usize {
        self.components.len()
    }
}

pub fn _2d(x: f64, y: f64) -> Vector {
    Vector::new(&"nothing".to_string(), vec![x, y])
}

#[cfg(test)]
mod tests {
    use crate::calculus::vector::Vector;

    #[test]
    fn create_vec_macro() {
        let v: Vector = V!(1.0, 2.0, 3.0, 4.0);
        assert!(v.size() == 4);
        assert_eq!(v.name, "nothing");
        assert!(
            v.to_string().eq("(1, 2, 3, 4)"),
            "Invalid vector output: {}",
            v.to_string()
        )
    }

    #[test]
    fn sub_is_not_substitutable() {
        let mut v: Vector = V!(1.0, 2.0, 3.0, 4.0);
        v.name = String::from("v1");
        let v2 = Vector::new(&String::from("v2"), vec![4.0, 3.0, 2.0, 1.0]);
        let v_v2 = v.sub(&v2);
        let v2_v = v2.sub(&v);
        if let Some(v_v2) = v_v2 {
            if let Some(v2_v) = v2_v {
                assert_eq!(v_v2.size(), v2_v.size());
                for i in 0..(v.components.len()) {
                    assert_ne!(
                        v_v2.at(i),
                        v2_v.at(i),
                        "Invalid result, v1 - v2 was equal to v2 - v1 ! {}, {}",
                        v.definition_string(),
                        v2.definition_string()
                    )
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn mismatch_sized_dot() {
        let v1 = V!(1.0, 2.0, 3.0);
        let v2 = V!(1.0, 2.0);
        if v1.dot(&v2) == None {
            // improper should_panic usage, its just for sample.
            panic!("Invalid dot");
        }
    }

    #[test]
    #[should_panic(expected = "Invalid vector index!")]
    fn out_of_index() {
        let v = V!(1.0, 2.0);
        v.at(5);
    }
}
