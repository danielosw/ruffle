use avm1::types::*;
use std::fs::File;
use std::vec::Vec;
use read::read_swf;
use read::tests::{read_tag_bytes_from_file, read_tag_bytes_from_file_with_index};
use tag_codes::TagCode;
use types::*;
use write::write_swf;

#[allow(dead_code)]
pub fn echo_swf(filename: &str) {
    let in_file = File::open(filename).unwrap();
    let swf = read_swf(in_file).unwrap();
    let out_file = File::create(filename).unwrap();
    write_swf(&swf, out_file).unwrap();
}

pub type TestData<T> = (u8, T, Vec<u8>);
pub type TagTestData = TestData<Tag>;
pub type Avm1TestData = TestData<Action>;

pub fn tag_tests() -> Vec<TagTestData> { vec![
    (
        9, // Minimum version not listed in SWF19.
        Tag::DefineBinaryData {
            id: 1,
            data: vec![84, 101, 115, 116, 105, 110, 103, 33]
        },
        read_tag_bytes_from_file("tests/swfs/definebinarydata.swf", TagCode::DefineBinaryData)
    ),

    (
        1,
        Tag::DefineBits {
            id: 1,
            jpeg_data: vec![
                255, 216, 255, 224, 0, 16, 74, 70, 73, 70, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 255, 192,
                0, 17, 8, 0, 5, 0, 6, 3, 1, 34, 0, 2, 17, 1, 3, 17, 1, 255, 218, 0, 12, 3, 1, 0, 2,
                17, 3, 17, 0, 63, 0, 252, 215, 162, 138, 43, 248, 28, 255, 0, 180, 3, 255, 217
            ],
        },
        read_tag_bytes_from_file("tests/swfs/DefineBits-JpegTables-MX.swf", TagCode::DefineBits)
    ),

    (
        1,
        Tag::DefineBitsJpeg2 {
            id: 1,
            jpeg_data: vec![
                255, 216, 255, 219, 0, 67, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 255, 219, 0, 67, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1, 255, 196, 0, 31, 0, 0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255, 196, 0, 181, 16, 0, 2, 1, 3, 3, 2, 4, 3,
                5, 5, 4, 4, 0, 0, 1, 125, 1, 2, 3, 0, 4, 17, 5, 18, 33, 49, 65, 6, 19, 81, 97, 7,
                34, 113, 20, 50, 129, 145, 161, 8, 35, 66, 177, 193, 21, 82, 209, 240, 36, 51, 98,
                114, 130, 9, 10, 22, 23, 24, 25, 26, 37, 38, 39, 40, 41, 42, 52, 53, 54, 55, 56,
                57, 58, 67, 68, 69, 70, 71, 72, 73, 74, 83, 84, 85, 86, 87, 88, 89, 90, 99, 100,
                101, 102, 103, 104, 105, 106, 115, 116, 117, 118, 119, 120, 121, 122, 131, 132,
                133, 134, 135, 136, 137, 138, 146, 147, 148, 149, 150, 151, 152, 153, 154, 162,
                163, 164, 165, 166, 167, 168, 169, 170, 178, 179, 180, 181, 182, 183, 184, 185,
                186, 194, 195, 196, 197, 198, 199, 200, 201, 202, 210, 211, 212, 213, 214, 215,
                216, 217, 218, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 241, 242, 243,
                244, 245, 246, 247, 248, 249, 250, 255, 196, 0, 31, 1, 0, 3, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255, 196, 0, 181, 17, 0,
                2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 119, 0, 1, 2, 3, 17, 4, 5, 33, 49, 6, 18,
                65, 81, 7, 97, 113, 19, 34, 50, 129, 8, 20, 66, 145, 161, 177, 193, 9, 35, 51, 82,
                240, 21, 98, 114, 209, 10, 22, 36, 52, 225, 37, 241, 23, 24, 25, 26, 38, 39, 40,
                41, 42, 53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71, 72, 73, 74, 83, 84, 85, 86, 87,
                88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106, 115, 116, 117, 118, 119, 120,
                121, 122, 130, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147, 148, 149, 150,
                151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170, 178, 179, 180,
                181, 182, 183, 184, 185, 186, 194, 195, 196, 197, 198, 199, 200, 201, 202, 210,
                211, 212, 213, 214, 215, 216, 217, 218, 226, 227, 228, 229, 230, 231, 232, 233,
                234, 242, 243, 244, 245, 246, 247, 248, 249, 250, 255, 217, 255, 216, 255, 224,
                0, 16, 74, 70, 73, 70, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 255, 192, 0, 17, 8, 0, 5, 0,
                5, 3, 1, 34, 0, 2, 17, 1, 3, 17, 1, 255, 218, 0, 12, 3, 1, 0, 2, 17, 3, 17, 0, 63,
                0, 252, 215, 162, 138, 43, 248, 28, 255, 0, 180, 3, 255, 217
            ],
        },
        read_tag_bytes_from_file("tests/swfs/DefineBitsJpeg2-MX.swf", TagCode::DefineBitsJpeg2)
    ),

    (
        1,
        Tag::DefineButton(Box::new(Button {
            id: 3,
            is_track_as_menu: false,
            records: vec![
                ButtonRecord {
                    id: 1,
                    states: vec![ButtonState::Up, ButtonState::Over].into_iter().collect(),
                    depth: 1,
                    matrix: Matrix::new(),
                    color_transform: ColorTransform::new(),
                    filters: vec![],
                    blend_mode: BlendMode::Normal,
                },
                ButtonRecord {
                    id: 2,
                    states: vec![ButtonState::Down, ButtonState::HitTest].into_iter().collect(),
                    depth: 1,
                    matrix: Matrix::new(),
                    color_transform: ColorTransform::new(),
                    filters: vec![],
                    blend_mode: BlendMode::Normal,
                }
            ],
            actions: vec![ButtonAction {
                conditions: vec![ButtonActionCondition::OverDownToOverUp].into_iter().collect(),
                key_code: None,
                action_data: vec![0],
            }],
        })),
        read_tag_bytes_from_file("tests/swfs/definebutton.swf", TagCode::DefineButton)
    ),

    (
        3,
        Tag::DefineButton2(Box::new(Button {
            id: 4,
            is_track_as_menu: true,
            records: vec![
                ButtonRecord {
                    id: 2,
                    states: vec![ButtonState::Up, ButtonState::Over].into_iter().collect(),
                    depth: 1,
                    matrix: Matrix::new(),
                    color_transform: ColorTransform { r_multiply: 1f32, g_multiply: 1f32, b_multiply: 1f32, a_multiply: 1f32, r_add: 200, g_add: 0, b_add: 0, a_add: 0 },
                    filters: vec![
                        Filter::BlurFilter(Box::new(BlurFilter {
                            blur_x: 5f64,
                            blur_y: 5f64,
                            num_passes: 1,
                        })),
                    ],
                    blend_mode: BlendMode::Difference,
                },
                ButtonRecord {
                    id: 3,
                    states: vec![ButtonState::Down, ButtonState::HitTest].into_iter().collect(),
                    depth: 1,
                    matrix: Matrix::new(),
                    color_transform: ColorTransform { r_multiply: 0f32, g_multiply: 1f32, b_multiply: 0f32, a_multiply: 1f32, r_add: 0, g_add: 0, b_add: 0, a_add: 0 },
                    filters: vec![],
                    blend_mode: BlendMode::Normal,
                },
            ],
            actions: vec![
                ButtonAction {
                    conditions: vec![ButtonActionCondition::OverDownToOverUp].into_iter().collect(),
                    key_code: None,
                    action_data: vec![150, 3, 0, 0, 65, 0, 38, 0], // trace("A");
                },
                ButtonAction {
                    conditions: vec![ButtonActionCondition::KeyPress].into_iter().collect(),
                    key_code: Some(3), // Home
                    action_data: vec![150, 3, 0, 0, 66, 0, 38, 0], // trace("B");
                },
            ],
        })),
        read_tag_bytes_from_file("tests/swfs/definebutton2.swf", TagCode::DefineButton2)
    ),

    (
        2,
        Tag::DefineButtonColorTransform {
            id: 3,
            color_transforms: vec![
                ColorTransform {
                    r_multiply: 1f32,
                    g_multiply: 0f32,
                    b_multiply: 0f32,
                    a_multiply: 1f32,
                    r_add: 1,
                    g_add: 0,
                    b_add: 0,
                    a_add: 0,
                },
                ColorTransform {
                    r_multiply: 0f32,
                    g_multiply: 1f32,
                    b_multiply: 0f32,
                    a_multiply: 1f32,
                    r_add: 0,
                    g_add: 1,
                    b_add: 0,
                    a_add: 0,
                },
                ColorTransform {
                    r_multiply: 0f32,
                    g_multiply: 0f32,
                    b_multiply: 1f32,
                    a_multiply: 1f32,
                    r_add: 0,
                    g_add: 0,
                    b_add: 1,
                    a_add: 0,
                },
            ],
        },
        read_tag_bytes_from_file("tests/swfs/definebuttoncxformsound.swf", TagCode::DefineButtonCxform)
    ),

    (
        2,
        Tag::DefineButtonSound(Box::new(ButtonSounds {
            id: 3,
            up_to_over_sound: Some((2, SoundInfo {
                event: SoundEvent::Event,
                in_sample: None,
                out_sample: None,
                num_loops: 1,
                envelope: None,
            })),
            over_to_down_sound: Some((2, SoundInfo {
                event: SoundEvent::Start,
                in_sample: None,
                out_sample: None,
                num_loops: 2,
                envelope: None,
            })),
            down_to_over_sound: None,
            over_to_up_sound: None,

        })),
        read_tag_bytes_from_file("tests/swfs/definebuttoncxformsound.swf", TagCode::DefineButtonSound)
    ),

    (
        1,
        Tag::DefineFont(Box::new(Font {
            id: 1,
            glyphs: vec![
                vec![
                    ShapeRecord::StyleChange(StyleChangeData {
                        move_to: Some((19.45, -14.0)),
                        fill_style_0: None,
                        fill_style_1: Some(1),
                        line_style: Some(0),
                        new_styles: None
                    }),
                    ShapeRecord::StraightEdge { delta_x: -15.6, delta_y: 0.0 },
                    ShapeRecord::StraightEdge { delta_x: 0.0, delta_y: -4.55 },
                    ShapeRecord::StraightEdge { delta_x: 15.6, delta_y: 0.0 },
                    ShapeRecord::StraightEdge { delta_x: 0.0, delta_y: 4.55 }
                ],
                vec![
                    ShapeRecord::StyleChange(StyleChangeData {
                        move_to: Some((32.65, 7.5)),
                        fill_style_0: None,
                        fill_style_1: Some(1),
                        line_style: Some(0),
                        new_styles: None
                    }),
                    ShapeRecord::StraightEdge { delta_x: -32.75, delta_y: 0.0 },
                    ShapeRecord::StraightEdge { delta_x: 0.0, delta_y: -3.0 },
                    ShapeRecord::StraightEdge { delta_x: 32.75, delta_y: 0.0 },
                    ShapeRecord::StraightEdge { delta_x: 0.0, delta_y: 3.0 }
                ],
            ],
        })),
        read_tag_bytes_from_file("tests/swfs/DefineFont-MX.swf", TagCode::DefineFont)
    ),

    (
        1,
        Tag::DefineFontInfo(Box::new(FontInfo {
            id: 1,
            name: "Verdana".to_string(),
            is_small_text: false,
            is_ansi: true,
            is_shift_jis: false,
            is_italic: false,
            is_bold: false,
            code_table: vec![45, 95],
        })),
        read_tag_bytes_from_file("tests/swfs/DefineFont-MX.swf", TagCode::DefineFontInfo)
    ),

    (
        8,
        Tag::DefineScalingGrid {
            id: 2,
            splitter_rect: Rectangle { x_min: 10f32, x_max: 40f32, y_min: 10f32, y_max: 40f32 },
        },
        read_tag_bytes_from_file("tests/swfs/definescalinggrid.swf", TagCode::DefineScalingGrid)
    ),

    (
        1, // Minimum version not listed in SWF19.
        Tag::DefineSceneAndFrameLabelData {
            scenes: vec![
                FrameLabel { frame_num: 0, label: "Scene 1".to_string() },
                FrameLabel {
                    frame_num: 25,
                    label: "Scene2Scene2Scene2Scene2Scene2".to_string()
                },
                FrameLabel { frame_num: 26, label: "test日本語test".to_string() },
            ],
            frame_labels: vec![
                FrameLabel { frame_num: 0, label: "a".to_string() },
                FrameLabel { frame_num: 9, label: "b".to_string() },
                FrameLabel { frame_num: 17, label: "❤😁aaa".to_string() },
                FrameLabel { frame_num: 25, label: "frameInScene2".to_string() },
            ],
        },
        read_tag_bytes_from_file(
            "tests/swfs/define_scene_and_frame_label_data.swf",
            TagCode::DefineSceneAndFrameLabelData
        )
    ),

    (
        1,
        Tag::DefineShape(Shape {
            version: 1,
            id: 1,
            shape_bounds: Rectangle { x_min: 0f32, x_max: 20f32, y_min: 0f32, y_max: 20f32 },
            edge_bounds: Rectangle { x_min: 0f32, x_max: 20f32, y_min: 0f32, y_max: 20f32 },
            has_fill_winding_rule: false,
            has_non_scaling_strokes: true,
            has_scaling_strokes: false,
            styles: ShapeStyles {
                fill_styles: vec![
                    FillStyle::Color(Color { r: 255, g: 0, b: 0, a: 255 })
                ],
                line_styles: vec![],
            },
            shape: vec![
                ShapeRecord::StyleChange(StyleChangeData {
                    move_to: None,
                    fill_style_0: None,
                    fill_style_1: Some(1),
                    line_style: None,
                    new_styles: None,
                }),
                ShapeRecord::StraightEdge {
                    delta_x: 20f32,
                    delta_y: 0f32,
                },
                ShapeRecord::StraightEdge {
                    delta_x: 0f32,
                    delta_y: 20f32,
                },
                ShapeRecord::StraightEdge {
                    delta_x: -20f32,
                    delta_y: 0f32,
                },
                ShapeRecord::StraightEdge {
                    delta_x: 0f32,
                    delta_y: -20f32,
                },
            ]
        }),
        read_tag_bytes_from_file("tests/swfs/define_shape.swf", TagCode::DefineShape)
    ),

    (
        8,
        Tag::DefineShape(Shape {
            version: 3,
            id: 1,
            shape_bounds: Rectangle { x_min: 0f32, x_max: 50f32, y_min: 0f32, y_max: 50f32 },
            edge_bounds: Rectangle { x_min: 0f32, x_max: 50f32, y_min: 0f32, y_max: 50f32 },
            has_fill_winding_rule: false,
            has_non_scaling_strokes: true,
            has_scaling_strokes: false,
            styles: ShapeStyles {
                fill_styles: vec![
                    FillStyle::RadialGradient(Gradient {
                        matrix: Matrix { translate_x: 24.95f32, translate_y: 24.95f32, scale_x: 0.030731201f32, scale_y: 0.030731201f32, rotate_skew_0: 0f32, rotate_skew_1: 0f32 },
                        spread: GradientSpread::Pad,
                        interpolation: GradientInterpolation::RGB,
                        records: vec![
                            GradientRecord { ratio: 0, color: Color { r: 255, g: 0, b: 0, a: 255 } },
                            GradientRecord { ratio: 255, color: Color { r: 0, g: 0, b: 0, a: 0 } }
                        ]
                    })
                ],
                line_styles: vec![]
            },
            shape: vec![
                ShapeRecord::StyleChange(StyleChangeData {
                    move_to: Some((50f32, 25f32)),
                    fill_style_0: None,
                    fill_style_1: Some(1),
                    line_style: None,
                    new_styles: None
                }),
                ShapeRecord::CurvedEdge { control_delta_x: 0f32, control_delta_y: 10.35f32, anchor_delta_x: -7.35f32, anchor_delta_y: 7.3f32 },
                ShapeRecord::CurvedEdge { control_delta_x: -7.3f32, control_delta_y: 7.35f32, anchor_delta_x: -10.35f32, anchor_delta_y: 0f32 },
                ShapeRecord::CurvedEdge { control_delta_x: -10.35f32, control_delta_y: 0f32, anchor_delta_x: -7.35f32, anchor_delta_y: -7.35f32 },
                ShapeRecord::CurvedEdge { control_delta_x: -7.3f32, control_delta_y: -7.3f32, anchor_delta_x: 0f32, anchor_delta_y: -10.35f32 },
                ShapeRecord::CurvedEdge { control_delta_x: 0f32, control_delta_y: -10.35f32, anchor_delta_x: 7.3f32, anchor_delta_y: -7.35f32 },
                ShapeRecord::CurvedEdge { control_delta_x: 7.35f32, control_delta_y: -7.3f32, anchor_delta_x: 10.35f32, anchor_delta_y: 0f32 },
                ShapeRecord::CurvedEdge { control_delta_x: 10.35f32, control_delta_y: 0f32, anchor_delta_x: 7.3f32, anchor_delta_y: 7.3f32 },
                ShapeRecord::CurvedEdge { control_delta_x: 7.35f32, control_delta_y: 7.35f32, anchor_delta_x: 0f32, anchor_delta_y: 10.35f32 }
            ]
        }),
        read_tag_bytes_from_file("tests/swfs/defineshape3.swf", TagCode::DefineShape3)
    ),

    (
        8,
        Tag::DefineShape(Shape {
            version: 4,
            id: 1,
            shape_bounds: Rectangle { x_min: -10f32, x_max: 260f32, y_min: -10f32, y_max: 110f32 },
            edge_bounds: Rectangle { x_min: 0f32, x_max: 250f32, y_min: 0f32, y_max: 100f32 },
            has_fill_winding_rule: false,
            has_non_scaling_strokes: true,
            has_scaling_strokes: false,
            styles: ShapeStyles {
                fill_styles: vec![
                    FillStyle::Color(Color { r: 255, g: 0, b: 0, a: 255 }),
                    FillStyle::FocalGradient {
                        gradient: Gradient {
                            matrix: Matrix { translate_x: 49.55f32, translate_y: 46.55f32, scale_x: 0.06199646f32, scale_y: 0.06199646f32, rotate_skew_0: 0f32, rotate_skew_1: 0f32 },
                            spread: GradientSpread::Pad,
                            interpolation: GradientInterpolation::LinearRGB,
                            records: vec![
                                GradientRecord { ratio: 0, color: Color { r: 255, g: 0, b: 0, a: 255 } },
                                GradientRecord { ratio: 255, color: Color { r: 0, g: 0, b: 0, a: 0 } }
                            ]
                        },
                        focal_point: 0.56640625f32
                    }
                ],
                line_styles: vec![
                    LineStyle { 
                        width: 400,
                        color: Color { r: 0, g: 153, b: 0, a: 255 },
                        start_cap: LineCapStyle::None,
                        end_cap: LineCapStyle::None,
                        join_style: LineJoinStyle::Bevel,
                        fill_style: None,
                        allow_scale_x: false,
                        allow_scale_y: false,
                        is_pixel_hinted: true,
                        allow_close: true
                    },
                    LineStyle {
                        width: 400,
                        color: Color { r: 0, g: 0, b: 0, a: 0 },
                        start_cap: LineCapStyle::Round,
                        end_cap: LineCapStyle::Round,
                        join_style: LineJoinStyle::Round,
                        fill_style: Some(
                            FillStyle::LinearGradient(Gradient {
                                matrix: Matrix { translate_x: 50f32, translate_y: 50f32, scale_x: 0.07324219f32, scale_y: 0.07324219f32, rotate_skew_0: 0f32, rotate_skew_1: 0f32 },
                                spread: GradientSpread::Pad,
                                interpolation: GradientInterpolation::RGB,
                                records: vec![
                                    GradientRecord { ratio: 0, color: Color { r: 255, g: 255, b: 255, a: 255 } },
                                    GradientRecord { ratio: 255, color: Color { r: 0, g: 0, b: 0, a: 255 } }
                                ]
                            })),
                            allow_scale_x: true,
                            allow_scale_y: false,
                            is_pixel_hinted: true,
                            allow_close: true
                        },
                        LineStyle {
                            width: 400,
                            color: Color { r: 0, g: 153, b: 0, a: 255 },
                            start_cap: LineCapStyle::Round,
                            end_cap: LineCapStyle::Round,
                            join_style: LineJoinStyle::Miter(56f32),
                            fill_style: None,
                            allow_scale_x: true,
                            allow_scale_y: false,
                            is_pixel_hinted: true,
                            allow_close: true
                        }
                    ]
            },
            shape: vec![
                ShapeRecord::StyleChange(StyleChangeData {
                    move_to: Some((150f32, 0f32)),
                    fill_style_0: None,
                    fill_style_1: Some(1),
                    line_style: Some(1),
                    new_styles: None
                }),
                ShapeRecord::StraightEdge { delta_x: 100f32, delta_y: 0f32 },
                ShapeRecord::StraightEdge { delta_x: 0f32, delta_y: 100f32 },
                ShapeRecord::StyleChange(StyleChangeData {
                    move_to: None,
                    fill_style_0: None,
                    fill_style_1: None,
                    line_style: Some(3),
                    new_styles: None
                }),
                ShapeRecord::StraightEdge { delta_x: -100f32, delta_y: 0f32 },
                ShapeRecord::StraightEdge { delta_x: 0f32, delta_y: -100f32 },
                ShapeRecord::StyleChange(StyleChangeData {
                    move_to: Some((100f32, 50f32)),
                    fill_style_0: None,
                    fill_style_1: Some(2),
                    line_style: Some(2),
                    new_styles: None
                }),
                ShapeRecord::CurvedEdge { control_delta_x: 0f32, control_delta_y: 20.65f32, anchor_delta_x: -14.65f32, anchor_delta_y: 14.6f32 },
                ShapeRecord::CurvedEdge { control_delta_x: -14.7f32, control_delta_y: 14.75f32, anchor_delta_x: -20.65f32, anchor_delta_y: 0f32 },
                ShapeRecord::CurvedEdge { control_delta_x: -20.7f32, control_delta_y: 0f32, anchor_delta_x: -14.65f32, anchor_delta_y: -14.75f32 },
                ShapeRecord::CurvedEdge { control_delta_x: -14.65f32, control_delta_y: -14.6f32, anchor_delta_x: 0f32, anchor_delta_y: -20.65f32 },
                ShapeRecord::CurvedEdge { control_delta_x: 0f32, control_delta_y: -20.7f32, anchor_delta_x: 14.65f32, anchor_delta_y: -14.7f32 },
                ShapeRecord::CurvedEdge { control_delta_x: 14.65f32, control_delta_y: -14.6f32, anchor_delta_x: 20.7f32, anchor_delta_y: 0f32 },
                ShapeRecord::CurvedEdge { control_delta_x: 20.65f32, control_delta_y: 0f32, anchor_delta_x: 14.7f32, anchor_delta_y: 14.6f32 },
                ShapeRecord::CurvedEdge { control_delta_x: 14.65f32, control_delta_y: 14.7f32, anchor_delta_x: 0f32, anchor_delta_y: 20.7f32 }
            ]
        }),
        read_tag_bytes_from_file("tests/swfs/defineshape4.swf", TagCode::DefineShape4)
    ),

    (
        4,
        Tag::DefineSound(Box::new(Sound {
            id: 1,
            format: SoundFormat {
                compression: AudioCompression::Uncompressed,
                sample_rate: 44100,
                is_16_bit: true,
                is_stereo: false,
            },
            num_samples: 10,
            data: vec![255, 127, 0, 128, 255, 127, 0, 128, 255, 127, 0, 128, 255, 127, 0, 128, 255, 127, 0, 128], 
        })),
        read_tag_bytes_from_file("tests/swfs/definesound.swf", TagCode::DefineSound)
    ),

    (
        3,
        Tag::DefineSprite(Sprite {
            id: 1,
            num_frames: 5,
            tags: vec![
                Tag::ShowFrame,
                Tag::ShowFrame,
                Tag::ShowFrame,
                Tag::ShowFrame,
                Tag::ShowFrame,
            ],
        }),
        read_tag_bytes_from_file("tests/swfs/define_sprite.swf", TagCode::DefineSprite)
    ),

    (
        1,
        Tag::DefineText(Box::new(Text {
            id: 2,
            bounds: Rectangle { x_min: 1.2, x_max: 38.65, y_min: 4.1, y_max: 18.45 },
            matrix: Matrix::new(),
            records: vec![
                TextRecord {
                    font_id: Some(1),
                    color: Some(Color { r: 0, g: 0, b: 0, a: 255 }),
                    x_offset: None,
                    y_offset: Some(16.1),
                    height: Some(320),
                    glyphs: vec![
                        GlyphEntry { index: 0, advance: 145 },
                        GlyphEntry { index: 1, advance: 203 },
                        GlyphEntry { index: 0, advance: 145 },
                    ],
                },
            ],
        })),
        read_tag_bytes_from_file("tests/swfs/DefineFont-MX.swf", TagCode::DefineText)
    ),

    (
        5,
        Tag::DoAction(
            vec![
                Action::Push(vec![Value::Str("Testing!".to_string())]),
                Action::Trace,
            ]
        ),
        read_tag_bytes_from_file("tests/swfs/doaction.swf", TagCode::DoAction)
    ),

    (
        6,
        Tag::DoInitAction {
            id: 2,
            action_data: vec![150, 6, 0, 0, 116, 101, 115, 116, 0, 38, 0],
        },
        read_tag_bytes_from_file("tests/swfs/doinitaction.swf", TagCode::DoInitAction)
    ),

    (
        6,
        Tag::EnableDebugger("$1$ve$EG3LE6bumvJ2pR8F5qXny/".to_string()),
        read_tag_bytes_from_file("tests/swfs/enabledebugger2.swf", TagCode::EnableDebugger2)
    ),

    (
        10,
        Tag::EnableTelemetry {
            password_hash: vec![]
        },
        read_tag_bytes_from_file("tests/swfs/enabletelemetry.swf", TagCode::EnableTelemetry)
    ),

    (
        10,
        Tag::EnableTelemetry {
            password_hash: vec![
                207, 128, 205, 138, 237, 72, 45, 93, 21, 39, 215, 220, 114, 252, 239, 248,
                78, 99, 38, 89, 40, 72, 68, 125, 45, 192, 176, 232, 125, 252, 154, 144
            ]
        },
        read_tag_bytes_from_file("tests/swfs/enabletelemetry-password.swf", TagCode::EnableTelemetry)
    ),

    (
        6,
        Tag::ExportAssets(vec![
            ExportedAsset { id: 2, name: "Test💯".to_string() },
        ]),
        read_tag_bytes_from_file("tests/swfs/exportassets.swf", TagCode::ExportAssets)
    ),

    (
        8,
        Tag::FileAttributes(FileAttributes {
            use_direct_blit: false,
            use_gpu: true,
            has_metadata: false,
            is_action_script_3: true,
            use_network_sandbox: false,
        }),
        vec![0b01_000100, 0b00010001, 0b00101000, 0, 0, 0]
    ),
        
    (
        3,
        Tag::FrameLabel { label: "test".to_string(), is_anchor: false },
        read_tag_bytes_from_file_with_index("tests/swfs/framelabel.swf", TagCode::FrameLabel, 0)
    ),

    (
        6, // Anchor tags supported in SWF version 6 and later.
        Tag::FrameLabel { label: "anchor_tag".to_string(), is_anchor: true },
        read_tag_bytes_from_file_with_index("tests/swfs/framelabel.swf", TagCode::FrameLabel, 1)
    ),

    (
        7,
        Tag::ImportAssets {
            url: "exportassets.swf".to_string(),
            imports: vec![ExportedAsset { id: 1, name: "Test💯".to_string() }],
        },
        read_tag_bytes_from_file("tests/swfs/importassets.swf", TagCode::ImportAssets)
    ),

    (
        8,
        Tag::ImportAssets {
            url: "exportassets.swf".to_string(),
            imports: vec![ExportedAsset { id: 1, name: "Test💯".to_string() }],
        },
        read_tag_bytes_from_file("tests/swfs/importassets2.swf", TagCode::ImportAssets2)
    ),

    (
        1,
        Tag::JpegTables(vec![
            255, 216, 255, 219, 0, 67, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 255, 219, 0, 67, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            255, 196, 0, 31, 0, 0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5,
            6, 7, 8, 9, 10, 11, 255, 196, 0, 181, 16, 0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1,
            125, 1, 2, 3, 0, 4, 17, 5, 18, 33, 49, 65, 6, 19, 81, 97, 7, 34, 113, 20, 50, 129, 145,
            161, 8, 35, 66, 177, 193, 21, 82, 209, 240, 36, 51, 98, 114, 130, 9, 10, 22, 23, 24,
            25, 26, 37, 38, 39, 40, 41, 42, 52, 53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71, 72, 73,
            74, 83, 84, 85, 86, 87, 88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106, 115, 116,
            117, 118, 119, 120, 121, 122, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147, 148,
            149, 150, 151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170, 178, 179,
            180, 181, 182, 183, 184, 185, 186, 194, 195, 196, 197, 198, 199, 200, 201, 202, 210,
            211, 212, 213, 214, 215, 216, 217, 218, 225, 226, 227, 228, 229, 230, 231, 232, 233,
            234, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 255, 196, 0, 31, 1, 0, 3, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255, 196, 0,
            181, 17, 0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 119, 0, 1, 2, 3, 17, 4, 5, 33,
            49, 6, 18, 65, 81, 7, 97, 113, 19, 34, 50, 129, 8, 20, 66, 145, 161, 177, 193, 9, 35,
            51, 82, 240, 21, 98, 114, 209, 10, 22, 36, 52, 225, 37, 241, 23, 24, 25, 26, 38, 39,
            40, 41, 42, 53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71, 72, 73, 74, 83, 84, 85, 86, 87,
            88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106, 115, 116, 117, 118, 119, 120, 121,
            122, 130, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147, 148, 149, 150, 151, 152,
            153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170, 178, 179, 180, 181, 182, 183,
            184, 185, 186, 194, 195, 196, 197, 198, 199, 200, 201, 202, 210, 211, 212, 213, 214,
            215, 216, 217, 218, 226, 227, 228, 229, 230, 231, 232, 233, 234, 242, 243, 244, 245,
            246, 247, 248, 249, 250, 255, 217
        ]),
        read_tag_bytes_from_file("tests/swfs/DefineBits-JpegTables-MX.swf", TagCode::JpegTables)
    ),

    (
        1,
        Tag::Metadata("aa!".to_string()),
        vec![0b01_000100, 0b000_10011, 'a' as u8, 'a' as u8, '!' as u8, 0]
    ),

    (
        4,
        Tag::PlaceObject(Box::new(PlaceObject {
            version: 2,
            action: PlaceObjectAction::Place(1),
            depth: 1,
            matrix: Some(Matrix::new()),
            color_transform: None,
            ratio: None,
            name: None,
            clip_depth: None,
            class_name: None,
            filters: vec![],
            background_color: None,
            blend_mode: BlendMode::Normal,
            clip_actions: vec![],
            is_image: false,
            is_bitmap_cached: false,
            is_visible: true,
        })),
        read_tag_bytes_from_file("tests/swfs/define_shape.swf", TagCode::PlaceObject2)
    ),

    (
        6, // ClipActions added in SWF version 5-6.
        Tag::PlaceObject(Box::new(PlaceObject {
            version: 2,
            action: PlaceObjectAction::Place(2),
            depth: 1,
            matrix: Some(Matrix::new()),
            color_transform: None,
            ratio: None,
            name: None,
            clip_depth: None,
            class_name: None,
            filters: vec![],
            background_color: None,
            blend_mode: BlendMode::Normal,
            clip_actions: vec![
                ClipAction {
                    events: vec![ClipEvent::Press, ClipEvent::Release].into_iter().collect(),
                    key_code: None,
                    action_data: vec![150, 3, 0, 0, 65, 0, 38, 0],
                },
                ClipAction {
                    events: vec![ClipEvent::KeyPress].into_iter().collect(),
                    key_code: Some(99),
                    action_data: vec![150, 3, 0, 0, 66, 0, 38, 0],
                },
                    ClipAction {
                    events: vec![ClipEvent::EnterFrame].into_iter().collect(),
                    key_code: None,
                    action_data: vec![150, 3, 0, 0, 67, 0, 38, 0],
                },
            ],
            is_image: false,
            is_bitmap_cached: false,
            is_visible: true,
        })),
        read_tag_bytes_from_file("tests/swfs/placeobject2-clipactions.swf", TagCode::PlaceObject2)
    ),

    (
        8,
        Tag::PlaceObject(Box::new(PlaceObject {
            version: 3,
            action: PlaceObjectAction::Place(2),
            depth: 1,
            matrix: Some(Matrix {
                translate_x: 10f32,
                translate_y: 10f32,
                rotate_skew_0: 0f32,
                rotate_skew_1: 0f32,
                scale_x: 2.0f32,
                scale_y: 2.0f32,
            }),
            color_transform: Some(ColorTransform {
                a_multiply: 1.0f32,
                a_add: 80,
                r_multiply: 0.5f32,
                r_add: 60,
                g_multiply: 0.25f32,
                g_add: 40,
                b_multiply: 0.75f32,
                b_add: 20,
            }),
            ratio: None,
            name: Some("test".to_string()),
            clip_depth: None,
            class_name: None,
            filters: vec![
                Filter::GradientBevelFilter(Box::new(GradientBevelFilter {
                    colors: vec![
                        GradientRecord { ratio: 0, color: Color { r: 255, g: 0, b: 0, a: 255 } },
                        GradientRecord { ratio: 128, color: Color { r: 0, g: 255, b: 0, a: 0 } },
                        GradientRecord { ratio: 255, color: Color { r: 0, g: 0, b: 255, a: 0 } }
                    ],
                    blur_x: 5f64,
                    blur_y: 5f64,
                    angle: 0.7853851318359375f64,
                    distance: 5f64,
                    strength: 1f32,
                    is_inner: true,
                    is_knockout: true,
                    is_on_top: false,
                    num_passes: 3,
                })),
                Filter::GradientGlowFilter(Box::new(GradientGlowFilter {
                    colors: vec![
                        GradientRecord { ratio: 0, color: Color { r: 255, g: 255, b: 255, a: 0 } },
                        GradientRecord { ratio: 255, color: Color { r: 0, g: 0, b: 0, a: 255 } },
                    ],
                    blur_x: 30f64,
                    blur_y: 30f64,
                    angle: 0.174530029296875f64,
                    distance: 5f64,
                    strength: 0.19921875f32,
                    is_inner: false,
                    is_knockout: false,
                    is_on_top: true,
                    num_passes: 1,
                })),
                Filter::BlurFilter(Box::new(BlurFilter {
                    blur_x: 30f64,
                    blur_y: 20f64,
                    num_passes: 2,
                }))
            ],
            background_color: Some(Color { r: 255, g: 0, b: 0, a: 255 }),
            blend_mode: BlendMode::Difference,
            clip_actions: vec![
                ClipAction {
                    events: vec![ClipEvent::ReleaseOutside, ClipEvent::RollOver].into_iter().collect(),
                    key_code: None,
                    action_data: vec![0],
                },
                ClipAction {
                    events: vec![ClipEvent::Data].into_iter().collect(),
                    key_code: None,
                    action_data: vec![150, 3, 0, 0, 66, 0, 38, 0],
                },
            ],
            is_image: false,
            is_bitmap_cached: true,
            is_visible: false,
        })),
        read_tag_bytes_from_file("tests/swfs/placeobject3-theworks.swf", TagCode::PlaceObject3)
    ),

    (
        5, // Password supported in SWF version 5 or later.
        Tag::Protect(Some("$1$d/$yMscKH17OJ0paJT.e67iz0".to_string())),
        read_tag_bytes_from_file(
            "tests/swfs/protect.swf",
            TagCode::Protect
        )
    ),

    (
        1,
        Tag::SetBackgroundColor(Color { r: 64, g: 150, b: 255, a: 255 }),
        vec![0b01_000011, 0b00000010, 64, 150, 255]
    ),

    (
        7,
        Tag::SetTabIndex { depth: 2, tab_index: 1 },
        vec![0b10_000100, 0b000_10000, 2, 0, 1, 0],
    ),

    (
        7,
        Tag::ScriptLimits { max_recursion_depth: 256, timeout_in_seconds: 42 },
        read_tag_bytes_from_file("tests/swfs/scriptlimits.swf", TagCode::ScriptLimits)
    ),

    (1, Tag::ShowFrame, vec![0b01_000000, 0]),

    (
        3,
        Tag::SoundStreamHead2(Box::new(SoundStreamInfo {
            stream_format: SoundFormat {
                compression: AudioCompression::Uncompressed,
                sample_rate: 5512,
                is_16_bit: true,
                is_stereo: false,
            },
            playback_format: SoundFormat {
                compression: AudioCompression::UncompressedUnknownEndian,
                sample_rate: 5512,
                is_16_bit: true,
                is_stereo: false,
            },
            num_samples_per_block: 229,
            latency_seek: 0,
        })),
        read_tag_bytes_from_file("tests/swfs/soundstreamhead2.swf", TagCode::SoundStreamHead2)
    ),

    (
        9,
        Tag::SymbolClass(vec![
            SymbolClassLink { id: 2, class_name: "foo.Test".to_string() },
            SymbolClassLink { id: 0, class_name: "DocumentTest".to_string() },
        ]),
        read_tag_bytes_from_file("tests/swfs/symbolclass.swf", TagCode::SymbolClass)
    ),

    (
        4,
        Tag::StartSound {
            id: 1,
            sound_info: Box::new(SoundInfo {
                event: SoundEvent::Start,
                in_sample: None,
                out_sample: None,
                num_loops: 3,
                envelope: None,
            }),
        },
        read_tag_bytes_from_file("tests/swfs/definesound.swf", TagCode::StartSound)
    ),

    (
        9,
        Tag::StartSound2 {
            class_name: "TestSound".to_string(),
            sound_info: Box::new(SoundInfo {
                event: SoundEvent::Event,
                in_sample: None,
                out_sample: None,
                num_loops: 1,
                envelope: Some(vec![
                    SoundEnvelopePoint {
                        sample: 0,
                        left_volume: 0.0,
                        right_volume: 1.0,
                    }
                ]),
            }),
        },
        read_tag_bytes_from_file("tests/swfs/startsound2.swf", TagCode::StartSound2)
    ),

    (1, Tag::Unknown { tag_code: 512, data: vec![] }, vec![0b00_000000, 0b10000000]),
    (1, Tag::Unknown { tag_code: 513, data: vec![1, 2] },  vec![0b01_000010, 0b10000000, 1, 2]),
    (
        1,
        Tag::Unknown { tag_code: 513, data: vec![0; 64] }, 
        vec![0b01_111111, 0b10000000, 64, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    ),
] }

pub fn avm1_tests() -> Vec<Avm1TestData> { vec![
    (4, Action::Add, vec![0x0A]),
    (4, Action::AsciiToChar, vec![0x33]),
    (4, Action::Call, vec![0x9E, 0, 0]),
    (4, Action::CharToAscii, vec![0x32]),
    (4, Action::Divide, vec![0x0D]),
    (4, Action::Equals, vec![0x0E]),
    (4, Action::GetTime, vec![0x34]),
    (
        3,
        Action::GetUrl { url: String::from("a"), target: String::from("b") },
        vec![0x83, 4, 0, 97, 0, 98, 0]
    ),
    (
        4,
        Action::GetUrl2 { send_vars_method: SendVarsMethod::Post, is_target_sprite: true, is_load_vars: false },
        vec![0x9A, 1, 0, 0b10_0000_10]
    ),
    (4, Action::GetVariable, vec![0x1C]),
    (3, Action::GotoFrame(11), vec![0x81, 2, 0, 11, 0]),
    (4, Action::GotoFrame2 { set_playing: false, scene_offset: 0 }, vec![0x9F, 1, 0, 0]),
    (4, Action::GotoFrame2 { set_playing: true, scene_offset: 259 }, vec![0x9F, 3, 0, 0b11, 3, 1]),
    (3, Action::GotoLabel("testb".to_string()), vec![0x8C, 6, 0, 116, 101, 115, 116, 98, 0]),
    (4, Action::If { offset: 1 }, vec![0x9D, 2, 0, 1, 0]),
    (4, Action::Jump { offset: 1 }, vec![0x99, 2, 0, 1, 0]),
    (4, Action::Less, vec![0x0F]),
    (4, Action::MBAsciiToChar, vec![0x37]),
    (4, Action::MBCharToAscii, vec![0x36]),
    (4, Action::MBStringExtract, vec![0x35]),
    (4, Action::MBStringLength, vec![0x31]),
    (4, Action::Multiply, vec![0x0C]),
    (3, Action::NextFrame, vec![0x04]),
    (4, Action::And, vec![0x10]),
    (4, Action::Not, vec![0x12]),
    (4, Action::Or, vec![0x11]),
    (3, Action::Play, vec![0x06]),
    (4, Action::Pop, vec![0x17]),
    (3, Action::PreviousFrame, vec![0x05]),
    (4, Action::Push(vec![Value::Str("test".to_string())]), vec![0x96, 6, 0, 0, 116, 101, 115, 116, 0]),
    (4, Action::Push(vec![Value::Float(0.0)]), vec![0x96, 5, 0, 1, 0, 0, 0, 0]),
    (5, Action::Push(vec![Value::Double(1.5)]), vec![0x96, 9, 0, 6, 0, 0, 248, 63, 0, 0, 0, 0]),
    (5, Action::Push(vec![Value::Null]), vec![0x96, 1, 0, 2]),
    (5, Action::Push(vec![Value::Undefined]), vec![0x96, 1, 0, 3]),
    (5, Action::Push(vec![Value::Null, Value::Undefined]), vec![0x96, 2, 0, 2, 3]),
    (5, Action::Push(vec![Value::Register(1)]), vec![0x96, 2, 0, 4, 1]),
    (5, Action::Push(vec![Value::Bool(false)]), vec![0x96, 2, 0, 5, 0]),
    (5, Action::Push(vec![Value::Bool(true)]), vec![0x96, 2, 0, 5, 1]),
    (5, Action::Push(vec![Value::Double(0.0)]), vec![0x96, 9, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0]),
    (5, Action::Push(vec![Value::Int(31)]), vec![0x96, 5, 0, 7, 31, 0, 0, 0]),
    (5, Action::Push(vec![Value::ConstantPool(77)]), vec![0x96, 2, 0, 8, 77]),
    (5, Action::Push(vec![Value::ConstantPool(257)]), vec![0x96, 3, 0, 9, 1, 1]),
    (4, Action::RandomNumber, vec![0x30]),
    (3, Action::SetTarget("test".to_string()), vec![0x8B, 5, 0, 116, 101, 115, 116, 0]),
    (4, Action::SetVariable, vec![0x1D]),
    (3, Action::Stop, vec![0x07]),
    (3, Action::StopSounds, vec![0x09]),
    (4, Action::StringAdd, vec![0x21]),
    (4, Action::StringEquals, vec![0x13]),
    (4, Action::StringExtract, vec![0x15]),
    (4, Action::StringLength, vec![0x14]),
    (4, Action::StringLess, vec![0x29]),
    (4, Action::Subtract, vec![0x0B]),
    (3, Action::ToggleQuality, vec![0x08]),
    (4, Action::ToInteger, vec![0x18]),
    (4, Action::Trace, vec![0x26]),
    (3, Action::WaitForFrame { frame: 4, num_actions_to_skip: 10 }, vec![0x8A, 3, 0, 4, 0, 10]),
    (4, Action::WaitForFrame2 { num_actions_to_skip: 34 }, vec![0x8D, 1, 0, 34]),
    (1, Action::Unknown { opcode: 0x79, data: vec![] }, vec![0x79]),
    (1, Action::Unknown { opcode: 0xA0, data: vec![2, 3] }, vec![0xA0, 2, 0, 2, 3]),
] }