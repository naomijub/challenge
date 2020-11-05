use crate::schema::{Request, LogicalPath};

pub fn base(data: &Request) -> Option<LogicalPath>  {
    if data.A && data.B && !data.C {
        Some(LogicalPath::M)
    } else if data.A && data.B && data.C {
        Some(LogicalPath::P)
    } else if !data.A && data.B && data.C {
        Some(LogicalPath::T)
    } else {
        None
    }
}

pub fn base_k(logical_path: &LogicalPath, data: &Request) -> f64 {
    match logical_path {
        LogicalPath::M => data.D + (data.D * data.E as f64) / 10f64,
        LogicalPath::P => data.D + (data.D * (data.E as f64 - (data.F as f64))) / 25.5f64,
        LogicalPath::T => data.D - (data.D as f64 * (data.F as f64)) / 30f64,
    }
}

#[cfg(test)]
mod test_base {
    use super::base;
    use crate::schema::{Request, LogicalPath};

    #[test]
    fn all_true_is_p() {
        let data = Request {
            A: true,
            B: true,
            C: true,
            D: 0f64,
            E: 0,
            F: 0
        };

        let result = base(&data);

        assert_eq!(result, Some(LogicalPath::P));
    }

    #[test]
    fn a_false_rest_true_is_t() {
        let data = Request {
            A: false,
            B: true,
            C: true,
            D: 0f64,
            E: 0,
            F: 0
        };

        let result = base(&data);

        assert_eq!(result, Some(LogicalPath::T));
    }

    #[test]
    fn c_false_rest_true_is_m() {
        let data = Request {
            A: true,
            B: true,
            C: false,
            D: 0f64,
            E: 0,
            F: 0
        };

        let result = base(&data);

        assert_eq!(result, Some(LogicalPath::M));
    }

    #[test]
    fn all_false_is_err() {
        let data = Request {
            A: false,
            B: false,
            C: false,
            D: 0f64,
            E: 0,
            F: 0
        };

        let result = base(&data);

        assert_eq!(result, None);
    }

    #[test]
    fn b_false_rest_true_is_err() {
        let data = Request {
            A: true,
            B: false,
            C: true,
            D: 0f64,
            E: 0,
            F: 0
        };

        let result = base(&data);

        assert_eq!(result, None);
    }

    #[test]
    fn b_c_false_a_true_is_err() {
        let data = Request {
            A: true,
            B: false,
            C: false,
            D: 0f64,
            E: 0,
            F: 0
        };

        let result = base(&data);

        assert_eq!(result, None);
    }

    #[test]
    fn a_c_false_b_true_is_err() {
        let data = Request {
            A: false,
            B: true,
            C: false,
            D: 0f64,
            E: 0,
            F: 0
        };

        let result = base(&data);

        assert_eq!(result, None);
    }

    #[test]
    fn a_b_false_c_true_is_err() {
        let data = Request {
            A: false,
            B: false,
            C: true,
            D: 0f64,
            E: 0,
            F: 0
        };

        let result = base(&data);

        assert_eq!(result, None);
    }
}

#[cfg(test)]
#[cfg(test)]
mod test_base_k {
    use super::base_k;
    use crate::schema::{Request, LogicalPath};

    #[test]
    fn h_is_m() {
        let m = LogicalPath::M;
        let data = Request {
            A: false,
            B: false,
            C: true,
            D: 5.4f64,
            E: 6,
            F: 7
        };

        let k = base_k(&m, &data);
        assert_eq!(k, 8.64f64);
    }

    #[test]
    fn h_is_p() {
        let p = LogicalPath::P;
        let data = Request {
            A: false,
            B: false,
            C: true,
            D: 5.4f64,
            E: 6,
            F: 7
        };

        let k = base_k(&p, &data);
        assert_eq!(k, 5.188235294117647f64);
    }

    #[test]
    fn h_is_t() {
        let t = LogicalPath::T;
        let data = Request {
            A: false,
            B: false,
            C: true,
            D: 5.4f64,
            E: 6,
            F: 7
        };

        let k = base_k(&t, &data);
        assert_eq!(k, 4.140000000000001f64);
    }

    #[test]
    fn h_is_p_and_values_are_zero() {
        let p = LogicalPath::P;
        let data = Request {
            A: false,
            B: false,
            C: true,
            D: 0f64,
            E: 0,
            F: 0
        };

        let k = base_k(&p, &data);
        assert_eq!(k, 0f64);
    }

    #[test]
    fn h_is_p_and_d_is_zero() {
        let m = LogicalPath::M;
        let data = Request {
            A: false,
            B: false,
            C: true,
            D: 0f64,
            E: 5,
            F: 7
        };

        let k = base_k(&m, &data);
        assert_eq!(k, 0f64);
    }
}