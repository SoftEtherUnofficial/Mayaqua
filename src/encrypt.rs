// encrypt.rs - Exact translation of encrypt.go

pub const MD5_SIZE: u32 = 16;
pub const SHA1_SIZE: u32 = 20;
pub const SHA256_SIZE: u32 = 32;
pub const SHA384_SIZE: u32 = 48;
pub const SHA512_SIZE: u32 = 64;

// Sha1Sum sha1 sum array
pub type Sha1Sum = [u8; SHA1_SIZE as usize];

// Sha0Context sha-0 context
pub struct Sha0Context {
    count: u64,
    buf: [u8; 64],
    state: [u32; 8],
}

impl Sha0Context {
    // Init sha0 init
    pub fn init(&mut self) {
        self.state[0] = 0x67452301;
        self.state[1] = 0xEFCDAB89;
        self.state[2] = 0x98BADCFE;
        self.state[3] = 0x10325476;
        self.state[4] = 0xC3D2E1F0;
        self.count = 0;
    }

    pub fn new() -> Self {
        let mut ctx = Self {
            count: 0,
            buf: [0; 64],
            state: [0; 8],
        };
        ctx.init();
        ctx
    }
}

fn rol(bits: i32, value: u32) -> u32 {
    (value << bits) | (value >> (32 - bits))
}

impl Sha0Context {
    // Transform sha0 transform
    pub fn transform(&mut self) {
        let mut w = [0u32; 80];

        let mut p = 0;
        let mut t = 0;

        while t < 16 {
            let mut tmp = (self.buf[p + 0] as u32) << 24;
            tmp |= (self.buf[p + 1] as u32) << 16;
            tmp |= (self.buf[p + 2] as u32) << 8;
            tmp |= (self.buf[p + 3] as u32) << 0;
            w[t] = tmp;
            p += 4;
            t += 1;
        }

        while t < 80 {
            w[t] = w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16];
            t += 1;
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];

        t = 0;
        while t < 80 {
            let mut tmp = rol(5, a).wrapping_add(e).wrapping_add(w[t]);
            if t < 20 {
                tmp = tmp.wrapping_add(d ^ (b & (c ^ d))).wrapping_add(0x5A827999);
            } else if t < 40 {
                tmp = tmp.wrapping_add(b ^ c ^ d).wrapping_add(0x6ED9EBA1);
            } else if t < 60 {
                tmp = tmp.wrapping_add((b & c) | (d & (b | c))).wrapping_add(0x8F1BBCDC);
            } else {
                tmp = tmp.wrapping_add(b ^ c ^ d).wrapping_add(0xCA62C1D6);
            }

            e = d;
            d = c;
            c = rol(30, b);
            b = a;
            a = tmp;
            t += 1;
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
    }

    // Update sha0 update
    pub fn update(&mut self, data: &[u8]) {
        let mut i = (self.count & 63) as usize;

        let l = data.len();
        self.count = self.count.wrapping_add(l as u64);

        for &d in data {
            self.buf[i] = d;
            i += 1;
            if i == 64 {
                self.transform();
                i = 0;
            }
        }
    }

    // Final sha0 final
    pub fn final_hash(&mut self) {
        let cnt = self.count * 8;

        self.update(&[0x80]);
        while self.count & 63 != 56 {
            self.update(&[0x0]);
        }

        for i in 0..8 {
            let tmp = (cnt >> ((7 - i) * 8)) as u8;
            self.update(&[tmp]);
        }

        let mut p = 0;
        for i in 0..5 {
            let tmp = self.state[i];
            self.buf[p + 0] = (tmp >> 24) as u8;
            self.buf[p + 1] = (tmp >> 16) as u8;
            self.buf[p + 2] = (tmp >> 8) as u8;
            self.buf[p + 3] = (tmp >> 0) as u8;
            p += 4;
        }
    }
}

// Sha0 calc sha-0
pub fn sha0(data: &[u8]) -> Sha1Sum {
    let mut ctx = Sha0Context::new();
    ctx.update(data);
    ctx.final_hash();
    let mut ret = [0u8; SHA1_SIZE as usize];
    ret.copy_from_slice(&ctx.buf[0..SHA1_SIZE as usize]);
    ret
}
