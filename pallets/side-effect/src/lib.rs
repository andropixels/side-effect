

#![cfg_attr(not(feature = "std"), no_std)]


pub use frame_system::pallet::*; 





pub mod types {


    // use bytes::{Bytes, BytesMut, Buf, BufMut};


    use codec::{Decode, Encode};
    use frame_support::traits::Currency;
    use frame_support::traits::tokens::Balance;
    use frame_support::{pallet_prelude::MaxEncodedLen, BoundedVec};
    use frame_system::Config;
    use scale_info::prelude::collections::VecDeque;
    use num::Zero; 
    use scale_info::TypeInfo;
    use sp_core::Get;
    use sp_runtime::RuntimeDebug;

    use crate::pallet::Config as PalletConfig;

    type SideEffectName = Vec<u8>; 
    pub type TargetId = [u8; 4];
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId; 
    // type BalanceOf<T:Config> = Currency<<T as frame_system::Config>::AccountId>>::Balance;
	pub type BalanceOf<T> = <<T as PalletConfig>::TestCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance; 
    pub type Bytes = Vec<u8>; 
    pub type SideEffectId<T> = <T as frame_system::Config>::Hash; 
    #[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, MaxEncodedLen,TypeInfo, Default)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    pub struct SideEffect<BoundedArguments> {
        pub id:TargetId,
        pub chain:ChainId,
        args:BoundedArguments,
   
        

        
    }


#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, MaxEncodedLen,TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct SideEffectInterfaceTest<T, S:Get<u32>> {

    id:TargetId, 
    chain:ChainId, 
    args:BoundedVec<T,S>
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, MaxEncodedLen,TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]

pub struct SideEffectInterface<S:Get<u32>> {

    id:TargetId, 
    chain:ChainId, 
    // args:BoundedVec<T,S>,
    args:BoundedVec<Vec<Vec<u8>>,S>
}

// impl<T, S:Get<u32>> SideEffectInterface<T,S>{

//     pub fn add(&mut self,encoded_data:T) {

//          self.args.try_push(encoded_data);

//     }
// }

impl<S:Get<u32> + frame_system::Config + PalletConfig > TryFrom<Vec<u8>> for SideEffectInterface<S> {

type Error = &'static str;

fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    
    let mut bytes: VecDeque<u8> = value.into();
    let mut take_next = || bytes.pop_front().ok_or("no more bytes");
    let target: TargetId = TargetByte(take_next()?).try_into()?;
    let action = Action::try_from(take_next()?)?;
    let mut  bytes: Vec<u8> = bytes.into();
    let args = extract_args::<<S as frame_system::Config>::AccountId,BalanceOf<S>, [u8; 32],S>(
        &action,
        &mut bytes::Bytes::from(bytes),
    )?;

    // let mut my_vec: BoundedVec<Vec<Bytes>, <T as PalletConfig>::StringLimit> = Default::default(); 
    // let mut my_vec: BoundedVec<T, S> = Default::default(); 


    // let mut vev = BoundedVec::try_push(my_vec); 
    // my_vec.try_push(args); 

    let mut side_effect = SideEffectInterface{
        id:target, 
        chain:ChainId::Polkadot, 
        args:args
    };

    // side_effect.add(args); 



    Ok(side_effect)


}

}





//     impl<BoundedArguments: Zero+Copy  + Encode + Decode + MaxEncodedLen,T:frame_system::Config + PalletConfig+Zero> TryFrom<Vec<u8>> for SideEffect<BoundedArguments,T> {
//         type Error = &'static str;

//         fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
//         let mut bytes: VecDeque<u8> = value.into();
//         let mut take_next = || bytes.pop_front().ok_or("no more bytes");
//         let target: TargetId = TargetByte(take_next()?).try_into()?;
//         let action = Action::try_from(take_next()?)?;
//         let mut  bytes: Vec<u8> = bytes.into();
//         // let args = extract_args::<<T as frame_system::Config>::AccountId,BalanceOf<T>, [u8; 32],T>(
//         //     &action,
//         //     &mut bytes::Bytes::from(bytes),
//         // )?;


//         // let mut my_vec: BoundedVec<Vec<Bytes>, <T as PalletConfig>::StringLimit> = Default::default(); 

//         // // let mut vev = BoundedVec::try_push(my_vec); 
//         // my_vec.try_push(args); 

//         // let mut myb_vec= BoundedVec::from_vec
//         // my_vec.push(bytes);

//         // let action_bytes: [u8; 4] = action.into();
//         // let action_bytes = action_bytes.encode();

//         // let bounded_vec = BoundedVec
// /*
//    pub id:TargetId,
//         pub chain:ChainId,
//         args:BoundedArguments,
//         test:T

// */
//         Ok(SideEffect {
//             id:target,
//             chain:ChainId::Polkadot,
//             args: Zero::zero(),
//             test:Zero::zero()
    
//         })



//     }
//     }


pub struct TargetByte(pub u8);

impl TryInto<TargetId> for TargetByte {
    type Error = &'static str;

    fn try_into(self) -> Result<TargetId, Self::Error> {
        match self.0 {
            0 => Ok(*b"ksma"),
            1 => Ok(*b"pdot"),
            2 => Ok(*b"karu"),
            3 => Ok(*b"t3rn"),
            _ => Err("Invalid target Id"),
        }
    }
}



    #[derive(Encode,Decode, Clone,PartialEq,Eq,Default,MaxEncodedLen,RuntimeDebug, scale_info::TypeInfo,serde::Serialize,serde::Deserialize)]

    pub enum ChainId {
        #[default]
        Polkadot,
        
        Kusama,
        
        t3rn,
        
        Rococo

    }


    pub const MULTI_TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"mult";
    pub const TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"tran";
    pub const SWAP_SIDE_EFFECT_ID: &[u8; 4] = b"swap";
    
