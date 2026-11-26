# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Người lớn tuổi Việt Nam (bố mẹ, ông bà) tự mở trang trên điện thoại của mình, nhập các ngày giỗ trong nhà theo âm lịch và đưa chúng vào app lịch sẵn có của máy để được nhắc trước. Họ không rành công nghệ, đọc chữ nhỏ khó, và chỉ dùng trang vài lần (lúc thêm giỗ mới), không dùng hằng ngày.

## Product Purpose

Tên sản phẩm: **Nhắc Giỗ**. Đổi ngày giỗ, mùng 1 và rằm từ âm lịch sang dương lịch cho nhiều năm tới, rồi đưa thẳng vào app Lịch của iPhone hoặc Google Calendar trên Android, kèm giờ nhắc. Thành công là: người dùng thêm được giỗ vào lịch của chính họ trong một lần, và điện thoại tự nhắc đúng ngày âm mỗi năm mà không cần mở lại trang.

## Positioning

Không cài app, không tài khoản, không gửi dữ liệu đi đâu: lịch tính ngay trên máy (WebAssembly, thuật toán Hồ Ngọc Đức theo giờ Việt Nam UTC+7) và đi vào app lịch người dùng đã quen. Xử lý đúng các trường hợp riêng của giỗ Việt: tháng nhuận (giỗ cúng tháng thường), tháng thiếu (giỗ ngày 30 dời về 29).

## Operating Context

- Công khai, miễn phí, cho mọi người Việt; chia sẻ bằng link `damgio.pages.dev` (bản sao trên GitHub Pages).
- iPhone: mở bằng Safari, bấm tải thì hiện màn hình "Thêm tất cả" của iOS; sự kiện vào lịch mặc định của máy.
- Android: Chrome tải file `lich-am.ics`, bấm Mở thì Google Calendar nhận file.
- Máy tính: nhập file vào Google Calendar web là tuỳ chọn, Google bỏ giờ nhắc trong file.
- Link thường được gửi qua Zalo/Messenger; trình duyệt trong app có thể không mở được màn hình lịch của iOS.

## Capabilities and Constraints

- Nhập nhiều ngày giỗ (tên, ngày âm 1–30, tháng âm 1–12); tích mùng 1, rằm; chọn 1/3/5/10 năm.
- Lời nhắc: giỗ báo 9 giờ sáng 3 ngày trước và 18 giờ hôm trước; mùng 1, rằm báo 18 giờ hôm trước.
- Danh sách giỗ lưu trong trình duyệt (localStorage), không gửi lên server.
- Trang tĩnh: `index.html` + `pkg/` (wasm do Rust build, `rust/lich-am`). Hành vi, id phần tử và lời gọi wasm trong `index.html` là hợp đồng với code, không đổi khi làm giao diện.
- Không có backend, không đăng nhập, không theo dõi.

## Brand Commitments

- Tên hiển thị: **Nhắc Giỗ**.
- Phong cách tối giản, sạch: nền trắng (hoặc đen khi chế độ tối), chữ đen, một màu trung tính cho nút chính, không khối màu lớn, không viền đậm, không trang trí. Người dùng đã thử và từ chối hướng "tờ lịch bloc" đỏ đen vì màu mè (26/11/2026).

## Evidence on Hand

- Đã kiểm chứng: khớp 4.018/4.018 ngày với một lịch âm Việt Nam độc lập (2022–2032); chạy thật trên iPhone (màn hình "Thêm tất cả", giờ nhắc giữ khi vào lịch Gmail) và Android 15 (Google Calendar nhận file).
- Không có người dùng, lời chứng thực, số liệu sử dụng hay báo chí nào. Không được bịa.

## Product Principles

1. Người lớn tuổi phải tự làm được trên điện thoại, không cần con cháu đứng cạnh.
2. Một việc, ít bước: nhập giỗ, bấm một nút, vào lịch.
3. Đúng ngày âm Việt Nam trước hết; sai một ngày giỗ là hỏng niềm tin.
4. Dữ liệu gia đình ở lại trên máy người dùng.
5. Dẫn đường bằng chính màn hình mà điện thoại sẽ hiện ra (Thêm tất cả, Mở, Google Calendar).

## Accessibility & Inclusion

Người dùng chính là người lớn tuổi: chữ to, độ tương phản cao, vùng bấm lớn, câu chữ tiếng Việt đơn giản, không dựa vào màu để truyền nghĩa, không cần thao tác chính xác (kéo, giữ). Hoạt động ở màn hình 375px trở lên và ở chế độ sáng lẫn tối.
