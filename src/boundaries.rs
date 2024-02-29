use crate::timeline_point::StateDynamicsTimeLineComponent;
use crate::zip_result::ZipResult;
use std::fmt::Debug;

pub struct Boundaries<'t, T: Clone> {
    pub left: Option<StateDynamicsTimeLineComponent<ZipResult<'t, T>>>,
    pub intersection: StateDynamicsTimeLineComponent<ZipResult<'t, T>>,
    pub right: Option<StateDynamicsTimeLineComponent<ZipResult<'t, T>>>,
}

impl<'t, T: Debug + Clone> Boundaries<'t, T> {
    pub fn get_boundaries(
        left: &'t StateDynamicsTimeLineComponent<T>,
        right: &'t StateDynamicsTimeLineComponent<T>,
    ) -> Boundaries<'t, T> {
        let intersection_boundary_left = left.t1.max(right.t1);

        let intersection_boundary_right = match (left.t2, right.t2) {
            (Some(t2), None) => Some(t2),
            (None, Some(t2)) => Some(t2),
            (Some(t2), Some(t2_other)) => Some(t2.min(t2_other)),
            (None, None) => None,
        };

        let intersection = StateDynamicsTimeLineComponent {
            t1: intersection_boundary_left,
            t2: intersection_boundary_right,
            value: ZipResult::Both((&left.value, &right.value)),
        };

        // left boundary optional: t1 -> t2
        let left_boundary = if left.t1 == right.t1 {
            None
        } else {
            let left_boundary_left = left.t1.min(right.t1);
            let left_boundary_right = intersection_boundary_left;
            Some(if left_boundary_left == left.t1 {
                StateDynamicsTimeLineComponent {
                    t1: left_boundary_left,
                    t2: Some(left_boundary_right),
                    value: ZipResult::Singleton(&left.value),
                }
            } else {
                // if t1x0 == other_point.t1, then it means t1 is before t2 and the value exists only in self time line.
                StateDynamicsTimeLineComponent {
                    t1: left_boundary_left,
                    t2: Some(left_boundary_right),
                    value: ZipResult::Singleton(&right.value),
                }
            })
        };

        // right boundary optional
        let right_boundary = if left.t2 == right.t2 {
            None
        } else {
            let right_boundary_left = intersection_boundary_right;
            let right_boundary_right = match (left.t2, right.t2) {
                (Some(t2), None) => Some(t2),
                (None, Some(t2)) => Some(t2),
                (Some(t2), Some(t2_other)) => Some(t2.max(t2_other)),
                (None, None) => None,
            };
            match right_boundary_left {
                Some(t2x0) => {
                    if Some(t2x0) == left.t2 {
                        Some(StateDynamicsTimeLineComponent {
                            t1: t2x0,
                            t2: right_boundary_right,
                            value: ZipResult::Singleton(&right.value),
                        })
                    } else {
                        Some(StateDynamicsTimeLineComponent {
                            t1: t2x0,
                            t2: right_boundary_right,
                            value: ZipResult::Singleton(&left.value),
                        })
                    }
                }
                None => None,
            }
        };

        Boundaries {
            left: left_boundary,
            intersection,
            right: right_boundary,
        }
    }
}
