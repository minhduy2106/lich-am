---
version: 1
slug: "index-html"
primary_target: "index.html"
related_targets: []
---

# Surface: index.html (Nhắc Giỗ)

Mode: operate. Audience: người lớn tuổi Việt tự dùng trên điện thoại. Task: nhập ngày giỗ âm lịch, tích mùng 1/rằm, bấm một nút để đưa vào app lịch. Constraints: giữ nguyên id, hành vi, lời gọi wasm; chữ to, tương phản cao, vùng bấm lớn; 375px trở lên; sáng và tối.

## Direction contract

THESIS: Một tờ giấy trắng yên tĩnh với đúng một việc: nhập giỗ, bấm một nút. Người dùng từ chối hướng tờ lịch bloc đỏ đen vì màu mè; đi theo chuẩn của thể loại, làm thật chỉn chu, không thêm trang trí.

OWN-WORLD: Nền trắng #FFFFFF (tối: #000000), chữ #111111, chữ phụ #5F6368, đường kẻ 1px #E5E7EB. Một họ chữ Be Vietnam Pro tự host, 400 và 600. Ô nhập bo 12px viền 1px, nhãn riêng phía trên từng ô. Nút chính khối đen bo 14px, chữ trắng. Không bóng đổ, không thẻ, không màu nhấn; đỏ chỉ dùng cho lỗi.

STORY: Người dùng đọc tên trang và ngày âm hôm nay (tin là đúng), nhập tên và ngày tháng âm của từng người, giữ hay bỏ mùng 1 và rằm, bấm Tải về lịch, rồi đọc một dòng xác nhận và đúng bước tiếp theo trên máy của mình.

FIRST VIEWPORT: 390x844: tiêu đề Nhắc Giỗ 32px, dòng ngày dương và dòng ngày âm hôm nay; tiêu đề "Ngày giỗ trong nhà" và dòng giỗ đầu tiên với nhãn Tên, Ngày âm, Tháng âm; nút thêm; các lựa chọn nhắc; nút Tải về lịch nằm trong màn hình đầu khi có một dòng giỗ. Desktop: cột giữa 560px. Signature move: không có trang trí; độ chỉn chu nằm ở nhịp khoảng cách, cỡ chữ lớn và nút rõ.

FORM: Chuẩn của thể loại (standing exit, người dùng chọn tối giản sau khi từ chối tờ lịch bloc). seed: none (pinned) "Tối giản, sạch"

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance
