#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as SideEffects;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use codec::{Encode, Decode, EncodeLike};
use sp_core::crypto::AccountId32;

use scale_info::{
    prelude::{fmt::Debug, vec, vec::Vec},

};
use frame_support::assert_ok; 

// use sp_std::vec::Vec;
type BlockNumber = u64;
type BalanceOf = u128;
type AccountId = AccountId32;

benchmarks! {


    commit_side_effect{
		let caller: T::AccountId = whitelisted_caller();

        let arg  = vec![0,4,5,2]; 
                let from: AccountId32 = AccountId32::new([1u8; 32]);
        let to: AccountId32 = AccountId32::new([2u8; 32]);
        let value: BalanceOf = 1u128;
        let target = *b"ksma";

        let encode_args = vec![target.encode(),from.encode(),to.encode(),value.encode()].encode(); 
    }: _(RawOrigin::Signed(caller), encode_args )
    verify{

    }
    
    revert_side_effect {

        let caller: T::AccountId = whitelisted_caller();

        let from: AccountId32 = AccountId32::new([1u8; 32]);
        let to: AccountId32 = AccountId32::new([2u8; 32]);
        let value: BalanceOf = 1u128;
        let target = *b"ksma";

        let encode_args = vec![target.encode(),from.encode(),to.encode(),value.encode()].encode(); 
        let id = vec![85, 40, 2, 119, 72, 156, 211, 80, 75, 167, 131, 127, 5, 239, 79, 158, 34, 233, 135, 104, 243, 13, 91, 147, 88, 120, 20, 31, 65, 165, 99, 243];

        assert_ok!(Pallet::<T>::commit_side_effect(RawOrigin::Signed(caller.clone()).into(), encode_args));




    }: _(RawOrigin::Signed(caller.clone()), id)
 
    verify{
        
    }
    
    impl_benchmark_test_suite!(
        SideEffects,
        crate::mock::new_test_ext(),
        crate::mock::Test,
       );

}

