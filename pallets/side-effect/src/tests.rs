#![cfg(test)]

use super::{Event, *};
use crate::mock::*;
use codec::Encode;
use frame_support::{assert_err, assert_ok, bounded_vec, BoundedVec};
use sp_runtime::AccountId32;
type BlockNumber = u64;
type BalanceOf = u128;
type AccountId = AccountId32;
type Hash = [u8; 32];

use scale_info::prelude::collections::VecDeque;



macro_rules! bvec {

    ($($x:tt)*) => {
        vec![$( $x)*].try_into().unwrap()
    }
}






#[test]

fn creates_empty_side_effect() {

let default_side_effect = SideEffectInterface::<Slimit>{

    id:[1,0,0,1],
    chain:ChainId::Polkadot, 
    args:BoundedVec::default()
}; 

assert_eq!(

    default_side_effect, 
    SideEffectInterface::default()

);

}






#[test]

fn successfully_creates_transfer_side_effect() {

    let from: AccountId32 = AccountId32::new([1u8; 32]);
    let to: AccountId32 = AccountId32::new([2u8; 32]);
    let value: BalanceOf = 1u128;
    let target = *b"ksma";

let encode_args = vec![target.encode(),from.encode(),to.encode(),value.encode()].encode(); 

    // let  mut args:BoundedVec<Vec<Vec<u8>>, Slimit> = BoundedVec::default(); 



let transfer_side_effect:SideEffectInterface<Slimit> = SideEffectInterface::try_from(encode_args).unwrap(); 


let mut res_b_vec:BoundedVec<Vec<Vec<u8>>, Slimit> = BoundedVec::default(); 

let  mut res_vec:Vec<Vec<u8>> = Vec::new(); 


res_vec.push([16,16,107,115,109,97,128,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1].into());
res_vec.push([1,1,1,1,1,1,1,128,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2].into());
res_vec.push([2,2,2,2,2,2,2,2,64,1,0,0,0,0,0,0].into());

res_b_vec.try_push(res_vec);




assert_eq!(transfer_side_effect.id, *b"pdot");


    assert_eq!(
        transfer_side_effect,
        SideEffectInterface {id:[112, 100, 111, 116],chain:ChainId::Polkadot,args:res_b_vec }
    );







}



#[test]
fn successfully_creates_swap_side_effect() {

    let amount_from = 1_u128;
        let amount_to = 2_u128;
        let asset_from = [3_u8; 32];
        let asset_to = [2_u8; 32];

    let from: AccountId32 = AccountId32::new([1u8; 32]);
    let to: AccountId32 = AccountId32::new([2u8; 32]);
    let target = *b"ksma";

let encode_args = vec![target.encode(),from.encode(),to.encode(),amount_from.encode(),amount_to.encode(),asset_from.encode(),asset_to.encode()].encode(); 

    // let  mut args:BoundedVec<Vec<Vec<u8>>, Slimit> = BoundedVec::default(); 

    

let swap_side_effect:SideEffectInterface<Slimit> = SideEffectInterface::try_from(encode_args).unwrap(); 


let mut res_b_vec:BoundedVec<Vec<Vec<u8>>, Slimit> = BoundedVec::default(); 

let  mut res_vec:Vec<Vec<u8>> = Vec::new(); 



res_vec.push([28, 16, 107, 115, 109, 97, 128, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1].into());
res_vec.push( [1, 1, 1, 1, 1, 1, 1, 128, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2].into());
res_vec.push([2, 2, 2, 2, 2, 2, 2, 2, 64, 1, 0, 0, 0, 0, 0, 0].into());
res_vec.push([0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 2, 0, 0, 0, 0, 0].into());
res_vec.push( [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3].into());
res_vec.push( [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 128, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2].into());

res_b_vec.try_push(res_vec);


assert_eq!(swap_side_effect.id, *b"ksma");



    assert_eq!(
        swap_side_effect,
        SideEffectInterface {id:[107,115,109,97],chain:ChainId::Polkadot,args:res_b_vec }
    );







}



#[test]
fn successfully_commit_side_effect(){


    Extuilder::default().build().execute_with(|| {
        let from: AccountId32 = AccountId32::new([1u8; 32]);
        let to: AccountId32 = AccountId32::new([2u8; 32]);
        let value: BalanceOf = 1u128;
        let target = *b"ksma";

        let encode_args = vec![target.encode(),from.encode(),to.encode(),value.encode()].encode(); 



    assert_ok!(SideEffects::commit_side_effect(RuntimeOrigin::signed(1), encode_args))    ;







    } );

}
