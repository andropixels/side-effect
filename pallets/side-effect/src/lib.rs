#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::pallet_prelude::*;
use sp_core::bytes;
use sp_runtime::{

    ArithmeticError, RuntimeDebug,
};
use scale_info::{
    prelude::{collections::VecDeque, fmt::Debug, vec, vec::Vec},
    TypeInfo,
};

use codec::Encode;
use sp_runtime::traits::Hash;
use frame_system::pallet::*;

pub type TargetId = [u8; 4];
pub type EventSignature = Vec<u8>;
pub type SideEffectName = Vec<u8>;
type Bytes = Vec<u8>;
pub type SideEffectId<T> = <T as frame_system::Config>::Hash;
pub type ExecutionId = u32;

/// Represents different chains as a single byte
#[derive(Clone, Eq, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[repr(u8)]
pub enum ChainId {
    Polkadot,
    Kusama,
    Rococo,
    T3rn,
}

impl TryInto<TargetId> for ChainId {
    type Error = &'static str;

    fn try_into(self) -> Result<TargetId, Self::Error> {
        match self {
            Self::Polkadot=> Ok(*b"ksma"),
            Self::Kusama => Ok(*b"pdot"),
            Self::Rococo => Ok(*b"roco"),
            Self::T3rn => Ok(*b"t3rn"),
            _ => Err("Invalid target Id"),
        }
    }
}






#[derive(Clone, Eq, PartialEq, Encode, MaxEncodedLen,Decode, TypeInfo)]

/// Represents different potential actions as a single byte
enum Action {
    Swap,
    Tran,
    MultiTran,
}

/// Volatile side effects that can be executed

#[derive(Clone, Eq, PartialEq, Encode,Decode,MaxEncodedLen, TypeInfo)]

pub struct SideEffect<T: Get<u32>, S: Get<u32>> {
    chain: ChainId,
    action: Action,
    args: BoundedVec<T,S>,
}


type SideEffectOf<T> =  SideEffect<<T as Config>::MaxBytesPerArg, <T as Config>::MaxArgs>;


#[frame_support::pallet]
pub mod pallet {
    use frame_system::{Origin, ensure_signed};

    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching RuntimeEvent type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;


 /// Maximum number of arguements in a side effect (should be 5 for this particular exercise)
 #[pallet::constant]
 type MaxArgs: Get<u32> +TypeInfo;

 /// Maximum number of bytes in an individual arguement
 #[pallet::constant]
 type MaxBytesPerArg: Get<u32>;

 /// Maximum number of side effects that can reside in a single step
 #[pallet::constant]
 type MaxSideEffects: Get<u32>;

       
    }
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        CannotDecodeValue,
        DecodesToNothing,
        ExecutionFailed,
    }

    /// Counter for `ExecutionId`
    #[pallet::storage]
    pub type NextExecutionId<T: Config> = StorageValue<_, ExecutionId, ValueQuery>;

    /// Maps the `ExecutionId` to a `BoundedVec` of side effects
    #[pallet::storage]
    pub type SideEffects<T: Config> = StorageMap<
        _,
        Twox64Concat,
        ExecutionId,
        BoundedVec<SideEffectOf<T>, T::MaxSideEffects>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
   
        /// Side effects are committed and removed from `ExecutionPending` storage
        SideEffectsCommitted { execution_id: ExecutionId },
        /// Side effects are reverted (not executed) and removed from `ExecutionPending` storage
        SideEffectsReverted { execution_id: ExecutionId },
        /// Emits event when side effect is executed and fails
        SideEffectFailed {
            execution_id: ExecutionId,
            index: u32,
        },
    }


	#[pallet::call]
	impl<T: Config> Pallet<T> {


		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn commit_transfer_side_effect(origin: OriginFor<T>,to:T::AccountId,amount:u128) -> DispatchResult{
            let _ =   ensure_signed(origin)?;

            let args  = vec![to.encode(),amount.encode()];
            let side_effects = Self::decode_to_side_effects(args,Action::Tran)?;

            let execution_id = NextExecutionId::<T>::get();
            let increment_execution_id = execution_id
                .checked_add(sp_runtime::traits::One::one())
                .ok_or(ArithmeticError::Overflow)?;
                NextExecutionId::<T>::set(increment_execution_id);
               

                ensure!(!SideEffects::<T>::contains_key(execution_id),Error::<T>::CannotDecodeValue);
                SideEffects::<T>::insert(execution_id, side_effects);
    
            Self::deposit_event(Event::<T>::SideEffectsCommitted { execution_id });
            Ok(())

        }

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn revert_side_effect(origin: OriginFor<T>,execution_id:ExecutionId) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            ensure!(!SideEffects::<T>::contains_key(execution_id),Error::<T>::ExecutionFailed);

            SideEffects::<T>::remove(execution_id);
            Self::deposit_event(Event::<T>::SideEffectsReverted { execution_id });
            Ok(())
        }
    }


    impl<T:Config> Pallet<T> {

        fn decode_to_side_effects(
            encoded_value: Vec<Bytes>, action:Action
        ) -> Result<BoundedVec<SideEffectOf<T>, T::MaxSideEffects>, DispatchError> {
            
       
         

        let side_effects: BoundedVec<SideEffectOf<T>, T::MaxSideEffects> =
        T::Hashing::hash(&encoded_value.encode());
        let side_effect = SideEffect{
            chain:ChainId::T3rn,
            action:action,
            args:side_effects
         };

         
            Ok(side_effects)
        }
    }

}

