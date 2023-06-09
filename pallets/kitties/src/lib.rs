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

// 这个看官方文章有说明
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

//
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// s2p01, 导入 Currency trait 等需要的 trait
	use frame_support::traits::{Currency, ReservableCurrency};
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, One};

	// step16，需要导入
	use frame_support::traits::Randomness;
	use sp_io::hashing::blake2_128;

	// step03, kitty 唯一标识 这个算是 kitty 配置，需要在 config 下
	// pub type KittyIndex = u32;

	// step04, kitty 结构体，就一个字段，是一个数组，长度为 16
	// 实现了 Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq,
	// Default, TypeInfo, MaxEncodedLen	这些 trait
	#[derive(
		Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
	)]
	pub struct Kitty(pub [u8; 16]);

	// s2p03,BalanceOf ?
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

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

		// s2p02, 定义 kitty 索引
		type KittyIndex: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaxEncodedLen
			+ Bounded;

		// s2p04, 加了一些配置
		type KittyReserve: Get<BalanceOf<Self>>;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		type MaxLength: Get<u32>;
	}

	// // step05，获取默认 id，第一个是 0 ，然后才能获取 next id 类型和之前不一样了
	// #[pallet::type_value]
	// pub fn GetDefaultValue() -> KittyIndex {
	// 	0_u32
	// }

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	// step06, kitty id 自增 存储
	// StorageMap
	// 用于存储键值对，第一个参数是存储的模块名，第二个参数是存储的键的类型，
	// 第三个参数是存储的值的类型，第四个参数是存储的查询类型
	pub type NextKittyId<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery>;

	// step07, kitty 存储
	// StorageMap 用于存储键值对，第一个参数是存储的模块名，第二个参数是存储的键的类型，
	// 第三个参数是存储的值的类型，第四个参数是存储的查询类型
	// 第五个参数是存储的修改类型，第六个参数是存储的是否可被外部访问
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;

	// step08, kitty 拥有者
	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;

	// kitty 父母
	#[pallet::storage]
	#[pallet::getter(fn all_owner_kitty)]
	pub type AllOwnerKitty<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<Kitty, T::MaxLength>, OptionQuery>;

	// setp12, event 定义
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyCreated {
			who: T::AccountId,
			kitty_id: T::KittyIndex,
			kitty: Kitty,
		},
		KittyBreed {
			who: T::AccountId,
			kitty_id: T::KittyIndex,
			kitty: Kitty,
		},
		KittyTransfer {
			who: T::AccountId,
			recipient: T::AccountId,
			kitty_id: T::KittyIndex,
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
		NotOwner,
		SameKittyId,
		KittiesCountOverflow,
		TokenNotEnough,
		ExeddMaxKittyLength,
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

			// - 1. .map_err(|_| ...) 是一个函数,它接收 Err(E) 并将其映射为另一个错误返回
			// Err(E')。这里我们使用 _ 忽略原错误并返回自定义的 InvalidKittyId 错误。
			// - 2. ? 运算符用于错误传播,如果上一行返回 Ok(value) 将返回 value,
			// 否则返回 .map_err 映射后的错误。
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			// 预留 token 配置到 Currency 字段
			T::Currency::reserve(&who, T::KittyReserve::get())
				.map_err(|_| Error::<T>::TokenNotEnough)?;

			// 无用 删除 后续研究 default
			// let kitty = Kitty(Default::default());
			// dns 唯一 id，生成 kitty
			let dna = Self::random_value(&who);
			let kitty = Kitty(dna);

			// kitty  存储
			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::put(kitty_id + One::one());
			// AllOwnerKitty::<T>::try_mutate(&who, |kitty_vec| kitty_vec.try_push(kitty.clone()))?;
			AllOwnerKitty::<T>::try_mutate

			// Emit an event.
			Self::deposit_event(Event::KittyCreated { who, kitty_id, kitty });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: T::KittyIndex,
			kitty_id_2: T::KittyIndex,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			T::Currency::reserve(&who, T::KittyReserve::get())
				.map_err(|_| Error::<T>::TokenNotEnough)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);

			// ensure!(Kitties::<T>::contains_key(kitty_id_1), Error::<T>::InvalidKittyId);
			// ensure!(Kitties::<T>::contains_key(kitty_id_2), Error::<T>::InvalidKittyId);
			let kitty_1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

			let kitty_id = Self::get_next_id()?;

			//
			let selector = Self::random_value(&who);
			let mut data = [0u8; 16];
			for i in 0..kitty_1.0.len() {
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
			}
			let new_kitty = Kitty(data);

			Kitties::<T>::insert(kitty_id, &new_kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::put(kitty_id + One::one());
			// AllOwnerKitty::<T>::try_mutate(&who, |kitty_vec| {
			// 	kitty_vec.try_push(new_kitty.clone())
			// })?;
			// Emit an event.
			Self::deposit_event(Event::KittyBreed { who, kitty_id, kitty: new_kitty });

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let prev_owner = ensure_signed(origin)?;

			let exsit_kitty = Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;

			ensure!(Self::kitty_owner(kitty_id) == Some(prev_owner.clone()), Error::<T>::NotOwner);

			T::Currency::reserve(&new_owner, T::KittyReserve::get())
				.map_err(|_| Error::<T>::TokenNotEnough)?;

			KittyOwner::<T>::insert(kitty_id, &new_owner);
			AllOwnerKitty::<T>::try_mutate(&prev_owner, |owned| {
				if let Some(index) = owned.iter().position(|kitty| kitty == &exsit_kitty) {
					owned.swap_remove(index);
					return Ok(())
				}
				Err(())
			})
			.map_err(|_| <Error<T>>::NotOwner)?;

			Self::deposit_event(Event::KittyTransfer { who, recipient: new_owner, kitty_id });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn get_next_id() -> Result<T::KittyIndex, DispatchError> {
			// step10, kitty id 自增 存储
			let kitty_id = Self::next_kitty_id();
			if kitty_id == T::KittyIndex::max_value() {
				return Err(Error::<T>::KittiesCountOverflow.into())
			}
			// NextKittyId::<T>::try_mutate(|next_id| -> Result<KittyId, DispatchError> {
			// 	//
			// 	let current_id = *next_id;

			// 	*next_id = next_id.checked_add(1).ok_or(Error::<T>::InvalidKittyId)?;

			Ok(kitty_id)
			// })
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
		fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, DispatchError> {
			match self::Kitties::<T>::get(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(Error::<T>::InvalidKittyId.into()),
			}
		}
	}
}
