
use sr_primitives::traits::{SimpleArithmetic, Bounded, CheckedAdd, CheckedSub, Member};
use support::{
        decl_module, decl_storage, decl_event, ensure, StorageValue, StorageMap,
        Parameter, dispatch::Result
};
use system::ensure_signed;


// use codec::{Encode, Decode};
// use runtime_io::blake2_128;
// use system::ensure_signed;
// use rstd::result;
use support::dispatch::Vec;
// use codec::alloc::string::String;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type NFTIndex: Parameter + Member + SimpleArithmetic + Bounded + Default + Copy;

}


impl<T: Trait> Module<T> {

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
    fn transfer_from( from: T::AccountId, to: T::AccountId, token_id: T::NFTIndex, data: Vec<u8>) -> Result{
		Ok(())
	}
	
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
	fn approve(origin: T::AccountId, to: T::AccountId, token_id: T::NFTIndex) -> Result{
		Ok(())
	}
	
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
    fn set_approval_for_all(origin: T::AccountId, to: T::AccountId, approved: bool) -> Result{
		Ok(())
	}
	

	/************************************************* 
	Function:       // issue_with_uri 发行代币
	Description:     
	Input:          					  
					to      接收代币用户ID
					uri     代币附加信息uri地址					  
	Output:           
	Return:         Result    执行结果
	*************************************************/ 
    fn _issue_with_uri(who: &T::AccountId,  uri: Vec<u8>) ->Result{

        let token_id = Self::total_supply();

		ensure!(!<TokenOwner<T>>::exists(token_id), "Token hash already exists");
		let balance_of = Self::balance_of(who);

        let new_balance_of = match balance_of.checked_add(&1.into()) {
            Some(c) => c,
            None => return Err("Overflow adding a new token to account balance"),
        };

		Self::supply_increase()?;
		<TokenUri<T>>::insert(token_id, uri);
		
        <TokenOwner<T>>::insert(token_id, who);
        <OwnedTokensCount<T>>::insert(who, new_balance_of);
		Nonce::mutate(|n| *n += 1);
        Self::deposit_event(RawEvent::Transfer(None, Some(who.clone()), token_id));
		
		Ok(())
	}

	/************************************************* 
	Function:       // burn销毁代币
	Description:     
	Input:          					  
					Index  NFT代币的下标
	Output:           
	Return:         Result    执行结果
	*************************************************/ 
    fn _burn(token_id: T::NFTIndex) -> Result{
		let owner = match Self::owner_of(token_id) {
            Some(c) => c,
            None => return Err("No owner for this token"),
        };

        let balance_of = Self::balance_of(&owner);

        let new_balance_of = match balance_of.checked_sub(&1.into()) {
            Some(c) => c,
            None => return Err("Underflow subtracting a token to account balance"),
        };
		
		Self::supply_decrease()?;
		<TokenUri<T>>::remove(token_id);
		// clear approval here, to do...
        // Self::_clear_approval(token_id)?;

        <OwnedTokensCount<T>>::insert(&owner, new_balance_of);
        <TokenOwner<T>>::remove(token_id);
		
		Nonce::mutate(|n| *n += 1);
        Self::deposit_event(RawEvent::Transfer(Some(owner), None, token_id));

		Ok(())
	}

	// below is helper functions 
	fn supply_increase()  -> Result{
		let total_supply = Self::total_supply();

        // Should never fail since overflow on user balance is checked before this
        let new_total_supply = match total_supply.checked_add(&1.into()) {
            Some (c) => c,
            None => return Err("Overflow when adding new token to total supply"),
        };

		<TotalSupply<T>>::put(new_total_supply);

        Ok(())

	}
	fn supply_decrease()  -> Result{
		let total_supply = Self::total_supply();

        // Should never fail because balance of underflow is checked before this
        let new_total_supply = match total_supply.checked_sub(&1.into()) {
            Some(c) => c,
            None => return Err("Underflow removing token from total supply"),
        };

        <TotalSupply<T>>::put(new_total_supply);

        Ok(())
	}

}

decl_storage! {
	trait Store for Module<T: Trait> as NFTS {
		//某个用户拥有的代币数量
		OwnedTokensCount get(balance_of): map T::AccountId => T::NFTIndex;
		//通过代币ID查找用户
		TokenOwner get(owner_of): map T::NFTIndex => Option<T::AccountId>;
		//查找代币的授权委托情况
		TokenApprovals get(get_approved): map T::NFTIndex => Option<T::AccountId>;
		//查找用户的高级授权情况
		OperatorApprovals get(is_approved_for_all): map (T::AccountId, T::AccountId) => bool;
		//当前的代币总量
		TotalSupply get(total_supply): T::NFTIndex;

		// token id => token uri
		// TokenUri get(token_uri): map T::NFTIndex => Option<Vec<u8>>;
		TokenUri get(token_uri): map T::NFTIndex => Vec<u8>;


		// Not a part of the ERC721 specification, but recommended to add.
		Nonce: u64;


    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn issue_with_uri(origin,  uri: Vec<u8>) ->Result{
			let sender = ensure_signed(origin)?;
			Self::_issue_with_uri(&sender, uri.clone())
		}
		 pub fn burn(origin, token_id:T::NFTIndex) -> Result{ 
			let sender = ensure_signed(origin)?;
			Self::_burn(token_id)
		 }
    }
}


decl_event!(
	pub enum Event<T> where
		<T as system::Trait>::AccountId,
		<T as Trait>::NFTIndex,
	{
		//转账事件
        Transfer(Option<AccountId>, Option<AccountId>, NFTIndex),
		//普通授权事件
        Approval(AccountId, AccountId, NFTIndex),
		//高级授权事件
        ApprovalForAll(AccountId, AccountId, bool),
	}
);