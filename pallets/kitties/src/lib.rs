#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

// 移除不需要的模块

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// 随机模块
	use frame_support::traits::Randomness;
	// blake2_128
	use sp_io::hashing::blake2_128;

	// 定义一个结构体
	#[derive(Encode, Decode, clone, RuntimeDebug, PartialEq, Eq)]
	pub struct Kitty(pub [u8; 16]);

	// KittyIndex 标识猫的id
	type KittyIndex = u32;

	// 这里不用改
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// 定义一个存储猫的数组
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	// 存储下一个猫的id
	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> = StorageValue<_, KittyIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	// 存储猫
	pub type Kitties<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	// 存储owner
	pub type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		// SomethingStored { something: u32, who: T::AccountId },

		// 创建猫的事件
		KittyCreate(T::AccountId, KittyIndex, kitties::Kitty),

		// 繁殖猫的事件
		KittyBreed(T::AccountId, KittyIndex, Kitty),

		// 转移猫的事件
		KittyTransfer(T::AccountId, T::AccountId, KittyIndex),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		// NoneValue,
		/// Errors should have helpful documentation associated with them.
		// StorageOverflow,
		InvalidKittyId,
		SameParentIndex,
		NotOwner,
		TransferToSelf,
		NotForSale,
		PriceTooLow,
		PriceTooHigh,
		NotEnoughBalance,
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
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			// 检查交易是否签名
			let who = ensure_signed(origin)?;

			// Update storage.
			// <Something<T>>::put(something);

			let kitty_id = Self::next_kitty_id().map_err(|_| Error::<T>::InvalidKittyId)?;
			let dna = Self::random_value(&who);
			let kitty = Kitty(dna);

			Kitties::<T>::insert(kitty_id, Some(kitty.clone()));
			kittieowner::<T>::insert(kitty_id, Some(who.clone()));
			NextKittyId::<T>::put(kitty_id + 1);

			// Emit an event.
			Self::deposit_event(Event::kittyCreate(who, kitty_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		// 繁殖猫
		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: KittyIndex,
			kitty_id_2: KittyIndex,
		) -> DispatchResult {	
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

			let kitty1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;

			let mut data = [0u8; 16];

			for i in 0..kitty_1.0.len() {
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
			}
			let new_kitty = Kitty(data);
			<Kitties<T>>::insert(kitty_id, &new_kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::set(kitty_id + 1);

			Self::deposit_event(Event::KittyCreated(who, kitty_id, new_kitty));

			Ok(())
		}


		// 转移猫
		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			kitty_id: u32,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
		
			Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;
			ensure!(Some(who.clone()) == <KittyOwner<T>>::get(kitty_id), Error::<T>::NotOwner);
			ensure!(who.clone() != new_owner.clone(), Error::<T>::TransferToSelf);

			<KittyOwner<T>>::insert(kitty_id, &new_owner);

			Ok(())	
		
		}
		
		// 
		impl<T: Config> Pallet<T> {
			// 其中extrinsic_index类似eth的nonce，用来和sender一起增加随机值的随机性。
		// 之后通过blake2_128哈希函数，产生随机[u8; 16]数组。
			fn random_value(sender: &T::AccountId) -> [u8; 16] {
				let payload = (
					T::Randomness::random_seed(),
					&sender,
					<frame_system::Pallet<T>>::extrinsic_index(),
				);
				payload.using_encoded(blake2_128)
			}
	
			//  
			fn get_next_id() -> Result<KittyIndex, ()> {
				match Self::next_kitty_id() {
					KittyIndex::MAX => Err(()),
					val => {
						log::info!("got next kitty id");
						Ok(val)
					}
				}
			}
	
			// 
			fn get_kitty(kitty_id: KittyIndex) -> Result<Kitty, ()> {
				match Self::kitties(kitty_id) {
					Some(kitty) => {
						log::info!("got kitty");
						Ok(kitty)
					},
					None => {
						log::error!("kitty not found");
						Err(())
					},
				}
			}
		}
	}
}