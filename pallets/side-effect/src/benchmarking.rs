#![cfg(feature="runtime-benchmarks")]



use crate::Pallet as SideEffects ; 

use super::{Event, *};
// use crate::mock::*;

use frame_benchmarking::{ impl_benchmark_test_suite, whitelisted_caller,account, benchmarks};

use sp_runtime::AccountId32;
type BlockNumber = u64;
type BalanceOf = u128;
type AccountId = AccountId32;

use frame_support::assert_ok; 

use frame_system::RawOrigin ; 
use codec::Encode;

// use create::vec::Vec; 

use scale_info::{
    prelude::{fmt::Debug, vec, vec::Vec},

};
use super::*;  

/*
#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{account, benchmarks};
use frame_support::traits::{Currency, EnsureOrigin, Get};
use frame_system::RawOrigin;
use sp_runtime::{
    traits::{Bounded, StaticLookup},
    FixedI64, FixedPointNumber,
};

use super::*;

benchmarks! {
    update_value_coefficient {
        let origin = T::UpdateOrigin::successful_origin();
        let coefficient = FixedI64::checked_from_rational(2_65, 100).unwrap();
    }: _<T::Origin>(origin, coefficient)
    verify {
        let stored_coefficient = ValueCoefficient::<T>::get();
        assert_eq!(coefficient, stored_coefficient);
    }

    generate_energy {
        let generator: T::AccountId = account("generator", 24, 0);
        let generator_balance = BalanceOf::<T>::max_value();
        <T as Config>::Currency::make_free_balance_be(&generator, generator_balance);
        let receiver: T::AccountId = account("receiver", 36, 0);

        // The minimum amount of energy that can be generated is T::ExistentialDeposit
        let burn_amount = T::ExistentialDeposit::get();
    }: _(RawOrigin::Signed(generator.clone()), T::Lookup::unlookup(receiver.clone()), burn_amount)
    verify {
        let energy = burn_amount;
        assert_eq!(Pallet::<T>::energy_balance(&receiver), energy);
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::ExtBuilder::default().value_coefficient(1.5).update_origin(1).build(),
        crate::mock::Test,
    );
}


*/

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}




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
    verify {

    //    assert_eq!( SideEffectStorage::<T>::contains_key(caller.clone(),id.clone() ), true); 


    //    assert_last_event::<T>(Event::<T>::SideEffectCommited(
    //             caller, id
    //    ).into());

    }

    revert_side_effect {

        let caller:T::AccountId = account("generator", 24, 0) ; 
        let from: AccountId32 = AccountId32::new([1u8; 32]);
        let to: AccountId32 = AccountId32::new([2u8; 32]);
        let value: BalanceOf = 1u128;
        let target = *b"ksma";

        let encode_args = vec![target.encode(),from.encode(),to.encode(),value.encode()].encode(); 
        let id = vec![85, 40, 2, 119, 72, 156, 211, 80, 75, 167, 131, 127, 5, 239, 79, 158, 34, 233, 135, 104, 243, 13, 91, 147, 88, 120, 20, 31, 65, 165, 99, 243];

        assert_ok!(Pallet::<T>::commit_side_effect(RawOrigin::Signed(caller.clone()).into(),, encode_args));




    }:_(RawOrigin::Signed(caller.clone()).into(), id)
   verify {
    //    assert_eq!( SideEffectStorage::<T>::contains_key(caller,id ), false); 


//     assert_last_event::<T>(Event::<T>::SideEffectReverted(
//         caller, id
// ).into());

    }



}




