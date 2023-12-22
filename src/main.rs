use std::collections::VecDeque;
use std::io;

#[derive(Clone)] // Derive Clone for the Santri struct
struct Santri {
    nama: String,
    kelas: String,
    alamat: String,
}

struct Aplikasi {
    daftar_santri: Vec<Santri>,
    stack_santri: Vec<Santri>,
    queue_santri: VecDeque<Santri>,
}

impl Santri {
    fn new(nama: &str, kelas: &str, alamat: &str) -> Santri {
        Santri {
            nama: nama.to_string(),
            kelas: kelas.to_string(),
            alamat: alamat.to_string(),
        }
    }
}

impl Aplikasi {
    fn tambah_santri(&mut self, santri: Santri) {
        self.daftar_santri.push(santri.clone());
        self.stack_santri.push(santri.clone());
        self.queue_santri.push_back(santri);
    }

    fn tampilkan_santri(&self) {
        println!("Daftar Santri:");
        for (index, santri) in self.daftar_santri.iter().enumerate() {
            println!(
                "{}. Nama: {}, Kelas: {}, Alamat: {}",
                index + 1,
                santri.nama,
                santri.kelas,
                santri.alamat
            );
        }
    }

    fn tampilkan_stack(&self) {
        println!("Isi Stack:");
        for santri in self.stack_santri.iter().rev() {
            println!("Nama: {}, Kelas: {}, Alamat: {}", santri.nama, santri.kelas, santri.alamat);
        }
    }

    fn tampilkan_queue(&self) {
        println!("Isi Queue:");
        for santri in self.queue_santri.iter() {
            println!("Nama: {}, Kelas: {}, Alamat: {}", santri.nama, santri.kelas, santri.alamat);
        }
    }

    fn edit_santri(&mut self, index: usize, santri: Santri) {
        if let Some(existing_santri) = self.daftar_santri.get_mut(index - 1) {
            *existing_santri = santri;
            println!("Santri berhasil diubah!");
        } else {
            println!("Indeks santri tidak valid!");
        }
    }

    fn hapus_santri(&mut self, index: usize) {
        if index > 0 && index <= self.daftar_santri.len() {
            self.daftar_santri.remove(index - 1);
            println!("Santri berhasil dihapus!");
        } else {
            println!("Indeks santri tidak valid!");
        }
    }

    fn pop_stack(&mut self) {
        if let Some(santri) = self.stack_santri.pop() {
            println!("Santri berhasil dihapus dari Stack:");
            println!("Nama: {}, Kelas: {}, Alamat: {}", santri.nama, santri.kelas, santri.alamat);
        } else {
            println!("Stack kosong!");
        }
    }

    fn pop_queue(&mut self) {
        if let Some(santri) = self.queue_santri.pop_front() {
            println!("Santri berhasil dihapus dari Queue:");
            println!("Nama: {}, Kelas: {}, Alamat: {}", santri.nama, santri.kelas, santri.alamat);
        } else {
            println!("Queue kosong!");
        }
    }
}

fn main() {
    let mut aplikasi = Aplikasi {
        daftar_santri: Vec::new(),
        stack_santri: Vec::new(),
        queue_santri: VecDeque::new(),
    };

    loop {
        println!("Menu:");
        println!("1. Tambah Santri");
        println!("2. Tampilkan Santri");
        println!("3. Tampilkan Stack");
        println!("4. Tampilkan Queue");
        println!("5. Edit Santri");
        println!("6. Hapus Santri");
        println!("7. Hapus dari Stack");
        println!("8. Hapus dari Queue");
        println!("9. Keluar");

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).expect("Gagal membaca pilihan");

        match pilihan.trim().parse::<u32>() {
            Ok(1) => {
                println!("Masukkan nama santri:");
                let mut nama = String::new();
                io::stdin().read_line(&mut nama).expect("Gagal membaca nama");

                println!("Masukkan kelas santri:");
                let mut kelas = String::new();
                io::stdin().read_line(&mut kelas).expect("Gagal membaca kelas");

                println!("Masukkan alamat santri:");
                let mut alamat = String::new();
                io::stdin().read_line(&mut alamat).expect("Gagal membaca alamat");

                let santri_baru = Santri::new(&nama.trim(), &kelas.trim(), &alamat.trim());
                aplikasi.tambah_santri(santri_baru);
            }
            Ok(2) => aplikasi.tampilkan_santri(),
            Ok(3) => aplikasi.tampilkan_stack(),
            Ok(4) => aplikasi.tampilkan_queue(),
            Ok(5) => {
                println!("Masukkan nomor santri yang ingin diubah:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).expect("Gagal membaca nomor santri");
                let index: usize = index_str.trim().parse().expect("Nomor santri harus berupa angka");

                println!("Masukkan nama santri:");
                let mut nama = String::new();
                io::stdin().read_line(&mut nama).expect("Gagal membaca nama");

                println!("Masukkan kelas santri:");
                let mut kelas = String::new();
                io::stdin().read_line(&mut kelas).expect("Gagal membaca kelas");

                println!("Masukkan alamat santri:");
                let mut alamat = String::new();
                io::stdin().read_line(&mut alamat).expect("Gagal membaca alamat");

                let santri_baru = Santri::new(&nama.trim(), &kelas.trim(), &alamat.trim());
                aplikasi.edit_santri(index, santri_baru);
            }
            Ok(6) => {
                println!("Masukkan nomor santri yang ingin dihapus:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).expect("Gagal membaca nomor santri");
                let index: usize = index_str.trim().parse().expect("Nomor santri harus berupa angka");

                aplikasi.hapus_santri(index);
            }
            Ok(7) => aplikasi.pop_stack(),
            Ok(8) => aplikasi.pop_queue(),
            Ok(9) => break,
            _ => println!("Pilihan tidak valid"),
        }
    }
}
