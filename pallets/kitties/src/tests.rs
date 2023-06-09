use super::*;
use crate::mock::*;
use frame_support::assert_ok;

#[test]
// 创建Kitty成功
fn create_kitty_works() {
	new_test_ext().execute_with(|| {
		// 设置区块号为1
		System::set_block_number(1);
		// 用账户1创建一个kitty,kitty编号为0
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(1)));

		// kitty编号为0，next_kitty_id为1
		assert_eq!(NextKittyId::<Test>::get(), 1);

		// 根据kitty编号0查所属账户为1
		assert_eq!(KittyOwner::<Test>::try_get(0), Ok(1));
	})
}
