use std::ops::Deref;

pub fn md5_secret(s: String) -> String {
    let mut x = 0;
    let digest = md5::compute(&s);
    let slice: &[u8] = b"000001dbbfa3a5c83a2d506429c7b00e";
    while digest.deref() != slice {
        if digest.deref() == slice {
            break;
        }
        let s = s.to_string() + &x.to_string();
        let digest = md5::compute(&s);
        println!("{:?}", &s);
        println!("{:?}", digest);
        x += 1;
    }

    println!("{:?}", digest);
    //assert_eq!(format!("{:x}", digest), "000001dbbfa3a5c83a2d506429c7b00e");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md5_secret_unit() {
        assert_eq!(md5_secret("abcdef".to_string()), "abcdef609043".to_string());
        //    assert_eq!(
        //        md5_secret("pqrstuv".to_string()),
        //        "pqrstuv1048970".to_string()
        //    );
    }

    //#[test]
    //fn md5_secret_input() {
    //    let mut file = File::open("../resources/day3.input").expect("file does not exist");
    //    let mut f = String::new();
    //    file.read_to_string(&mut f).expect("unable to read file");
    //    let f = f.trim_matches('\n');
    //    let result = add(2, 2);
    //    assert_eq!(result, 4);
    //}
}
