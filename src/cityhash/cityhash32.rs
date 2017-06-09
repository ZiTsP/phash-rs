const MAGICNUM32_MUR1: u32 = 0xcc9e2d51;
const MAGICNUM32_MUR2: u32 = 0x1b873593;
const MAGICNUM32_CH1: u32 = 0xe6546b64;

pub fn reverse_bit(i: u32) -> u32 {
    let mut i: u32 = (i & 0x55555555) << 1 | (i >> 1) & 0x55555555;
    i = (i & 0x33333333) << 2 | (i >> 2) & 0x33333333;
    i = (i & 0x0f0f0f0f) << 4 | (i >> 4) & 0x0f0f0f0f;
    (i << 24) | ((i & 0xff00) << 8) | ((i >> 8) & 0xff00) | (i >> 24)
}

pub fn to_hash(text: &str) -> u32 {
    let size = text.len() as u32;
    match size {
        _ if size > 24 => hash_over24(text, size),
        _ if 12 < size && size <= 24 => hash_under24(text, size),
        _ if 4 < size && size <= 12 => hash_under12(text, size),
        _ if 0 < size && size <= 4 => hash_under4(text, size),
        _ =>  0
    }
}


fn hash_over24(text: &str, size: u32) -> u32 {
    let mut h: u32 = size;
    let (tmp, _) = size.overflowing_mul(MAGICNUM32_MUR1);
    let mut g: u32 = tmp;
    let f: u32 = g;
    let (tmp, _) = fetch32(text, size - 4).overflowing_mul(MAGICNUM32_MUR1);
    let (a0, _) = (rotate32(tmp, 17)).overflowing_mul(MAGICNUM32_MUR2);
    let (tmp, _) = fetch32(text, size - 8).overflowing_mul(MAGICNUM32_MUR1);
    let (a1, _) = (rotate32(tmp, 17)).overflowing_mul(MAGICNUM32_MUR2);
    let (tmp, _) = fetch32(text, size - 16).overflowing_mul(MAGICNUM32_MUR1);
    let (a2, _) = (rotate32(tmp, 17)).overflowing_mul(MAGICNUM32_MUR2);
    let (tmp, _) = fetch32(text, size - 12).overflowing_mul(MAGICNUM32_MUR1);
    let (a3, _) = (rotate32(tmp, 17)).overflowing_mul(MAGICNUM32_MUR2);
    let (tmp, _) = fetch32(text, size - 20).overflowing_mul(MAGICNUM32_MUR1);
    let (a4, _) = (rotate32(tmp, 17)).overflowing_mul(MAGICNUM32_MUR2);
    h = rotate32(h ^ a0, 19);
    let (tmp, _) = h.overflowing_mul(5);
    let (mut h, _) = tmp.overflowing_add(MAGICNUM32_CH1);
    h = rotate32(h ^ a2, 19);
    let (tmp, _) = h.overflowing_mul(5);
    let (mut h, _) = tmp.overflowing_add(MAGICNUM32_CH1);
    g = rotate32(g ^ a1, 19);
    let (tmp, _) = g.overflowing_mul(5);
    let (mut g, _) = tmp.overflowing_add(MAGICNUM32_CH1);
    g = rotate32(g ^ a3, 19);
    let (tmp, _) = g.overflowing_mul(5);
    let (mut g, _) = tmp.overflowing_add(MAGICNUM32_CH1);
    let (mut f, _) = f.overflowing_add(a4);
    f = rotate32(f, 19);
    let (tmp, _) = f.overflowing_mul(5);
    let (mut f, _) = tmp.overflowing_add(MAGICNUM32_CH1);
    let mut iters = (size - 1) / 20;
    let mut sindex: u32 = 0;
    loop {
        let (tmp, _) = fetch32(text, sindex + 0).overflowing_mul(MAGICNUM32_MUR1);
        let (a0, _) = (rotate32(tmp, 17)).overflowing_mul(MAGICNUM32_MUR2);
        let a1 = fetch32(text, sindex + 4);
        let (tmp, _) = fetch32(text, sindex + 8).overflowing_mul(MAGICNUM32_MUR1);
        let (a2, _) = (rotate32(tmp, 17)).overflowing_mul(MAGICNUM32_MUR2);
        let (tmp, _) = fetch32(text, sindex + 12).overflowing_mul(MAGICNUM32_MUR1);
        let (a3, _) = (rotate32(tmp, 17)).overflowing_mul(MAGICNUM32_MUR2);
        let a4 = fetch32(text, sindex + 16);
        // let mut h_ = h^ a0;
        let h_ = rotate32(h ^ a0, 18);
        let (h_, _) = h_.overflowing_mul(5);
        let (h_, _) = h_.overflowing_mul(MAGICNUM32_CH1);
        let (tmp, _) = a3.overflowing_add(a1);
        let h_ = rotate32(h_ ^ tmp, 19);
        let (h_, _) = h_.overflowing_mul(5);
        let (h_, _) = h_.overflowing_add(MAGICNUM32_CH1);
        let (tmp, _) = a4.overflowing_mul(5);
        let (h_, _) = h_.overflowing_add(tmp);
        let h_ = reverse_bit(h_);
        let (f_, _) = f.overflowing_add(a1);
        let f_ = rotate32(f_, 19);
        let (f_, _) = f_.overflowing_add(a0);
        let (f_, _) = f_.overflowing_mul(MAGICNUM32_MUR1);
        let (g_, _) = g.overflowing_add(a2);
        let g_ = rotate32(g_, 18);
        let (g_, _) = g_.overflowing_mul(5);
        let (g_, _) = g_.overflowing_mul(MAGICNUM32_CH1);
        let (g_, _) = (reverse_bit(g_ ^ a4)).overflowing_mul(5);
        f = g_;
        g = h_;
        h = f_;
        sindex += 20;
        iters -= 1;
        if iters <= 0 {
          break;
        }
    }
    let (g, _) = (rotate32(g, 11)).overflowing_mul(MAGICNUM32_MUR1);
    let (g, _) = (rotate32(g, 17)).overflowing_mul(MAGICNUM32_MUR1);
    let (f, _) = (rotate32(f, 11)).overflowing_mul(MAGICNUM32_MUR1);
    let (f, _) = (rotate32(f, 17)).overflowing_mul(MAGICNUM32_MUR1);
    let (tmp, _) =  h.overflowing_add(g);
    let h = rotate32(tmp, 19);
    let (tmp, _) = h.overflowing_mul(5);
    let (h, _) = tmp.overflowing_add(MAGICNUM32_CH1);
    let (h, _) = (rotate32(h, 17)).overflowing_mul(MAGICNUM32_MUR1);
    let (tmp, _) = h.overflowing_add(f);
    let h = rotate32(tmp, 19);
    let (tmp, _) = h.overflowing_mul(5);
    let (h, _) = tmp.overflowing_add(MAGICNUM32_CH1);
    let (h, _) = (rotate32(h, 17)).overflowing_mul(MAGICNUM32_MUR1);
    h
}

