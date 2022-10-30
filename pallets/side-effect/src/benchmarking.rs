#![cfg(feature="runtime-benchmarks")]



#[allow(unused)]


use super::*;  



use crate::Pallet as SideEffects ; 

use frame_benchmarking::{benchmarks, whitelisted_caller};

use sp_runtime::AccountId32;
type BlockNumber = u64;
type BalanceOf = u128;
type AccountId = AccountId32;

use frame_support::assert_ok; 

use frame_system::RawOrigin ; 
use codec::Encode;

use scale_info::{
    prelude::{fmt::Debug, vec, vec::Vec},

};


fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}


// benchmarks! {}


benchmarks! {

    
 

    
    commit_side_effect {

        let caller:T::AccountId = account("generator", 24, 0) ; 
        let from: AccountId32 = AccountId32::new([1u8; 32]);
        let to: AccountId32 = AccountId32::new([2u8; 32]);
        let value: BalanceOf = 1u128;
        let target = *b"ksma";

        let encode_args = vec![target.encode(),from.encode(),to.encode(),value.encode()].encode(); 
        let id = vec![85, 40, 2, 119, 72, 156, 211, 80, 75, 167, 131, 127, 5, 239, 79, 158, 34, 233, 135, 104, 243, 13, 91, 147, 88, 120, 20, 31, 65, 165, 99, 243];


    }: _(RawOrigin::Signed(caller.clone()).into(),,encode_args)


    revert_side_effect {

        let caller:T::AccountId = account("generator", 24, 0) ; 
        let from: AccountId32 = AccountId32::new([1u8; 32]);
        let to: AccountId32 = AccountId32::new([2u8; 32]);
        let value: BalanceOf = 1u128;
        let target = *b"ksma";

        let encode_args = vec![target.encode(),from.encode(),to.encode(),value.encode()].encode(); 
        let id = vec![85, 40, 2, 119, 72, 156, 211, 80, 75, 167, 131, 127, 5, 239, 79, 158, 34, 233, 135, 104, 243, 13, 91, 147, 88, 120, 20, 31, 65, 165, 99, 243];

        assert_ok!(Pallet::<T>::commit_side_effect(RawOrigin::Signed(caller.clone()).into(),, encode_args));




    }: _(RawOrigin::Signed(caller.clone()).into(), id)
 



}




