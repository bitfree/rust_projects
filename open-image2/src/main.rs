use image::GenericImageView;  // 이미지 관련 기능을 가져옴
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() -> Result<(), Error> {
    // 이벤트 루프 및 윈도우 생성
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(800.0, 600.0);
        WindowBuilder::new()
            .with_title("Rust Image Viewer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // 이미지 파일 읽기
    let img = image::open("example.png").expect("이미지를 열 수 없습니다.");
    let (img_width, img_height) = img.dimensions();

    // 창 크기를 이미지 크기로 조정
    window.set_inner_size(LogicalSize::new(img_width as f64, img_height as f64));

    // 픽셀 버퍼 생성
    let surface_texture = SurfaceTexture::new(img_width, img_height, &window);
    let mut pixels = Pixels::new(img_width, img_height, surface_texture)?;

    // 이미지 데이터를 픽셀 버퍼에 복사
    let img_data = img.to_rgba8();
    pixels.frame_mut().copy_from_slice(&img_data);

    // 이벤트 루프 시작
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                // 화면 업데이트
                if let Err(e) = pixels.render() {
                    eprintln!("렌더링 에러: {}", e);
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}
