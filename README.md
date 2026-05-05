### 1. Perbedaan Metode RPC yang Saya Gunakan
Dalam proyek ini, saya mengimplementasikan tiga pola komunikasi. Pertama, Unary RPC untuk pembayaran, di mana satu request klien dibalas satu response server. Kedua, Server Streaming untuk riwayat transaksi; klien mengirim satu permintaan, lalu server mengirimkan aliran data (30 transaksi) secara bertahap. Terakhir, Bidirectional Streaming untuk fitur chat, di mana klien dan server bisa saling mengirim pesan secara real-time dalam satu koneksi aktif

### 2. Pertimbangan Keamanan dalam Implementasi Saya
Saat membangun layanan ini dengan Rust, saya menyadari pentingnya enkripsi data menggunakan TLS agar komunikasi tidak bisa disadap. Untuk autentikasi, saya bisa menambahkan interceptor guna memvalidasi token keamanan pada setiap pesan yang masuk. Selain itu, saya perlu mengatur otorisasi agar hanya user tertentu yang bisa memanggil fungsi sensitif seperti proses pembayaran

### 3. Tantangan Menangani Bidirectional Streaming
Menangani chat dua arah di Rust memberikan tantangan tersendiri bagi saya, terutama dalam mengelola `tokio::spawn` agar proses kirim dan terima berjalan asinkron tanpa saling mengunci (blocking). Saya juga harus memastikan manajemen channel `mpsc` dilakukan dengan benar agar jika koneksi terputus, sumber daya server segera dibersihkan dan tidak terjadi kebocoran memori

### 4. Penggunaan `ReceiverStream` dalam Kode Saya
Saya memilih menggunakan `tokio_stream::wrappers::ReceiverStream` karena alat ini sangat memudahkan saya mengubah channel internal Rust menjadi aliran data yang kompatibel dengan gRPC. Keuntungannya adalah kode saya jadi jauh lebih ringkas. Namun, saya menyadari bahwa ini menambah ketergantungan pada library eksternal dan sedikit membatasi kontrol detail jika dibandingkan dengan membuat implementasi stream manual

### 5. Struktur Kode untuk Pemeliharaan Jangka Panjang
Agar kode saya mudah dikelola, saya memisahkan definisi kontrak di file `.proto`, implementasi logika bisnis di modul service, dan pengaturan server di file utama. Dengan struktur modular ini, saya bisa memperbarui fitur chat tanpa perlu menyentuh atau merusak logika pembayaran yang sudah stabil

### 6. Pengembangan Logika pada `MyPaymentService`
Implementasi yang saya buat saat ini masih sangat dasar. Untuk sistem produksi, saya perlu menambahkan validasi saldo, integrasi dengan API bank atau payment gateway pihak ketiga, serta mekanisme penyimpanan log transaksi ke database untuk memastikan keamanan finansial

### 7. Dampak Penggunaan gRPC pada Arsitektur Saya
Dengan memilih gRPC, saya menciptakan sistem yang memiliki kontrak data sangat ketat lewat Protocol Buffers. Ini memudahkan sistem saya untuk berkomunikasi dengan layanan lain yang mungkin ditulis dalam bahasa berbeda. Meskipun desainnya sedikit lebih kompleks karena bergantung pada HTTP/2, performanya jauh lebih efisien untuk sistem terdistribusi

### 8. Keunggulan HTTP/2 Menurut Pengamatan Saya
Layanan yang saya buat berjalan di atas HTTP/2, yang jauh lebih cepat daripada HTTP/1.1 karena menggunakan format biner dan mendukung multiplexing (banyak permintaan dalam satu jalur). Ini jauh lebih hemat sumber daya dibandingkan jika saya menggunakan REST tradisional dengan polling atau websocket standar

### 9. Responsivitas: Bandingkan dengan REST
Saya merasa gRPC jauh lebih unggul untuk fitur real-time karena server bisa langsung mendorong data ke klien tanpa harus menunggu klien bertanya. Jika saya menggunakan REST, aplikasi akan terasa lebih lambat karena adanya jeda waktu dalam model request-response yang kaku

### 10. Pendekatan Skema Protobuf vs JSON
Saya lebih menyukai pendekatan schema-based dari Protocol Buffers karena kesalahan tipe data bisa terdeteksi saat proses kompilasi (compile-time). Ini jauh lebih aman dibandingkan JSON yang bersifat schema-less, di mana kesalahan penulisan field sering kali baru muncul saat aplikasi sudah berjalan dan menyebabkan error