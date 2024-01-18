use frame_support::{
	pallet_prelude::*, storage::StoragePrefixedMap, traits::GetStorageVersion, weights::Weight,
};

use frame_support::{migration::storage_key_iter, Blake2_128Concat};
use frame_system::pallet_prelude::*;
use crate::{Config, Kitties, KittyId, Pallet , Kitty};

#[derive(Encode, Decode, Clone, Debug ,PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct OldKitty{dna :[u8; 16],name:[u8;4] ,}

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();
    
	//let a = Pallet::<T>::current_storage_version();
	//let a = Pallet::<T>::on_chain_storage_version();

	if on_chain_version != 1 {
		return Weight::zero();
	}

	if on_chain_version != 2 {
		return Weight::zero();
	}

	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, OldKitty, Blake2_128Concat>(module, item).drain()
	{
        let mut name = [0u8; 8];
		[0..4].copy_from_slice(&kitty.name[..4]);
		let new_kitty = Kitty { dna: kitty.0, name };
		Kitties::<T>::insert(index, &new_kitty);
	}
	Weight::zero()
}
