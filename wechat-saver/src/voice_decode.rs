use silkv3_rs::bindings::*;

pub fn get_version() -> Result<String, std::str::Utf8Error>{
    unsafe {
        let result = SKP_Silk_SDK_get_version();
        let c_str = std::ffi::CStr::from_ptr(result);
        let str_slice = c_str.to_str()?;
        Ok(str_slice.to_string())
    }
}
use std::fs::File;
use std::io::{Read, Write};

const MAX_BYTES_PER_FRAME: usize = 1024;
const MAX_INPUT_FRAMES: usize = 5;
const MAX_FRAME_LENGTH: usize = 480;
const FRAME_LENGTH_MS: usize = 20;
const MAX_API_FS_KHZ: usize = 48;
const MAX_LBRR_DELAY: usize = 2;

pub fn silk_v3_decoder(in_file: &str, out_file: &str) -> i32 {
    let mut tottime: u64 = 0;
    let mut totPackets: i32 = 0;
    let mut payload = vec![0u8; MAX_BYTES_PER_FRAME * MAX_INPUT_FRAMES * (MAX_LBRR_DELAY + 1)];
    let mut FECpayload = vec![0u8; MAX_BYTES_PER_FRAME * MAX_INPUT_FRAMES];
    let mut nBytesPerPacket = vec![0i16; MAX_LBRR_DELAY + 1];
    let mut out = vec![0i16; ((FRAME_LENGTH_MS * MAX_API_FS_KHZ) << 1) * MAX_INPUT_FRAMES];
    let mut DecControl = SKP_SILK_SDK_DecControlStruct {
        API_sampleRate: 24000,
        frameSize: 0,
        framesPerPacket: 1,
        moreInternalDecoderFrames: 0,
        inBandFECOffset: 0,
    };

    let mut bitInFile = File::open(in_file).expect("Error: could not open input file");
    let mut speechOutFile = File::create(out_file).expect("Error: could not open output file");

    let mut header_buf = vec![0u8; 50];
    bitInFile.read_exact(&mut header_buf[..1]).unwrap();
    if header_buf[0] != 0x02 {
        bitInFile.read_exact(&mut header_buf[..8]).unwrap();
        if &header_buf[..8] != b"!SILK_V3" {
            panic!("Error: Wrong Header");
        }
    } else {
        bitInFile.read_exact(&mut header_buf[..9]).unwrap();
        if &header_buf[..9] != b"#!SILK_V3" {
            panic!("Error: Wrong Header");
        }
    }

    let mut decSizeBytes: i32 = 0;
    unsafe {
        if SKP_Silk_SDK_Get_Decoder_Size(&mut decSizeBytes) != 0 {
            panic!("SKP_Silk_SDK_Get_Decoder_Size failed");
        }
    }

    let psDec = unsafe { libc::malloc(decSizeBytes as usize) };
    if psDec.is_null() {
        panic!("Failed to allocate decoder");
    }

    unsafe {
        if SKP_Silk_SDK_InitDecoder(psDec) != 0 {
            panic!("SKP_Silk_SDK_InitDecoder failed");
        }
    }

    let mut payloadEnd = 0;
    for i in 0..MAX_LBRR_DELAY {
        let mut nBytes: i16 = 0;
        bitInFile.read_exact(unsafe { std::slice::from_raw_parts_mut(&mut nBytes as *mut _ as *mut u8, 2) }).unwrap();
        bitInFile.read_exact(&mut payload[payloadEnd..payloadEnd + nBytes as usize]).unwrap();
        nBytesPerPacket[i] = nBytes;
        payloadEnd += nBytes as usize;
        totPackets += 1;
    }

    loop {
        let mut nBytes: i16 = 0;
        if bitInFile.read_exact(unsafe { std::slice::from_raw_parts_mut(&mut nBytes as *mut _ as *mut u8, 2) }).is_err() {
            break;
        }
        if nBytes < 0 {
            break;
        }
        if bitInFile.read_exact(&mut payload[payloadEnd..payloadEnd + nBytes as usize]).is_err() {
            break;
        }

        let mut lost = 0;
        let mut payloadToDec = &payload[..];
        unsafe {
            if ((libc::rand() >> 16) + (1 << 15)) as f32 / 65535.0 >= 0.0 {
                nBytesPerPacket[MAX_LBRR_DELAY] = nBytes;
                payloadEnd += nBytes as usize;
            } else {
                nBytesPerPacket[MAX_LBRR_DELAY] = 0;
            }
        }

        if nBytesPerPacket[0] == 0 {
            lost = 1;
            let mut payloadPtr = &payload[..];
            for i in 0..MAX_LBRR_DELAY {
                if nBytesPerPacket[i + 1] > 0 {
                    let mut nBytesFEC: i16 = 0;
                    unsafe {
                        SKP_Silk_SDK_search_for_LBRR(
                            payloadPtr.as_ptr(),
                            nBytesPerPacket[i + 1] as i32,
                            (i + 1) as i32,
                            FECpayload.as_mut_ptr(),
                            &mut nBytesFEC,
                        );
                    }
                    if nBytesFEC > 0 {
                        payloadToDec = &FECpayload[..];
                        nBytes = nBytesFEC;
                        lost = 0;
                        break;
                    }
                }
                payloadPtr = &payloadPtr[nBytesPerPacket[i + 1] as usize..];
            }
        } else {
            lost = 0;
            nBytes = nBytesPerPacket[0];
            payloadToDec = &payload[..];
        }

        let mut outPtr = &mut out[..];
        let mut tot_len = 0;
        let starttime = std::time::Instant::now();

        if lost == 0 {
            let mut frames = 0;
            loop {
                let mut len: i16 = 0;
                unsafe {
                    if SKP_Silk_SDK_Decode(
                        psDec,
                        &mut DecControl,
                        0,
                        payloadToDec.as_ptr(),
                        nBytes as i32,
                        outPtr.as_mut_ptr(),
                        &mut len,
                    ) != 0
                    {
                        panic!("SKP_Silk_SDK_Decode failed");
                    }
                }
                frames += 1;
                outPtr = &mut outPtr[len as usize..];
                tot_len += len as usize;
                if frames > MAX_INPUT_FRAMES {
                    outPtr = &mut out[..];
                    tot_len = 0;
                    frames = 0;
                }
                if DecControl.moreInternalDecoderFrames == 0 {
                    break;
                }
            }
        } else {
            for _ in 0..DecControl.framesPerPacket {
                let mut len: i16 = 0;
                unsafe {
                    if SKP_Silk_SDK_Decode(
                        psDec,
                        &mut DecControl,
                        1,
                        payloadToDec.as_ptr(),
                        nBytes as i32,
                        outPtr.as_mut_ptr(),
                        &mut len,
                    ) != 0
                    {
                        panic!("SKP_Silk_SDK_Decode failed");
                    }
                }
                outPtr = &mut outPtr[len as usize..];
                tot_len += len as usize;
            }
        }

        tottime += starttime.elapsed().as_micros() as u64;
        totPackets += 1;

        speechOutFile.write_all(unsafe { std::slice::from_raw_parts(out.as_ptr() as *const u8, tot_len * 2) }).unwrap();

        let mut totBytes = 0;
        for i in 0..MAX_LBRR_DELAY {
            totBytes += nBytesPerPacket[i + 1] as usize;
        }
        payload.copy_within(nBytesPerPacket[0] as usize..payloadEnd, 0);
        payloadEnd -= nBytesPerPacket[0] as usize;
        nBytesPerPacket.copy_within(1.., 0);
    }

    unsafe {
        libc::free(psDec);
    }

    // let filetime = totPackets as f64 * 1e-3 * (tot_len as f64 / (DecControl.API_sampleRate as f64 / 1000.0));
    // println!(
    //     "File length: {:.3} s\nTime for decoding: {:.3} s ({:.3}% of realtime)",
    //     100,
    //     tottime as f64 * 1e-6,
    //     tottime as f64 * 1e-4 / 100
    // );

    0
}