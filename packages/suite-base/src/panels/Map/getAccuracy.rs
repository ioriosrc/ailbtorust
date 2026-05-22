```rust
use mathrs::prelude::*;

/// Calculates the accuracy of a NavSatFix message, based on its type, and returns
/// information suitable for display as a leaflet Ellipse.
pub fn get_accuracy(msg: NavSatFixMsg) -> Option<(f64, f64, f64)> {
    let covariance = msg.position_covariance;
    if covariance.is_none() {
        return None;
    }

    match msg.position_covariance_type {
        NavSatFixPositionCovarianceType::COVARIANCE_TYPE_UNKNOWN | NavSatFixPositionCovarianceType::COVARIANCE_TYPE_DIAGONAL_KNOWN => {
            // Tilt is degrees from west
            let east_variance = covariance.unwrap()[0];
            let north_variance = covariance.unwrap()[4];
            if !is_finite(east_variance) || !is_finite(north_variance) {
                return None;
            }
            Some((east_variance.sqrt(), north_variance.sqrt(), 0.0))
        }
        NavSatFixPositionCovarianceType::COVARIANCE_TYPE_APPROXIMATED | NavSatFixPositionCovarianceType::COVARIANCE_TYPE_KNOWN => {
            // Discard altitude
            let K = covariance.unwrap();
            let mut Klatlon = [[K[0], K[1]], [K[3], K[4]]];

            // Compute the eigenvalues & vectors of the covariance matrix. They will
            // be sorted in ascending order, so the largest value is eigenvalues[1]
            // and the corresponding vector is in the rightmost column. Ellipse radii
            // are based on the eigenvalues, and orientation on the vector.
            let eigen = eigs(&mut Klatlon);

            // Extract the eigenvector corresponding to the largest eigenvalue (index 1, as they're sorted ascending)
            // and the eigenvalues. Both correspond to the major axis of the error ellipse.
            let eigenvector_largest = eigen.eigenvectors[1].unwrap();
            let eigenvalues = [eigen.eigenvectors[0].unwrap().value(), eigen.eigenvectors[1].unwrap().value()];

            // Extract x and y components from the eigenvector (MathCollection)
            // This is the direction of the major axis of the error ellipse, and is used to calculate the tilt.
            let eigenvector_x = eigenvector_largest.slice(0..2).to_vec();
            let eigenvector_y = eigenvector_largest.slice(2..4).to_vec();

            if !is_finite(eigenvector_x) || !is_finite(eigenvector_y) || !is_finite(eigenvalues[0]) || !is_finite(eigenvalues[1]) {
                return None;
            }

            // Ellipse `tilt` is defined as number of degrees from the negative x axis
            let theta = (eigenvector_y.iter().zip(&eigenvector_x).map(|(y, x)| y.atan2(*x)).sum::<f64>() * 180.0 / std::f64::consts::PI).abs();
            let tilt = -1.0 * theta;

            // Now that we've calculated tilt, we can calculate the ellipse radii, which are based on the eigenvalues. The larger eigenvalue corresponds to the major axis of the error ellipse, and the smaller to the minor axis.
            let primary_radius = eigenvalues[1];
            let secondary_radius = eigenvalues[0];

            if !is_finite(tilt) || !is_finite(primary_radius) || !is_finite(secondary_radius) {
                return None;
            }

            Some((primary_radius, secondary_radius, tilt))
        }
    }
}
```