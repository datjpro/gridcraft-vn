# Hướng Dẫn Tính Năng Phiên Bản Tiếng Việt (GridCraft Vietnamese Edition)

> Tài liệu hướng dẫn chi tiết các tính năng bản địa hóa, các hàm bảng tính đặc thù và công cụ tối ưu cho người dùng Việt Nam trong GridCraft.

---

## 1. Phiên Bản Tiếng Việt Có Thêm Gì? (Tổng Quan)

GridCraft cung cấp giải pháp bảng tính độc lập, mã nguồn mở viết bằng Rust với sự hỗ trợ sâu rộng cho người dùng Việt Nam, giải quyết triệt để những điểm hạn chế mà Microsoft Excel chưa hỗ trợ nguyên bản:

1. **Giao diện tiếng Việt hoàn chỉnh (UI Localization)**: Toàn bộ thanh Ribbon, Tabs, Menus, hộp thoại định dạng và thông báo đã được Việt hóa theo chuẩn thuật ngữ tin học văn phòng quen thuộc.
2. **Ký hiệu tiền tệ Việt Nam (₫ / VNĐ)**: Tích hợp sẵn nút định dạng kế toán tiền tệ `₫` ngay trên thanh công cụ Home.
3. **Bộ hàm tiện ích thuần Việt (Worksheet Functions)**: Tích hợp trực tiếp vào engine tính toán mà không cần cài macro VBA hay Add-in ngoài:
   - `=DOCSO(...)` / `=VND_TEXT(...)`: Đọc số thành chữ tiếng Việt kèm đuôi tiền tệ.
   - `=KHONGDAU(...)` / `=REMOVE_TONE(...)`: Chuyển đổi văn bản có dấu thành không dấu.
   - `=TACHTEN(...)` / `=GET_FIRST_NAME(...)`: Tách phần Tên chính trong chuỗi Họ và Tên.
   - `=TACHHO(...)` / `=GET_LAST_NAME(...)`: Tách phần Họ và Tên đệm.
   - `=PROPER_VN(...)`: Chuẩn hóa chữ hoa / chữ thường tiếng Việt chuẩn xác.
   - `=TCVN3_TO_UNICODE(...)` / `=CONVERT_TCVN3(...)`: Chuyển đổi bảng mã TCVN3 (font ABC: `.VnTime`, `.VnArial`...) sang Unicode dựng sẵn.
   - `=UNICODE_TO_TCVN3(...)`: Chuyển đổi từ Unicode sang bảng mã TCVN3.
4. **Sắp xếp theo Tên người Việt & Thứ tự từ điển tiếng Việt (Collation & Sorting)**:
   - Sắp xếp danh sách theo Tên trước - Họ sau (An < Bình < Cúc < Hà).
   - Collation tiếng Việt chuẩn: Chữ `Đ` đứng ngay sau chữ `D` (thay vì bị đẩy xuống sau `Z` như Excel mặc định); phân cấp thanh điệu: *Không dấu < Huyền < Hỏi < Ngã < Sắc < Nặng*.
   - Nút lệnh sắp xếp nhanh 1-click trên Ribbon và tùy chọn chuyên dụng trong hộp thoại Sắp xếp tùy chỉnh (*Custom Sort*).

---

## 2. Chi Tiết Các Tính Năng & Hướng Dẫn Sử Dụng

### 2.1. Bộ hàm bảng tính thuần Việt

Các hàm này có thể gõ trực tiếp trên thanh công thức (Formula Bar) hoặc nhập vào ô tính giống như các hàm chuẩn (`SUM`, `VLOOKUP`...):

#### ① Hàm `=DOCSO(number, [suffix])` (Bí danh: `=VND_TEXT`)
* **Mục đích:** Chuyển đổi số tiền hoặc giá trị số thành chữ tiếng Việt chuẩn văn phạm.
* **Cú pháp:**
  - `number`: Số tiền hoặc ô chứa số cần đọc (bắt buộc).
  - `suffix`: Hậu tố đơn vị tiền tệ kèm theo (tùy chọn, mặc định là `"đồng chẵn"`). Nếu truyền `""`, hàm chỉ đọc số.
* **Ví dụ:**
  | Công thức | Kết quả |
  |---|---|
  | `=DOCSO(1250000)` | `"Một triệu hai trăm năm mươi nghìn đồng chẵn"` |
  | `=DOCSO(500000, "đồng")` | `"Năm trăm nghìn đồng"` |
  | `=DOCSO(21, "")` | `"Hai mươi mốt"` |
  | `=DOCSO(-50000)` | `"Âm năm mươi nghìn đồng chẵn"` |
  | `=VND_TEXT(105, "VND")` | `"Một trăm lẻ năm VND"` |

---

