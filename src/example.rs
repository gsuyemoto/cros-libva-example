use libva::bindings;
use libva::BufferType;
use libva::Display;
use libva::IQMatrix;
use libva::IQMatrixBufferMPEG2;
use libva::Image;
use libva::MPEG2PictureCodingExtension;
use libva::Picture;
use libva::PictureParameter;
use libva::PictureParameterBufferMPEG2;
use libva::SliceParameter;
use libva::SliceParameterBufferH264;
use libva::UsageHint;
use std::rc::Rc;

fn main() {
    let display = Display::open().unwrap();
    assert!(!display.query_vendor_string().unwrap().is_empty());

    let profiles = display.query_config_profiles().unwrap();
    assert!(!profiles.is_empty());

    let profile = bindings::VAProfile::VAProfileH264Baseline;
    let entrypoints = display.query_config_entrypoints(profile).unwrap();
    assert!(!entrypoints.is_empty());
    assert!(entrypoints
        .iter()
        .any(|e| *e == bindings::VAEntrypoint::VAEntrypointVLD));

    let format = bindings::constants::VA_RT_FORMAT_YUV420;
    let width = 16u32;
    let height = 16u32;
    let mut attrs = vec![bindings::VAConfigAttrib {
        type_: bindings::VAConfigAttribType::VAConfigAttribRTFormat,
        value: 0,
    }];
    let entrypoint = bindings::VAEntrypoint::VAEntrypointVLD;
    display
        .get_config_attributes(profile, entrypoint, &mut attrs)
        .unwrap();
    assert!(attrs[0].value != bindings::constants::VA_ATTRIB_NOT_SUPPORTED);
    assert!(attrs[0].value & bindings::constants::VA_RT_FORMAT_YUV420 != 0);
    let config = display.create_config(attrs, profile, entrypoint).unwrap();
    let mut surfaces = display
        .create_surfaces(
            format,
            None,
            width,
            height,
            Some(UsageHint::USAGE_HINT_DECODER),
            vec![()],
        )
        .unwrap();
    let context = display
        .create_context(
            &config,
            width,
            ((height + 15) / 16) * 16,
            Some(&surfaces),
            true,
        )
        .unwrap();
    // The picture data is adapted from libva-utils at decode/mpeg2vldemo.cpp
    // Data dump of a 16x16 MPEG2 video clip,it has one I frame
    let mut mpeg2_clip: Vec<u8> = vec![
        0x00, 0x00, 0x01, 0xb3, 0x01, 0x00, 0x10, 0x13, 0xff, 0xff, 0xe0, 0x18, 0x00, 0x00, 0x01,
        0xb5, 0x14, 0x8a, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x01, 0xb8, 0x00, 0x08, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x00, 0x0f, 0xff, 0xf8, 0x00, 0x00, 0x01, 0xb5, 0x8f, 0xff, 0xf3,
        0x41, 0x80, 0x00, 0x00, 0x01, 0x01, 0x13, 0xe1, 0x00, 0x15, 0x81, 0x54, 0xe0, 0x2a, 0x05,
        0x43, 0x00, 0x2d, 0x60, 0x18, 0x01, 0x4e, 0x82, 0xb9, 0x58, 0xb1, 0x83, 0x49, 0xa4, 0xa0,
        0x2e, 0x05, 0x80, 0x4b, 0x7a, 0x00, 0x01, 0x38, 0x20, 0x80, 0xe8, 0x05, 0xff, 0x60, 0x18,
        0xe0, 0x1d, 0x80, 0x98, 0x01, 0xf8, 0x06, 0x00, 0x54, 0x02, 0xc0, 0x18, 0x14, 0x03, 0xb2,
        0x92, 0x80, 0xc0, 0x18, 0x94, 0x42, 0x2c, 0xb2, 0x11, 0x64, 0xa0, 0x12, 0x5e, 0x78, 0x03,
        0x3c, 0x01, 0x80, 0x0e, 0x80, 0x18, 0x80, 0x6b, 0xca, 0x4e, 0x01, 0x0f, 0xe4, 0x32, 0xc9,
        0xbf, 0x01, 0x42, 0x69, 0x43, 0x50, 0x4b, 0x01, 0xc9, 0x45, 0x80, 0x50, 0x01, 0x38, 0x65,
        0xe8, 0x01, 0x03, 0xf3, 0xc0, 0x76, 0x00, 0xe0, 0x03, 0x20, 0x28, 0x18, 0x01, 0xa9, 0x34,
        0x04, 0xc5, 0xe0, 0x0b, 0x0b, 0x04, 0x20, 0x06, 0xc0, 0x89, 0xff, 0x60, 0x12, 0x12, 0x8a,
        0x2c, 0x34, 0x11, 0xff, 0xf6, 0xe2, 0x40, 0xc0, 0x30, 0x1b, 0x7a, 0x01, 0xa9, 0x0d, 0x00,
        0xac, 0x64,
    ];
    let picture_coding_extension =
        MPEG2PictureCodingExtension::new(0, 3, 0, 1, 0, 0, 0, 0, 0, 1, 1);
    let pic_param = PictureParameterBufferMPEG2::new(
        16,
        16,
        0xffffffff,
        0xffffffff,
        1,
        0xffff,
        &picture_coding_extension,
    );
    let pic_param = BufferType::PictureParameter(PictureParameter::MPEG2(pic_param));
    let iq_matrix = IQMatrixBufferMPEG2::new(
        1,
        1,
        0,
        0,
        [
            8, 16, 16, 19, 16, 19, 22, 22, 22, 22, 22, 22, 26, 24, 26, 27, 27, 27, 26, 26, 26, 26,
            27, 27, 27, 29, 29, 29, 34, 34, 34, 29, 29, 29, 27, 27, 29, 29, 32, 32, 34, 34, 37, 38,
            37, 35, 35, 34, 35, 38, 38, 40, 40, 40, 48, 48, 46, 46, 56, 56, 58, 69, 69, 83,
        ],
        [
            16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
        [0; 64],
        [0; 64],
    );
    let iq_matrix = BufferType::IQMatrix(IQMatrix::MPEG2(iq_matrix));
    let slice_param = SliceParameterBufferH264::new(150, 0, 0, 38, 0, 0, 2, 0);
    let slice_param = BufferType::SliceParameter(SliceParameter::MPEG2(slice_param));
    let test_data_offset = 47;
    let slice_data = BufferType::SliceData(mpeg2_clip.drain(test_data_offset..).collect());
    let buffers = vec![
        context.create_buffer(pic_param).unwrap(),
        context.create_buffer(slice_param).unwrap(),
        context.create_buffer(iq_matrix).unwrap(),
        context.create_buffer(slice_data).unwrap(),
    ];
    let mut picture = Picture::new(0, Rc::clone(&context), surfaces.remove(0));
    for buffer in buffers {
        picture.add_buffer(buffer);
    }
    // Actual client code can just chain the calls.
    let picture = picture.begin().unwrap();
    let picture = picture.render().unwrap();
    let picture = picture.end().unwrap();
    let picture = picture.sync().map_err(|(e, _)| e).unwrap();

    // Test whether we can map the resulting surface to obtain the raw yuv
    // data
    let image_fmts = display.query_image_formats().unwrap();
    let image_fmt = image_fmts
        .into_iter()
        .find(|f| f.fourcc == bindings::constants::VA_FOURCC_NV12)
        .expect("No valid VAImageFormat found for NV12");
    let resolution = (width, height);
    let image = picture
        .create_image(image_fmt, resolution, resolution)
        .unwrap();
}
