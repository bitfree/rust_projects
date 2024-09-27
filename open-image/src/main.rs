use image::{GenericImageView, DynamicImage};
use minifb::{Window, WindowOptions, Key, Scale, ScaleMode};

fn main() {
    let img_path = "road.webp"; // 표시할 이미지 파일 경로

    // 이미지 파일을 읽어옵니다.
    let img = image::open(img_path).expect("Failed to open image");

    // 이미지 크기를 가져옵니다.
    let (width, height) = img.dimensions();
    let mut width = width as usize;
    let mut height = height as usize;

    // 이미지를 RGB값으로 변환합니다.
    let rgb_image = img.to_rgb8();
    let buffer: Vec<u32> = rgb_image.pixels()
        .map(|p| {
            let r = p[0] as u32;
            let g = p[1] as u32;
            let b = p[2] as u32;
            (r << 16) | (g << 8) | b
        })
        .collect();

    // 창 생성
    let mut window = Window::new(
        "Image Viewer - ESC to exit",
        width,
        height,
        WindowOptions {
            resize: true,        // 창 크기 조절 가능
            scale: Scale::X1,    // 1배 확대
            scale_mode: ScaleMode::Stretch, // 종횡비 유지
            borderless: false,   // 테두리 유지
            title: true,         // 타이틀 바 표시
            topmost: false,      // 항상 최상단 창 아님
            transparency: false, // 투명하지 않음
            none: false,         // none을 false 로 설정
        },
    ).expect("Failed to create window");

    // 창에 이미지 그리기
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // 창 크기가 변경되었는지 확인
        // if let Some((new_width, new_height)) = window.get_size() {
        let  (new_width, new_height) = window.get_size();
            // 크기가 변경되었으면 버퍼 크기를 새 창 크기에 맞게 업데이트
            if new_width != width || new_height != height {
            // 새 크기에 맞는 버퍼를 생성
            let new_buffer: Vec<u32> = vec![0; width * height];
            window.update_with_buffer(&new_buffer, width, height).expect("Failed to update window");
            } else {
                // 크기를 가져올 수 없으면 기존 버퍼를 사용
                window.update_with_buffer(&buffer, width, height).expect("Failed to update window");
            }
        
        
        // 이미지 데이터를 창에 업데이트
        window
            .update_with_buffer(&buffer, width, height)
            .expect("Failed to update window");
    }
}