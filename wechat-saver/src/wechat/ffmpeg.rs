use std::io::Error;
use std::process::Command;

pub fn verify_ffmpeg_install() -> Result<(), Error> {
    Command::new("ffmpeg").arg("-version").output()?;
    Ok(())
}

fn transcode_pcm_to_mp3(input_pcm: &str, output_mp3: &str) -> Result<(), Error> {
    let ffmpeg_command = Command::new("ffmpeg")
        .arg("-y") // 覆盖输出文件
        .arg("-f")
        .arg("s16le") // 输入文件格式
        .arg("-ar")
        .arg("24000") // 采样率
        .arg("-ac")
        .arg("1") // 声道数量
        .arg("-i")
        .arg(input_pcm) // 输入PCM文件
        .arg(output_mp3) // 输出MP3文件
        .output()?;
    // 检查命令的输出结果
    if ffmpeg_command.status.success() {
        println!("Transcoding successful! Output file: {}", output_mp3);
    } else {
        return Err(Error::new(std::io::ErrorKind::Other, "Transcoding failed!"));
    }
    Ok(())
}

mod test {
    use super::*;
    #[test]
    fn test_verify_ffmpeg_install() {
        match verify_ffmpeg_install() {
            Ok(_) => {
                println!("ffmpeg found!");
            }
            Err(e) => {
                panic!("ERR: {}", e);
            }
        }
    }

    #[test]
    fn test_transcode_pcm_to_mp3() {
        match transcode_pcm_to_mp3("/tmp/msg_152059061922b0890a24269102.pcm", "/tmp/test.mp3") {
            Ok(_) => {
                println!("transcode success!");
            }
            Err(e) => {
                panic!("ERR: {}", e);
            }
        }
    }
}
