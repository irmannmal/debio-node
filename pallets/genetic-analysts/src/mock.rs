use crate as genetic_analysts;
use frame_support::{parameter_types, PalletId};
use frame_system as system;
use pallet_balances::AccountData;
use scale_info::TypeInfo;
use sp_core::{Decode, Encode, RuntimeDebug, H256};
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct EthereumAddress(pub [u8; 20]);

pub type AccountId = u64;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		GeneticAnalysts: genetic_analysts::{Pallet, Call, Storage, Event<T>},
		GeneticAnalystServices: genetic_analyst_services::{Pallet, Call, Storage, Event<T>},
		GeneticAnalystQualifications: genetic_analyst_qualifications::{Pallet, Call, Storage, Event<T>},
		UserProfile: user_profile::{Pallet, Call, Storage, Event<T>}
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub BlockWeights: frame_system::limits::BlockWeights =
	frame_system::limits::BlockWeights::simple_max(1024);
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type AccountId = AccountId;
	type Call = Call;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Event = Event;
	type Origin = Origin;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = AccountData<Balance>;
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

pub type Moment = u64;
pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
	pub const GeneticAnalystPalletId: PalletId = PalletId(*b"dbio/gen");
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl genetic_analysts::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type PalletId = GeneticAnalystPalletId;
	type GeneticAnalystServices = GeneticAnalystServices;
	type GeneticAnalystQualifications = GeneticAnalystQualifications;
	type EthereumAddress = EthereumAddress;
	type UserProfile = UserProfile;
	type GeneticAnalystWeightInfo = ();
}

impl genetic_analyst_services::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type GeneticAnalystServiceOwner = GeneticAnalysts;
	type WeightInfo = ();
}

impl genetic_analyst_qualifications::Config for Test {
	type Event = Event;
	type GeneticAnalystQualificationOwner = GeneticAnalysts;
	type WeightInfo = ();
}

impl user_profile::Config for Test {
	type Event = Event;
	type EthereumAddress = EthereumAddress;
	type WeightInfo = ();
}

type Balance = u128;

parameter_types! {
	pub static ExistentialDeposit: Balance = 0;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

pub struct ExternalityBuilder {
	existential_deposit: u128,
}

impl Default for ExternalityBuilder {
	fn default() -> Self {
		Self { existential_deposit: 1 }
	}
}

impl ExternalityBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u128) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}
	pub fn set_associated_consts(&self) {
		EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
	}
	pub fn build(&self) -> TestExternalities {
		self.set_associated_consts();
		let mut storage = system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> { balances: { vec![] } }
			.assimilate_storage(&mut storage)
			.unwrap();
		let mut ext = sp_io::TestExternalities::new(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
