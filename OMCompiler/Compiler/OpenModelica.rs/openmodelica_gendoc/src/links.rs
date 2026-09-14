//! `modelica://` URI rewriting, replacing the old Tidy.py/BeautifulSoup pass.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Resource {
    pub source: PathBuf,
    pub target: String,
}

#[derive(Default)]
pub struct Resolver {
    /// Qualified class name -> the directory its source file lives in. Doubles
    /// as the set of classes that have a generated page.
    class_dir: HashMap<String, PathBuf>,
    /// Top-level class name -> the library root directory.
    library_root: HashMap<String, PathBuf>,
}

impl Resolver {
    pub fn add_class(&mut self, qualified_name: &str, source_file: &str) {
        let dir = Path::new(source_file).parent().unwrap_or(Path::new(""));
        self.class_dir
            .insert(qualified_name.to_string(), dir.to_path_buf());
        if !qualified_name.contains('.') {
            self.library_root
                .insert(qualified_name.to_string(), dir.to_path_buf());
        }
    }

    pub fn rewrite(&self, html: &str, resources: &mut Vec<Resource>) -> String {
        self.rewrite_from("", html, resources)
    }

    /// As `rewrite`, for a document that is not at the output root: `prefix`
    /// joins the resolved relative URL to it. An icon lives in `Icons/`, so the
    /// bitmap it names resolves to `../resources/…`.
    pub fn rewrite_from(&self, prefix: &str, html: &str, resources: &mut Vec<Resource>) -> String {
        let mut out = String::with_capacity(html.len());
        let mut rest = html;
        while let Some(start) = find_scheme(rest) {
            out.push_str(&rest[..start]);
            let tail = &rest[start + SCHEME.len()..];
            // `&` ends it too: documentation strings that mention the scheme in
            // escaped markup would otherwise swallow the `&quot;` after it.
            let end = tail
                .find(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | '>' | '&' | '\\'))
                .unwrap_or(tail.len());
            let url = self.resolve(&tail[..end], resources);
            if !url.starts_with(SCHEME) {
                out.push_str(prefix);
            }
            out.push_str(&url);
            rest = &tail[end..];
        }
        out.push_str(rest);
        out
    }

    /// A URI that names neither a class we generated a page for nor a file that
    /// exists is left as it was: the documentation is discussing the scheme,
    /// not linking with it.
    fn resolve(&self, uri: &str, resources: &mut Vec<Resource>) -> String {
        let (target, anchor) = match uri.split_once('#') {
            Some((u, a)) => (u, Some(a)),
            None => (uri, None),
        };
        let resolved = match target.split_once('/') {
            Some((class, file)) => self.resolve_file(class, file, resources),
            None => self
                .class_dir
                .contains_key(target)
                .then(|| format!("{}.html", uri_encode(&file_stem(target)))),
        };
        match (resolved, anchor) {
            (Some(page), Some(anchor)) => format!("{page}#{anchor}"),
            (Some(page), None) => page,
            (None, _) => format!("{SCHEME}{uri}"),
        }
    }

    /// The file a `modelica://Lib.Sub/Resources/x.png` URI names.
    fn source_file(&self, class: &str, file: &str) -> Option<PathBuf> {
        let file = percent_decode(file);
        let mut dir = self.class_dir.get(class);
        if dir.is_none() {
            // The class may be declared inside a package.mo, in which case the
            // enclosing package's directory is the one the URI resolves against.
            dir = class.rsplit_once('.').and_then(|(p, _)| self.class_dir.get(p));
        }
        let source = dir?.join(&file);
        source.is_file().then_some(source)
    }

    /// `modelica://Lib.Sub/Resources/x.png` -> `resources/Lib/Sub/Resources/x.png`.
    fn resolve_file(
        &self,
        class: &str,
        file: &str,
        resources: &mut Vec<Resource>,
    ) -> Option<String> {
        let source = self.source_file(class, file)?;
        let library = class.split('.').next()?;
        let root = self.library_root.get(library)?;
        let relative = source.strip_prefix(root).ok()?;
        let target = format!(
            "resources/{library}/{}",
            relative.to_string_lossy().replace('\\', "/")
        );
        let encoded = uri_encode(&target);
        resources.push(Resource { source, target });
        Some(encoded)
    }
}

impl Resolver {
    /// Replace every `modelica://` URI in an SVG with the file's bytes as a
    /// `data:` URI. A linked file cannot be used here: a browser renders an SVG
    /// loaded through `<img>` in secure static mode, where external references
    /// are never fetched, so the bitmap silently does not appear — on the class
    /// page and in the sidebar alike. Embedding costs nothing in practice,
    /// since a Bitmap in an icon is rare and the icon store is content
    /// addressed, so one logo is stored once however many classes show it.
    pub fn inline_uris(&self, svg: &str) -> String {
        let mut out = String::with_capacity(svg.len());
        let mut rest = svg;
        while let Some(start) = find_scheme(rest) {
            out.push_str(&rest[..start]);
            let tail = &rest[start + SCHEME.len()..];
            let end = tail
                .find(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | '>' | '&' | '\\'))
                .unwrap_or(tail.len());
            let uri = &tail[..end];
            match uri
                .split_once('/')
                .and_then(|(class, file)| self.source_file(class, file))
                .and_then(|path| std::fs::read(&path).ok().map(|bytes| (path, bytes)))
            {
                Some((path, bytes)) => {
                    out.push_str("data:");
                    out.push_str(media_type(&path));
                    out.push_str(";base64,");
                    base64_into(&bytes, &mut out);
                }
                None => {
                    out.push_str(SCHEME);
                    out.push_str(uri);
                }
            }
            rest = &tail[end..];
        }
        out.push_str(rest);
        out
    }
}

fn media_type(path: &Path) -> &'static str {
    match path
        .extension()
        .map(|e| e.to_string_lossy().to_ascii_lowercase())
        .as_deref()
    {
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("bmp") => "image/bmp",
        Some("webp") => "image/webp",
        _ => "image/png",
    }
}

fn base64_into(bytes: &[u8], out: &mut String) {
    const ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    for chunk in bytes.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(ALPHABET[(n >> 18) as usize & 63] as char);
        out.push(ALPHABET[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { ALPHABET[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[n as usize & 63] as char } else { '=' });
    }
}

const SCHEME: &str = "modelica://";

fn find_scheme(haystack: &str) -> Option<usize> {
    let bytes = haystack.as_bytes();
    let scheme = SCHEME.as_bytes();
    bytes
        .windows(scheme.len())
        .position(|w| w.eq_ignore_ascii_case(scheme))
}

/// The same replacements the old GenerateDoc.mos used, so existing links to
/// generated pages keep working.
pub fn file_stem(class: &str) -> String {
    class
        .replace('/', "Division")
        .replace('*', "Multiplication")
        .replace('<', "x3C")
        .replace('>', "x3E")
}

pub fn uri_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            ' ' => out.push_str("%20"),
            '\'' => out.push_str("%27"),
            '"' => out.push_str("%22"),
            '<' => out.push_str("%3C"),
            '>' => out.push_str("%3E"),
            _ => out.push(ch),
        }
    }
    out
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(b) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}
