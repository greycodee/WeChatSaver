pub mod file{
    use std::fs::File;
    use std::io;
    use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
    use tar::Archive;

    const START_HEADER: &str = "apps/";

    pub fn find_capture_position(file:&mut File) -> Result<u64, Error>{
        let mut window = vec![0; START_HEADER.len()];
        let mut position: u64 = 0;

        loop {
            if position > 200 {
                return Err(Error::new(ErrorKind::NotFound, "not found!"))
            }
            file.seek(SeekFrom::Start(position))?;
            file.read_exact(&mut window)?;
            if window == START_HEADER.as_bytes() {
                file.seek(SeekFrom::Start(position))?;
                return Ok(position);
            }
            position += 1;
        }
    }

    pub fn file_extract(file:&mut File,out_dir: &str) -> io::Result<()> {
        let mut archive = Archive::new(file);
        archive.unpack(out_dir)?;
        Ok(())
    }

}

mod test{
    use std::fs::File;
    use crate::android_backup::file::{find_capture_position, file_extract};

    #[test]
    fn test_file_capture(){
        let file = File::open("/Users/zheng/Downloads/20241024_091952/wechat.bak");
        match file {
            Ok(mut f)=>{
                let pointer = find_capture_position(&mut f);
                match pointer {
                    Ok(p) =>{
                        match file_extract(&mut f,"/Users/zheng/Downloads/20241024_091952"){
                            Ok(_) => {}
                            Err(e) => {
                                panic!("{}",e);
                            }
                        }
                        println!("find in :{}",p);
                    },
                    Err(e) =>{
                        panic!("{}",e);
                    }
                }
            },
            Err(e) =>{

                panic!("ERROR: {}",e);
            }
        }
    }
}