fn hash_under24(text: &str, size: u32) -> u32 {
    let tmp1: u32 = fetch32(text, (size >> 1) - 4);
    let tmp2: u32 = fetch32(text, 4);
    let tmp3: u32 = fetch32(text, size - 8);
    let tmp4: u32 = fetch32(text, size >> 1);
    let tmp5: u32 = fetch32(text, 0);
    let tmp6: u32 = fetch32(text, size - 4);
    let tmp7: u32 = size;
    fmix(mur(tmp6, mur(tmp5, mur(tmp4, mur(tmp3, mur(tmp2, mur(tmp1, tmp7)))))))
}

fn hash_under12(text: &str, size: u32) -> u32 {
    let mut tmp1 = size;
    let mut tmp2 = size * 5;
    let mut tmp3 = 9;
    let tmp4 = tmp2;
    tmp1 += fetch32(text, 0);
    tmp2 += fetch32(text, size - 4);
    tmp3 += fetch32(text, (size >> 1) & 4);
    fmix(mur(tmp3, mur(tmp2, mur(tmp1, tmp4))))
}

fn hash_under4(text: &str, size: u32) -> u32 {
    let mut u32_1: u32 = 0;
    let mut u32_2: u32 = 9;
    for byte in text.bytes() {
        let (mul, _) = u32_1.overflowing_mul(MAGICNUM32_MUR1);
        let (sum, _) = mul.overflowing_add(u32::from(byte));
        u32_1 = sum;
        u32_2 ^= u32_1;
    }
    fmix(mur(u32_1, mur(size, u32_2)))
}

fn rotate32(val: u32, shift: u32) -> u32 {
    match shift {
        0 => val,
        _ => ((val >> shift) | (val << (32 - shift)))
    }
}

fn fmix(val: u32) -> u32 {
    let mut res: u32 = val;
    res ^= res >> 16;
    let (mut res, _) = res.overflowing_mul(0x85ebca6b);
    res ^= res >> 13;
    let (mut res , _) = res.overflowing_mul(0xc2b2ae35);
    res ^= res >> 16;
    res as u32
}

fn mur(val1: u32, val2: u32) -> u32 {
    let (tmp1, _) = val1.overflowing_mul(MAGICNUM32_MUR1);
    let tmp1 = rotate32(tmp1, 17);
    let (tmp2, _) = val2.overflowing_mul(MAGICNUM32_MUR2);
    let tmp2 = rotate32((tmp2 ^ tmp1), 19);
    let (tmp2, _) = tmp2.overflowing_mul(5);
    let (tmp2, _) = tmp2.overflowing_add(MAGICNUM32_CH1);
    tmp2 as u32
}

fn fetch32(text: &str, index: u32) -> u32 {
    let mut val: u32 = 0;
    let vector = text.as_bytes();
    let (_, newvec) = vector.split_at(index as usize);
    let mut size = 4;
    if newvec.len() < 4 {
        size = vector.len();
    }
    for i in 0..size {
        val += (newvec[i] as u32) << (8 * i);
    }
    val
}
