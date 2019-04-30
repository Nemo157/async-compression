use brotli2::bufread::BrotliDecoder;
use bytes::Bytes;
use futures::{
    executor::block_on,
    io::AsyncReadExt,
    stream::{self, StreamExt},
};
use std::io::{self, Read};

#[test]
fn brotli_stream() {
    use async_compression::brotli;

    let stream = stream::iter(vec![
        Bytes::from_static(&[1, 2, 3]),
        Bytes::from_static(&[4, 5, 6]),
    ]);
    let compress = brotli::Compress::new();
    let compressed = brotli::compress_stream(stream.map(Ok), compress);
    let data: Vec<_> = block_on(compressed.collect());
    let data: io::Result<Vec<_>> = data.into_iter().collect();
    let data: Vec<u8> = data.unwrap().into_iter().flatten().collect();
    let mut output = vec![];
    BrotliDecoder::new(&data[..])
        .read_to_end(&mut output)
        .unwrap();
    assert_eq!(output, vec![1, 2, 3, 4, 5, 6]);
}

//#[test]
//fn brotli_read() {
//    use async_compression::brotli;
//
//    let input = &[1, 2, 3, 4, 5, 6];
//    let compress = brotli::Compress::new();
//    let mut compressed = brotli::compress_read(&input[..], compress);
//    let mut data = vec![];
//    block_on(compressed.read_to_end(&mut data)).unwrap();
//    let mut output = vec![];
//    BrotliDecoder::new(&data[..])
//        .read_to_end(&mut output)
//        .unwrap();
//    assert_eq!(output, input);
//}