#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, MaxEncodedLen,TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum Action {
    #[default]
    Transfer,
    TransferMulti,
    Swap,
    
}

impl TryFrom<u8> for Action {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Action::Transfer),
            1 => Ok(Action::TransferMulti),
            3 => Ok(Action::Swap),
            _ => Err("Invalid action id"),
        }
    }
}

impl Into<[u8; 4]> for Action {
    fn into(self) -> [u8; 4] {
        match self {
            Action::Transfer => *TRANSFER_SIDE_EFFECT_ID,
            
            Action::Swap => *SWAP_SIDE_EFFECT_ID,
            
            Action::TransferMulti => *MULTI_TRANSFER_SIDE_EFFECT_ID,
        }
    }
}







pub fn extract_args<
    AccountId: MaxEncodedLen,
    BalanceOf: MaxEncodedLen,
    Hash: MaxEncodedLen,
    T:Get<u32>
 
>(
    action: &Action,
    bytes: &mut bytes::Bytes,
) -> Result<BoundedVec<Vec<Vec<u8>>, T >, &'static str> {
    let mut args = Vec::new();
    let mut my_vec: BoundedVec<Vec<Vec<u8>>, T > = Default::default(); 




    match *action {
        Action::Transfer => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to

            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt

        
             my_vec.try_push(args); 
            Ok(my_vec)
        },
      
        Action::Swap => {
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt_left
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt_right
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset_left
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset_right
        
            my_vec.try_push(args); 

            Ok(my_vec)

        },
       
      
        Action::TransferMulti => {
            args.push(bytes.split_to(Hash::max_encoded_len()).to_vec()); // asset id
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // from
            args.push(bytes.split_to(AccountId::max_encoded_len()).to_vec()); // to
            args.push(bytes.split_to(BalanceOf::max_encoded_len()).to_vec()); // amt
        
            my_vec.try_push(args); 

            Ok(my_vec)

        },

    }
}
  
  }

  pub use pallet::*;


  use types::*;

  #[frame_support::pallet] 
  pub mod pallet {
    use codec::{Encode, Decode};
    use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

    use frame_support::{pallet_prelude::MaxEncodedLen, BoundedVec};
    use scale_info::TypeInfo;
    use frame_support::traits::{WithdrawReasons, Currency, EnsureOrigin, GenesisBuild};
    use scale_info::prelude::collections::VecDeque;
    

    use super::*; 

    #[pallet::config]
    pub trait Config :frame_system::Config +Encode +Decode + TypeInfo + MaxEncodedLen +Default +Get<u32>{

        type RuntimeEvent : From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;  
        type PCurrency : Currency<Self::AccountId>; 
        type Type:Encode+Decode+MaxEncodedLen+Eq+PartialEq+Clone+Default+TypeInfo;
        type StringLimit:Get<u32>; 
        type TestCurrency:Currency<Self::AccountId>; 


    }

//   pub  type BalanceOf<T:Config> = <<T as Config>::PCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance ; 

  


//   pub type SideEffectOf<T> = SideEffectInterface<<T as Config>::StringLimit>; 



// pub type SideEffectOf<T:Config> = SideEffectInterface<<T as Config>::StringLimit>; 


pub type SideEffectOf<T> = SideEffect<BoundedVec<u8, <T as Config>::StringLimit>>; 

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_); 


    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T:Config> {

    }

    #[pallet::storage]
    pub type SideEffectStorage<T:Config> = StorageMap<_,Blake2_128Concat,SideEffectId<T>,SideEffectOf<T>, ValueQuery>; 



    #[pallet::call]
    impl<T:Config > Pallet<T> {


        #[pallet::weight(0)]
        pub fn commit_side_effect(origin:OriginFor<T>,value:Vec<u8>) -> DispatchResult{

        // let mut bytes: VecDeque<u8> = value.into();
        // let mut take_next = || bytes.pop_front().ok_or("no more bytes");
        // let target: TargetId = TargetByte(take_next()?).try_into()?;
        // let action = Action::try_from(take_next()?)?;
        // let mut  bytes: Vec<u8> = bytes.into();
        // let args = extract_args::<<T as frame_system::Config>::AccountId,BalanceOf<T>, [u8; 32],<T as Config>::StringLimit>(
        //     &action,
        //     &mut bytes::Bytes::from(bytes),
        // )?;


        // // let mut my_vec: BoundedVec<Vec<Bytes>, <T as Config>::StringLimit> = Default::default(); 

        // // // let mut vev = BoundedVec::try_push(my_vec); 
        // // my_vec.try_push(args); 

        // // let mut myb_vec= BoundedVec::from_vec
        // // my_vec.push(bytes);

        // let action_bytes: [u8; 4] = action.into();
        // let action_bytes = action_bytes.encode();

        let sn:SideEffectInterface<T> = SideEffectInterface::try_from(value).unwrap(); 

// 

        // let side_effect = SideEffect{
        //     id:target, 
        //     chain:ChainId::Polkadot, 
        //     args,
        // };

        // let bounded_vec = BoundedVec
// /*
//    pub id:TargetId,
//         pub chain:ChainId,
//         args:BoundedArguments,
//         test:T

// */
//         Ok(SideEffect {
//             id:target,
//             chain:ChainId::Polkadot,
//             args: Zero::zero(),
//             test:Zero::zero()
    
//         })
            Ok(())

        }


        #[pallet::weight(0)]
        pub fn revert_side_effect(origin:OriginFor<T>) -> DispatchResult {

            Ok(())
        }
    }


  }


