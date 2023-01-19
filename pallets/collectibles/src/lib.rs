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
		BoundedVec,
		Twox64Concat,
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
	pub(super) type CollectibleMap<T: Config> =
		StorageMap<_, Twox64Concat, [u8; 16], Collectible<T>>;

	#[pallet::storage]
	pub(super) type OwnerOfCollectibles<T: Config> = StorageMap<
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
		// A new collectible created
		CollectibleCreated { collectible: [u8; 16], owner: T::AccountId },
		// A Collectible was transfered Successfuly 
		TransferSucceeded {collectible: [u8; 16], from: T::AccountId, to: T::AccountId},
		
		// Set price of Collectible
		PriceSet {collectible: [u8; 16], price: Option<BalanceOf<T>>}, 

		// Collectible Sold
		CollectibleSold {collectible: [u8; 16], seller: T::AccountId, buyer: T::AccountId, price: Option<BalanceOf<T>>}, 
	}

	// Internal and Callable Functions
	impl<T: Config> Pallet<T> {
		fn gen_unique_id() -> ([u8; 16], Color) {
			let random = T::CollectionRandomness::random(&b"unique_id"[..]).0;
			let unique_payload = (
				random,
				frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),
				frame_system::Pallet::<T>::block_number(),
			);
			let encoded_payload = unique_payload.encode();
			let hash = frame_support::Hashable::blake2_128(&encoded_payload);
			if hash[0] % 2 == 0 {
				(hash, Color::Red)
			} else {
				(hash, Color::Yellow)
			}
		}

		pub fn mint (
			owner: &T::AccountId,
			unique_id: [u8; 16], 
			color: Color,  
		) -> Result<[u8; 16], DispatchError> {
			let collectible = Collectible::<T> {unique_id, price: None, color, owner: owner.clone()};
			ensure!(!CollectibleMap::<T>::contains_key(&collectible.unique_id), Error::<T>::DuplicateCollectible);

			let count = CollectiblesCount::<T>::get();
			let new_count = count.checked_add(1).ok_or(Error::<T>::BoundsOverflow)?;
			OwnerOfCollectibles::<T>::try_append(&owner, collectible.unique_id)
			.map_err(|_| Error::<T>::MaximumCollectiblesOwned)?;

			CollectibleMap::<T>::insert(collectible.unique_id, collectible);
			CollectiblesCount::<T>::put(new_count);

			Self::deposit_event(Event::CollectibleCreated { collectible: unique_id, owner: owner.clone() });
			Ok(unique_id)
		}
	}
}
