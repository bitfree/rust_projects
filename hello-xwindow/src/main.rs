use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    // 새로운 GTK 애플리케이션 생성 (앱 ID는 고유해야 합니다)
    let app = Application::builder()
        .application_id("com.example.HelloWorld")
        .build();

    // 애플리케이션이 시작될 때 호출되는 콜백 함수
    app.connect_activate(|app| {
        // 새 윈도우 생성
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hello, GNOME!")
            .default_width(300)
            .default_height(100)
            .build();

        // 새 버튼 생성
        let button = Button::builder()
            .label("Hello, World!")
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        // 버튼이 클릭되었을 때 수행할 동작
        button.connect_clicked(|_| {
            println!("Hello, World!");
        });

        // 버튼을 윈도우에 추가
        window.set_child(Some(&button));

        // 윈도우를 화면에 표시
        // window.show();
        window.present();
    });

    // 애플리케이션 실행
    app.run();
}
