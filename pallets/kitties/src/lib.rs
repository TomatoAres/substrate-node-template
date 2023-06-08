// step02, lib 文件框架
// #![cfg_attr(not(feature = "std"), no_std)]

// pub use pallet::*;

// #[frame_support::pallet]
// pub mod pallet {
//   use frame_support::pallet_prelude::*;
//     use frame_system::pallet_prelude::*;
//     use frame_support::{
//         sp_runtime::traits::Hash,
//         traits::{ Randomness, Currency, tokens::ExistenceRequirement },
//         transactional
//     };
//     use sp_io::hashing::blake2_128;

//     #[cfg(feature = "std")]
//     use serde::{Deserialize, Serialize};

//     // ACTION #1: Write a Struct to hold Kitty information.

//     // ACTION #2: Enum declaration for Gender.

//     // ACTION #3: Implementation to handle Gender type in Kitty struct.

//     #[pallet::pallet]
//     #[pallet::generate_store(trait Store)]
//     pub struct Pallet<T>(_);

//     /// Configure the pallet by specifying the parameters and types it depends on.
//     #[pallet::config]
//     pub trait Config: pallet_balances::Config + frame_system::Config {
//         /// Because this pallet emits events, it depends on the runtime's definition of an event.
//         type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

//         /// The Currency handler for the Kitties pallet.
//         type Currency: Currency<Self::AccountId>;

//         // ACTION #5: Specify the type for Randomness we want to specify for runtime.
//     }

//     // Errors.
//     #[pallet::error]
//     pub enum Error<T> {
//         // TODO Part III
//     }

//     // Events.
//     #[pallet::event]
//     #[pallet::metadata(T::AccountId = "AccountId")]
//     #[pallet::generate_deposit(pub(super) fn deposit_event)]
//     pub enum Event<T: Config> {
//         // TODO Part III
//     }

//     #[pallet::storage]
//     #[pallet::getter(fn all_kitties_count)]
//     pub(super) type AllKittiesCount<T: Config> = StorageValue<_, u64, ValueQuery>;

//     // ACTION #6: Add Nonce storage item.

//     // ACTION #9: Remaining storage items.

//     // TODO Part IV: Our pallet's genesis configuration.

//     #[pallet::call]
//     impl<T: Config> Pallet<T> {

//         // TODO Part III: create_kitty

//         // TODO Part III: set_price

//         // TODO Part III: transfer

//         // TODO Part III: buy_kitty

//         // TODO Part III: breed_kitty
//     }

//     // ACTION #4: helper function for Kitty struct

//     impl<T: Config> Pallet<T> {
//         // TODO Part III: helper functions for dispatchable functions

//         // ACTION #7: increment_nonce helper

//         // ACTION #8: random_hash helper

//         // TODO: mint, transfer_from

//     }
// }

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// step16，需要导入
	use frame_support::traits::Randomness;
	use sp_io::hashing::blake2_128;

	// step03, kitty 唯一标识
	pub type KittyId = u32;

	// step04, kitty 结构体，就一个字段，是一个数组，长度为 16
	// 实现了 Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq,
	// Default, TypeInfo, MaxEncodedLen	这些 trait
	//
	#[derive(
		Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
	)]
	pub struct Kitty(pub [u8; 16]);

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// 配置
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// step14, event 配置: pallet 会抛出 event，
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// step15, 随机数配置
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	// step05，获取默认 id，第一个是 0 ，然后才能获取 next id
	#[pallet::type_value]
	pub fn GetDefaultValue() -> KittyId {
		0_u32
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	// step06, kitty id 自增 存储
	// StorageMap
	// 用于存储键值对，第一个参数是存储的模块名，第二个参数是存储的键的类型，
	// 第三个参数是存储的值的类型，第四个参数是存储的查询类型
	pub type NextKittyId<T: Config> = StorageValue<_, KittyId, ValueQuery, GetDefaultValue>;

	// step07, kitty 存储
	// StorageMap 用于存储键值对，第一个参数是存储的模块名，第二个参数是存储的键的类型，
	// 第三个参数是存储的值的类型，第四个参数是存储的查询类型
	// 第五个参数是存储的修改类型，第六个参数是存储的是否可被外部访问
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

	// step08, kitty 拥有者
	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KittyParents<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId), OptionQuery>;

	// setp12, event 定义
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyCreated {
			who: T::AccountId,
			kitty_id: KittyId,
			kitty: Kitty,
		},
		KittyBreed {
			who: T::AccountId,
			kitty_id: KittyId,
			kitty: Kitty,
		},
		KittyTransfer {
			who: T::AccountId,
			recipient: T::AccountId,
			kitty_id: KittyId,
		},
	}

	// step13, error 定义
	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		InvalidKittyId,
		SameKittyId,
		NotOwner,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			let kitty_id = Self::get_next_id()?;
			// let kitty = Kitty(Default::default());
			let kitty = Kitty(Self::random_value(&who));

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);

			// Emit an event.
			Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: KittyId,
			kitty_id_2: KittyId,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);

			// ensure!(Kitties::<T>::contains_key(kitty_id_1), Error::<T>::InvalidKittyId);
			// ensure!(Kitties::<T>::contains_key(kitty_id_2), Error::<T>::InvalidKittyId);
			let kitty_1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

			let kitty_id = Self::get_next_id()?;

			// let kitty = Kitty(Self::random_value(&who));

			let selector = Self::random_value(&who);
			let mut data = [0u8; 16];

			for i in 0..kitty_1.0.len() {
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
			}

			let kitty = Kitty(data);

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			KittyParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));

			// Emit an event.
			Self::deposit_event(Event::KittyBreed { who, kitty_id, kitty });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			kitty_id: KittyId,
			recipient: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);

			let owner = Self::kitty_owner(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			ensure!(owner == who, Error::<T>::NotOwner);

			KittyOwner::<T>::insert(kitty_id, &recipient);

			Self::deposit_event(Event::KittyTransfer { who, recipient, kitty_id });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn get_next_id() -> Result<KittyId, DispatchError> {
			// step10, kitty id 自增 存储
			NextKittyId::<T>::try_mutate(|next_id| -> Result<KittyId, DispatchError> {
				//
				let current_id = *next_id;

				*next_id = next_id.checked_add(1).ok_or(Error::<T>::InvalidKittyId)?;

				Ok(current_id)
			})
		}
		// stp09 随机数
		//
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);

			payload.using_encoded(blake2_128)
		}

		// step11, 获取 kitty 信息，
		// 似乎没有用，暂时注释
		// fn get_kitty(kitty_id: KittyId) -> Result<Kitty, DispatchError> {
		// 	match self::Kitties::<T>::get(kitty_id) {
		// 		Some(kitty) => Ok(kitty),
		// 		None => Err(Error::<T>::InvalidKittyId.into()),
		// 	}
		// }
	}
}
