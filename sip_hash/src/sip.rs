/*

implements

 */



///
/// 这一行使用了 cfg_attr 属性，该属性根据条件编译设置来选择性地应用其他属性。
///
/// 如果特性（feature）"serde" 被启用，就会应用后面的 derive 属性，
/// 其中包含了 serde::Serialize 和 serde::Deserialize，用于支持序列化和反序列化
///
///
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SipHasher13 {}


#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SipHasher24 {}
