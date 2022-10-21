

#![cfg_attr(not(feature = "std"), no_std)]

// pub use frame_system::pallet::*; 


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod types {

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
    use sp_core::crypto::AccountId32;

    use crate::pallet::Config as PalletConfig;

    type SideEffectName = Vec<u8>; 
    pub type TargetId = [u8; 4];
    pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId; 
    // type BalanceOf<T:Config> = Currency<<T as frame_system::Config>::AccountId>>::Balance;
	// pub type BalanceOf<T> = <<T as PalletConfig>::TestCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance; 
    pub type Bytes = Vec<u8>; 

    pub type SideEffectId<T> = <T as sp_core::Hasher>::Out; 
    pub type SideEffectIdHash<T> = <T as frame_system::Config>::Hash;

    type BlockNumber = u64;
    type BalanceOf = u128;
    type AccountId = AccountId32;
    type Hash = [u8; 32];


#[derive(Clone, Encode, Decode, Eq, PartialEq, Debug, MaxEncodedLen,TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]

pub struct SideEffectInterface<S:Get<u32>> {

    pub(crate) id:TargetId, 
    pub (crate) chain:ChainId, 
   
    pub (crate) args:BoundedVec<Vec<Vec<u8>>,S>
}

impl<S:Get<u32>> Default for SideEffectInterface<S> {

fn default() -> Self {
    let id:TargetId = [1,0,0,1]; 
    let args = BoundedVec::default(); 
    Self { id, chain:ChainId::Polkadot, args }
}

}

impl<S:Get<u32>> SideEffectInterface<S> {

    pub fn generate_id<Hasher: sp_core::Hasher>(&self) -> <Hasher as sp_core::Hasher>::Out {
        Hasher::hash(Encode::encode(self).as_ref())
    }

    pub fn id_as_bytes<Hasher: sp_core::Hasher>(&self,id: <Hasher as sp_core::Hasher>::Out) -> Bytes {
        id.as_ref().to_vec()
    }
}

impl<S:Get<u32> > TryFrom<Vec<u8>> for SideEffectInterface<S> {

type Error = &'static str;

fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    
    let mut bytes: VecDeque<u8> = value.into();
    let mut take_next = || bytes.pop_back().ok_or("no more bytes");
    let target: TargetId = TargetByte(take_next()?).try_into()?;
    let action = Action::try_from(take_next()?)?;
    let mut  bytes: Vec<u8> = bytes.into();


    let args = extract_args::<AccountId,BalanceOf, Hash,S>(
        &action,
        &mut bytes::Bytes::from(bytes),
    )?;

    let mut side_effect = SideEffectInterface{
        id:target, 
        chain:ChainId::Polkadot, 
        args:args
    };

    Ok(side_effect)


}

}


pub struct TargetByte(pub u8);

impl TryInto<TargetId> for TargetByte {
    type Error = &'static str;

    fn try_into(self) -> Result<TargetId, Self::Error> {
        match self.0 {
            0 => Ok(*b"pdot"),
            1 => Ok(*b"karu"),
            2 => Ok(*b"ksma"),
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
            2 => Ok(Action::Swap),
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
    use codec::{Encode, Decode, EncodeLike};
    use frame_support::{pallet_prelude::{*, OptionQuery, ValueQuery, StorageValue, StorageDoubleMap}, Blake2_128Concat, Blake2_128,debug};
	use frame_system::pallet_prelude::*;
    pub type SystemHashing<T> = <T as frame_system::Config>::Hashing;

    use frame_support::{pallet_prelude::MaxEncodedLen, BoundedVec};
    use scale_info::TypeInfo;
    // use frame_support::traits::{WithdrawReasons, Currency, EnsureOrigin, GenesisBuild};

    use super::*; 

    #[pallet::config]
    pub trait Config :frame_system::Config {

        type RuntimeEvent : From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;  
        
		#[pallet::constant]
        type StringLimit:Get<u32> +Encode +Decode + TypeInfo + MaxEncodedLen +Default; 
   
        
        
        


    }

//   pub  type BalanceOf<T:Config> = <<T as Config>::PCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance ; 

//   pub type SideEffectOf<T> = SideEffectInterface<<T as Config>::StringLimit>; 



pub type SideEffectInterfaceOf<T> = SideEffectInterface<<T as Config>::StringLimit>; 

    #[pallet::pallet]
    #[pallet::without_storage_info]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_); 

    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
        SideEffectCommited(T::AccountId,Vec<u8>),
        SideEffectReverted(T::AccountId,Vec<u8>),

	}

    #[pallet::error]
    pub enum Error<T>{

        /// if the sideEffect is already commited
        SideEffectAlreadyPresent,
        SideEffectDoesNotExist,
    }


type SideId = Vec<u8>; 


#[pallet::storage]
pub type SideEffectStorage<T:Config > = StorageDoubleMap<_,Blake2_128,T::AccountId,Blake2_128,SideId,SideEffectInterfaceOf<T>,ValueQuery>; 




    #[pallet::call]
    impl<T:Config > Pallet<T> {


        #[pallet::weight(0)]
        pub fn commit_side_effect(origin:OriginFor<T>,value:Vec<u8>) -> DispatchResult{
            let  who = ensure_signed(origin)?; 

            let sn:SideEffectInterface<<T as Config>::StringLimit> = SideEffectInterface::try_from(value).unwrap(); 



            let id = sn.generate_id::<SystemHashing<T>>(); 
            // let id_to_vec = sn.id_as_bytes::<T::hasher>(id);
            let id_to_vec = id.as_ref().to_vec(); 

            ensure!(!SideEffectStorage::<T>::contains_key(&who, &id_to_vec), Error::<T>::SideEffectAlreadyPresent);

            SideEffectStorage::<T>::insert(who.clone(),id_to_vec.clone(),sn );

            

            Self::deposit_event(Event::SideEffectCommited(who, id_to_vec));



            Ok(())

        }


        #[pallet::weight(0)]
        pub fn revert_side_effect(origin:OriginFor<T>,id:Vec<u8>) -> DispatchResult {
            let  who = ensure_signed(origin)?; 

            ensure!(SideEffectStorage::<T>::contains_key(&who, &id), Error::<T>::SideEffectDoesNotExist);

            SideEffectStorage::<T>::remove(who.clone(),id.clone());


            Self::deposit_event(Event::SideEffectReverted(who, id));
            Ok(())


        }
    }


  }

