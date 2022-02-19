use std::io;

use anyhow::{self, bail};
use clap::Parser;
extern crate hound;
extern crate q_compress;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    compress: bool,
    #[clap(short, long)]
    decompress: bool,
    #[clap(short, long)]
    order: Option<usize>,
}

fn read_wav() -> Result<(hound::WavSpec, Vec<i16>), anyhow::Error> {
    let mut reader = hound::WavReader::new(io::stdin())?;
    let header = reader.spec();
    if header.sample_format != hound::SampleFormat::Int {
        bail!("we only handle int wavs");
    }
    if header.channels != 1 {
        bail!("we only handle 1 channel wavs");
    }
    if header.bits_per_sample != 16 {
        bail!("we only handle 16-bit wavs");
    }
    let samples = reader.samples().collect::<Result<Vec<i16>, _>>()?;
    eprintln!("samples {}", samples.len());
    Ok((header, samples))
}

fn write_compressed(header: hound::WavSpec, data: &[u8]) -> Result<(), anyhow::Error> {
    use io::Write;

    let mut header_bytes: Vec<u8> = Vec::new();
    let cursor = io::Cursor::new(&mut header_bytes);
    let mut writer = hound::WavWriter::new(cursor, header)?;
    writer.flush()?;
    drop(writer);
    io::stdout().lock().write_all(&header_bytes)?;
    io::stdout().lock().write_all(data)?;
    Ok(())
}

fn read_compressed() -> Result<(hound::WavSpec, Vec<u8>), anyhow::Error> {
    use io::Read;

    let reader = hound::WavReader::new(io::stdin())?;
    let header = reader.spec();
    drop(reader);
    let mut csamples: Vec<u8> = Vec::new();
    io::stdin().lock().read_to_end(&mut csamples)?;
    Ok((header, csamples))
}

fn write_wav(header: hound::WavSpec, samples: &[i16]) -> Result<(), anyhow::Error> {
    use io::Write;

    eprintln!("samples {}", samples.len());
    let mut sbytes: Vec<u8> = Vec::new();
    let cursor = io::Cursor::new(&mut sbytes);
    let mut writer = hound::WavWriter::new(cursor, header)?;
    let mut swriter = writer.get_i16_writer(samples.len().try_into()?);
    for sample in samples {
        swriter.write_sample(*sample);
    }
    swriter.flush()?;
    writer.flush()?;
    drop(writer);
    eprintln!("sbytes {}", sbytes.len());
    io::stdout().lock().write_all(&sbytes)?;
    io::stdout().flush()?;
    Ok(())
}

fn run() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    if args.compress {
        let (header, samples) = read_wav()?;
        let nsamples = samples.len();
        let compressor = q_compress::Compressor::<i16>::from_config(q_compress::CompressorConfig {
            compression_level: 12,
            delta_encoding_order: args.order.unwrap_or(2),
        });
        let csamples = compressor.simple_compress(&samples);
        let ncsamples = csamples.len();
        eprintln!(
            "{}/{} ({:04.3})",
            ncsamples,
            nsamples * 2,
            ncsamples as f64 * 0.5 / nsamples as f64,
        );
        write_compressed(header, &csamples)?;
    } else if args.decompress {
        let (header, csamples) = read_compressed()?;
        let decompressor = q_compress::Decompressor::<i16>::default();
        let samples = decompressor.simple_decompress(&csamples)?;
        write_wav(header, &samples)?;
    } else {
        bail!("must specify -c or -d");
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
