use std::io;
use std::io::Write; // flush komutu kullanmak için kütüphane

fn sayılarıal() -> Vec<f64> {
    let mut sayılar: Vec<f64> = Vec::new();
    let mut sayaç = 0;

    loop {
        sayaç += 1;

        println!("{}. Değer Giriniz: ", sayaç);
        io::stdout().flush().expect("Yazma hatası");

        let mut secenek = String::new();
        io::stdin().read_line(&mut secenek).expect("Okuma hatası"); //Girilen sayıyı okuma ve hata kontrolü

        let sayı: f64 = match secenek.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Geçersiz giriş, program sonlandırılıyor."); //Hatalı giriş durumunda döngüyü sonlandırma
                break;
            }
        };

        if sayı == 0.0 {
            break; //Kullanıcı 0 girince döngüyü durdurma
        }

        sayılar.push(sayı);
    }

    sayılar
}

fn main() {
    println!("Gelişmiş Hesap Makinası");
    println!("İşlemi seçin:");  //Seçeneklerin yazdırılması
    println!("1 - Toplama");
    println!("2 - Çıkarma");
    println!("3 - Çarpma");
    println!("4 - Bölme");
    println!("5 - Ortalama");

    let mut sonuc = 0.0; // Float olarak sonucu tanımla
    let mut secenek = String::new(); //Boş metin değişkeni oluşturma
    io::stdin().read_line(&mut secenek).expect("Okuma hatası"); // Girilen değeri okuma ve hata kontrolü

    let secenek: u32 = secenek.trim().parse().expect("Geçersiz giriş"); // Metni sayıya çevirme ve hata kontrolü

    match secenek {
        1 => {
            for sayı in sayılarıal() {
                sonuc += sayı;
            }
            println!("Sonuç: {}", sonuc);
        }
        2 => {
            for sayı in sayılarıal() {
                sonuc -= sayı;
            }
            println!("Sonuç: {}", sonuc);
        }
        3 => {
            sonuc = 1.0; // Çarpım için etkisiz elemanı atama
            for sayı in sayılarıal() {
                sonuc *= sayı;
            }
            println!("Sonuç: {}", sonuc);
        }
        4 => {
            sonuc = 1.0; // Bölüm için etkisiz elemanı atama
            for sayı in sayılarıal() {
                if sayı == 0.0 {
                    println!("Bölme işleminde sıfıra bölme hatası!");
                    return;
                }
                sonuc /= sayı;
            }
            println!("Sonuç: {}", sonuc);
        }
        5 => {
            let sayılar = sayılarıal();
            let toplam: f64 = sayılar.iter().sum();
            let ortalama = toplam / sayılar.len() as f64;
            println!("Ortalama: {}", ortalama);
        }
        _ => println!("Geçersiz seçim"),
    }
}