#### ② Hàm `=KHONGDAU(text)` (Bí danh: `=REMOVE_TONE`)
* **Mục đích:** Xóa toàn bộ dấu tiếng Việt trong chuỗi văn bản (kể cả chữ `đ`/`Đ` chuyển thành `d`/`D`).
* **Cú pháp:** `text`: Chuỗi ký tự hoặc tham chiếu ô cần xóa dấu.
* **Ví dụ:**
  | Công thức | Kết quả |
  |---|---|
  | `=KHONGDAU("Nguyễn Văn An")` | `"Nguyen Van An"` |
  | `=KHONGDAU("Đại học Quốc gia TP.HCM")` | `"Dai hoc Quoc gia TP.HCM"` |
  | `=KHONGDAU("Đơn giá / Số lượng")` | `"Don gia / So luong"` |

---

#### ③ Hàm `=TACHTEN(text)` (Bí danh: `=GET_FIRST_NAME`)
* **Mục đích:** Trích xuất phần **Tên chính** (từ cuối cùng) trong chuỗi Họ và Tên tiếng Việt.
* **Cú pháp:** `text`: Chuỗi họ và tên cần trích xuất.
* **Ví dụ:**
  | Công thức | Kết quả |
  |---|---|
  | `=TACHTEN("Nguyễn Văn An")` | `"An"` |
  | `=TACHTEN("Đặng Thị Thu Hà")` | `"Hà"` |
  | `=TACHTEN("  Lê   Thị   Cúc  ")` | `"Cúc"` |
  | `=TACHTEN("John")` | `"John"` |

---

#### ④ Hàm `=TACHHO(text)` (Bí danh: `=GET_LAST_NAME`)
* **Mục đích:** Trích xuất phần **Họ và Tên đệm** (tất cả các từ đứng trước từ cuối cùng) trong chuỗi Họ và Tên.
* **Cú pháp:** `text`: Chuỗi họ và tên cần tách họ.
* **Ví dụ:**
  | Công thức | Kết quả |
  |---|---|
  | `=TACHHO("Nguyễn Văn An")` | `"Nguyễn Văn"` |
  | `=TACHHO("Đặng Thị Thu Hà")` | `"Đặng Thị Thu"` |
  | `=TACHHO("John")` | `""` |

---

#### ⑤ Hàm `=PROPER_VN(text)`
* **Mục đích:** Chuẩn hóa viết hoa chữ cái đầu tiên của từng từ trong chuỗi tiếng Việt, các chữ còn lại viết thường. Xử lý chính xác các nguyên âm có dấu và phụ âm `đ`/`Đ`.
* **Cú pháp:** `text`: Chuỗi cần chuẩn hóa danh từ riêng / họ tên.
* **Ví dụ:**
  | Công thức | Kết quả |
  |---|---|
  | `=PROPER_VN("nguyễn văn an")` | `"Nguyễn Văn An"` |
  | `=PROPER_VN("ĐẶNG THỊ THU HÀ")` | `"Đặng Thị Thu Hà"` |
  | `=PROPER_VN("lê văn tám, lớp 12a1")` | `"Lê Văn Tám, Lớp 12A1"` |

---

#### ⑥ Hàm `=TCVN3_TO_UNICODE(text)` (Bí danh: `=CONVERT_TCVN3`)
* **Mục đích:** Chuyển đổi dữ liệu bị lỗi font cũ TCVN3 (tiêu chuẩn Việt Nam 5712:1993, dùng font `.VnTime`, `.VnArial`...) sang font Unicode hiện đại. Cực kỳ hữu ích để mở các file văn bản, đề thi, báo cáo cũ.
* **Cú pháp:** `text`: Chuỗi văn bản viết bằng bảng mã TCVN3.
* **Ví dụ:**
  | Công thức | Kết quả |
  |---|---|
  | `=TCVN3_TO_UNICODE("hép møt tÕt")` | `"hộp mứt tết"` |
  | `=CONVERT_TCVN3("Céng hoµ x· héi chñ nghÜa ViÖt Nam")` | `"Cộng hoà xã hội chủ nghĩa Việt Nam"` |

*(Hàm ngược lại: `=UNICODE_TO_TCVN3(text)` hỗ trợ chuyển đổi từ Unicode về TCVN3 khi cần xuất file nộp cho các hệ thống phần mềm cũ).*

---

### 2.2. Sắp xếp danh sách theo Tên tiếng Việt (Vietnamese Name Sorting)

Trong tiếng Việt, danh sách học sinh, sinh viên, cán bộ, nhân viên luôn được sắp xếp theo **Tên** (từ cuối cùng), chứ không sắp xếp theo **Họ** (từ đầu tiên) như tiếng Anh.

#### Cách 1: Sử dụng Ribbon Home
1. Chọn vùng dữ liệu danh sách cần sắp xếp.
2. Tại tab **Trang chủ (Home)**, bấm vào nút **Sắp xếp & Lọc (Sort & Filter)**.
3. Chọn một trong hai tùy chọn:
   - **Sắp xếp theo Tên tiếng Việt (A đến Z)** (*Sort by Vietnamese Name (A to Z)*)
   - **Sắp xếp theo Tên tiếng Việt (Z đến A)** (*Sort by Vietnamese Name (Z to A)*)
