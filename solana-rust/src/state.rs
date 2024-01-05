use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    program_pack::{IsInitialized, Sealed},
};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MovieAccountState {
    pub initialized: bool,
    pub rating: u8,
    pub title: String,
    pub description: String
}

// 使用 sealed Trait特征的默认实现，获取对象的 Sized
impl Sealed for MovieAccountState {}

// 判断该评论是否已存在
impl IsInitialized for MovieAccountState {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}