use ark_bls12_381::Bls12_381;
use ark_ec::pairing::Pairing;
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::kzg10::UniversalParams;

pub struct Proof {}

#[derive(Clone)]
pub struct PublicParameters {
    pub(crate) kzg_params: UniversalParams<Bls12_381>,
}

pub(crate) type UniPoly381 = DensePolynomial<<Bls12_381 as Pairing>::ScalarField>;
