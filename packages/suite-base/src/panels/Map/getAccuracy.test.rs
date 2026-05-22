```rust
use std::f64;

fn get_accuracy(nav_sat_fix_msg: &NavSatFixMsg) -> Option<(f64, f64, f64)> {
    if nav_sat_fix_msg.position_covariance_type == NavSatFixPositionCovarianceType::COVARIANCE_TYPE_DIAGONAL_KNOWN {
        let mut radii = [0.0; 2];
        for i in 0..3 {
            radii[i] = f64::sqrt(nav_sat_fix_msg.position_covariance[i]);
        }
        Some((radii[0], radii[1], 0.0))
    } else if nav_sat_fix_msg.position_covariance_type == NavSatFixPositionCovarianceType::COVARIANCE_TYPE_KNOWN {
        let mut radii = [0.0; 2];
        for i in 0..3 {
            let val = nav_sat_fix_msg.position_covariance[i];
            if val != 0.0 {
                radii[i] = f64::sqrt(val);
            }
        }
        Some((radii[0], radii[1], f64::atan2(nav_sat_fix_msg.position_covariance[3], nav_sat_fix_msg.position_covariance[2]))) as _
    } else {
        None
    }
}
```