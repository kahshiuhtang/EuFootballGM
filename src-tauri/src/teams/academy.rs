pub struct Academy{
    funding:i32,
}

pub fn get_academy_impact(academy: &Academy) -> [f64; 5] {
    if academy.funding == 0{
        return [0.5, 0.3, 0.15, 0.1, 0.05];
    }
    return [0.5, 0.3, 0.15, 0.1, 0.05];
}