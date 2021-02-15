use super::*;
use sp_arithmetic::traits::BaseArithmetic;
use sp_std::marker::PhantomData;

/// Get the amount of staking per Era in a module in the Plasm Network.
pub trait ComputeEraWithParam<EraIndex> {
    type Param;
    fn compute(era: &EraIndex) -> Self::Param;
}

pub struct DefaultForSecurity<T: Trait> {
    _phantom: PhantomData<T>,
}
impl<T: Trait> ComputeEraWithParam<EraIndex> for DefaultForSecurity<T> {
    type Param = BalanceOf<T>;
    fn compute(era: &EraIndex) -> BalanceOf<T> {
        BalanceOf::<T>::from(T::SessionInterface::validators().len().try_into().unwrap_or(0u32))
    }
}
