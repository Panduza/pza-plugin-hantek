mod offset;
mod scale;
use tokio::sync::Mutex;

use panduza_platform_core::std::attribute::boolean as std_att_boolean;
use panduza_platform_core::std::attribute::r#enum as std_att_enum;
use panduza_platform_core::{log_debug_mount_end, log_debug_mount_start, Container, Error};
use std::sync::Arc;

use crate::dso2c10::device::interface::BooleanIndex;
use crate::dso2c10::device::interface::StringIndex;

use super::interface::DSO2C10Interface;

///
///
pub async fn mount<C: Container + 'static>(
    mut parent: C,
    channel_id: usize,
    interface: Arc<Mutex<DSO2C10Interface>>,
) -> Result<(), Error> {
    let class_chan = parent
        .create_class(format!("{}", channel_id))
        .finish()
        .await;

    let logger = class_chan.logger().clone();
    log_debug_mount_start!(logger);

    //
    //
    std_att_boolean::mount(
        class_chan.clone(),
        interface.clone(),
        match channel_id {
            1 => BooleanIndex::Channel1BwLimit,
            2 => BooleanIndex::Channel2BwLimit,
            _ => BooleanIndex::Channel1BwLimit,
        } as usize,
        "bandwidth_limit",
        "* OFF: Turn off the 20MHz bandwidth limit, and the high-frequency components
contained in the measured signal can pass.
* ON: Turn on the bandwidth limitation, and the high-frequency components contained in
the signal under test are attenuated.
Turning on bandwidth limiting reduces waveform noise, but attenuates high-frequency
components.
",
    )
    .await?;

    //
    //
    std_att_boolean::mount(
        class_chan.clone(),
        interface.clone(),
        match channel_id {
            1 => BooleanIndex::Channel1Display,
            2 => BooleanIndex::Channel2Display,
            _ => BooleanIndex::Channel1Display,
        } as usize,
        "display",
        "",
    )
    .await?;

    //
    //
    std_att_boolean::mount(
        class_chan.clone(),
        interface.clone(),
        match channel_id {
            1 => BooleanIndex::Channel1Invert,
            2 => BooleanIndex::Channel2Invert,
            _ => BooleanIndex::Channel1Invert,
        } as usize,
        "invert",
        "",
    )
    .await?;

    //
    //
    std_att_boolean::mount(
        class_chan.clone(),
        interface.clone(),
        match channel_id {
            1 => BooleanIndex::Channel1Vernier,
            2 => BooleanIndex::Channel2Vernier,
            _ => BooleanIndex::Channel1Vernier,
        } as usize,
        "vertical_fine_tuning",
        "Manage the fine adjustment function of the vertical scale.

        The trim setting is off by default. At this time, you can only set the vertical scale in 1-
        2-5 steps, that is, 500u, 1mV, 2mV, 5mV, 10mV ... 10V (probe ratio is 1X). When the
        trim setting is on, you can further adjust the vertical scale within a smaller range to
        improve vertical resolution. If the amplitude of the input waveform is slightly larger
        than the full scale in the current scale, and the amplitude displayed by the waveform
        of the next gear is slightly lower, you can use fine adjustment to improve the
        waveform display amplitude to facilitate observation of signal details.
        ",
    )
    .await?;

    offset::mount(class_chan.clone(), channel_id, interface.clone()).await?;
    scale::mount(class_chan.clone(), channel_id, interface.clone()).await?;

    std_att_enum::mount(
        class_chan.clone(),
        interface.clone(),
        match channel_id {
            1 => StringIndex::Channel1Coupling,
            2 => StringIndex::Channel1Coupling,
            _ => StringIndex::Channel1Coupling,
        } as usize,
        "coupling",
        "",
        vec!["DC", "AC", "GND"],
    )
    .await?;

    //
    //
    log_debug_mount_end!(logger);
    Ok(())
}
