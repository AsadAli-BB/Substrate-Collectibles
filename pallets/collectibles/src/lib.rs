#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, pallet, traits::{Currency, Randomness, Get}, StorageValue, sp_runtime::app_crypto::sp_core::storage::StorageMap, Twox64Concat};
    use frame_system::{pallet_prelude::*, Config};
    use frame_support::sp_runtime::traits::IntegerSquareRoot;
    type BalanceOf<T> =
      <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
    
    #[pallet::config]
    pub trait Config: frame_system::Config {
      type Currency: Currency<Self::AccountId>;
      type CollectionRandomness: Randomness<Self::Hash, Self::BlockNumber>;
      
    
    
      #[pallet::constant]
      type MaximumOwned : Get<u32>; 
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

  #[pallet::storage]
  pub(super) type CollectiblesCount<T: Config>  = StorageValue<_,  u64, ValueQuery>;
  pub (super) type CollectibleMap<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Collectible<T>>;
  

}

