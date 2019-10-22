use support::{
	decl_module, decl_storage, decl_event
};
// use sr_primitives::traits::{SimpleArithmetic, Bounded, Member};
// use codec::{Encode, Decode};
// use runtime_io::blake2_128;
// use system::ensure_signed;
use rstd::result;
use support::dispatch::Vec;
use codec::alloc::string::String;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


pub trait NFT<AccountId>: system::Trait {

	/************************************************* 
	Function:       // transfer_from转账 
	Description:    // 函数功能、性能等的描述 
	Input:           
					from  发送代币的用户ID
					to    接收代币的用户ID
					token_id NFT代币的下标
					data     发送函数的附加数据
	Output:           
	Return:           Result    执行结构

	*************************************************/ 
    fn transfer_from( from: Self::AccountId, to: Self::AccountId, token_id: Self::Index, data: Vec<u8>) -> result::Result<(), &'static str>;
	
	/************************************************* 
	Function:       // approve设置普通授权 
	Description:    // 普通授权，是指针对单个代币转账权限的授权，只能同时存在一个，当拥有权限变更时，会清0
	Input:          
					origin  设置授权用户ID
					to      接收授权用户ID
					token_id NFT代币的下标					  
	Output:           
	Return:         Result    执行结果
	*************************************************/ 
	fn approve(origin: Self::AccountId, to: Self::AccountId, token_id: Self::Index) -> result::Result<(), &'static str>;
	
	/************************************************* 
	Function:       // set_approval_for_all设置高级授权
	Description:    // 是指地址对地址的授权，被授权者可以操作授权者的所有代币，包括改变普通的授权。可以同时授权多个地址
	Input:          
					origin  设置授权用户ID
					to      接收授权用户ID
					approved 设置授权标识,true为允许					  
	Output:           
	Return:         Result    执行结果
	*************************************************/ 
    fn set_approval_for_all(origin: Self::AccountId, to: Self::AccountId, approved: bool) -> result::Result<(), &'static str>;
	

/************************************************* 
	Function:       // issue_with_uri 发行代币
	Description:     
	Input:          					  
					to      接收代币用户ID
					uri     代币附加信息uri地址					  
	Output:           
	Return:         Result    执行结果
	*************************************************/ 
    fn issue_with_uri(to: &Self::AccountId,  uri: String) ->result::Result<(), &'static str>;


/************************************************* 
	Function:       // burn销毁代币
	Description:     
	Input:          					  
					Index  NFT代币的下标
	Output:           
	Return:         Result    执行结果
	*************************************************/ 
    fn burn(token_index: Self::Index) -> result::Result<(), &'static str>;
}

decl_storage! {
	trait Store for Module<T: Trait> as NFTS {
		//某个用户拥有的代币数量
		OwnedTokensCount get(balance_of): map T::AccountId => u64;
		//通过代币ID查找用户
		TokenOwner get(owner_of): map T::Index => Option<T::AccountId>;
		//查找代币的授权委托情况
		TokenApprovals get(get_approved): map T::Index => Option<T::AccountId>;
		//查找用户的高级授权情况
		OperatorApprovals get(is_approved_for_all): map (T::AccountId, T::AccountId) => bool;
		//当前的代币总量
		TotalSupply get(total_supply): u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    }
}


decl_event!(
	pub enum Event<T> where
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Index,
	{
		//转账事件
        Transfer(Option<AccountId>, Option<AccountId>, Index),
		//普通授权事件
        Approval(AccountId, AccountId, Index),
		//高级授权事件
        ApprovalForAll(AccountId, AccountId, bool),
	}
);