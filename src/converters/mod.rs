use std::path::Path;

pub trait Converter {
    fn build_args(&self, input: &Path, output: &Path) -> Vec<String>;
}

struct ImageMagick;
struct LibreOffice;
struct Ffmpeg;
struct Pandoc;

pub fn get_converter(path: &Path) -> Option<Box<dyn Converter>> {
    let ext = path.extension().and_then(|e| e.to_str())?;

    match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" => Some(Box::new(ImageMagick)),
        "mp4" | "mkv" | "avi" | "mov" | "mp3" | "wav" | "flac" => Some(Box::new(Ffmpeg)),
        "md" | "rst" | "html" | "tex" | "epub" => Some(Box::new(Pandoc)),
        "docx" | "xlsx" | "pptx" | "odt" | "pdf" => Some(Box::new(LibreOffice)),
        _ => None,
    }
}

impl Converter for ImageMagick {
    fn build_args(&self, input: &Path, output: &Path) -> Vec<String> {
        let input = input.to_string_lossy().to_string();
        let output = output.to_string_lossy().to_string();

        vec!["magick".to_string(), input, output]
    }
}

impl Converter for Ffmpeg {
    fn build_args(&self, input: &Path, output: &Path) -> Vec<String> {
        let input = input.to_string_lossy().to_string();
        let output = output.to_string_lossy().to_string();

        vec!["ffmpeg".to_string(),"-i".to_string(),input, output]
    }
}

impl Converter for Pandoc {
    fn build_args(&self, input: &Path, output: &Path) -> Vec<String> {
        let input = input.to_string_lossy().to_string();
        let output = output.to_string_lossy().to_string();

        vec!["pandoc".to_string(), input, "-o".to_string(), output]

    }
}

impl Converter for LibreOffice {
    fn build_args(&self, input: &Path, output: &Path) -> Vec<String> {
        let out_ext = output.extension().and_then(|e| e.to_str()).unwrap_or("pdf").to_string();
        let input = input.to_string_lossy().to_string();

        vec![
            "libreoffice".to_string(),
            "--headless".to_string(),
            "--convert-to".to_string(),
            out_ext,
            input
        ]
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn unknown_file_extenstion() {
        assert!(get_converter(Path::new("hello.bob")).is_none());
    }

    #[test]
    fn image_extention() {
        assert!(get_converter(Path::new("hello.png")).is_some());
    }
}
