# substrate nfts

substrate nfts是用于substrate，面向非同质化代币的开源协议



# 一 使用流程

```
_issue_with_uri发行代币
|
V
transfer_from《--》_approve《--》_set_approval_for_all  //转账，授权，高级授权三个行为间流转代币
|
V
_burn 最终销毁代币
```



# 二 函数

1 //给某个用户发行代币，uri参数一般为http或ipfs协议的地址，指向代币的附加属性（一般为json参数）

​        fn _issue_with_uri(who: &T::AccountId, uri: Vec<u8>) -> Result



2 //销毁代币

​        fn _burn(token_id: T::NFTIndex) -> Result



3 //从账户from给某个账户to转账，下标为token_id的代币，转账带自定义的字节数组参数

​        fn transfer_from(from: T::AccountId, to: T::AccountId, token_id: T::NFTIndex, data: Vec<u8>) -> Result



4 //设置普通授权，普通授权，是指针对单个代币转账权限的授权，只能同时存在一个，当拥有权限变更时，会清0

​        fn _approve(origin: T::AccountId, to: T::AccountId, token_id: T::NFTIndex) -> Result 



5//设置高级授权，是指地址对地址的授权，被授权者可以操作授权者的所有代币，包括改变普通的授权。可以同时授权多个地址

​        fn _set_approval_for_all(origin: T::AccountId, to: T::AccountId, approved: bool) -> Result



# 三 事件

1 //转账事件

​        Transfer(Option<AccountId>, Option<AccountId>, NFTIndex),

 2  //普通授权事件

​        Approval(AccountId, AccountId, NFTIndex),

3 //高级授权事件

​        ApprovalForAll(AccountId, AccountId, bool),



# 四 数据

​        1 //某个用户拥有的代币数量

​        OwnedTokensCount get(balance_of): map T::AccountId => T::NFTIndex;

​        2 //通过代币ID查找用户

​        TokenOwner get(owner_of): map T::NFTIndex => Option<T::AccountId>;

​        3 //查找代币的授权委托情况

​        TokenApprovals get(get_approved): map T::NFTIndex => Option<T::AccountId>;

​        4 //查找用户的高级授权情况

​        OperatorApprovals get(is_approved_for_all): map (T::AccountId, T::AccountId) => bool;

​       5  //当前的代币总量

​        TotalSupply get(total_supply): T::NFTIndex;

​       6  // 获取代币的uri

​       TokenUri get(token_uri): map T::NFTIndex => Vec<u8>;



# 五 参考

 https://eips.ethereum.org/EIPS/eip-721