4. Bảng tính sẽ tự động bóc tách tên cuối cùng để so sánh thứ tự bảng chữ cái tiếng Việt. Nếu trùng tên (ví dụ: *Nguyễn Văn An* và *Trần Văn An*), phần mềm tự động so sánh tiếp phần Họ và Đệm.

#### Cách 2: Sử dụng Hộp thoại Sắp xếp tùy chỉnh (Custom Sort)
1. Chọn vùng dữ liệu > bấm nút **Sắp xếp (Sort)** trên tab **Dữ liệu (Data)** hoặc menu `Custom Sort…`.
2. Tại cột **Sắp xếp theo (Sort by)**:
   - Chọn cột Họ và Tên.
   - Tại ô **Sắp xếp trên (Sort On)**: Chọn **Tên tiếng Việt (Tên trước - Họ sau)** hoặc **Bảng chữ cái tiếng Việt**.
   - Bấm **OK**.

---

### 2.3. Định dạng Tiền tệ Việt Nam Đồng (₫)

1. Chọn các ô chứa giá trị số tiền cần định dạng.
2. Tại tab **Trang chủ (Home)** > nhóm **Số (Number)**:
   - Bấm vào mũi tên cạnh biểu tượng tiền tệ `$`.
   - Chọn mục **₫ Tiếng Việt** (hoặc `₫ Vietnamese`).
3. Giá trị số sẽ được tự động hiển thị theo định dạng tiền tệ kế toán Việt Nam.

---

### 2.4. Chuyển đổi Ngôn ngữ Giao diện sang Tiếng Việt

Nếu ứng dụng chưa tự động chuyển sang tiếng Việt theo hệ điều hành:
1. Vào tab **Hiển thị (View)** trên thanh Ribbon.
2. Tìm nhóm **Ngôn ngữ (Language)**.
3. Chọn **Tiếng Việt**.
4. Toàn bộ giao diện (Thanh Ribbon, menu chuột phải, các bảng điều khiển bên và các hộp thoại) sẽ chuyển sang tiếng Việt ngay lập tức mà không cần khởi động lại ứng dụng.

---

## 3. Ứng Dụng Cho Các Nhóm Đối Tượng Thực Tế

| Nhóm đối tượng | Vấn đề thường gặp với Excel gốc | Giải pháp vượt trội của GridCraft |
|---|---|---|
| **Kế toán & Tài chính** | • Phải cài Add-in VBA từ nguồn không tin cậy để đọc số tiền bằng chữ.<br>• Định dạng tiền tệ VNĐ không có sẵn nhanh trên Ribbon. | • Có sẵn hàm native `=DOCSO()` tốc độ cực cao, an toàn tuyệt đối.<br>• Nút định dạng `₫` có sẵn trên Ribbon Home. |
| **Giáo viên & Quản lý trường học** | • Danh sách lớp sắp xếp theo Họ thay vì Tên (phải tạo cột phụ cắt Tên rồi ghép lại).<br>• Kho đề thi, giáo án cũ thập niên 2000 viết bằng font `.VnTime` bị lỗi "rác font". | • Lệnh 1-click **Sắp xếp theo Tên tiếng Việt** tự động nhận diện Tên trước, Họ sau.<br>• Hàm `=TCVN3_TO_UNICODE()` cứu sạch 100% tài liệu font cũ. |
| **Hành chính & Nhân sự (HR)** | • Phải gõ lại hoặc dùng công thức phức tạp để tách Họ và Tên cán bộ.<br>• Xuất file chi lương cho cổng ngân hàng yêu cầu tên không dấu.<br>• Dữ liệu nhân viên nhập lộn xộn hoa/thường. | • Bộ đôi `=TACHTEN()` và `=TACHHO()` cực kỳ nhanh gọn.<br>• Hàm `=KHONGDAU()` chuyển dữ liệu gửi ngân hàng trong 1 giây.<br>• Hàm `=PROPER_VN()` chuẩn hóa danh xưng, họ tên chính xác. |

---

## 4. Bảng Tra Cứu Nhanh Lệnh & Cú Pháp

```excel
=DOCSO(A1)                   --> Đọc số tiền ô A1 kèm chữ "đồng chẵn"
=DOCSO(A1, "VND")            --> Đọc số tiền kèm chữ "VND"
=KHONGDAU(A1)                --> Xóa toàn bộ dấu tiếng Việt
=TACHTEN(A1)                 --> Lấy Tên chính (từ cuối cùng)
=TACHHO(A1)                  --> Lấy Họ và Tên đệm
=PROPER_VN(A1)               --> Viết hoa chữ cái đầu chuẩn tiếng Việt
=TCVN3_TO_UNICODE(A1)        --> Đổi chữ font .VnTime (TCVN3) sang Unicode
```
