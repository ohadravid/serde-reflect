#![allow(unused)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::redundant_static_lifetimes)]

use serde::Deserialize;

mod meta;
mod raw_api;
mod serde_manual_impl;
mod v0_api;
mod v1_api;
mod v2_api;

fn main() -> anyhow::Result<()> {
    serde_manual_impl::serde_example()?;
    serde_manual_impl::serde_manual_impl_example()?;

    let res: Vec<v0_api::Fan> = v0_api::query_fans();

    for fan in res {
        if fan.active_cooling {
            println!(
                "v0: Fan `{}` is running at {} RPM",
                fan.name, fan.desired_speed
            );
        }
    }

    let res: Vec<v1_api::Fan> = v1_api::query();

    for fan in res {
        if fan.active_cooling {
            println!(
                "v1: Fan `{}` is running at {} RPM",
                fan.name, fan.desired_speed
            );
        }
    }

    let res: Vec<v2_api::Fan> = v2_api::query()?;

    for fan in res {
        if fan.active_cooling {
            println!(
                "v2: Fan `{}` is running at {} RPM",
                fan.name, fan.desired_speed
            );
        }
    }

    #[derive(Default, Deserialize)]
    #[serde(rename = "Win32_Fan")]
    #[serde(rename_all = "PascalCase")]
    pub struct BorrowedFan<'a> {
        name: &'a str,
        active_cooling: bool,
        desired_speed: u64,
    }

    // Error: implementation of `api::_::_serde::Deserialize` is not general enough
    // let res: Vec<BorrowedFan> = v2_api::query()?;

    Ok(())
}
