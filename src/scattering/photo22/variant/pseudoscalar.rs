use super::*;
use crate::math::number::*;
use crate::quantum::{*, relativistic::*};


pub struct PseudoscalarInvariantAmpl {
    pub h_photon: Helicity,
}

impl InvariantAmplStates for PseudoscalarInvariantAmpl {}

impl PhotoproductionBase for Photoproduction<PseudoscalarInvariantAmpl> {
    #[inline]
    fn get_reaction_data(&self) -> &ReactionData {
        &self.rx
    }

    #[inline]
    fn calc_amplitude_avgsq(&self) -> Real {
        if self.rx.energy == 0. { return 0. }

        let ampl_sq = |h1: Helicity, s1: Spin, s2: Spin| -> Real {
            let mut sum = Complex::zero();

            let u: &StateVector     = &dirac_spinor(&self.rx.particles.inc_b, s1);
            let u_bar: &StateVector = &dirac_adjoint(&dirac_spinor(&self.rx.particles.out_d, s2));

            for xch in &self.exchanges {
                let m = xch.calc_invariant_amplitude(&self.rx, PseudoscalarInvariantAmpl { h_photon: h1 });

                // < f | M | i >
                sum += u_bar * m * u;
            }

            // | < f | M | i >_1 + < f | M | i >_2 + ... < f | M | i >_n |^2
            return sum.norm_sq();
        };

        let mut sum = 0.;

        // TODO: Parallelization and iter zip.
        for h_photon in &self.ampl_param.h_photon {
            for s_inc_b in &self.ampl_param.s_inc_b {
                for s_out_d in &self.ampl_param.s_out_d {
                    sum += ampl_sq(*h_photon, *s_inc_b, *s_out_d);
                }
            }
        }

        return sum / self.ampl_param.spin_avg;
    }
}

pub type PseudoscalarPhotoproduction = Photoproduction<PseudoscalarInvariantAmpl>;