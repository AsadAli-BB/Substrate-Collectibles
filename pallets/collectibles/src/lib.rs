#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet;
use frame_support::sp_runtime::traits::IntegerSquareRoot;
	// use frame_support::sp_tracing::event::Event;
use frame_support::{
		// pallet,
		pallet_prelude::{ValueQuery, *},
		// sp_runtime::app_crypto::sp_core::storage::StorageMap,
		traits::{Currency, Get, Randomness},
		BoundedVec, Twox64Concat,
	};
	use frame_system::pallet_prelude::*;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);



  #[pallet::storage]
  pub(super) type CollectiblesCount<T: Config> = StorageValue<_, u64, ValueQuery>;
  
  #[pallet::storage] 
  pub (super) type CollectibleMap <T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Collectible<T>>;

  #[pallet::storage]
  pub (super) type OwnerOfCollectibles<T: Config> = StorageMap <
  _,
  Twox64Concat, 
  T::AccountId, 
  BoundedVec<[u8; 16], T::MaximumOwned>,
  ValueQuery, 
  >;
  
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Currency: Currency<Self::AccountId>;
		type CollectionRandomness: Randomness<Self::Hash, Self::BlockNumber>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type MaximumOwned: Get<u32>;
	}

	#[derive(Encode, Decode, Clone, PartialEq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
	pub enum Color {
		Red,
		Yellow,
		Blue,
		Green,
	}

	#[derive(Encode, Decode, Clone, PartialEq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Collectible<T: Config> {
		pub unique_id: [u8; 16],
		pub price: Option<BalanceOf<T>>,
		pub color: Color,
		pub owner: T::AccountId,
	}
 
 
  #[pallet::error]
	pub enum Error<T> {
		/// Each collectible must have a unique identifier
		DuplicateCollectible,
		/// An account can't exceed the MaxOwned constant
		MaximumCollectiblesOwned,
		/// The total supply of collectibles can't exceed the u64 limit
		BoundsOverflow,
		/// The collectible doesn't exist
		NoCollectible,
		/// You are not the owner
		NotOwner,
		/// Trying to transfer a collectible to yourself
		TransferToYourself,
		/// The bid is lower than the ask price
		BidPriceTooSlow,
		/// The collectible is nor for sale
		NotForSale,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CollectibleCreated {collectible: [u8; 16], owner: T::AccountId}, 
	}


}
