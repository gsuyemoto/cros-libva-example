use libva::Display;
use libva::bindings:
use anyhow::*;

fn main() -> Result<()> {
    let display = match Display::open() {
        Some(display) => display,
        None = > Err(anyhow!("Problem opening display")),
    }

    let profile_list = match display.query_config_profiles() {
        Ok(profiles) => profiles,
        Err(e) => Err(anyhow!("Error querying config profiles {e}")),
    }

    println!("//--------------------- Profile List");
    profile_list.iter().inspect(|p| println!("Profile: {p}"));

    let profile = bindings::VAProfile::VAProfileH264Baseline;
    let entrypts_list = match display.query_config_entrypoints(profile) {
        Ok(entries) => entries,
        Err(e) => Err(anyhow!(Error querying display entrypoints {e})),
    }

    println!("//--------------------- Entrypoints List");
    entrypts_list.iter().inspect(|p| println!("Entry: {p}"));
    
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 352;

    let mut attrs = vec![bindings::VAConfigAttrib {
        type_: bindings::VAConfigAttribType::VAConfigAttribRTFormat,
        value: 0,
    }];

    display.get_config_attributes(
        profile,
        bindings::VAEntrypoint::VAEntrypointVLD,
        &mut attrs)
    
    let mut surfaces = display
        .create_surfaces(
            bindings::constants::VA_RT_FORMAT_YUV420,
            None,
            WIDTH,
            HEIGHT,
            Some(UsageHint::USAGE_HINT_DECODER),
            vec![()],
        )
        .unwrap();

    let context = display
        .create_context(
            &config,
            WIDTH,
            HEIGHT,
            Some(&surfaces),
            true,
        )
        .unwrap();

    Ok(())
}
