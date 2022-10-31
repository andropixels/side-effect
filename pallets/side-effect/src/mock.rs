use crate::{self as pallet_side_effect, weights::SubstrateWeight} ; 

use codec::{Decode, MaxEncodedLen, Encode};
use frame_system as system ; 
use scale_info::TypeInfo;
use sp_core::{H256, Get};
use frame_support::{traits::{ConstU16, ConstU64}, parameter_types};
use sp_runtime::{

	testing::Header, 
	traits::{BlakeTwo256,IdentityLookup}
};


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>; 

type Block = frame_system::mocking::MockBlock<Test>;



frame_support::construct_runtime!(

	pub enum Test where
	Block =Block,
	NodeBlock=Block, 
	UncheckedExtrinsic=UncheckedExtrinsic
	
	{
			System:frame_system,
			SideEffects:pallet_side_effect,

	}
); 



impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}



parameter_types! {

	const StringLimit:u32 = 5; 
  


}



// pub type TypeHasher = sp_core::Hasher::hash<TypeHash>;

#[derive(Debug,Default,Encode,Decode,TypeInfo,MaxEncodedLen,PartialEq, Eq)]
pub struct Slimit ; 

impl Get<u32> for Slimit {
    fn get() -> u32 {
        5
    }
}


impl pallet_side_effect::Config for Test {

	type RuntimeEvent =  RuntimeEvent;

	type StringLimit = Slimit;
	type WeightInfo = SubstrateWeight<Test>;




}



pub struct Extuilder ; 


impl Default for Extuilder {

	fn default() -> Self {
		Self
	}
}



impl Extuilder {

pub	fn build(self) -> sp_io::TestExternalities {

		let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap(); 
		let mut  ext = sp_io::TestExternalities::new(t); 

		ext.execute_with(|| System::set_block_number(1));

		ext

	}
}


pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
