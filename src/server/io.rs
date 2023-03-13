/// 一切`Cetacea`协议使用`MessagePack`作为应用层（表示层）协议。
/// 暂时以`jsonmpk`库作为事实标准。
///
/// 一切`Cetacea`协议使用`WebSocket`作为传输层协议。
///
/// 使用`Json`作为协议的人类可读格式，仅在调试和格式转换时使用。
/// `Json`到`MessagePack`的转换行为标准暂时以`jsonmpk`库为事实标准。
///
/// ## 通用协议
///
/// ### 消息回复格式
///
/// 正常回复，以`ok`作为返回标志：
///
/// ```json
/// {
///  "status": "ok",
///  "value": "any data struct",
/// }
/// ```
///
/// <!-- 异常回复，以一切不为`ok`的`status`标识作为异常的标志： -->
/// 异常回复，以err作为`status`标志：
///
/// ```json
/// {
///  "status": "err",
///  // "error": "error type",
///  "value": "any error detail info",
/// }
/// ```
///
/// ### 消息回复格式在编程语言中的表示（参考实现）
///
/// Rust：

#[allow(non_camel_case_types)]
enum Status {
    ok,
    err,
}

// pub trait GetStatus {
// fn get_status(&self) -> Status;
// }

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct MessageResult<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned>(
    pub Result<T, E>,
);

struct MessageResponse {
    status: Status,
    value: Value,
}

impl<T: Serialize + DeserializeOwned, E: Serialize + DeserializeOwned> From<MessageResult<T, E>>
    for Result<MessageResponse, serde_json::Error>
{
    fn from(i: MessageResult<T, E>) -> Result<MessageResponse, serde_json::Error> {
        let r = match i.0 {
            Ok(v) => MessageResponse {
                status: Status::ok,
                value: serde_json::to_value(v)?,
            },
            Err(e) => MessageResponse {
                status: Status::err,
                value: serde_json::to_value(e)?,
            },
        };
        Ok(r)
    }
}

pub enum DeviceType {
    Desktop,
    Mobile,
    IoT,
}

#[allow(non_camel_case_types)]
pub enum ArchType {
    arm,
    thumb,
    aarch64,
    loongarch32,
    loongarch64,
    mips,
    mips64,
    riscv32,
    riscv64,
    x86,
    x86_64,
    wasm32,
    wasm64,
    OtherArch(String),
}

pub enum VendorType {
    UnknownVendor,
    Apple,
    PC,
    SCEI,
    Freescale,
    IBM,
    ImaginationTechnologies,
    MipsTechnologies,
    NVIDIA,
    CSR,
    Myriad,
    AMD,
    Mesa,
    SUSE,
    OpenEmbedded,
    OtherVendor(String),
}

pub enum OSType {
    UnknownOS,
    Ananas,
    CloudABI,
    Darwin,
    DragonFly,
    FreeBSD,
    Fuchsia,
    IOS,
    KFreeBSD,
    Linux,
    Lv2,
    MacOSX,
    NetBSD,
    OpenBSD,
    Solaris,
    Win32,
    ZOS,
    Haiku,
    Minix,
    RTEMS,
    NaCl,
    AIX,
    CUDA,
    NVCL,
    AMDHSA,
    PS4,
    PS5,
    ELFIAMCU,
    TvOS,
    WatchOS,
    DriverKit,
    Mesa3D,
    Contiki,
    AMDPAL,
    HermitCore,
    Hurd,
    WASI,
    Emscripten,
    ShaderModel,
    LiteOS,
    OtherOS(String),
}

pub struct PlatformTriple(pub ArchType, pub VendorType, pub OSType);
