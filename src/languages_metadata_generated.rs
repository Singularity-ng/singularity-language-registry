// AUTO-GENERATED FILE - DO NOT EDIT MANUALLY
// Generated from GitHub Linguist languages.yml
// Run: cargo run --bin sync-linguist --features sync-tool

//! Auto-generated language metadata from GitHub Linguist.
//! Contains comprehensive language definitions including extensions,
//! colors, interpreters, and editor modes.

#[derive(Debug, Clone)]
pub struct LanguageMetadata {
pub name: &'static str,
pub language_type: &'static str,
pub color: Option<&'static str>,
pub extensions: &'static [&'static str],
pub aliases: &'static [&'static str],
pub interpreters: &'static [&'static str],
pub filenames: &'static [&'static str],
pub ace_mode: Option<&'static str>,
pub tm_scope: Option<&'static str>,
pub language_id: Option<i64>,
pub codemirror_mode: Option<&'static str>,
pub codemirror_mime_type: Option<&'static str>,
pub group: Option<&'static str>,
pub wrap: Option<bool>,
pub fs_name: Option<&'static str>,
}

/// All language definitions from GitHub Linguist.
pub const LANGUAGES: &[LanguageMetadata] = &[
    LanguageMetadata {
        name: "1C Enterprise",
        language_type: "programming",
        color: Some("#814CCC"),
        extensions: &[
            ".bsl",
            ".os",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.bsl"),
        language_id: Some(0),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "2-Dimensional Array",
        language_type: "data",
        color: Some("#38761D"),
        extensions: &[
            ".2da",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.2da"),
        language_id: Some(387204628),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "4D",
        language_type: "programming",
        color: Some("#004289"),
        extensions: &[
            ".4dm",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.4dm"),
        language_id: Some(577529595),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ABAP",
        language_type: "programming",
        color: Some("#E8274B"),
        extensions: &[
            ".abap",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("abap"),
        tm_scope: Some("source.abap"),
        language_id: Some(1),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ABAP CDS",
        language_type: "programming",
        color: Some("#555e25"),
        extensions: &[
            ".asddls",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.abapcds"),
        language_id: Some(452681853),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ABNF",
        language_type: "data",
        color: None,
        extensions: &[
            ".abnf",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.abnf"),
        language_id: Some(429),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "AGS Script",
        language_type: "programming",
        color: Some("#B9D9FF"),
        extensions: &[
            ".asc",
            ".ash",
        ],
        aliases: &[
            "ags",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c++"),
        language_id: Some(2),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-c++src"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "AIDL",
        language_type: "programming",
        color: Some("#34EB6B"),
        extensions: &[
            ".aidl",
        ],
        aliases: &[
        ],
        interpreters: &[
            "aidl",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.aidl"),
        language_id: Some(451700185),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "AL",
        language_type: "programming",
        color: Some("#3AA2B5"),
        extensions: &[
            ".al",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.al"),
        language_id: Some(658971832),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "AMPL",
        language_type: "programming",
        color: Some("#E6EFBB"),
        extensions: &[
            ".ampl",
            ".mod",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ampl"),
        language_id: Some(3),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ANTLR",
        language_type: "programming",
        color: Some("#9DC3FF"),
        extensions: &[
            ".g4",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.antlr"),
        language_id: Some(4),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "API Blueprint",
        language_type: "markup",
        color: Some("#2ACCA8"),
        extensions: &[
            ".apib",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("markdown"),
        tm_scope: Some("text.html.markdown.source.gfm.apib"),
        language_id: Some(5),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "APL",
        language_type: "programming",
        color: Some("#5A8164"),
        extensions: &[
            ".apl",
            ".dyalog",
        ],
        aliases: &[
        ],
        interpreters: &[
            "apl",
            "aplx",
            "dyalog",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.apl"),
        language_id: Some(6),
        codemirror_mode: Some("apl"),
        codemirror_mime_type: Some("text/apl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ASL",
        language_type: "programming",
        color: None,
        extensions: &[
            ".asl",
            ".dsl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("asl"),
        tm_scope: Some("source.asl"),
        language_id: Some(124996147),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ASN.1",
        language_type: "data",
        color: None,
        extensions: &[
            ".asn",
            ".asn1",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.asn"),
        language_id: Some(7),
        codemirror_mode: Some("asn.1"),
        codemirror_mime_type: Some("text/x-ttcn-asn"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ASP.NET",
        language_type: "programming",
        color: Some("#9400ff"),
        extensions: &[
            ".asax",
            ".ascx",
            ".ashx",
            ".asmx",
            ".aspx",
            ".axd",
        ],
        aliases: &[
            "aspx",
            "aspx-vb",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.html.asp"),
        language_id: Some(564186416),
        codemirror_mode: Some("htmlembedded"),
        codemirror_mime_type: Some("application/x-aspx"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ATS",
        language_type: "programming",
        color: Some("#1ac620"),
        extensions: &[
            ".dats",
            ".hats",
            ".sats",
        ],
        aliases: &[
            "ats2",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ocaml"),
        tm_scope: Some("source.ats"),
        language_id: Some(9),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ActionScript",
        language_type: "programming",
        color: Some("#882B0F"),
        extensions: &[
            ".as",
        ],
        aliases: &[
            "actionscript 3",
            "actionscript3",
            "as3",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("actionscript"),
        tm_scope: Some("source.actionscript.3"),
        language_id: Some(10),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ada",
        language_type: "programming",
        color: Some("#02f88c"),
        extensions: &[
            ".adb",
            ".ada",
            ".ads",
        ],
        aliases: &[
            "ada95",
            "ada2005",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ada"),
        tm_scope: Some("source.ada"),
        language_id: Some(11),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Adblock Filter List",
        language_type: "data",
        color: Some("#800000"),
        extensions: &[
            ".txt",
        ],
        aliases: &[
            "ad block filters",
            "ad block",
            "adb",
            "adblock",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.adblock"),
        language_id: Some(884614762),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Adobe Font Metrics",
        language_type: "data",
        color: Some("#fa0f00"),
        extensions: &[
            ".afm",
        ],
        aliases: &[
            "acfm",
            "adobe composite font metrics",
            "adobe multiple font metrics",
            "amfm",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.afm"),
        language_id: Some(147198098),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Agda",
        language_type: "programming",
        color: Some("#315665"),
        extensions: &[
            ".agda",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.agda"),
        language_id: Some(12),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Aiken",
        language_type: "programming",
        color: Some("#640ff8"),
        extensions: &[
            ".ak",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.aiken"),
        language_id: Some(899409497),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Alloy",
        language_type: "programming",
        color: Some("#64C800"),
        extensions: &[
            ".als",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.alloy"),
        language_id: Some(13),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Alpine Abuild",
        language_type: "programming",
        color: Some("#0D597F"),
        extensions: &[
        ],
        aliases: &[
            "abuild",
            "apkbuild",
        ],
        interpreters: &[
        ],
        filenames: &[
            "APKBUILD",
        ],
        ace_mode: Some("sh"),
        tm_scope: Some("source.shell"),
        language_id: Some(14),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: Some("Shell"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Altium Designer",
        language_type: "data",
        color: Some("#A89663"),
        extensions: &[
            ".OutJob",
            ".PcbDoc",
            ".PrjPCB",
            ".SchDoc",
        ],
        aliases: &[
            "altium",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ini"),
        tm_scope: Some("source.ini"),
        language_id: Some(187772328),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "AngelScript",
        language_type: "programming",
        color: Some("#C7D7DC"),
        extensions: &[
            ".as",
            ".angelscript",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.angelscript"),
        language_id: Some(389477596),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-c++src"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Answer Set Programming",
        language_type: "programming",
        color: Some("#A9CC29"),
        extensions: &[
            ".lp",
        ],
        aliases: &[
        ],
        interpreters: &[
            "clingo",
        ],
        filenames: &[
        ],
        ace_mode: Some("prolog"),
        tm_scope: Some("source.answersetprogramming"),
        language_id: Some(433009171),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ant Build System",
        language_type: "data",
        color: Some("#A9157E"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "ant.xml",
            "build.xml",
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml.ant"),
        language_id: Some(15),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("application/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Antlers",
        language_type: "markup",
        color: Some("#ff269e"),
        extensions: &[
            ".antlers.html",
            ".antlers.php",
            ".antlers.xml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.html.statamic"),
        language_id: Some(1067292663),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ApacheConf",
        language_type: "data",
        color: Some("#d12127"),
        extensions: &[
            ".apacheconf",
            ".vhost",
        ],
        aliases: &[
            "aconf",
            "apache",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".htaccess",
            "apache2.conf",
            "httpd.conf",
        ],
        ace_mode: Some("apache_conf"),
        tm_scope: Some("source.apacheconf"),
        language_id: Some(16),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Apex",
        language_type: "programming",
        color: Some("#1797c0"),
        extensions: &[
            ".cls",
            ".apex",
            ".trigger",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("apex"),
        tm_scope: Some("source.apex"),
        language_id: Some(17),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-java"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Apollo Guidance Computer",
        language_type: "programming",
        color: Some("#0B3D91"),
        extensions: &[
            ".agc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("assembly_x86"),
        tm_scope: Some("source.agc"),
        language_id: Some(18),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Assembly"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "AppleScript",
        language_type: "programming",
        color: Some("#101F1F"),
        extensions: &[
            ".applescript",
            ".scpt",
        ],
        aliases: &[
            "osascript",
        ],
        interpreters: &[
            "osascript",
        ],
        filenames: &[
        ],
        ace_mode: Some("applescript"),
        tm_scope: Some("source.applescript"),
        language_id: Some(19),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Arc",
        language_type: "programming",
        color: Some("#aa2afe"),
        extensions: &[
            ".arc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(20),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ArkTS",
        language_type: "programming",
        color: Some("#0080ff"),
        extensions: &[
            ".ets",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("typescript"),
        tm_scope: Some("source.ets"),
        language_id: Some(56341321),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/typescript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "AsciiDoc",
        language_type: "prose",
        color: Some("#73a0c5"),
        extensions: &[
            ".asciidoc",
            ".adoc",
            ".asc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("asciidoc"),
        tm_scope: Some("text.html.asciidoc"),
        language_id: Some(22),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "AspectJ",
        language_type: "programming",
        color: Some("#a957b0"),
        extensions: &[
            ".aj",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.aspectj"),
        language_id: Some(23),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Assembly",
        language_type: "programming",
        color: Some("#6E4C13"),
        extensions: &[
            ".asm",
            ".a51",
            ".i",
            ".inc",
            ".nas",
            ".nasm",
            ".s",
        ],
        aliases: &[
            "asm",
            "nasm",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("assembly_x86"),
        tm_scope: Some("source.assembly"),
        language_id: Some(24),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Astro",
        language_type: "markup",
        color: Some("#ff5a03"),
        extensions: &[
            ".astro",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("astro"),
        tm_scope: Some("source.astro"),
        language_id: Some(578209015),
        codemirror_mode: Some("jsx"),
        codemirror_mime_type: Some("text/jsx"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Asymptote",
        language_type: "programming",
        color: Some("#ff0000"),
        extensions: &[
            ".asy",
        ],
        aliases: &[
        ],
        interpreters: &[
            "asy",
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c++"),
        language_id: Some(591605007),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-kotlin"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Augeas",
        language_type: "programming",
        color: Some("#9CC134"),
        extensions: &[
            ".aug",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(25),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "AutoHotkey",
        language_type: "programming",
        color: Some("#6594b9"),
        extensions: &[
            ".ahk",
            ".ahkl",
        ],
        aliases: &[
            "ahk",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("autohotkey"),
        tm_scope: Some("source.ahk"),
        language_id: Some(26),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "AutoIt",
        language_type: "programming",
        color: Some("#1C3552"),
        extensions: &[
            ".au3",
        ],
        aliases: &[
            "au3",
            "AutoIt3",
            "AutoItScript",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("autohotkey"),
        tm_scope: Some("source.autoit"),
        language_id: Some(27),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Avro IDL",
        language_type: "data",
        color: Some("#0040FF"),
        extensions: &[
            ".avdl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.avro"),
        language_id: Some(785497837),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Awk",
        language_type: "programming",
        color: Some("#c30e9b"),
        extensions: &[
            ".awk",
            ".auk",
            ".gawk",
            ".mawk",
            ".nawk",
        ],
        aliases: &[
        ],
        interpreters: &[
            "awk",
            "gawk",
            "mawk",
            "nawk",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.awk"),
        language_id: Some(28),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "B4X",
        language_type: "programming",
        color: Some("#00e4ff"),
        extensions: &[
            ".bas",
        ],
        aliases: &[
            "basic for android",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.vba"),
        language_id: Some(96642275),
        codemirror_mode: Some("vb"),
        codemirror_mime_type: Some("text/x-vb"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "BASIC",
        language_type: "programming",
        color: Some("#ff0000"),
        extensions: &[
            ".bas",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("basic"),
        tm_scope: Some("source.basic"),
        language_id: Some(28923963),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "BQN",
        language_type: "programming",
        color: Some("#2b7067"),
        extensions: &[
            ".bqn",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.bqn"),
        language_id: Some(330386870),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ballerina",
        language_type: "programming",
        color: Some("#FF5000"),
        extensions: &[
            ".bal",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ballerina"),
        language_id: Some(720859680),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Batchfile",
        language_type: "programming",
        color: Some("#C1F12E"),
        extensions: &[
            ".bat",
            ".cmd",
        ],
        aliases: &[
            "bat",
            "batch",
            "dosbatch",
            "winbatch",
        ],
        interpreters: &[
        ],
        filenames: &[
            "gradlew.bat",
            "mvnw.cmd",
        ],
        ace_mode: Some("batchfile"),
        tm_scope: Some("source.batchfile"),
        language_id: Some(29),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Beef",
        language_type: "programming",
        color: Some("#a52f4e"),
        extensions: &[
            ".bf",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("csharp"),
        tm_scope: Some("source.cs"),
        language_id: Some(545626333),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csharp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Befunge",
        language_type: "programming",
        color: None,
        extensions: &[
            ".befunge",
            ".bf",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.befunge"),
        language_id: Some(30),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Berry",
        language_type: "programming",
        color: Some("#15A13C"),
        extensions: &[
            ".be",
        ],
        aliases: &[
            "be",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.berry"),
        language_id: Some(121855308),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "BibTeX",
        language_type: "markup",
        color: Some("#778899"),
        extensions: &[
            ".bib",
            ".bibtex",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("bibtex"),
        tm_scope: Some("text.bibtex"),
        language_id: Some(982188347),
        codemirror_mode: Some("stex"),
        codemirror_mime_type: Some("text/x-stex"),
        group: Some("TeX"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "BibTeX Style",
        language_type: "programming",
        color: None,
        extensions: &[
            ".bst",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.bst"),
        language_id: Some(909569041),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Bicep",
        language_type: "programming",
        color: Some("#519aba"),
        extensions: &[
            ".bicep",
            ".bicepparam",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.bicep"),
        language_id: Some(321200902),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Bikeshed",
        language_type: "markup",
        color: Some("#5562ac"),
        extensions: &[
            ".bs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html"),
        tm_scope: Some("source.csswg"),
        language_id: Some(1055528081),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Bison",
        language_type: "programming",
        color: Some("#6A463F"),
        extensions: &[
            ".bison",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.yacc"),
        language_id: Some(31),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Yacc"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "BitBake",
        language_type: "programming",
        color: Some("#00bce4"),
        extensions: &[
            ".bb",
            ".bbappend",
            ".bbclass",
            ".inc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.bb"),
        language_id: Some(32),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Blade",
        language_type: "markup",
        color: Some("#f7523f"),
        extensions: &[
            ".blade",
            ".blade.php",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("php_laravel_blade"),
        tm_scope: Some("text.html.php.blade"),
        language_id: Some(33),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "BlitzBasic",
        language_type: "programming",
        color: Some("#00FFAE"),
        extensions: &[
            ".bb",
            ".decls",
        ],
        aliases: &[
            "b3d",
            "blitz3d",
            "blitzplus",
            "bplus",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.blitzmax"),
        language_id: Some(34),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "BlitzMax",
        language_type: "programming",
        color: Some("#cd6400"),
        extensions: &[
            ".bmx",
        ],
        aliases: &[
            "bmax",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.blitzmax"),
        language_id: Some(35),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Bluespec",
        language_type: "programming",
        color: Some("#12223c"),
        extensions: &[
            ".bsv",
        ],
        aliases: &[
            "bluespec bsv",
            "bsv",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("verilog"),
        tm_scope: Some("source.bsv"),
        language_id: Some(36),
        codemirror_mode: Some("verilog"),
        codemirror_mime_type: Some("text/x-systemverilog"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Bluespec BH",
        language_type: "programming",
        color: Some("#12223c"),
        extensions: &[
            ".bs",
        ],
        aliases: &[
            "bh",
            "bluespec classic",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("haskell"),
        tm_scope: Some("source.bh"),
        language_id: Some(641580358),
        codemirror_mode: Some("haskell"),
        codemirror_mime_type: Some("text/x-haskell"),
        group: Some("Bluespec"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Boo",
        language_type: "programming",
        color: Some("#d4bec1"),
        extensions: &[
            ".boo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.boo"),
        language_id: Some(37),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Boogie",
        language_type: "programming",
        color: Some("#c80fa0"),
        extensions: &[
            ".bpl",
        ],
        aliases: &[
        ],
        interpreters: &[
            "boogie",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.boogie"),
        language_id: Some(955017407),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Brainfuck",
        language_type: "programming",
        color: Some("#2F2530"),
        extensions: &[
            ".b",
            ".bf",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.bf"),
        language_id: Some(38),
        codemirror_mode: Some("brainfuck"),
        codemirror_mime_type: Some("text/x-brainfuck"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "BrighterScript",
        language_type: "programming",
        color: Some("#66AABB"),
        extensions: &[
            ".bs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.brs"),
        language_id: Some(943571030),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Brightscript",
        language_type: "programming",
        color: Some("#662D91"),
        extensions: &[
            ".brs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.brs"),
        language_id: Some(39),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Browserslist",
        language_type: "data",
        color: Some("#ffd539"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            ".browserslistrc",
            "browserslist",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.browserslist"),
        language_id: Some(153503348),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Bru",
        language_type: "markup",
        color: Some("#F4AA41"),
        extensions: &[
            ".bru",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.bru"),
        language_id: Some(906627898),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "BuildStream",
        language_type: "data",
        color: Some("#006bff"),
        extensions: &[
            ".bst",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.yaml"),
        language_id: Some(84359046),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "C",
        language_type: "programming",
        color: Some("#555555"),
        extensions: &[
            ".c",
            ".cats",
            ".h",
            ".h.in",
            ".idc",
        ],
        aliases: &[
        ],
        interpreters: &[
            "tcc",
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c"),
        language_id: Some(41),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "C#",
        language_type: "programming",
        color: Some("#178600"),
        extensions: &[
            ".cs",
            ".cake",
            ".cs.pp",
            ".csx",
            ".linq",
        ],
        aliases: &[
            "csharp",
            "cake",
            "cakescript",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("csharp"),
        tm_scope: Some("source.cs"),
        language_id: Some(42),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csharp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "C++",
        language_type: "programming",
        color: Some("#f34b7d"),
        extensions: &[
            ".cpp",
            ".c++",
            ".cc",
            ".cp",
            ".cppm",
            ".cxx",
            ".h",
            ".h++",
            ".hh",
            ".hpp",
            ".hxx",
            ".inc",
            ".inl",
            ".ino",
            ".ipp",
            ".ixx",
            ".re",
            ".tcc",
            ".tpp",
            ".txx",
        ],
        aliases: &[
            "cpp",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c++"),
        language_id: Some(43),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-c++src"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "C-ObjDump",
        language_type: "data",
        color: None,
        extensions: &[
            ".c-objdump",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("assembly_x86"),
        tm_scope: Some("objdump.x86asm"),
        language_id: Some(44),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "C2hs Haskell",
        language_type: "programming",
        color: None,
        extensions: &[
            ".chs",
        ],
        aliases: &[
            "c2hs",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("haskell"),
        tm_scope: Some("source.haskell"),
        language_id: Some(45),
        codemirror_mode: Some("haskell"),
        codemirror_mime_type: Some("text/x-haskell"),
        group: Some("Haskell"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "C3",
        language_type: "programming",
        color: Some("#2563eb"),
        extensions: &[
            ".c3",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c3"),
        language_id: Some(769248603),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CAP CDS",
        language_type: "programming",
        color: Some("#0092d1"),
        extensions: &[
            ".cds",
        ],
        aliases: &[
            "cds",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cds"),
        language_id: Some(390788699),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CIL",
        language_type: "data",
        color: None,
        extensions: &[
            ".cil",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cil"),
        language_id: Some(29176339),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CLIPS",
        language_type: "programming",
        color: Some("#00A300"),
        extensions: &[
            ".clp",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.clips"),
        language_id: Some(46),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CMake",
        language_type: "programming",
        color: Some("#DA3434"),
        extensions: &[
            ".cmake",
            ".cmake.in",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "CMakeLists.txt",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cmake"),
        language_id: Some(47),
        codemirror_mode: Some("cmake"),
        codemirror_mime_type: Some("text/x-cmake"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "COBOL",
        language_type: "programming",
        color: None,
        extensions: &[
            ".cob",
            ".cbl",
            ".ccp",
            ".cobol",
            ".cpy",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("cobol"),
        tm_scope: Some("source.cobol"),
        language_id: Some(48),
        codemirror_mode: Some("cobol"),
        codemirror_mime_type: Some("text/x-cobol"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CODEOWNERS",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "CODEOWNERS",
        ],
        ace_mode: Some("gitignore"),
        tm_scope: Some("text.codeowners"),
        language_id: Some(321684729),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "COLLADA",
        language_type: "data",
        color: Some("#F1A42B"),
        extensions: &[
            ".dae",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml"),
        language_id: Some(49),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CSON",
        language_type: "data",
        color: Some("#244776"),
        extensions: &[
            ".cson",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("coffee"),
        tm_scope: Some("source.coffee"),
        language_id: Some(424),
        codemirror_mode: Some("coffeescript"),
        codemirror_mime_type: Some("text/x-coffeescript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CSS",
        language_type: "markup",
        color: Some("#663399"),
        extensions: &[
            ".css",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("css"),
        tm_scope: Some("source.css"),
        language_id: Some(50),
        codemirror_mode: Some("css"),
        codemirror_mime_type: Some("text/css"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CSV",
        language_type: "data",
        color: Some("#237346"),
        extensions: &[
            ".csv",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("csv"),
        tm_scope: Some("none"),
        language_id: Some(51),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CUE",
        language_type: "programming",
        color: Some("#5886E1"),
        extensions: &[
            ".cue",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cue"),
        language_id: Some(356063509),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CWeb",
        language_type: "programming",
        color: Some("#00007a"),
        extensions: &[
            ".w",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(657332628),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cabal Config",
        language_type: "data",
        color: Some("#483465"),
        extensions: &[
            ".cabal",
        ],
        aliases: &[
            "Cabal",
        ],
        interpreters: &[
        ],
        filenames: &[
            "cabal.config",
            "cabal.project",
        ],
        ace_mode: Some("haskell_cabal"),
        tm_scope: Some("source.cabal"),
        language_id: Some(677095381),
        codemirror_mode: Some("haskell"),
        codemirror_mime_type: Some("text/x-haskell"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Caddyfile",
        language_type: "data",
        color: Some("#22b638"),
        extensions: &[
            ".caddyfile",
        ],
        aliases: &[
            "Caddy",
        ],
        interpreters: &[
        ],
        filenames: &[
            "Caddyfile",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.Caddyfile"),
        language_id: Some(615465151),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cadence",
        language_type: "programming",
        color: Some("#00ef8b"),
        extensions: &[
            ".cdc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cadence"),
        language_id: Some(270184138),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cairo",
        language_type: "programming",
        color: Some("#ff4a48"),
        extensions: &[
            ".cairo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cairo"),
        language_id: Some(620599567),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Cairo"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cairo Zero",
        language_type: "programming",
        color: Some("#ff4a48"),
        extensions: &[
            ".cairo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cairo0"),
        language_id: Some(891399890),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Cairo"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CameLIGO",
        language_type: "programming",
        color: Some("#3be133"),
        extensions: &[
            ".mligo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ocaml"),
        tm_scope: Some("source.mligo"),
        language_id: Some(829207807),
        codemirror_mode: Some("mllike"),
        codemirror_mime_type: Some("text/x-ocaml"),
        group: Some("LigoLANG"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cangjie",
        language_type: "programming",
        color: Some("#00868B"),
        extensions: &[
            ".cj",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("swift"),
        tm_scope: Some("source.cj"),
        language_id: Some(581895317),
        codemirror_mode: Some("swift"),
        codemirror_mime_type: Some("text/x-swift"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cap'n Proto",
        language_type: "programming",
        color: Some("#c42727"),
        extensions: &[
            ".capnp",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.capnp"),
        language_id: Some(52),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Carbon",
        language_type: "programming",
        color: Some("#222222"),
        extensions: &[
            ".carbon",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("golang"),
        tm_scope: Some("source.v"),
        language_id: Some(55627273),
        codemirror_mode: Some("go"),
        codemirror_mime_type: Some("text/x-go"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CartoCSS",
        language_type: "programming",
        color: None,
        extensions: &[
            ".mss",
        ],
        aliases: &[
            "Carto",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.css.mss"),
        language_id: Some(53),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ceylon",
        language_type: "programming",
        color: Some("#dfa535"),
        extensions: &[
            ".ceylon",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ceylon"),
        language_id: Some(54),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Chapel",
        language_type: "programming",
        color: Some("#8dc63f"),
        extensions: &[
            ".chpl",
        ],
        aliases: &[
            "chpl",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.chapel"),
        language_id: Some(55),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Charity",
        language_type: "programming",
        color: None,
        extensions: &[
            ".ch",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(56),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Checksums",
        language_type: "data",
        color: None,
        extensions: &[
            ".crc32",
            ".md2",
            ".md4",
            ".md5",
            ".sha1",
            ".sha2",
            ".sha224",
            ".sha256",
            ".sha256sum",
            ".sha3",
            ".sha384",
            ".sha512",
        ],
        aliases: &[
            "checksum",
            "hash",
            "hashes",
            "sum",
            "sums",
        ],
        interpreters: &[
        ],
        filenames: &[
            "MD5SUMS",
            "SHA1SUMS",
            "SHA256SUMS",
            "SHA256SUMS.txt",
            "SHA512SUMS",
            "checksums.txt",
            "cksums",
            "md5sum.txt",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.checksums"),
        language_id: Some(372063053),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ChucK",
        language_type: "programming",
        color: Some("#3f8000"),
        extensions: &[
            ".ck",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("java"),
        tm_scope: Some("source.java"),
        language_id: Some(57),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-java"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Circom",
        language_type: "programming",
        color: Some("#707575"),
        extensions: &[
            ".circom",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.circom"),
        language_id: Some(1042332086),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cirru",
        language_type: "programming",
        color: Some("#ccccff"),
        extensions: &[
            ".cirru",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("cirru"),
        tm_scope: Some("source.cirru"),
        language_id: Some(58),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Clarion",
        language_type: "programming",
        color: Some("#db901e"),
        extensions: &[
            ".clw",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.clarion"),
        language_id: Some(59),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Clarity",
        language_type: "programming",
        color: Some("#5546ff"),
        extensions: &[
            ".clar",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.clar"),
        language_id: Some(91493841),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Classic ASP",
        language_type: "programming",
        color: Some("#6a40fd"),
        extensions: &[
            ".asp",
        ],
        aliases: &[
            "asp",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.html.asp"),
        language_id: Some(8),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Clean",
        language_type: "programming",
        color: Some("#3F85AF"),
        extensions: &[
            ".icl",
            ".dcl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.clean"),
        language_id: Some(60),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Click",
        language_type: "programming",
        color: Some("#E4E6F3"),
        extensions: &[
            ".click",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.click"),
        language_id: Some(61),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Clojure",
        language_type: "programming",
        color: Some("#db5855"),
        extensions: &[
            ".clj",
            ".bb",
            ".boot",
            ".cl2",
            ".cljc",
            ".cljs",
            ".cljs.hl",
            ".cljscm",
            ".cljx",
            ".hic",
        ],
        aliases: &[
        ],
        interpreters: &[
            "bb",
        ],
        filenames: &[
            "riemann.config",
        ],
        ace_mode: Some("clojure"),
        tm_scope: Some("source.clojure"),
        language_id: Some(62),
        codemirror_mode: Some("clojure"),
        codemirror_mime_type: Some("text/x-clojure"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Closure Templates",
        language_type: "markup",
        color: Some("#0d948f"),
        extensions: &[
            ".soy",
        ],
        aliases: &[
            "soy",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("soy_template"),
        tm_scope: Some("text.html.soy"),
        language_id: Some(357046146),
        codemirror_mode: Some("soy"),
        codemirror_mime_type: Some("text/x-soy"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cloud Firestore Security Rules",
        language_type: "data",
        color: Some("#FFA000"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "firestore.rules",
        ],
        ace_mode: Some("less"),
        tm_scope: Some("source.firestore"),
        language_id: Some(407996372),
        codemirror_mode: Some("css"),
        codemirror_mime_type: Some("text/css"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Clue",
        language_type: "programming",
        color: Some("#0009b5"),
        extensions: &[
            ".clue",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.clue"),
        language_id: Some(163763508),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CoNLL-U",
        language_type: "data",
        color: None,
        extensions: &[
            ".conllu",
            ".conll",
        ],
        aliases: &[
            "CoNLL",
            "CoNLL-X",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.conllu"),
        language_id: Some(421026389),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CodeQL",
        language_type: "programming",
        color: Some("#140f46"),
        extensions: &[
            ".ql",
            ".qll",
        ],
        aliases: &[
            "ql",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ql"),
        language_id: Some(424259634),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "CoffeeScript",
        language_type: "programming",
        color: Some("#244776"),
        extensions: &[
            ".coffee",
            "._coffee",
            ".cake",
            ".cjsx",
            ".iced",
        ],
        aliases: &[
            "coffee",
            "coffee-script",
        ],
        interpreters: &[
            "coffee",
        ],
        filenames: &[
            "Cakefile",
        ],
        ace_mode: Some("coffee"),
        tm_scope: Some("source.coffee"),
        language_id: Some(63),
        codemirror_mode: Some("coffeescript"),
        codemirror_mime_type: Some("text/x-coffeescript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ColdFusion",
        language_type: "programming",
        color: Some("#ed2cd6"),
        extensions: &[
            ".cfm",
            ".cfml",
        ],
        aliases: &[
            "cfm",
            "cfml",
            "coldfusion html",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("coldfusion"),
        tm_scope: Some("text.html.cfm"),
        language_id: Some(64),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ColdFusion CFC",
        language_type: "programming",
        color: Some("#ed2cd6"),
        extensions: &[
            ".cfc",
        ],
        aliases: &[
            "cfc",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("coldfusion"),
        tm_scope: Some("source.cfscript"),
        language_id: Some(65),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("ColdFusion"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Common Lisp",
        language_type: "programming",
        color: Some("#3fb68b"),
        extensions: &[
            ".lisp",
            ".asd",
            ".cl",
            ".l",
            ".lsp",
            ".ny",
            ".podsl",
            ".sexp",
        ],
        aliases: &[
            "lisp",
        ],
        interpreters: &[
            "lisp",
            "sbcl",
            "ccl",
            "clisp",
            "ecl",
        ],
        filenames: &[
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.commonlisp"),
        language_id: Some(66),
        codemirror_mode: Some("commonlisp"),
        codemirror_mime_type: Some("text/x-common-lisp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Common Workflow Language",
        language_type: "programming",
        color: Some("#B5314C"),
        extensions: &[
            ".cwl",
        ],
        aliases: &[
            "cwl",
        ],
        interpreters: &[
            "cwl-runner",
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.cwl"),
        language_id: Some(988547172),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Component Pascal",
        language_type: "programming",
        color: Some("#B0CE4E"),
        extensions: &[
            ".cp",
            ".cps",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("pascal"),
        tm_scope: Some("source.pascal"),
        language_id: Some(67),
        codemirror_mode: Some("pascal"),
        codemirror_mime_type: Some("text/x-pascal"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cooklang",
        language_type: "markup",
        color: Some("#E15A29"),
        extensions: &[
            ".cook",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cooklang"),
        language_id: Some(788037493),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cool",
        language_type: "programming",
        color: None,
        extensions: &[
            ".cl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cool"),
        language_id: Some(68),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cpp-ObjDump",
        language_type: "data",
        color: None,
        extensions: &[
            ".cppobjdump",
            ".c++-objdump",
            ".c++objdump",
            ".cpp-objdump",
            ".cxx-objdump",
        ],
        aliases: &[
            "c++-objdump",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("assembly_x86"),
        tm_scope: Some("objdump.x86asm"),
        language_id: Some(70),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Creole",
        language_type: "prose",
        color: None,
        extensions: &[
            ".creole",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.html.creole"),
        language_id: Some(71),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Crystal",
        language_type: "programming",
        color: Some("#000100"),
        extensions: &[
            ".cr",
        ],
        aliases: &[
        ],
        interpreters: &[
            "crystal",
        ],
        filenames: &[
        ],
        ace_mode: Some("crystal"),
        tm_scope: Some("source.crystal"),
        language_id: Some(72),
        codemirror_mode: Some("crystal"),
        codemirror_mime_type: Some("text/x-crystal"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Csound",
        language_type: "programming",
        color: Some("#1a1a1a"),
        extensions: &[
            ".orc",
            ".udo",
        ],
        aliases: &[
            "csound-orc",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("csound_orchestra"),
        tm_scope: Some("source.csound"),
        language_id: Some(73),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Csound Document",
        language_type: "programming",
        color: Some("#1a1a1a"),
        extensions: &[
            ".csd",
        ],
        aliases: &[
            "csound-csd",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("csound_document"),
        tm_scope: Some("source.csound-document"),
        language_id: Some(74),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Csound Score",
        language_type: "programming",
        color: Some("#1a1a1a"),
        extensions: &[
            ".sco",
        ],
        aliases: &[
            "csound-sco",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("csound_score"),
        tm_scope: Some("source.csound-score"),
        language_id: Some(75),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cuda",
        language_type: "programming",
        color: Some("#3A4E3A"),
        extensions: &[
            ".cu",
            ".cuh",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.cuda-c++"),
        language_id: Some(77),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-c++src"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cue Sheet",
        language_type: "data",
        color: None,
        extensions: &[
            ".cue",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cuesheet"),
        language_id: Some(942714150),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Curry",
        language_type: "programming",
        color: Some("#531242"),
        extensions: &[
            ".curry",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("haskell"),
        tm_scope: Some("source.curry"),
        language_id: Some(439829048),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cycript",
        language_type: "programming",
        color: None,
        extensions: &[
            ".cy",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("javascript"),
        tm_scope: Some("source.js"),
        language_id: Some(78),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("text/javascript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cylc",
        language_type: "data",
        color: Some("#00b3fd"),
        extensions: &[
            ".cylc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "suite.rc",
        ],
        ace_mode: Some("ini"),
        tm_scope: Some("source.cylc"),
        language_id: Some(476447814),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("INI"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cypher",
        language_type: "programming",
        color: Some("#34c0eb"),
        extensions: &[
            ".cyp",
            ".cypher",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cypher"),
        language_id: Some(850806976),
        codemirror_mode: Some("cypher"),
        codemirror_mime_type: Some("application/x-cypher-query"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Cython",
        language_type: "programming",
        color: Some("#fedf5b"),
        extensions: &[
            ".pyx",
            ".pxd",
            ".pxi",
        ],
        aliases: &[
            "pyrex",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.cython"),
        language_id: Some(79),
        codemirror_mode: Some("python"),
        codemirror_mime_type: Some("text/x-cython"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "D",
        language_type: "programming",
        color: Some("#ba595e"),
        extensions: &[
            ".d",
            ".di",
        ],
        aliases: &[
            "Dlang",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("d"),
        tm_scope: Some("source.d"),
        language_id: Some(80),
        codemirror_mode: Some("d"),
        codemirror_mime_type: Some("text/x-d"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "D-ObjDump",
        language_type: "data",
        color: None,
        extensions: &[
            ".d-objdump",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("assembly_x86"),
        tm_scope: Some("objdump.x86asm"),
        language_id: Some(81),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "D2",
        language_type: "markup",
        color: Some("#526ee8"),
        extensions: &[
            ".d2",
        ],
        aliases: &[
            "d2lang",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.d2"),
        language_id: Some(37531557),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "DIGITAL Command Language",
        language_type: "programming",
        color: None,
        extensions: &[
            ".com",
        ],
        aliases: &[
            "dcl",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(82),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "DM",
        language_type: "programming",
        color: Some("#447265"),
        extensions: &[
            ".dm",
        ],
        aliases: &[
            "byond",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.dm"),
        language_id: Some(83),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "DNS Zone",
        language_type: "data",
        color: None,
        extensions: &[
            ".zone",
            ".arpa",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.zone_file"),
        language_id: Some(84),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "DTrace",
        language_type: "programming",
        color: None,
        extensions: &[
            ".d",
        ],
        aliases: &[
            "dtrace-script",
        ],
        interpreters: &[
            "dtrace",
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c"),
        language_id: Some(85),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Dafny",
        language_type: "programming",
        color: Some("#FFEC25"),
        extensions: &[
            ".dfy",
        ],
        aliases: &[
        ],
        interpreters: &[
            "dafny",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.dfy.dafny"),
        language_id: Some(969323346),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Darcs Patch",
        language_type: "data",
        color: Some("#8eff23"),
        extensions: &[
            ".darcspatch",
            ".dpatch",
        ],
        aliases: &[
            "dpatch",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(86),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Dart",
        language_type: "programming",
        color: Some("#00B4AB"),
        extensions: &[
            ".dart",
        ],
        aliases: &[
        ],
        interpreters: &[
            "dart",
        ],
        filenames: &[
        ],
        ace_mode: Some("dart"),
        tm_scope: Some("source.dart"),
        language_id: Some(87),
        codemirror_mode: Some("dart"),
        codemirror_mime_type: Some("application/dart"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Daslang",
        language_type: "programming",
        color: Some("#d3d3d3"),
        extensions: &[
            ".das",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.daslang"),
        language_id: Some(648759486),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "DataWeave",
        language_type: "programming",
        color: Some("#003a52"),
        extensions: &[
            ".dwl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.data-weave"),
        language_id: Some(974514097),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Debian Package Control File",
        language_type: "data",
        color: Some("#D70751"),
        extensions: &[
            ".dsc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.deb-control"),
        language_id: Some(527438264),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "DenizenScript",
        language_type: "programming",
        color: Some("#FBEE96"),
        extensions: &[
            ".dsc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.denizenscript"),
        language_id: Some(435000929),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Dhall",
        language_type: "programming",
        color: Some("#dfafff"),
        extensions: &[
            ".dhall",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("haskell"),
        tm_scope: Some("source.haskell"),
        language_id: Some(793969321),
        codemirror_mode: Some("haskell"),
        codemirror_mime_type: Some("text/x-haskell"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Diff",
        language_type: "data",
        color: None,
        extensions: &[
            ".diff",
            ".patch",
        ],
        aliases: &[
            "udiff",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("diff"),
        tm_scope: Some("source.diff"),
        language_id: Some(88),
        codemirror_mode: Some("diff"),
        codemirror_mime_type: Some("text/x-diff"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "DirectX 3D File",
        language_type: "data",
        color: Some("#aace60"),
        extensions: &[
            ".x",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(201049282),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Dockerfile",
        language_type: "programming",
        color: Some("#384d54"),
        extensions: &[
            ".dockerfile",
            ".containerfile",
        ],
        aliases: &[
            "Containerfile",
        ],
        interpreters: &[
        ],
        filenames: &[
            "Containerfile",
            "Dockerfile",
        ],
        ace_mode: Some("dockerfile"),
        tm_scope: Some("source.dockerfile"),
        language_id: Some(89),
        codemirror_mode: Some("dockerfile"),
        codemirror_mime_type: Some("text/x-dockerfile"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Dogescript",
        language_type: "programming",
        color: Some("#cca760"),
        extensions: &[
            ".djs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(90),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Dotenv",
        language_type: "data",
        color: Some("#e5d559"),
        extensions: &[
            ".env",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            ".env",
            ".env.ci",
            ".env.dev",
            ".env.development",
            ".env.development.local",
            ".env.example",
            ".env.local",
            ".env.prod",
            ".env.production",
            ".env.sample",
            ".env.staging",
            ".env.template",
            ".env.test",
            ".env.testing",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.dotenv"),
        language_id: Some(111148035),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Dune",
        language_type: "programming",
        color: Some("#89421e"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "dune-project",
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.dune"),
        language_id: Some(754574151),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Dylan",
        language_type: "programming",
        color: Some("#6c616e"),
        extensions: &[
            ".dylan",
            ".dyl",
            ".intr",
            ".lid",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.dylan"),
        language_id: Some(91),
        codemirror_mode: Some("dylan"),
        codemirror_mime_type: Some("text/x-dylan"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "E",
        language_type: "programming",
        color: Some("#ccce35"),
        extensions: &[
            ".e",
        ],
        aliases: &[
        ],
        interpreters: &[
            "rune",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(92),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "E-mail",
        language_type: "data",
        color: None,
        extensions: &[
            ".eml",
            ".mbox",
        ],
        aliases: &[
            "email",
            "eml",
            "mail",
            "mbox",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.eml.basic"),
        language_id: Some(529653389),
        codemirror_mode: Some("mbox"),
        codemirror_mime_type: Some("application/mbox"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "EBNF",
        language_type: "data",
        color: None,
        extensions: &[
            ".ebnf",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ebnf"),
        language_id: Some(430),
        codemirror_mode: Some("ebnf"),
        codemirror_mime_type: Some("text/x-ebnf"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ECL",
        language_type: "programming",
        color: Some("#8a1267"),
        extensions: &[
            ".ecl",
            ".eclxml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ecl"),
        language_id: Some(93),
        codemirror_mode: Some("ecl"),
        codemirror_mime_type: Some("text/x-ecl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ECLiPSe",
        language_type: "programming",
        color: Some("#001d9d"),
        extensions: &[
            ".ecl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("prolog"),
        tm_scope: Some("source.prolog.eclipse"),
        language_id: Some(94),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Prolog"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "EJS",
        language_type: "markup",
        color: Some("#a91e50"),
        extensions: &[
            ".ejs",
            ".ect",
            ".ejs.t",
            ".jst",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ejs"),
        tm_scope: Some("text.html.js"),
        language_id: Some(95),
        codemirror_mode: Some("htmlembedded"),
        codemirror_mime_type: Some("application/x-ejs"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "EQ",
        language_type: "programming",
        color: Some("#a78649"),
        extensions: &[
            ".eq",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("csharp"),
        tm_scope: Some("source.cs"),
        language_id: Some(96),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csharp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Eagle",
        language_type: "data",
        color: None,
        extensions: &[
            ".sch",
            ".brd",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml"),
        language_id: Some(97),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Earthly",
        language_type: "programming",
        color: Some("#2af0ff"),
        extensions: &[
        ],
        aliases: &[
            "Earthfile",
        ],
        interpreters: &[
        ],
        filenames: &[
            "Earthfile",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.earthfile"),
        language_id: Some(963512632),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Easybuild",
        language_type: "data",
        color: Some("#069406"),
        extensions: &[
            ".eb",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("python"),
        tm_scope: Some("source.python"),
        language_id: Some(342840477),
        codemirror_mode: Some("python"),
        codemirror_mime_type: Some("text/x-python"),
        group: Some("Python"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ecere Projects",
        language_type: "data",
        color: Some("#913960"),
        extensions: &[
            ".epj",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("json"),
        tm_scope: Some("source.json"),
        language_id: Some(98),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/json"),
        group: Some("JavaScript"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ecmarkup",
        language_type: "markup",
        color: Some("#eb8131"),
        extensions: &[
            ".html",
        ],
        aliases: &[
            "ecmarkdown",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html"),
        tm_scope: Some("text.html.ecmarkup"),
        language_id: Some(844766630),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: Some("HTML"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Edge",
        language_type: "markup",
        color: Some("#0dffe0"),
        extensions: &[
            ".edge",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html"),
        tm_scope: Some("text.html.edge"),
        language_id: Some(460509620),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "EdgeQL",
        language_type: "programming",
        color: Some("#31A7FF"),
        extensions: &[
            ".edgeql",
            ".esdl",
        ],
        aliases: &[
            "esdl",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.edgeql"),
        language_id: Some(925235833),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "EditorConfig",
        language_type: "data",
        color: Some("#fff1f2"),
        extensions: &[
            ".editorconfig",
        ],
        aliases: &[
            "editor-config",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".editorconfig",
        ],
        ace_mode: Some("ini"),
        tm_scope: Some("source.editorconfig"),
        language_id: Some(96139566),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: Some("INI"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Edje Data Collection",
        language_type: "data",
        color: None,
        extensions: &[
            ".edc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c++"),
        language_id: Some(342840478),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-c++src"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Eiffel",
        language_type: "programming",
        color: Some("#4d6977"),
        extensions: &[
            ".e",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("eiffel"),
        tm_scope: Some("source.eiffel"),
        language_id: Some(99),
        codemirror_mode: Some("eiffel"),
        codemirror_mime_type: Some("text/x-eiffel"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Elixir",
        language_type: "programming",
        color: Some("#6e4a7e"),
        extensions: &[
            ".ex",
            ".exs",
        ],
        aliases: &[
        ],
        interpreters: &[
            "elixir",
        ],
        filenames: &[
            "mix.lock",
        ],
        ace_mode: Some("elixir"),
        tm_scope: Some("source.elixir"),
        language_id: Some(100),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Elm",
        language_type: "programming",
        color: Some("#60B5CC"),
        extensions: &[
            ".elm",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("elm"),
        tm_scope: Some("source.elm"),
        language_id: Some(101),
        codemirror_mode: Some("elm"),
        codemirror_mime_type: Some("text/x-elm"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Elvish",
        language_type: "programming",
        color: Some("#55BB55"),
        extensions: &[
            ".elv",
        ],
        aliases: &[
        ],
        interpreters: &[
            "elvish",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.elvish"),
        language_id: Some(570996448),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Elvish Transcript",
        language_type: "programming",
        color: Some("#55BB55"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.elvish-transcript"),
        language_id: Some(452025714),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Elvish"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Emacs Lisp",
        language_type: "programming",
        color: Some("#c065db"),
        extensions: &[
            ".el",
            ".emacs",
            ".emacs.desktop",
        ],
        aliases: &[
            "elisp",
            "emacs",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".abbrev_defs",
            ".emacs",
            ".emacs.desktop",
            ".gnus",
            ".spacemacs",
            ".viper",
            "Cask",
            "Project.ede",
            "_emacs",
            "abbrev_defs",
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.emacs.lisp"),
        language_id: Some(102),
        codemirror_mode: Some("commonlisp"),
        codemirror_mime_type: Some("text/x-common-lisp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "EmberScript",
        language_type: "programming",
        color: Some("#FFF4F3"),
        extensions: &[
            ".em",
            ".emberscript",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("coffee"),
        tm_scope: Some("source.coffee"),
        language_id: Some(103),
        codemirror_mode: Some("coffeescript"),
        codemirror_mime_type: Some("text/x-coffeescript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Erlang",
        language_type: "programming",
        color: Some("#B83998"),
        extensions: &[
            ".erl",
            ".app",
            ".app.src",
            ".es",
            ".escript",
            ".hrl",
            ".xrl",
            ".yrl",
        ],
        aliases: &[
        ],
        interpreters: &[
            "escript",
        ],
        filenames: &[
            "Emakefile",
            "rebar.config",
            "rebar.config.lock",
            "rebar.lock",
        ],
        ace_mode: Some("erlang"),
        tm_scope: Some("source.erlang"),
        language_id: Some(104),
        codemirror_mode: Some("erlang"),
        codemirror_mime_type: Some("text/x-erlang"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Euphoria",
        language_type: "programming",
        color: Some("#FF790B"),
        extensions: &[
            ".e",
            ".ex",
        ],
        aliases: &[
        ],
        interpreters: &[
            "eui",
            "euiw",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.euphoria"),
        language_id: Some(880693982),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "F#",
        language_type: "programming",
        color: Some("#b845fc"),
        extensions: &[
            ".fs",
            ".fsi",
            ".fsx",
        ],
        aliases: &[
            "fsharp",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("fsharp"),
        tm_scope: Some("source.fsharp"),
        language_id: Some(105),
        codemirror_mode: Some("mllike"),
        codemirror_mime_type: Some("text/x-fsharp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "F*",
        language_type: "programming",
        color: Some("#572e30"),
        extensions: &[
            ".fst",
            ".fsti",
        ],
        aliases: &[
            "fstar",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.fstar"),
        language_id: Some(336943375),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: Some("Fstar"),
    },
    LanguageMetadata {
        name: "FIGlet Font",
        language_type: "data",
        color: Some("#FFDDBB"),
        extensions: &[
            ".flf",
        ],
        aliases: &[
            "FIGfont",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.figfont"),
        language_id: Some(686129783),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "FIRRTL",
        language_type: "programming",
        color: Some("#2f632f"),
        extensions: &[
            ".fir",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.firrtl"),
        language_id: Some(906694254),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "FLUX",
        language_type: "programming",
        color: Some("#88ccff"),
        extensions: &[
            ".fx",
            ".flux",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(106),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Factor",
        language_type: "programming",
        color: Some("#636746"),
        extensions: &[
            ".factor",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            ".factor-boot-rc",
            ".factor-rc",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.factor"),
        language_id: Some(108),
        codemirror_mode: Some("factor"),
        codemirror_mime_type: Some("text/x-factor"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Fancy",
        language_type: "programming",
        color: Some("#7b9db4"),
        extensions: &[
            ".fy",
            ".fancypack",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "Fakefile",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.fancy"),
        language_id: Some(109),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Fantom",
        language_type: "programming",
        color: Some("#14253c"),
        extensions: &[
            ".fan",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.fan"),
        language_id: Some(110),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Faust",
        language_type: "programming",
        color: Some("#c37240"),
        extensions: &[
            ".dsp",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.faust"),
        language_id: Some(622529198),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Fennel",
        language_type: "programming",
        color: Some("#fff3d7"),
        extensions: &[
            ".fnl",
        ],
        aliases: &[
        ],
        interpreters: &[
            "fennel",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.fnl"),
        language_id: Some(239946126),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Filebench WML",
        language_type: "programming",
        color: Some("#F6B900"),
        extensions: &[
            ".f",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(111),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Filterscript",
        language_type: "programming",
        color: None,
        extensions: &[
            ".fs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(112),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("RenderScript"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Flix",
        language_type: "programming",
        color: Some("#d44a45"),
        extensions: &[
            ".flix",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("flix"),
        tm_scope: Some("source.flix"),
        language_id: Some(800935960),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Fluent",
        language_type: "programming",
        color: Some("#ffcc33"),
        extensions: &[
            ".ftl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ftl"),
        language_id: Some(206353404),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Formatted",
        language_type: "data",
        color: None,
        extensions: &[
            ".for",
            ".eam.fs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(113),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Forth",
        language_type: "programming",
        color: Some("#341708"),
        extensions: &[
            ".fth",
            ".4th",
            ".f",
            ".for",
            ".forth",
            ".fr",
            ".frt",
            ".fs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("forth"),
        tm_scope: Some("source.forth"),
        language_id: Some(114),
        codemirror_mode: Some("forth"),
        codemirror_mime_type: Some("text/x-forth"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Fortran",
        language_type: "programming",
        color: Some("#4d41b1"),
        extensions: &[
            ".f",
            ".f77",
            ".for",
            ".fpp",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("fortran"),
        tm_scope: Some("source.fortran"),
        language_id: Some(107),
        codemirror_mode: Some("fortran"),
        codemirror_mime_type: Some("text/x-fortran"),
        group: Some("Fortran"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Fortran Free Form",
        language_type: "programming",
        color: Some("#4d41b1"),
        extensions: &[
            ".f90",
            ".f03",
            ".f08",
            ".f95",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("fortran"),
        tm_scope: Some("source.fortran.modern"),
        language_id: Some(761352333),
        codemirror_mode: Some("fortran"),
        codemirror_mime_type: Some("text/x-fortran"),
        group: Some("Fortran"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "FreeBASIC",
        language_type: "programming",
        color: Some("#141AC9"),
        extensions: &[
            ".bi",
            ".bas",
        ],
        aliases: &[
            "fb",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.vbnet"),
        language_id: Some(472896659),
        codemirror_mode: Some("vb"),
        codemirror_mime_type: Some("text/x-vb"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "FreeMarker",
        language_type: "programming",
        color: Some("#0050b2"),
        extensions: &[
            ".ftl",
            ".ftlh",
        ],
        aliases: &[
            "ftl",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ftl"),
        tm_scope: Some("text.html.ftl"),
        language_id: Some(115),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Frege",
        language_type: "programming",
        color: Some("#00cafe"),
        extensions: &[
            ".fr",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("haskell"),
        tm_scope: Some("source.haskell"),
        language_id: Some(116),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Futhark",
        language_type: "programming",
        color: Some("#5f021f"),
        extensions: &[
            ".fut",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.futhark"),
        language_id: Some(97358117),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "G-code",
        language_type: "programming",
        color: Some("#D08CF2"),
        extensions: &[
            ".g",
            ".cnc",
            ".gco",
            ".gcode",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("gcode"),
        tm_scope: Some("source.gcode"),
        language_id: Some(117),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GAML",
        language_type: "programming",
        color: Some("#FFC766"),
        extensions: &[
            ".gaml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(290345951),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GAMS",
        language_type: "programming",
        color: Some("#f49a22"),
        extensions: &[
            ".gms",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(118),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GAP",
        language_type: "programming",
        color: Some("#0000cc"),
        extensions: &[
            ".g",
            ".gap",
            ".gd",
            ".gi",
            ".tst",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gap"),
        language_id: Some(119),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GCC Machine Description",
        language_type: "programming",
        color: Some("#FFCFAB"),
        extensions: &[
            ".md",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.lisp"),
        language_id: Some(121),
        codemirror_mode: Some("commonlisp"),
        codemirror_mime_type: Some("text/x-common-lisp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GDB",
        language_type: "programming",
        color: None,
        extensions: &[
            ".gdb",
            ".gdbinit",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gdb"),
        language_id: Some(122),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GDScript",
        language_type: "programming",
        color: Some("#355570"),
        extensions: &[
            ".gd",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gdscript"),
        language_id: Some(123),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GDShader",
        language_type: "programming",
        color: Some("#478CBF"),
        extensions: &[
            ".gdshader",
            ".gdshaderinc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("glsl"),
        tm_scope: Some("source.gdshader"),
        language_id: Some(694638086),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GEDCOM",
        language_type: "data",
        color: Some("#003058"),
        extensions: &[
            ".ged",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gedcom"),
        language_id: Some(459577965),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GLSL",
        language_type: "programming",
        color: Some("#5686a5"),
        extensions: &[
            ".glsl",
            ".fp",
            ".frag",
            ".frg",
            ".fs",
            ".fsh",
            ".fshader",
            ".geo",
            ".geom",
            ".glslf",
            ".glslv",
            ".gs",
            ".gshader",
            ".rchit",
            ".rmiss",
            ".shader",
            ".tesc",
            ".tese",
            ".vert",
            ".vrx",
            ".vs",
            ".vsh",
            ".vshader",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("glsl"),
        tm_scope: Some("source.glsl"),
        language_id: Some(124),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GN",
        language_type: "data",
        color: None,
        extensions: &[
            ".gn",
            ".gni",
        ],
        aliases: &[
        ],
        interpreters: &[
            "gn",
        ],
        filenames: &[
            ".gn",
        ],
        ace_mode: Some("python"),
        tm_scope: Some("source.gn"),
        language_id: Some(302957008),
        codemirror_mode: Some("python"),
        codemirror_mime_type: Some("text/x-python"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GSC",
        language_type: "programming",
        color: Some("#FF6800"),
        extensions: &[
            ".gsc",
            ".csc",
            ".gsh",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.gsc"),
        language_id: Some(257856279),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Game Maker Language",
        language_type: "programming",
        color: Some("#71b417"),
        extensions: &[
            ".gml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c++"),
        language_id: Some(125),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-c++src"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gemfile.lock",
        language_type: "data",
        color: Some("#701516"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "Gemfile.lock",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gemfile-lock"),
        language_id: Some(907065713),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gemini",
        language_type: "prose",
        color: Some("#ff6900"),
        extensions: &[
            ".gmi",
        ],
        aliases: &[
            "gemtext",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gemini"),
        language_id: Some(310828396),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Genero 4gl",
        language_type: "programming",
        color: Some("#63408e"),
        extensions: &[
            ".4gl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.genero-4gl"),
        language_id: Some(986054050),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Genero per",
        language_type: "markup",
        color: Some("#d8df39"),
        extensions: &[
            ".per",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.genero-per"),
        language_id: Some(902995658),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Genie",
        language_type: "programming",
        color: Some("#fb855d"),
        extensions: &[
            ".gs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(792408528),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Genshi",
        language_type: "programming",
        color: Some("#951531"),
        extensions: &[
            ".kid",
        ],
        aliases: &[
            "xml+genshi",
            "xml+kid",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml.genshi"),
        language_id: Some(126),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gentoo Ebuild",
        language_type: "programming",
        color: Some("#9400ff"),
        extensions: &[
            ".ebuild",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("sh"),
        tm_scope: Some("source.shell"),
        language_id: Some(127),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: Some("Shell"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gentoo Eclass",
        language_type: "programming",
        color: Some("#9400ff"),
        extensions: &[
            ".eclass",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("sh"),
        tm_scope: Some("source.shell"),
        language_id: Some(128),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: Some("Shell"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gerber Image",
        language_type: "data",
        color: Some("#d20b00"),
        extensions: &[
            ".gbr",
            ".cmp",
            ".gbl",
            ".gbo",
            ".gbp",
            ".gbs",
            ".gko",
            ".gml",
            ".gpb",
            ".gpt",
            ".gtl",
            ".gto",
            ".gtp",
            ".gts",
            ".ncl",
            ".sol",
        ],
        aliases: &[
            "rs-274x",
        ],
        interpreters: &[
            "gerbv",
            "gerbview",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gerber"),
        language_id: Some(404627610),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gettext Catalog",
        language_type: "prose",
        color: None,
        extensions: &[
            ".po",
            ".pot",
        ],
        aliases: &[
            "pot",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.po"),
        language_id: Some(129),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gherkin",
        language_type: "programming",
        color: Some("#5B2063"),
        extensions: &[
            ".feature",
            ".story",
        ],
        aliases: &[
            "cucumber",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("gherkin"),
        tm_scope: Some("text.gherkin.feature"),
        language_id: Some(76),
        codemirror_mode: Some("gherkin"),
        codemirror_mime_type: Some("text/x-feature"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Git Attributes",
        language_type: "data",
        color: Some("#F44D27"),
        extensions: &[
        ],
        aliases: &[
            "gitattributes",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".gitattributes",
        ],
        ace_mode: Some("gitignore"),
        tm_scope: Some("source.gitattributes"),
        language_id: Some(956324166),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Git Commit",
        language_type: "data",
        color: Some("#F44D27"),
        extensions: &[
        ],
        aliases: &[
            "commit",
        ],
        interpreters: &[
        ],
        filenames: &[
            "COMMIT_EDITMSG",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.git-commit"),
        language_id: Some(131750475),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Git Config",
        language_type: "data",
        color: Some("#F44D27"),
        extensions: &[
            ".gitconfig",
        ],
        aliases: &[
            "gitconfig",
            "gitmodules",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".gitconfig",
            ".gitmodules",
        ],
        ace_mode: Some("ini"),
        tm_scope: Some("source.gitconfig"),
        language_id: Some(807968997),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: Some("INI"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Git Revision List",
        language_type: "data",
        color: Some("#F44D27"),
        extensions: &[
        ],
        aliases: &[
            "Git Blame Ignore Revs",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".git-blame-ignore-revs",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.git-revlist"),
        language_id: Some(461881235),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gleam",
        language_type: "programming",
        color: Some("#ffaff3"),
        extensions: &[
            ".gleam",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gleam"),
        language_id: Some(1054258749),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Glimmer JS",
        language_type: "programming",
        color: Some("#F5835F"),
        extensions: &[
            ".gjs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("javascript"),
        tm_scope: Some("source.gjs"),
        language_id: Some(5523150),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("JavaScript"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Glimmer TS",
        language_type: "programming",
        color: Some("#3178c6"),
        extensions: &[
            ".gts",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("typescript"),
        tm_scope: Some("source.gts"),
        language_id: Some(95110458),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("TypeScript"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Glyph",
        language_type: "programming",
        color: Some("#c1ac7f"),
        extensions: &[
            ".glf",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("tcl"),
        tm_scope: Some("source.tcl"),
        language_id: Some(130),
        codemirror_mode: Some("tcl"),
        codemirror_mime_type: Some("text/x-tcl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Glyph Bitmap Distribution Format",
        language_type: "data",
        color: None,
        extensions: &[
            ".bdf",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.bdf"),
        language_id: Some(997665271),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gnuplot",
        language_type: "programming",
        color: Some("#f0a9f0"),
        extensions: &[
            ".gp",
            ".gnu",
            ".gnuplot",
            ".p",
            ".plot",
            ".plt",
        ],
        aliases: &[
        ],
        interpreters: &[
            "gnuplot",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gnuplot"),
        language_id: Some(131),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Go",
        language_type: "programming",
        color: Some("#00ADD8"),
        extensions: &[
            ".go",
        ],
        aliases: &[
            "golang",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("golang"),
        tm_scope: Some("source.go"),
        language_id: Some(132),
        codemirror_mode: Some("go"),
        codemirror_mime_type: Some("text/x-go"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Go Checksums",
        language_type: "data",
        color: Some("#00ADD8"),
        extensions: &[
        ],
        aliases: &[
            "go.sum",
            "go sum",
            "go.work.sum",
            "go work sum",
        ],
        interpreters: &[
        ],
        filenames: &[
            "go.sum",
            "go.work.sum",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("go.sum"),
        language_id: Some(1054391671),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Go Module",
        language_type: "data",
        color: Some("#00ADD8"),
        extensions: &[
        ],
        aliases: &[
            "go.mod",
            "go mod",
        ],
        interpreters: &[
        ],
        filenames: &[
            "go.mod",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("go.mod"),
        language_id: Some(947461016),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Go Workspace",
        language_type: "data",
        color: Some("#00ADD8"),
        extensions: &[
        ],
        aliases: &[
            "go.work",
            "go work",
        ],
        interpreters: &[
        ],
        filenames: &[
            "go.work",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("go.mod"),
        language_id: Some(934546256),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Godot Resource",
        language_type: "data",
        color: Some("#355570"),
        extensions: &[
            ".gdnlib",
            ".gdns",
            ".tres",
            ".tscn",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "project.godot",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gdresource"),
        language_id: Some(738107771),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Golo",
        language_type: "programming",
        color: Some("#88562A"),
        extensions: &[
            ".golo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.golo"),
        language_id: Some(133),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gosu",
        language_type: "programming",
        color: Some("#82937f"),
        extensions: &[
            ".gs",
            ".gst",
            ".gsx",
            ".vark",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.gosu.2"),
        language_id: Some(134),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Grace",
        language_type: "programming",
        color: Some("#615f8b"),
        extensions: &[
            ".grace",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.grace"),
        language_id: Some(135),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gradle",
        language_type: "data",
        color: Some("#02303a"),
        extensions: &[
            ".gradle",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.groovy.gradle"),
        language_id: Some(136),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Gradle Kotlin DSL",
        language_type: "data",
        color: Some("#02303a"),
        extensions: &[
            ".gradle.kts",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.kotlin"),
        language_id: Some(432600901),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Gradle"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Grammatical Framework",
        language_type: "programming",
        color: Some("#ff0000"),
        extensions: &[
            ".gf",
        ],
        aliases: &[
            "gf",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("haskell"),
        tm_scope: Some("source.gf"),
        language_id: Some(137),
        codemirror_mode: Some("haskell"),
        codemirror_mime_type: Some("text/x-haskell"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Graph Modeling Language",
        language_type: "data",
        color: None,
        extensions: &[
            ".gml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(138),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "GraphQL",
        language_type: "data",
        color: Some("#e10098"),
        extensions: &[
            ".graphql",
            ".gql",
            ".graphqls",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("graphqlschema"),
        tm_scope: Some("source.graphql"),
        language_id: Some(139),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Graphviz (DOT)",
        language_type: "data",
        color: Some("#2596be"),
        extensions: &[
            ".dot",
            ".gv",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("dot"),
        tm_scope: Some("source.dot"),
        language_id: Some(140),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Groovy",
        language_type: "programming",
        color: Some("#4298b8"),
        extensions: &[
            ".groovy",
            ".grt",
            ".gtpl",
            ".gvy",
        ],
        aliases: &[
        ],
        interpreters: &[
            "groovy",
        ],
        filenames: &[
            "Jenkinsfile",
        ],
        ace_mode: Some("groovy"),
        tm_scope: Some("source.groovy"),
        language_id: Some(142),
        codemirror_mode: Some("groovy"),
        codemirror_mime_type: Some("text/x-groovy"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Groovy Server Pages",
        language_type: "programming",
        color: Some("#4298b8"),
        extensions: &[
            ".gsp",
        ],
        aliases: &[
            "gsp",
            "java server page",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("jsp"),
        tm_scope: Some("text.html.jsp"),
        language_id: Some(143),
        codemirror_mode: Some("htmlembedded"),
        codemirror_mime_type: Some("application/x-jsp"),
        group: Some("Groovy"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HAProxy",
        language_type: "data",
        color: Some("#106da9"),
        extensions: &[
            ".cfg",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "haproxy.cfg",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.haproxy-config"),
        language_id: Some(366607477),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HCL",
        language_type: "programming",
        color: Some("#844FBA"),
        extensions: &[
            ".hcl",
            ".nomad",
            ".tf",
            ".tfvars",
            ".workflow",
        ],
        aliases: &[
            "HashiCorp Configuration Language",
            "terraform",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("terraform"),
        tm_scope: Some("source.hcl"),
        language_id: Some(144),
        codemirror_mode: Some("ruby"),
        codemirror_mime_type: Some("text/x-ruby"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HIP",
        language_type: "programming",
        color: Some("#4F3A4F"),
        extensions: &[
            ".hip",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c++"),
        language_id: Some(674379998),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-c++src"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HLSL",
        language_type: "programming",
        color: Some("#aace60"),
        extensions: &[
            ".hlsl",
            ".cginc",
            ".fx",
            ".fxh",
            ".hlsli",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.hlsl"),
        language_id: Some(145),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HOCON",
        language_type: "data",
        color: Some("#9ff8ee"),
        extensions: &[
            ".hocon",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            ".scalafix.conf",
            ".scalafmt.conf",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.hocon"),
        language_id: Some(679725279),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HTML",
        language_type: "markup",
        color: Some("#e34c26"),
        extensions: &[
            ".html",
            ".hta",
            ".htm",
            ".html.hl",
            ".inc",
            ".xht",
            ".xhtml",
        ],
        aliases: &[
            "xhtml",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html"),
        tm_scope: Some("text.html.basic"),
        language_id: Some(146),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HTML+ECR",
        language_type: "markup",
        color: Some("#2e1052"),
        extensions: &[
            ".ecr",
        ],
        aliases: &[
            "ecr",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html_ruby"),
        tm_scope: Some("text.html.ecr"),
        language_id: Some(148),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: Some("HTML"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HTML+EEX",
        language_type: "markup",
        color: Some("#6e4a7e"),
        extensions: &[
            ".html.eex",
            ".heex",
            ".leex",
        ],
        aliases: &[
            "eex",
            "heex",
            "leex",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html_elixir"),
        tm_scope: Some("text.html.elixir"),
        language_id: Some(149),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: Some("HTML"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HTML+ERB",
        language_type: "markup",
        color: Some("#701516"),
        extensions: &[
            ".erb",
            ".erb.deface",
            ".rhtml",
        ],
        aliases: &[
            "erb",
            "rhtml",
            "html+ruby",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html_ruby"),
        tm_scope: Some("text.html.erb"),
        language_id: Some(150),
        codemirror_mode: Some("htmlembedded"),
        codemirror_mime_type: Some("application/x-erb"),
        group: Some("HTML"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HTML+PHP",
        language_type: "markup",
        color: Some("#4f5d95"),
        extensions: &[
            ".phtml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("php"),
        tm_scope: Some("text.html.php"),
        language_id: Some(151),
        codemirror_mode: Some("php"),
        codemirror_mime_type: Some("application/x-httpd-php"),
        group: Some("HTML"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HTML+Razor",
        language_type: "markup",
        color: Some("#512be4"),
        extensions: &[
            ".cshtml",
            ".razor",
        ],
        aliases: &[
            "razor",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("razor"),
        tm_scope: Some("text.html.cshtml"),
        language_id: Some(479039817),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: Some("HTML"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HTTP",
        language_type: "data",
        color: Some("#005C9C"),
        extensions: &[
            ".http",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.httpspec"),
        language_id: Some(152),
        codemirror_mode: Some("http"),
        codemirror_mime_type: Some("message/http"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HXML",
        language_type: "data",
        color: Some("#f68712"),
        extensions: &[
            ".hxml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.hxml"),
        language_id: Some(786683730),
        codemirror_mode: Some("haxe"),
        codemirror_mime_type: Some("text/x-hxml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Hack",
        language_type: "programming",
        color: Some("#878787"),
        extensions: &[
            ".hack",
            ".hh",
            ".hhi",
            ".php",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("php"),
        tm_scope: Some("source.hack"),
        language_id: Some(153),
        codemirror_mode: Some("php"),
        codemirror_mime_type: Some("application/x-httpd-php"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Haml",
        language_type: "markup",
        color: Some("#ece2a9"),
        extensions: &[
            ".haml",
            ".haml.deface",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("haml"),
        tm_scope: Some("text.haml"),
        language_id: Some(154),
        codemirror_mode: Some("haml"),
        codemirror_mime_type: Some("text/x-haml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Handlebars",
        language_type: "markup",
        color: Some("#f7931e"),
        extensions: &[
            ".handlebars",
            ".hbs",
        ],
        aliases: &[
            "hbs",
            "htmlbars",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("handlebars"),
        tm_scope: Some("text.html.handlebars"),
        language_id: Some(155),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Harbour",
        language_type: "programming",
        color: Some("#0e60e3"),
        extensions: &[
            ".hb",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.harbour"),
        language_id: Some(156),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Hare",
        language_type: "programming",
        color: Some("#9d7424"),
        extensions: &[
            ".ha",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(463518941),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Haskell",
        language_type: "programming",
        color: Some("#5e5086"),
        extensions: &[
            ".hs",
            ".hs-boot",
            ".hsc",
        ],
        aliases: &[
        ],
        interpreters: &[
            "runghc",
            "runhaskell",
            "runhugs",
        ],
        filenames: &[
        ],
        ace_mode: Some("haskell"),
        tm_scope: Some("source.haskell"),
        language_id: Some(157),
        codemirror_mode: Some("haskell"),
        codemirror_mime_type: Some("text/x-haskell"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Haxe",
        language_type: "programming",
        color: Some("#df7900"),
        extensions: &[
            ".hx",
            ".hxsl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("haxe"),
        tm_scope: Some("source.hx"),
        language_id: Some(158),
        codemirror_mode: Some("haxe"),
        codemirror_mime_type: Some("text/x-haxe"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HiveQL",
        language_type: "programming",
        color: Some("#dce200"),
        extensions: &[
            ".q",
            ".hql",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("sql"),
        tm_scope: Some("source.hql"),
        language_id: Some(931814087),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HolyC",
        language_type: "programming",
        color: Some("#ffefaf"),
        extensions: &[
            ".hc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.hc"),
        language_id: Some(928121743),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Hosts File",
        language_type: "data",
        color: Some("#308888"),
        extensions: &[
        ],
        aliases: &[
            "hosts",
        ],
        interpreters: &[
        ],
        filenames: &[
            "HOSTS",
            "hosts",
            "hosts.txt",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.hosts"),
        language_id: Some(231021894),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Hurl",
        language_type: "programming",
        color: Some("#FF0288"),
        extensions: &[
            ".hurl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.hurl"),
        language_id: Some(959040217),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Hy",
        language_type: "programming",
        color: Some("#7790B2"),
        extensions: &[
            ".hy",
        ],
        aliases: &[
            "hylang",
        ],
        interpreters: &[
            "hy",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.hy"),
        language_id: Some(159),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "HyPhy",
        language_type: "programming",
        color: None,
        extensions: &[
            ".bf",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(160),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "IDL",
        language_type: "programming",
        color: Some("#a3522f"),
        extensions: &[
            ".pro",
            ".dlm",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.idl"),
        language_id: Some(161),
        codemirror_mode: Some("idl"),
        codemirror_mime_type: Some("text/x-idl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "IGOR Pro",
        language_type: "programming",
        color: Some("#0000cc"),
        extensions: &[
            ".ipf",
        ],
        aliases: &[
            "igor",
            "igorpro",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.igor"),
        language_id: Some(162),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "INI",
        language_type: "data",
        color: Some("#d1dbe0"),
        extensions: &[
            ".ini",
            ".cfg",
            ".cnf",
            ".dof",
            ".frm",
            ".lektorproject",
            ".prefs",
            ".pro",
            ".properties",
            ".url",
        ],
        aliases: &[
            "dosini",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".buckconfig",
            ".coveragerc",
            ".flake8",
            ".pylintrc",
            "HOSTS",
            "buildozer.spec",
            "hosts",
            "pylintrc",
            "vlcrc",
        ],
        ace_mode: Some("ini"),
        tm_scope: Some("source.ini"),
        language_id: Some(163),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "IRC log",
        language_type: "data",
        color: None,
        extensions: &[
            ".irclog",
            ".weechatlog",
        ],
        aliases: &[
            "irc",
            "irc logs",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(164),
        codemirror_mode: Some("mirc"),
        codemirror_mime_type: Some("text/mirc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ISPC",
        language_type: "programming",
        color: Some("#2D68B1"),
        extensions: &[
            ".ispc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.ispc"),
        language_id: Some(327071),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Idris",
        language_type: "programming",
        color: Some("#b30000"),
        extensions: &[
            ".idr",
            ".lidr",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.idris"),
        language_id: Some(165),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ignore List",
        language_type: "data",
        color: Some("#000000"),
        extensions: &[
            ".gitignore",
        ],
        aliases: &[
            "ignore",
            "gitignore",
            "git-ignore",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".atomignore",
            ".babelignore",
            ".bzrignore",
            ".coffeelintignore",
            ".cvsignore",
            ".dockerignore",
            ".easignore",
            ".eleventyignore",
            ".eslintignore",
            ".gitignore",
            ".ignore",
            ".markdownlintignore",
            ".nodemonignore",
            ".npmignore",
            ".prettierignore",
            ".stylelintignore",
            ".vercelignore",
            ".vscodeignore",
            "gitignore-global",
            "gitignore_global",
        ],
        ace_mode: Some("gitignore"),
        tm_scope: Some("source.gitignore"),
        language_id: Some(74444240),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ImageJ Macro",
        language_type: "programming",
        color: Some("#99AAFF"),
        extensions: &[
            ".ijm",
        ],
        aliases: &[
            "ijm",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(575143428),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Imba",
        language_type: "programming",
        color: Some("#16cec6"),
        extensions: &[
            ".imba",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.imba"),
        language_id: Some(1057618448),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Inform 7",
        language_type: "programming",
        color: None,
        extensions: &[
            ".ni",
            ".i7x",
        ],
        aliases: &[
            "i7",
            "inform7",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.inform7"),
        language_id: Some(166),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ink",
        language_type: "programming",
        color: None,
        extensions: &[
            ".ink",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ink"),
        language_id: Some(838252715),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Inno Setup",
        language_type: "programming",
        color: Some("#264b99"),
        extensions: &[
            ".iss",
            ".isl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.inno"),
        language_id: Some(167),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Io",
        language_type: "programming",
        color: Some("#a9188d"),
        extensions: &[
            ".io",
        ],
        aliases: &[
        ],
        interpreters: &[
            "io",
        ],
        filenames: &[
        ],
        ace_mode: Some("io"),
        tm_scope: Some("source.io"),
        language_id: Some(168),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ioke",
        language_type: "programming",
        color: Some("#078193"),
        extensions: &[
            ".ik",
        ],
        aliases: &[
        ],
        interpreters: &[
            "ioke",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ioke"),
        language_id: Some(169),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Isabelle",
        language_type: "programming",
        color: Some("#FEFE00"),
        extensions: &[
            ".thy",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.isabelle.theory"),
        language_id: Some(170),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Isabelle ROOT",
        language_type: "programming",
        color: Some("#FEFE00"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "ROOT",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.isabelle.root"),
        language_id: Some(171),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Isabelle"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "J",
        language_type: "programming",
        color: Some("#9EEDFF"),
        extensions: &[
            ".ijs",
        ],
        aliases: &[
        ],
        interpreters: &[
            "jconsole",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.j"),
        language_id: Some(172),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JAR Manifest",
        language_type: "data",
        color: Some("#b07219"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "MANIFEST.MF",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.yaml"),
        language_id: Some(447261135),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JCL",
        language_type: "programming",
        color: Some("#d90e09"),
        extensions: &[
            ".jcl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.jcl"),
        language_id: Some(316620079),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JFlex",
        language_type: "programming",
        color: Some("#DBCA00"),
        extensions: &[
            ".flex",
            ".jflex",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.jflex"),
        language_id: Some(173),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Lex"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JSON",
        language_type: "data",
        color: Some("#292929"),
        extensions: &[
            ".json",
            ".4DForm",
            ".4DProject",
            ".avsc",
            ".geojson",
            ".gltf",
            ".har",
            ".ice",
            ".JSON-tmLanguage",
            ".json.example",
            ".jsonl",
            ".mcmeta",
            ".sarif",
            ".tact",
            ".tfstate",
            ".tfstate.backup",
            ".topojson",
            ".webapp",
            ".webmanifest",
            ".yy",
            ".yyp",
        ],
        aliases: &[
            "geojson",
            "jsonl",
            "sarif",
            "topojson",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".all-contributorsrc",
            ".arcconfig",
            ".auto-changelog",
            ".c8rc",
            ".htmlhintrc",
            ".imgbotconfig",
            ".nycrc",
            ".tern-config",
            ".tern-project",
            ".watchmanconfig",
            "MODULE.bazel.lock",
            "Package.resolved",
            "Pipfile.lock",
            "bun.lock",
            "composer.lock",
            "deno.lock",
            "flake.lock",
            "mcmod.info",
        ],
        ace_mode: Some("json"),
        tm_scope: Some("source.json"),
        language_id: Some(174),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/json"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JSON with Comments",
        language_type: "data",
        color: Some("#292929"),
        extensions: &[
            ".jsonc",
            ".code-snippets",
            ".code-workspace",
            ".sublime-build",
            ".sublime-color-scheme",
            ".sublime-commands",
            ".sublime-completions",
            ".sublime-keymap",
            ".sublime-macro",
            ".sublime-menu",
            ".sublime-mousemap",
            ".sublime-project",
            ".sublime-settings",
            ".sublime-theme",
            ".sublime-workspace",
            ".sublime_metrics",
            ".sublime_session",
        ],
        aliases: &[
            "jsonc",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".babelrc",
            ".devcontainer.json",
            ".eslintrc.json",
            ".jscsrc",
            ".jshintrc",
            ".jslintrc",
            ".swcrc",
            "api-extractor.json",
            "devcontainer.json",
            "jsconfig.json",
            "language-configuration.json",
            "tsconfig.json",
            "tslint.json",
        ],
        ace_mode: Some("javascript"),
        tm_scope: Some("source.json.comments"),
        language_id: Some(423),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("text/javascript"),
        group: Some("JSON"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JSON5",
        language_type: "data",
        color: Some("#267CB9"),
        extensions: &[
            ".json5",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("json5"),
        tm_scope: Some("source.js"),
        language_id: Some(175),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/json"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JSONLD",
        language_type: "data",
        color: Some("#0c479c"),
        extensions: &[
            ".jsonld",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("javascript"),
        tm_scope: Some("source.js"),
        language_id: Some(176),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/ld+json"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JSONiq",
        language_type: "programming",
        color: Some("#40d47e"),
        extensions: &[
            ".jq",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("jsoniq"),
        tm_scope: Some("source.jsoniq"),
        language_id: Some(177),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/json"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Jai",
        language_type: "programming",
        color: Some("#ab8b4b"),
        extensions: &[
            ".jai",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.jai"),
        language_id: Some(70127133),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Janet",
        language_type: "programming",
        color: Some("#0886a5"),
        extensions: &[
            ".janet",
        ],
        aliases: &[
        ],
        interpreters: &[
            "janet",
        ],
        filenames: &[
        ],
        ace_mode: Some("scheme"),
        tm_scope: Some("source.janet"),
        language_id: Some(1028705371),
        codemirror_mode: Some("scheme"),
        codemirror_mime_type: Some("text/x-scheme"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Jasmin",
        language_type: "programming",
        color: Some("#d03600"),
        extensions: &[
            ".j",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("java"),
        tm_scope: Some("source.jasmin"),
        language_id: Some(180),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Java",
        language_type: "programming",
        color: Some("#b07219"),
        extensions: &[
            ".java",
            ".jav",
            ".jsh",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("java"),
        tm_scope: Some("source.java"),
        language_id: Some(181),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-java"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Java Properties",
        language_type: "data",
        color: Some("#2A6277"),
        extensions: &[
            ".properties",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("properties"),
        tm_scope: Some("source.java-properties"),
        language_id: Some(519377561),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Java Server Pages",
        language_type: "programming",
        color: Some("#2A6277"),
        extensions: &[
            ".jsp",
            ".tag",
        ],
        aliases: &[
            "jsp",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("jsp"),
        tm_scope: Some("text.html.jsp"),
        language_id: Some(182),
        codemirror_mode: Some("htmlembedded"),
        codemirror_mime_type: Some("application/x-jsp"),
        group: Some("Java"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Java Template Engine",
        language_type: "programming",
        color: Some("#2A6277"),
        extensions: &[
            ".jte",
        ],
        aliases: &[
            "jte",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.html.jte"),
        language_id: Some(599494012),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Java"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JavaScript",
        language_type: "programming",
        color: Some("#f1e05a"),
        extensions: &[
            ".js",
            "._js",
            ".bones",
            ".cjs",
            ".es",
            ".es6",
            ".frag",
            ".gs",
            ".jake",
            ".javascript",
            ".jsb",
            ".jscad",
            ".jsfl",
            ".jslib",
            ".jsm",
            ".jspre",
            ".jss",
            ".jsx",
            ".mjs",
            ".njs",
            ".pac",
            ".sjs",
            ".ssjs",
            ".xsjs",
            ".xsjslib",
        ],
        aliases: &[
            "js",
            "node",
        ],
        interpreters: &[
            "chakra",
            "d8",
            "gjs",
            "js",
            "node",
            "nodejs",
            "qjs",
            "rhino",
            "v8",
            "v8-shell",
        ],
        filenames: &[
            "Jakefile",
        ],
        ace_mode: Some("javascript"),
        tm_scope: Some("source.js"),
        language_id: Some(183),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("text/javascript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JavaScript+ERB",
        language_type: "programming",
        color: Some("#f1e05a"),
        extensions: &[
            ".js.erb",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("javascript"),
        tm_scope: Some("source.js"),
        language_id: Some(914318960),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/javascript"),
        group: Some("JavaScript"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Jest Snapshot",
        language_type: "data",
        color: Some("#15c213"),
        extensions: &[
            ".snap",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("javascript"),
        tm_scope: Some("source.jest.snap"),
        language_id: Some(774635084),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/javascript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "JetBrains MPS",
        language_type: "programming",
        color: Some("#21D789"),
        extensions: &[
            ".mps",
            ".mpl",
            ".msd",
        ],
        aliases: &[
            "mps",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("none"),
        language_id: Some(465165328),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Jinja",
        language_type: "markup",
        color: Some("#a52a22"),
        extensions: &[
            ".jinja",
            ".j2",
            ".jinja2",
        ],
        aliases: &[
            "django",
            "html+django",
            "html+jinja",
            "htmldjango",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("django"),
        tm_scope: Some("text.html.django"),
        language_id: Some(147),
        codemirror_mode: Some("jinja2"),
        codemirror_mime_type: Some("text/jinja2"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Jison",
        language_type: "programming",
        color: Some("#56b3cb"),
        extensions: &[
            ".jison",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.jison"),
        language_id: Some(284531423),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Yacc"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Jison Lex",
        language_type: "programming",
        color: Some("#56b3cb"),
        extensions: &[
            ".jisonlex",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.jisonlex"),
        language_id: Some(406395330),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Lex"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Jolie",
        language_type: "programming",
        color: Some("#843179"),
        extensions: &[
            ".ol",
            ".iol",
        ],
        aliases: &[
        ],
        interpreters: &[
            "jolie",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.jolie"),
        language_id: Some(998078858),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Jsonnet",
        language_type: "programming",
        color: Some("#0064bd"),
        extensions: &[
            ".jsonnet",
            ".libsonnet",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.jsonnet"),
        language_id: Some(664885656),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Julia",
        language_type: "programming",
        color: Some("#a270ba"),
        extensions: &[
            ".jl",
        ],
        aliases: &[
        ],
        interpreters: &[
            "julia",
        ],
        filenames: &[
        ],
        ace_mode: Some("julia"),
        tm_scope: Some("source.julia"),
        language_id: Some(184),
        codemirror_mode: Some("julia"),
        codemirror_mime_type: Some("text/x-julia"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Julia REPL",
        language_type: "programming",
        color: Some("#a270ba"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.julia.console"),
        language_id: Some(220689142),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Julia"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Jupyter Notebook",
        language_type: "markup",
        color: Some("#DA5B0B"),
        extensions: &[
            ".ipynb",
        ],
        aliases: &[
            "IPython Notebook",
        ],
        interpreters: &[
        ],
        filenames: &[
            "Notebook",
        ],
        ace_mode: Some("json"),
        tm_scope: Some("source.json"),
        language_id: Some(185),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/json"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Just",
        language_type: "programming",
        color: Some("#384d54"),
        extensions: &[
            ".just",
        ],
        aliases: &[
            "Justfile",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".JUSTFILE",
            ".Justfile",
            ".justfile",
            "JUSTFILE",
            "Justfile",
            "justfile",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.just"),
        language_id: Some(128447695),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "KDL",
        language_type: "data",
        color: Some("#ffb3b3"),
        extensions: &[
            ".kdl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("tcl"),
        tm_scope: Some("source.kdl"),
        language_id: Some(931123626),
        codemirror_mode: Some("yacas"),
        codemirror_mime_type: Some("text/x-yacas"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "KRL",
        language_type: "programming",
        color: Some("#28430A"),
        extensions: &[
            ".krl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(186),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Kaitai Struct",
        language_type: "programming",
        color: Some("#773b37"),
        extensions: &[
            ".ksy",
        ],
        aliases: &[
            "ksy",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.yaml"),
        language_id: Some(818804755),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "KakouneScript",
        language_type: "programming",
        color: Some("#6f8042"),
        extensions: &[
            ".kak",
        ],
        aliases: &[
            "kak",
            "kakscript",
        ],
        interpreters: &[
        ],
        filenames: &[
            "kakrc",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.kakscript"),
        language_id: Some(603336474),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "KerboScript",
        language_type: "programming",
        color: Some("#41adf0"),
        extensions: &[
            ".ks",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.kerboscript"),
        language_id: Some(59716426),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "KiCad Layout",
        language_type: "data",
        color: Some("#2f4aab"),
        extensions: &[
            ".kicad_pcb",
            ".kicad_mod",
            ".kicad_wks",
        ],
        aliases: &[
            "pcbnew",
        ],
        interpreters: &[
        ],
        filenames: &[
            "fp-lib-table",
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.pcb.sexp"),
        language_id: Some(187),
        codemirror_mode: Some("commonlisp"),
        codemirror_mime_type: Some("text/x-common-lisp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "KiCad Legacy Layout",
        language_type: "data",
        color: Some("#2f4aab"),
        extensions: &[
            ".brd",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pcb.board"),
        language_id: Some(140848857),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "KiCad Schematic",
        language_type: "data",
        color: Some("#2f4aab"),
        extensions: &[
            ".kicad_sch",
            ".kicad_sym",
            ".sch",
        ],
        aliases: &[
            "eeschema schematic",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pcb.schematic"),
        language_id: Some(622447435),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Kickstart",
        language_type: "data",
        color: None,
        extensions: &[
            ".ks",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.kickstart"),
        language_id: Some(692635484),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Kit",
        language_type: "markup",
        color: None,
        extensions: &[
            ".kit",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html"),
        tm_scope: Some("text.html.basic"),
        language_id: Some(188),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "KoLmafia ASH",
        language_type: "programming",
        color: Some("#B9D9B9"),
        extensions: &[
            ".ash",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ash"),
        language_id: Some(852099832),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Koka",
        language_type: "programming",
        color: Some("#215166"),
        extensions: &[
            ".kk",
        ],
        aliases: &[
        ],
        interpreters: &[
            "koka",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.koka"),
        language_id: Some(597930447),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Kotlin",
        language_type: "programming",
        color: Some("#A97BFF"),
        extensions: &[
            ".kt",
            ".ktm",
            ".kts",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("kotlin"),
        tm_scope: Some("source.kotlin"),
        language_id: Some(189),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-kotlin"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Kusto",
        language_type: "data",
        color: None,
        extensions: &[
            ".csl",
            ".kql",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.kusto"),
        language_id: Some(225697190),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LFE",
        language_type: "programming",
        color: Some("#4C3023"),
        extensions: &[
            ".lfe",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.lisp"),
        language_id: Some(190),
        codemirror_mode: Some("commonlisp"),
        codemirror_mime_type: Some("text/x-common-lisp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LLVM",
        language_type: "programming",
        color: Some("#185619"),
        extensions: &[
            ".ll",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.llvm"),
        language_id: Some(191),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LOLCODE",
        language_type: "programming",
        color: Some("#cc9900"),
        extensions: &[
            ".lol",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.lolcode"),
        language_id: Some(192),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LSL",
        language_type: "programming",
        color: Some("#3d9970"),
        extensions: &[
            ".lsl",
            ".lslp",
        ],
        aliases: &[
        ],
        interpreters: &[
            "lsl",
        ],
        filenames: &[
        ],
        ace_mode: Some("lsl"),
        tm_scope: Some("source.lsl"),
        language_id: Some(193),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LTspice Symbol",
        language_type: "data",
        color: None,
        extensions: &[
            ".asy",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ltspice.symbol"),
        language_id: Some(1013566805),
        codemirror_mode: Some("spreadsheet"),
        codemirror_mime_type: Some("text/x-spreadsheet"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LabVIEW",
        language_type: "programming",
        color: Some("#fede06"),
        extensions: &[
            ".lvproj",
            ".lvclass",
            ".lvlib",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml"),
        language_id: Some(194),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Lark",
        language_type: "data",
        color: Some("#2980B9"),
        extensions: &[
            ".lark",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.lark"),
        language_id: Some(758480799),
        codemirror_mode: Some("ebnf"),
        codemirror_mime_type: Some("text/x-ebnf"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Lasso",
        language_type: "programming",
        color: Some("#999999"),
        extensions: &[
            ".lasso",
            ".las",
            ".lasso8",
            ".lasso9",
        ],
        aliases: &[
            "lassoscript",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("file.lasso"),
        language_id: Some(195),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Latte",
        language_type: "markup",
        color: Some("#f2a542"),
        extensions: &[
            ".latte",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("latte"),
        tm_scope: Some("text.html.smarty"),
        language_id: Some(196),
        codemirror_mode: Some("smarty"),
        codemirror_mime_type: Some("text/x-smarty"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Lean",
        language_type: "programming",
        color: None,
        extensions: &[
            ".lean",
            ".hlean",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.lean"),
        language_id: Some(197),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Lean 4",
        language_type: "programming",
        color: None,
        extensions: &[
            ".lean",
        ],
        aliases: &[
            "lean4",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.lean4"),
        language_id: Some(455147478),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Lean"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Leo",
        language_type: "programming",
        color: Some("#C4FFC2"),
        extensions: &[
            ".leo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.leo"),
        language_id: Some(916034822),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Less",
        language_type: "markup",
        color: Some("#1d365d"),
        extensions: &[
            ".less",
        ],
        aliases: &[
            "less-css",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("less"),
        tm_scope: Some("source.css.less"),
        language_id: Some(198),
        codemirror_mode: Some("css"),
        codemirror_mime_type: Some("text/x-less"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Lex",
        language_type: "programming",
        color: Some("#DBCA00"),
        extensions: &[
            ".l",
            ".lex",
        ],
        aliases: &[
            "flex",
        ],
        interpreters: &[
        ],
        filenames: &[
            "Lexer.x",
            "lexer.x",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.lex"),
        language_id: Some(199),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LigoLANG",
        language_type: "programming",
        color: Some("#0e74ff"),
        extensions: &[
            ".ligo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("pascal"),
        tm_scope: Some("source.ligo"),
        language_id: Some(1040646257),
        codemirror_mode: Some("pascal"),
        codemirror_mime_type: Some("text/x-pascal"),
        group: Some("LigoLANG"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LilyPond",
        language_type: "programming",
        color: Some("#9ccc7c"),
        extensions: &[
            ".ly",
            ".ily",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.lilypond"),
        language_id: Some(200),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Limbo",
        language_type: "programming",
        color: None,
        extensions: &[
            ".b",
            ".m",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(201),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Linear Programming",
        language_type: "programming",
        color: None,
        extensions: &[
            ".lp",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(377204539),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Linker Script",
        language_type: "programming",
        color: None,
        extensions: &[
            ".ld",
            ".lds",
            ".x",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "ld.script",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.c.linker"),
        language_id: Some(202),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Linux Kernel Module",
        language_type: "data",
        color: None,
        extensions: &[
            ".mod",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(203),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Liquid",
        language_type: "markup",
        color: Some("#67b8de"),
        extensions: &[
            ".liquid",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("liquid"),
        tm_scope: Some("text.html.liquid"),
        language_id: Some(204),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Literate Agda",
        language_type: "programming",
        color: Some("#315665"),
        extensions: &[
            ".lagda",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(205),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Agda"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Literate CoffeeScript",
        language_type: "programming",
        color: Some("#244776"),
        extensions: &[
            ".litcoffee",
            ".coffee.md",
        ],
        aliases: &[
            "litcoffee",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.litcoffee"),
        language_id: Some(206),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("CoffeeScript"),
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Literate Haskell",
        language_type: "programming",
        color: Some("#5e5086"),
        extensions: &[
            ".lhs",
        ],
        aliases: &[
            "lhaskell",
            "lhs",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.tex.latex.haskell"),
        language_id: Some(207),
        codemirror_mode: Some("haskell-literate"),
        codemirror_mime_type: Some("text/x-literate-haskell"),
        group: Some("Haskell"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LiveCode Script",
        language_type: "programming",
        color: Some("#0c5ba5"),
        extensions: &[
            ".livecodescript",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.livecodescript"),
        language_id: Some(891017),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LiveScript",
        language_type: "programming",
        color: Some("#499886"),
        extensions: &[
            ".ls",
            "._ls",
        ],
        aliases: &[
            "live-script",
            "ls",
        ],
        interpreters: &[
        ],
        filenames: &[
            "Slakefile",
        ],
        ace_mode: Some("livescript"),
        tm_scope: Some("source.livescript"),
        language_id: Some(208),
        codemirror_mode: Some("livescript"),
        codemirror_mime_type: Some("text/x-livescript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Logos",
        language_type: "programming",
        color: None,
        extensions: &[
            ".xm",
            ".x",
            ".xi",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.logos"),
        language_id: Some(209),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Logtalk",
        language_type: "programming",
        color: Some("#295b9a"),
        extensions: &[
            ".lgt",
            ".logtalk",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("logtalk"),
        tm_scope: Some("source.logtalk"),
        language_id: Some(210),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LookML",
        language_type: "programming",
        color: Some("#652B81"),
        extensions: &[
            ".lkml",
            ".lookml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.yaml"),
        language_id: Some(211),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "LoomScript",
        language_type: "programming",
        color: None,
        extensions: &[
            ".ls",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.loomscript"),
        language_id: Some(212),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Lua",
        language_type: "programming",
        color: Some("#000080"),
        extensions: &[
            ".lua",
            ".fcgi",
            ".nse",
            ".p8",
            ".pd_lua",
            ".rbxs",
            ".rockspec",
            ".wlua",
        ],
        aliases: &[
        ],
        interpreters: &[
            "lua",
        ],
        filenames: &[
            ".luacheckrc",
        ],
        ace_mode: Some("lua"),
        tm_scope: Some("source.lua"),
        language_id: Some(213),
        codemirror_mode: Some("lua"),
        codemirror_mime_type: Some("text/x-lua"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Luau",
        language_type: "programming",
        color: Some("#00A2FF"),
        extensions: &[
            ".luau",
        ],
        aliases: &[
        ],
        interpreters: &[
            "luau",
        ],
        filenames: &[
        ],
        ace_mode: Some("lua"),
        tm_scope: Some("source.luau"),
        language_id: Some(365050359),
        codemirror_mode: Some("lua"),
        codemirror_mime_type: Some("text/x-lua"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "M",
        language_type: "programming",
        color: None,
        extensions: &[
            ".mumps",
            ".m",
        ],
        aliases: &[
            "mumps",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(214),
        codemirror_mode: Some("mumps"),
        codemirror_mime_type: Some("text/x-mumps"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "M3U",
        language_type: "data",
        color: Some("#179C7D"),
        extensions: &[
            ".m3u",
            ".m3u8",
        ],
        aliases: &[
            "hls playlist",
            "m3u playlist",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.m3u"),
        language_id: Some(89638692),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "M4",
        language_type: "programming",
        color: None,
        extensions: &[
            ".m4",
            ".mc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.m4"),
        language_id: Some(215),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "M4Sugar",
        language_type: "programming",
        color: None,
        extensions: &[
            ".m4",
        ],
        aliases: &[
            "autoconf",
        ],
        interpreters: &[
        ],
        filenames: &[
            "configure.ac",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.m4"),
        language_id: Some(216),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("M4"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MATLAB",
        language_type: "programming",
        color: Some("#e16737"),
        extensions: &[
            ".matlab",
            ".m",
        ],
        aliases: &[
            "octave",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("matlab"),
        tm_scope: Some("source.matlab"),
        language_id: Some(225),
        codemirror_mode: Some("octave"),
        codemirror_mime_type: Some("text/x-octave"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MAXScript",
        language_type: "programming",
        color: Some("#00a6a6"),
        extensions: &[
            ".ms",
            ".mcr",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.maxscript"),
        language_id: Some(217),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MDX",
        language_type: "markup",
        color: Some("#fcb32c"),
        extensions: &[
            ".mdx",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("markdown"),
        tm_scope: Some("source.mdx"),
        language_id: Some(512838272),
        codemirror_mode: Some("gfm"),
        codemirror_mime_type: Some("text/x-gfm"),
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "MLIR",
        language_type: "programming",
        color: Some("#5EC8DB"),
        extensions: &[
            ".mlir",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.mlir"),
        language_id: Some(448253929),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MQL4",
        language_type: "programming",
        color: Some("#62A8D6"),
        extensions: &[
            ".mq4",
            ".mqh",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.mql5"),
        language_id: Some(426),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MQL5",
        language_type: "programming",
        color: Some("#4A76B8"),
        extensions: &[
            ".mq5",
            ".mqh",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.mql5"),
        language_id: Some(427),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MTML",
        language_type: "markup",
        color: Some("#b7e1f4"),
        extensions: &[
            ".mtml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html"),
        tm_scope: Some("text.html.basic"),
        language_id: Some(218),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MUF",
        language_type: "programming",
        color: None,
        extensions: &[
            ".muf",
            ".m",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("forth"),
        tm_scope: Some("none"),
        language_id: Some(219),
        codemirror_mode: Some("forth"),
        codemirror_mime_type: Some("text/x-forth"),
        group: Some("Forth"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Macaulay2",
        language_type: "programming",
        color: Some("#d8ffff"),
        extensions: &[
            ".m2",
        ],
        aliases: &[
            "m2",
        ],
        interpreters: &[
            "M2",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.m2"),
        language_id: Some(34167825),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Makefile",
        language_type: "programming",
        color: Some("#427819"),
        extensions: &[
            ".mak",
            ".d",
            ".make",
            ".makefile",
            ".mk",
            ".mkfile",
        ],
        aliases: &[
            "bsdmake",
            "make",
            "mf",
        ],
        interpreters: &[
            "make",
        ],
        filenames: &[
            "BSDmakefile",
            "GNUmakefile",
            "Kbuild",
            "Makefile",
            "Makefile.am",
            "Makefile.boot",
            "Makefile.frag",
            "Makefile.in",
            "Makefile.inc",
            "Makefile.wat",
            "makefile",
            "makefile.sco",
            "mkfile",
        ],
        ace_mode: Some("makefile"),
        tm_scope: Some("source.makefile"),
        language_id: Some(220),
        codemirror_mode: Some("cmake"),
        codemirror_mime_type: Some("text/x-cmake"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Mako",
        language_type: "programming",
        color: Some("#7e858d"),
        extensions: &[
            ".mako",
            ".mao",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.html.mako"),
        language_id: Some(221),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Markdown",
        language_type: "prose",
        color: Some("#083fa1"),
        extensions: &[
            ".md",
            ".livemd",
            ".markdown",
            ".mdown",
            ".mdwn",
            ".mkd",
            ".mkdn",
            ".mkdown",
            ".ronn",
            ".scd",
            ".workbook",
        ],
        aliases: &[
            "md",
            "pandoc",
        ],
        interpreters: &[
        ],
        filenames: &[
            "contents.lr",
        ],
        ace_mode: Some("markdown"),
        tm_scope: Some("text.md"),
        language_id: Some(222),
        codemirror_mode: Some("gfm"),
        codemirror_mime_type: Some("text/x-gfm"),
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Marko",
        language_type: "markup",
        color: Some("#42bff2"),
        extensions: &[
            ".marko",
        ],
        aliases: &[
            "markojs",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.marko"),
        language_id: Some(932782397),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Mask",
        language_type: "markup",
        color: Some("#f97732"),
        extensions: &[
            ".mask",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("mask"),
        tm_scope: Some("source.mask"),
        language_id: Some(223),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Maven POM",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "pom.xml",
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml.pom"),
        language_id: Some(226),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: Some("XML"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Max",
        language_type: "programming",
        color: Some("#c4a79c"),
        extensions: &[
            ".maxpat",
            ".maxhelp",
            ".maxproj",
            ".mxt",
            ".pat",
        ],
        aliases: &[
            "max/msp",
            "maxmsp",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("json"),
        tm_scope: Some("source.json"),
        language_id: Some(227),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/json"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Mercury",
        language_type: "programming",
        color: Some("#ff2b2b"),
        extensions: &[
            ".m",
            ".moo",
        ],
        aliases: &[
        ],
        interpreters: &[
            "mmi",
        ],
        filenames: &[
        ],
        ace_mode: Some("prolog"),
        tm_scope: Some("source.mercury"),
        language_id: Some(229),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Mermaid",
        language_type: "markup",
        color: Some("#ff3670"),
        extensions: &[
            ".mmd",
            ".mermaid",
        ],
        aliases: &[
            "mermaid example",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.mermaid"),
        language_id: Some(385992043),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Meson",
        language_type: "programming",
        color: Some("#007800"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "meson.build",
            "meson_options.txt",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.meson"),
        language_id: Some(799141244),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Metal",
        language_type: "programming",
        color: Some("#8f14e9"),
        extensions: &[
            ".metal",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c++"),
        language_id: Some(230),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-c++src"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Microsoft Developer Studio Project",
        language_type: "data",
        color: None,
        extensions: &[
            ".dsp",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(800983837),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Microsoft Visual Studio Solution",
        language_type: "data",
        color: None,
        extensions: &[
            ".sln",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.solution"),
        language_id: Some(849523096),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MiniD",
        language_type: "programming",
        color: None,
        extensions: &[
            ".minid",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(231),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MiniYAML",
        language_type: "data",
        color: Some("#ff1111"),
        extensions: &[
            ".yaml",
            ".yml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.miniyaml"),
        language_id: Some(4896465),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MiniZinc",
        language_type: "programming",
        color: Some("#06a9e6"),
        extensions: &[
            ".mzn",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.mzn"),
        language_id: Some(238874535),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MiniZinc Data",
        language_type: "data",
        color: None,
        extensions: &[
            ".dzn",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.mzn"),
        language_id: Some(938193433),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Mint",
        language_type: "programming",
        color: Some("#02b046"),
        extensions: &[
            ".mint",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.mint"),
        language_id: Some(968740319),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Mirah",
        language_type: "programming",
        color: Some("#c7a938"),
        extensions: &[
            ".druby",
            ".duby",
            ".mirah",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ruby"),
        tm_scope: Some("source.ruby"),
        language_id: Some(232),
        codemirror_mode: Some("ruby"),
        codemirror_mime_type: Some("text/x-ruby"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Modelica",
        language_type: "programming",
        color: Some("#de1d31"),
        extensions: &[
            ".mo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.modelica"),
        language_id: Some(233),
        codemirror_mode: Some("modelica"),
        codemirror_mime_type: Some("text/x-modelica"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Modula-2",
        language_type: "programming",
        color: Some("#10253f"),
        extensions: &[
            ".mod",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.modula2"),
        language_id: Some(234),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Modula-3",
        language_type: "programming",
        color: Some("#223388"),
        extensions: &[
            ".i3",
            ".ig",
            ".m3",
            ".mg",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.modula-3"),
        language_id: Some(564743864),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Module Management System",
        language_type: "programming",
        color: None,
        extensions: &[
            ".mms",
            ".mmk",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "descrip.mmk",
            "descrip.mms",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(235),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Mojo",
        language_type: "programming",
        color: Some("#ff4c1f"),
        extensions: &[
            ".mojo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("python"),
        tm_scope: Some("source.mojo"),
        language_id: Some(1045019587),
        codemirror_mode: Some("python"),
        codemirror_mime_type: Some("text/x-python"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Monkey",
        language_type: "programming",
        color: None,
        extensions: &[
            ".monkey",
            ".monkey2",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.monkey"),
        language_id: Some(236),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Monkey C",
        language_type: "programming",
        color: Some("#8D6747"),
        extensions: &[
            ".mc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.mc"),
        language_id: Some(231751931),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Moocode",
        language_type: "programming",
        color: None,
        extensions: &[
            ".moo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(237),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MoonBit",
        language_type: "programming",
        color: Some("#b92381"),
        extensions: &[
            ".mbt",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.moonbit"),
        language_id: Some(181453007),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "MoonScript",
        language_type: "programming",
        color: Some("#ff4585"),
        extensions: &[
            ".moon",
        ],
        aliases: &[
        ],
        interpreters: &[
            "moon",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.moonscript"),
        language_id: Some(238),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Motoko",
        language_type: "programming",
        color: Some("#fbb03b"),
        extensions: &[
            ".mo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.mo"),
        language_id: Some(202937027),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Motorola 68K Assembly",
        language_type: "programming",
        color: Some("#005daa"),
        extensions: &[
            ".asm",
            ".i",
            ".inc",
            ".s",
            ".x68",
        ],
        aliases: &[
            "m68k",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("assembly_x86"),
        tm_scope: Some("source.m68k"),
        language_id: Some(477582706),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Assembly"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Move",
        language_type: "programming",
        color: Some("#4a137a"),
        extensions: &[
            ".move",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.move"),
        language_id: Some(638334599),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Muse",
        language_type: "prose",
        color: None,
        extensions: &[
            ".muse",
        ],
        aliases: &[
            "amusewiki",
            "emacs muse",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.muse"),
        language_id: Some(474864066),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Mustache",
        language_type: "markup",
        color: Some("#724b3b"),
        extensions: &[
            ".mustache",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("smarty"),
        tm_scope: Some("text.html.smarty"),
        language_id: Some(638334590),
        codemirror_mode: Some("smarty"),
        codemirror_mime_type: Some("text/x-smarty"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Myghty",
        language_type: "programming",
        color: None,
        extensions: &[
            ".myt",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(239),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NASL",
        language_type: "programming",
        color: None,
        extensions: &[
            ".nasl",
            ".inc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.nasl"),
        language_id: Some(171666519),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NCL",
        language_type: "programming",
        color: Some("#28431f"),
        extensions: &[
            ".ncl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ncl"),
        language_id: Some(240),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NEON",
        language_type: "data",
        color: None,
        extensions: &[
            ".neon",
        ],
        aliases: &[
            "nette object notation",
            "ne-on",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.neon"),
        language_id: Some(481192983),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NL",
        language_type: "data",
        color: None,
        extensions: &[
            ".nl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(241),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NMODL",
        language_type: "programming",
        color: Some("#00356B"),
        extensions: &[
            ".mod",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(136456478),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NPM Config",
        language_type: "data",
        color: Some("#cb3837"),
        extensions: &[
        ],
        aliases: &[
            "npmrc",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".npmrc",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ini.npmrc"),
        language_id: Some(685022663),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("INI"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NSIS",
        language_type: "programming",
        color: None,
        extensions: &[
            ".nsi",
            ".nsh",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("nsis"),
        tm_scope: Some("source.nsis"),
        language_id: Some(242),
        codemirror_mode: Some("nsis"),
        codemirror_mime_type: Some("text/x-nsis"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NWScript",
        language_type: "programming",
        color: Some("#111522"),
        extensions: &[
            ".nss",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c.nwscript"),
        language_id: Some(731233819),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nasal",
        language_type: "programming",
        color: Some("#1d2c4e"),
        extensions: &[
            ".nas",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("nasal"),
        tm_scope: Some("source.nasal"),
        language_id: Some(178322513),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nearley",
        language_type: "programming",
        color: Some("#990000"),
        extensions: &[
            ".ne",
            ".nearley",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ne"),
        language_id: Some(521429430),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nemerle",
        language_type: "programming",
        color: Some("#3d3c6e"),
        extensions: &[
            ".n",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.nemerle"),
        language_id: Some(243),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NetLinx",
        language_type: "programming",
        color: Some("#0aa0ff"),
        extensions: &[
            ".axs",
            ".axi",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.netlinx"),
        language_id: Some(244),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NetLinx+ERB",
        language_type: "programming",
        color: Some("#747faa"),
        extensions: &[
            ".axs.erb",
            ".axi.erb",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.netlinx.erb"),
        language_id: Some(245),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NetLogo",
        language_type: "programming",
        color: Some("#ff6375"),
        extensions: &[
            ".nlogo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.lisp"),
        language_id: Some(246),
        codemirror_mode: Some("commonlisp"),
        codemirror_mime_type: Some("text/x-common-lisp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NewLisp",
        language_type: "programming",
        color: Some("#87AED7"),
        extensions: &[
            ".nl",
            ".lisp",
            ".lsp",
        ],
        aliases: &[
        ],
        interpreters: &[
            "newlisp",
        ],
        filenames: &[
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.lisp"),
        language_id: Some(247),
        codemirror_mode: Some("commonlisp"),
        codemirror_mime_type: Some("text/x-common-lisp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nextflow",
        language_type: "programming",
        color: Some("#3ac486"),
        extensions: &[
            ".nf",
        ],
        aliases: &[
        ],
        interpreters: &[
            "nextflow",
        ],
        filenames: &[
            "nextflow.config",
        ],
        ace_mode: Some("groovy"),
        tm_scope: Some("source.nextflow"),
        language_id: Some(506780613),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nginx",
        language_type: "data",
        color: Some("#009639"),
        extensions: &[
            ".nginx",
            ".nginxconf",
            ".vhost",
        ],
        aliases: &[
            "nginx configuration file",
        ],
        interpreters: &[
        ],
        filenames: &[
            "nginx.conf",
        ],
        ace_mode: Some("nginx"),
        tm_scope: Some("source.nginx"),
        language_id: Some(248),
        codemirror_mode: Some("nginx"),
        codemirror_mime_type: Some("text/x-nginx-conf"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nickel",
        language_type: "programming",
        color: Some("#E0C3FC"),
        extensions: &[
            ".ncl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.nickel"),
        language_id: Some(1067292664),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nim",
        language_type: "programming",
        color: Some("#ffc200"),
        extensions: &[
            ".nim",
            ".nim.cfg",
            ".nimble",
            ".nimrod",
            ".nims",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "nim.cfg",
        ],
        ace_mode: Some("nim"),
        tm_scope: Some("source.nim"),
        language_id: Some(249),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ninja",
        language_type: "data",
        color: None,
        extensions: &[
            ".ninja",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ninja"),
        language_id: Some(250),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nit",
        language_type: "programming",
        color: Some("#009917"),
        extensions: &[
            ".nit",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.nit"),
        language_id: Some(251),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nix",
        language_type: "programming",
        color: Some("#7e7eff"),
        extensions: &[
            ".nix",
        ],
        aliases: &[
            "nixos",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("nix"),
        tm_scope: Some("source.nix"),
        language_id: Some(252),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Noir",
        language_type: "programming",
        color: Some("#2f1f49"),
        extensions: &[
            ".nr",
        ],
        aliases: &[
            "nargo",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("rust"),
        tm_scope: Some("source.nr"),
        language_id: Some(813068465),
        codemirror_mode: Some("rust"),
        codemirror_mime_type: Some("text/x-rustsrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nu",
        language_type: "programming",
        color: Some("#c9df40"),
        extensions: &[
            ".nu",
        ],
        aliases: &[
            "nush",
        ],
        interpreters: &[
            "nush",
        ],
        filenames: &[
            "Nukefile",
        ],
        ace_mode: Some("scheme"),
        tm_scope: Some("source.nu"),
        language_id: Some(253),
        codemirror_mode: Some("scheme"),
        codemirror_mime_type: Some("text/x-scheme"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "NumPy",
        language_type: "programming",
        color: Some("#9C8AF9"),
        extensions: &[
            ".numpy",
            ".numpyw",
            ".numsc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(254),
        codemirror_mode: Some("python"),
        codemirror_mime_type: Some("text/x-python"),
        group: Some("Python"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nunjucks",
        language_type: "markup",
        color: Some("#3d8137"),
        extensions: &[
            ".njk",
        ],
        aliases: &[
            "njk",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("nunjucks"),
        tm_scope: Some("text.html.nunjucks"),
        language_id: Some(461856962),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Nushell",
        language_type: "programming",
        color: Some("#4E9906"),
        extensions: &[
            ".nu",
        ],
        aliases: &[
            "nu-script",
            "nushell-script",
        ],
        interpreters: &[
            "nu",
        ],
        filenames: &[
        ],
        ace_mode: Some("sh"),
        tm_scope: Some("source.nushell"),
        language_id: Some(446573572),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OASv2-json",
        language_type: "data",
        color: Some("#85ea2d"),
        extensions: &[
            ".json",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("json"),
        tm_scope: Some("source.json"),
        language_id: Some(834374816),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/json"),
        group: Some("OpenAPI Specification v2"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OASv2-yaml",
        language_type: "data",
        color: Some("#85ea2d"),
        extensions: &[
            ".yaml",
            ".yml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.yaml"),
        language_id: Some(105187618),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: Some("OpenAPI Specification v2"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OASv3-json",
        language_type: "data",
        color: Some("#85ea2d"),
        extensions: &[
            ".json",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("json"),
        tm_scope: Some("source.json"),
        language_id: Some(980062566),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/json"),
        group: Some("OpenAPI Specification v3"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OASv3-yaml",
        language_type: "data",
        color: Some("#85ea2d"),
        extensions: &[
            ".yaml",
            ".yml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.yaml"),
        language_id: Some(51239111),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: Some("OpenAPI Specification v3"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OCaml",
        language_type: "programming",
        color: Some("#ef7a08"),
        extensions: &[
            ".ml",
            ".eliom",
            ".eliomi",
            ".ml4",
            ".mli",
            ".mll",
            ".mly",
        ],
        aliases: &[
        ],
        interpreters: &[
            "ocaml",
            "ocamlrun",
            "ocamlscript",
        ],
        filenames: &[
        ],
        ace_mode: Some("ocaml"),
        tm_scope: Some("source.ocaml"),
        language_id: Some(255),
        codemirror_mode: Some("mllike"),
        codemirror_mime_type: Some("text/x-ocaml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OMNeT++ MSG",
        language_type: "programming",
        color: Some("#a0e0a0"),
        extensions: &[
            ".msg",
        ],
        aliases: &[
            "omnetpp-msg",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.msg"),
        language_id: Some(664100008),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OMNeT++ NED",
        language_type: "programming",
        color: Some("#08607c"),
        extensions: &[
            ".ned",
        ],
        aliases: &[
            "omnetpp-ned",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ned"),
        language_id: Some(924868392),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Oberon",
        language_type: "programming",
        color: None,
        extensions: &[
            ".ob2",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.modula2"),
        language_id: Some(677210597),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ObjDump",
        language_type: "data",
        color: None,
        extensions: &[
            ".objdump",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("assembly_x86"),
        tm_scope: Some("objdump.x86asm"),
        language_id: Some(256),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Object Data Instance Notation",
        language_type: "data",
        color: None,
        extensions: &[
            ".odin",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.odin-ehr"),
        language_id: Some(985227236),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ObjectScript",
        language_type: "programming",
        color: Some("#424893"),
        extensions: &[
            ".cls",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.objectscript"),
        language_id: Some(202735509),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Objective-C",
        language_type: "programming",
        color: Some("#438eff"),
        extensions: &[
            ".m",
            ".h",
        ],
        aliases: &[
            "obj-c",
            "objc",
            "objectivec",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("objectivec"),
        tm_scope: Some("source.objc"),
        language_id: Some(257),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-objectivec"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Objective-C++",
        language_type: "programming",
        color: Some("#6866fb"),
        extensions: &[
            ".mm",
        ],
        aliases: &[
            "obj-c++",
            "objc++",
            "objectivec++",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("objectivec"),
        tm_scope: Some("source.objc++"),
        language_id: Some(258),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-objectivec++"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Objective-J",
        language_type: "programming",
        color: Some("#ff0c5a"),
        extensions: &[
            ".j",
            ".sj",
        ],
        aliases: &[
            "obj-j",
            "objectivej",
            "objj",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.js.objj"),
        language_id: Some(259),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Odin",
        language_type: "programming",
        color: Some("#60AFFE"),
        extensions: &[
            ".odin",
        ],
        aliases: &[
            "odinlang",
            "odin-lang",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("odin"),
        tm_scope: Some("source.odin"),
        language_id: Some(889244082),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Omgrofl",
        language_type: "programming",
        color: Some("#cabbff"),
        extensions: &[
            ".omgrofl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(260),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Opa",
        language_type: "programming",
        color: None,
        extensions: &[
            ".opa",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.opa"),
        language_id: Some(261),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Opal",
        language_type: "programming",
        color: Some("#f7ede0"),
        extensions: &[
            ".opal",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.opal"),
        language_id: Some(262),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Open Policy Agent",
        language_type: "programming",
        color: Some("#7d9199"),
        extensions: &[
            ".rego",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.rego"),
        language_id: Some(840483232),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OpenAPI Specification v2",
        language_type: "data",
        color: Some("#85ea2d"),
        extensions: &[
        ],
        aliases: &[
            "oasv2",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(848295328),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OpenAPI Specification v3",
        language_type: "data",
        color: Some("#85ea2d"),
        extensions: &[
        ],
        aliases: &[
            "oasv3",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(557959099),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OpenCL",
        language_type: "programming",
        color: Some("#ed2e2d"),
        extensions: &[
            ".cl",
            ".opencl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c"),
        language_id: Some(263),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: Some("C"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OpenEdge ABL",
        language_type: "programming",
        color: Some("#5ce600"),
        extensions: &[
            ".p",
            ".cls",
            ".w",
        ],
        aliases: &[
            "progress",
            "openedge",
            "abl",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.abl"),
        language_id: Some(264),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OpenQASM",
        language_type: "programming",
        color: Some("#AA70FF"),
        extensions: &[
            ".qasm",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.qasm"),
        language_id: Some(153739399),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OpenRC runscript",
        language_type: "programming",
        color: None,
        extensions: &[
        ],
        aliases: &[
            "openrc",
        ],
        interpreters: &[
            "openrc-run",
        ],
        filenames: &[
        ],
        ace_mode: Some("sh"),
        tm_scope: Some("source.shell"),
        language_id: Some(265),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: Some("Shell"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OpenSCAD",
        language_type: "programming",
        color: Some("#e5cd45"),
        extensions: &[
            ".scad",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("scad"),
        tm_scope: Some("source.scad"),
        language_id: Some(266),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OpenStep Property List",
        language_type: "data",
        color: None,
        extensions: &[
            ".plist",
            ".glyphs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.plist"),
        language_id: Some(598917541),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "OpenType Feature File",
        language_type: "data",
        color: None,
        extensions: &[
            ".fea",
        ],
        aliases: &[
            "AFDKO",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.opentype"),
        language_id: Some(374317347),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Option List",
        language_type: "data",
        color: Some("#476732"),
        extensions: &[
        ],
        aliases: &[
            "opts",
            "ackrc",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".ackrc",
            ".rspec",
            ".yardopts",
            "ackrc",
            "mocha.opts",
        ],
        ace_mode: Some("sh"),
        tm_scope: Some("source.opts"),
        language_id: Some(723589315),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Org",
        language_type: "prose",
        color: Some("#77aa99"),
        extensions: &[
            ".org",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(267),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "OverpassQL",
        language_type: "programming",
        color: Some("#cce2aa"),
        extensions: &[
            ".overpassql",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.overpassql"),
        language_id: Some(689079655),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ox",
        language_type: "programming",
        color: None,
        extensions: &[
            ".ox",
            ".oxh",
            ".oxo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ox"),
        language_id: Some(268),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Oxygene",
        language_type: "programming",
        color: Some("#cdd0e3"),
        extensions: &[
            ".oxygene",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(269),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Oz",
        language_type: "programming",
        color: Some("#fab738"),
        extensions: &[
            ".oz",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.oz"),
        language_id: Some(270),
        codemirror_mode: Some("oz"),
        codemirror_mime_type: Some("text/x-oz"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "P4",
        language_type: "programming",
        color: Some("#7055b5"),
        extensions: &[
            ".p4",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.p4"),
        language_id: Some(348895984),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PDDL",
        language_type: "programming",
        color: Some("#0d00ff"),
        extensions: &[
            ".pddl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pddl"),
        language_id: Some(736235603),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PEG.js",
        language_type: "programming",
        color: Some("#234d6b"),
        extensions: &[
            ".pegjs",
            ".peggy",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("javascript"),
        tm_scope: Some("source.peggy"),
        language_id: Some(81442128),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("text/javascript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PHP",
        language_type: "programming",
        color: Some("#4F5D95"),
        extensions: &[
            ".php",
            ".aw",
            ".ctp",
            ".fcgi",
            ".inc",
            ".php3",
            ".php4",
            ".php5",
            ".phps",
            ".phpt",
        ],
        aliases: &[
            "inc",
        ],
        interpreters: &[
            "php",
        ],
        filenames: &[
            ".php",
            ".php_cs",
            ".php_cs.dist",
            "Phakefile",
        ],
        ace_mode: Some("php"),
        tm_scope: Some("text.html.php"),
        language_id: Some(272),
        codemirror_mode: Some("php"),
        codemirror_mime_type: Some("application/x-httpd-php"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PLSQL",
        language_type: "programming",
        color: Some("#dad8d8"),
        extensions: &[
            ".pls",
            ".bdy",
            ".ddl",
            ".fnc",
            ".pck",
            ".pkb",
            ".pks",
            ".plb",
            ".plsql",
            ".prc",
            ".spc",
            ".sql",
            ".tpb",
            ".tps",
            ".trg",
            ".vw",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("plsql"),
        tm_scope: Some("none"),
        language_id: Some(273),
        codemirror_mode: Some("sql"),
        codemirror_mime_type: Some("text/x-plsql"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PLpgSQL",
        language_type: "programming",
        color: Some("#336790"),
        extensions: &[
            ".pgsql",
            ".sql",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("pgsql"),
        tm_scope: Some("source.sql"),
        language_id: Some(274),
        codemirror_mode: Some("sql"),
        codemirror_mime_type: Some("text/x-sql"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "POV-Ray SDL",
        language_type: "programming",
        color: Some("#6bac65"),
        extensions: &[
            ".pov",
            ".inc",
        ],
        aliases: &[
            "pov-ray",
            "povray",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pov-ray sdl"),
        language_id: Some(275),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pact",
        language_type: "programming",
        color: Some("#F7A8B8"),
        extensions: &[
            ".pact",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pact"),
        language_id: Some(756774415),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pan",
        language_type: "programming",
        color: Some("#cc0000"),
        extensions: &[
            ".pan",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pan"),
        language_id: Some(276),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Papyrus",
        language_type: "programming",
        color: Some("#6600cc"),
        extensions: &[
            ".psc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.papyrus.skyrim"),
        language_id: Some(277),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Parrot",
        language_type: "programming",
        color: Some("#f3ca0a"),
        extensions: &[
            ".parrot",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(278),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Parrot Assembly",
        language_type: "programming",
        color: None,
        extensions: &[
            ".pasm",
        ],
        aliases: &[
            "pasm",
        ],
        interpreters: &[
            "parrot",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(279),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Parrot"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Parrot Internal Representation",
        language_type: "programming",
        color: None,
        extensions: &[
            ".pir",
        ],
        aliases: &[
            "pir",
        ],
        interpreters: &[
            "parrot",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.parrot.pir"),
        language_id: Some(280),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Parrot"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pascal",
        language_type: "programming",
        color: Some("#E3F171"),
        extensions: &[
            ".pas",
            ".dfm",
            ".dpr",
            ".inc",
            ".lpr",
            ".pascal",
            ".pp",
        ],
        aliases: &[
            "delphi",
            "objectpascal",
        ],
        interpreters: &[
            "instantfpc",
        ],
        filenames: &[
        ],
        ace_mode: Some("pascal"),
        tm_scope: Some("source.pascal"),
        language_id: Some(281),
        codemirror_mode: Some("pascal"),
        codemirror_mime_type: Some("text/x-pascal"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pawn",
        language_type: "programming",
        color: Some("#dbb284"),
        extensions: &[
            ".pwn",
            ".inc",
            ".sma",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pawn"),
        language_id: Some(271),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pep8",
        language_type: "programming",
        color: Some("#C76F5B"),
        extensions: &[
            ".pep",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pep8"),
        language_id: Some(840372442),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Perl",
        language_type: "programming",
        color: Some("#0298c3"),
        extensions: &[
            ".pl",
            ".al",
            ".cgi",
            ".fcgi",
            ".perl",
            ".ph",
            ".plx",
            ".pm",
            ".psgi",
            ".t",
        ],
        aliases: &[
            "cperl",
        ],
        interpreters: &[
            "cperl",
            "perl",
        ],
        filenames: &[
            ".latexmkrc",
            "Makefile.PL",
            "Rexfile",
            "ack",
            "cpanfile",
            "latexmkrc",
        ],
        ace_mode: Some("perl"),
        tm_scope: Some("source.perl"),
        language_id: Some(282),
        codemirror_mode: Some("perl"),
        codemirror_mime_type: Some("text/x-perl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pic",
        language_type: "markup",
        color: None,
        extensions: &[
            ".pic",
            ".chem",
        ],
        aliases: &[
            "pikchr",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pic"),
        language_id: Some(425),
        codemirror_mode: Some("troff"),
        codemirror_mime_type: Some("text/troff"),
        group: Some("Roff"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pickle",
        language_type: "data",
        color: None,
        extensions: &[
            ".pkl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(284),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PicoLisp",
        language_type: "programming",
        color: Some("#6067af"),
        extensions: &[
            ".l",
        ],
        aliases: &[
        ],
        interpreters: &[
            "picolisp",
            "pil",
        ],
        filenames: &[
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.lisp"),
        language_id: Some(285),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PigLatin",
        language_type: "programming",
        color: Some("#fcd7de"),
        extensions: &[
            ".pig",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("pig"),
        tm_scope: Some("source.pig_latin"),
        language_id: Some(286),
        codemirror_mode: Some("pig"),
        codemirror_mime_type: Some("text/x-pig"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pike",
        language_type: "programming",
        color: Some("#005390"),
        extensions: &[
            ".pike",
            ".pmod",
        ],
        aliases: &[
        ],
        interpreters: &[
            "pike",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pike"),
        language_id: Some(287),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pip Requirements",
        language_type: "data",
        color: Some("#FFD343"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "requirements-dev.txt",
            "requirements.lock.txt",
            "requirements.txt",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pip-requirements"),
        language_id: Some(684385621),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pkl",
        language_type: "programming",
        color: Some("#6b9543"),
        extensions: &[
            ".pkl",
        ],
        aliases: &[
        ],
        interpreters: &[
            "pkl",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pkl"),
        language_id: Some(288822799),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PlantUML",
        language_type: "data",
        color: Some("#fbbd16"),
        extensions: &[
            ".puml",
            ".iuml",
            ".plantuml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.wsd"),
        language_id: Some(833504686),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pod",
        language_type: "prose",
        color: None,
        extensions: &[
            ".pod",
        ],
        aliases: &[
        ],
        interpreters: &[
            "perl",
        ],
        filenames: &[
        ],
        ace_mode: Some("perl"),
        tm_scope: Some("none"),
        language_id: Some(288),
        codemirror_mode: Some("perl"),
        codemirror_mime_type: Some("text/x-perl"),
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pod 6",
        language_type: "prose",
        color: None,
        extensions: &[
            ".pod",
            ".pod6",
        ],
        aliases: &[
        ],
        interpreters: &[
            "perl6",
        ],
        filenames: &[
        ],
        ace_mode: Some("perl"),
        tm_scope: Some("source.raku"),
        language_id: Some(155357471),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "PogoScript",
        language_type: "programming",
        color: Some("#d80074"),
        extensions: &[
            ".pogo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pogoscript"),
        language_id: Some(289),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Polar",
        language_type: "programming",
        color: Some("#ae81ff"),
        extensions: &[
            ".polar",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.polar"),
        language_id: Some(839112914),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pony",
        language_type: "programming",
        color: None,
        extensions: &[
            ".pony",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.pony"),
        language_id: Some(290),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Portugol",
        language_type: "programming",
        color: Some("#f8bd00"),
        extensions: &[
            ".por",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.portugol"),
        language_id: Some(832391833),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PostCSS",
        language_type: "markup",
        color: Some("#dc3a0c"),
        extensions: &[
            ".pcss",
            ".postcss",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.postcss"),
        language_id: Some(262764437),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("CSS"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PostScript",
        language_type: "markup",
        color: Some("#da291c"),
        extensions: &[
            ".ps",
            ".eps",
            ".epsi",
            ".pfa",
        ],
        aliases: &[
            "postscr",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.postscript"),
        language_id: Some(291),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PowerBuilder",
        language_type: "programming",
        color: Some("#8f0f8d"),
        extensions: &[
            ".pbt",
            ".sra",
            ".sru",
            ".srw",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.powerbuilder"),
        language_id: Some(292),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PowerShell",
        language_type: "programming",
        color: Some("#012456"),
        extensions: &[
            ".ps1",
            ".psd1",
            ".psm1",
        ],
        aliases: &[
            "posh",
            "pwsh",
        ],
        interpreters: &[
            "pwsh",
        ],
        filenames: &[
        ],
        ace_mode: Some("powershell"),
        tm_scope: Some("source.powershell"),
        language_id: Some(293),
        codemirror_mode: Some("powershell"),
        codemirror_mime_type: Some("application/x-powershell"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Praat",
        language_type: "programming",
        color: Some("#c8506d"),
        extensions: &[
            ".praat",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("praat"),
        tm_scope: Some("source.praat"),
        language_id: Some(106029007),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Prisma",
        language_type: "data",
        color: Some("#0c344b"),
        extensions: &[
            ".prisma",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("prisma"),
        tm_scope: Some("source.prisma"),
        language_id: Some(499933428),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Processing",
        language_type: "programming",
        color: Some("#0096D8"),
        extensions: &[
            ".pde",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.processing"),
        language_id: Some(294),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Procfile",
        language_type: "programming",
        color: Some("#3B2F63"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "Procfile",
        ],
        ace_mode: Some("batchfile"),
        tm_scope: Some("source.procfile"),
        language_id: Some(305313959),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Proguard",
        language_type: "data",
        color: None,
        extensions: &[
            ".pro",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(716513858),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Prolog",
        language_type: "programming",
        color: Some("#74283c"),
        extensions: &[
            ".pl",
            ".plt",
            ".pro",
            ".prolog",
            ".yap",
        ],
        aliases: &[
        ],
        interpreters: &[
            "swipl",
            "yap",
        ],
        filenames: &[
        ],
        ace_mode: Some("prolog"),
        tm_scope: Some("source.prolog"),
        language_id: Some(295),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Promela",
        language_type: "programming",
        color: Some("#de0000"),
        extensions: &[
            ".pml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.promela"),
        language_id: Some(441858312),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Propeller Spin",
        language_type: "programming",
        color: Some("#7fa2a7"),
        extensions: &[
            ".spin",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.spin"),
        language_id: Some(296),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Protocol Buffer",
        language_type: "data",
        color: None,
        extensions: &[
            ".proto",
        ],
        aliases: &[
            "proto",
            "protobuf",
            "Protocol Buffers",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("protobuf"),
        tm_scope: Some("source.proto"),
        language_id: Some(297),
        codemirror_mode: Some("protobuf"),
        codemirror_mime_type: Some("text/x-protobuf"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Protocol Buffer Text Format",
        language_type: "data",
        color: None,
        extensions: &[
            ".textproto",
            ".pbt",
            ".pbtxt",
        ],
        aliases: &[
            "text proto",
            "protobuf text format",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.textproto"),
        language_id: Some(436568854),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Public Key",
        language_type: "data",
        color: None,
        extensions: &[
            ".asc",
            ".pub",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(298),
        codemirror_mode: Some("asciiarmor"),
        codemirror_mime_type: Some("application/pgp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pug",
        language_type: "markup",
        color: Some("#a86454"),
        extensions: &[
            ".jade",
            ".pug",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("jade"),
        tm_scope: Some("text.jade"),
        language_id: Some(179),
        codemirror_mode: Some("pug"),
        codemirror_mime_type: Some("text/x-pug"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Puppet",
        language_type: "programming",
        color: Some("#302B6D"),
        extensions: &[
            ".pp",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "Modulefile",
        ],
        ace_mode: Some("puppet"),
        tm_scope: Some("source.puppet"),
        language_id: Some(299),
        codemirror_mode: Some("puppet"),
        codemirror_mime_type: Some("text/x-puppet"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pure Data",
        language_type: "data",
        color: None,
        extensions: &[
            ".pd",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(300),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PureBasic",
        language_type: "programming",
        color: Some("#5a6986"),
        extensions: &[
            ".pb",
            ".pbi",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(301),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "PureScript",
        language_type: "programming",
        color: Some("#1D222D"),
        extensions: &[
            ".purs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("haskell"),
        tm_scope: Some("source.purescript"),
        language_id: Some(302),
        codemirror_mode: Some("haskell"),
        codemirror_mime_type: Some("text/x-haskell"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Pyret",
        language_type: "programming",
        color: Some("#ee1e10"),
        extensions: &[
            ".arr",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("python"),
        tm_scope: Some("source.arr"),
        language_id: Some(252961827),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Python",
        language_type: "programming",
        color: Some("#3572A5"),
        extensions: &[
            ".py",
            ".cgi",
            ".fcgi",
            ".gyp",
            ".gypi",
            ".lmi",
            ".py3",
            ".pyde",
            ".pyi",
            ".pyp",
            ".pyt",
            ".pyw",
            ".rpy",
            ".spec",
            ".tac",
            ".wsgi",
            ".xpy",
        ],
        aliases: &[
            "python3",
            "rusthon",
        ],
        interpreters: &[
            "python",
            "python2",
            "python3",
            "py",
            "pypy",
            "pypy3",
            "uv",
        ],
        filenames: &[
            ".gclient",
            "DEPS",
            "SConscript",
            "SConstruct",
            "wscript",
        ],
        ace_mode: Some("python"),
        tm_scope: Some("source.python"),
        language_id: Some(303),
        codemirror_mode: Some("python"),
        codemirror_mime_type: Some("text/x-python"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Python console",
        language_type: "programming",
        color: Some("#3572A5"),
        extensions: &[
        ],
        aliases: &[
            "pycon",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.python.console"),
        language_id: Some(428),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Python"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Python traceback",
        language_type: "data",
        color: Some("#3572A5"),
        extensions: &[
            ".pytb",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.python.traceback"),
        language_id: Some(304),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Python"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Q#",
        language_type: "programming",
        color: Some("#fed659"),
        extensions: &[
            ".qs",
        ],
        aliases: &[
            "qsharp",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.qsharp"),
        language_id: Some(697448245),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "QML",
        language_type: "programming",
        color: Some("#44a51c"),
        extensions: &[
            ".qml",
            ".qbs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("qml"),
        tm_scope: Some("source.qml"),
        language_id: Some(305),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "QMake",
        language_type: "programming",
        color: None,
        extensions: &[
            ".pro",
            ".pri",
        ],
        aliases: &[
        ],
        interpreters: &[
            "qmake",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.qmake"),
        language_id: Some(306),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Qt Script",
        language_type: "programming",
        color: Some("#00b841"),
        extensions: &[
            ".qs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "installscript.qs",
            "toolchain_installscript.qs",
        ],
        ace_mode: Some("javascript"),
        tm_scope: Some("source.js"),
        language_id: Some(558193693),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("text/javascript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Quake",
        language_type: "programming",
        color: Some("#882233"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "m3makefile",
            "m3overrides",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.quake"),
        language_id: Some(375265331),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "QuakeC",
        language_type: "programming",
        color: Some("#975777"),
        extensions: &[
            ".qc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.quakec"),
        language_id: Some(472308069),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "QuickBASIC",
        language_type: "programming",
        color: Some("#008080"),
        extensions: &[
            ".bas",
        ],
        aliases: &[
            "qb",
            "qbasic",
            "qb64",
            "classic qbasic",
            "classic quickbasic",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.QB64"),
        language_id: Some(593107205),
        codemirror_mode: Some("vb"),
        codemirror_mime_type: Some("text/x-vb"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "R",
        language_type: "programming",
        color: Some("#198CE7"),
        extensions: &[
            ".r",
            ".rd",
            ".rsx",
        ],
        aliases: &[
            "Rscript",
            "splus",
        ],
        interpreters: &[
            "Rscript",
        ],
        filenames: &[
            ".Rprofile",
            "expr-dist",
        ],
        ace_mode: Some("r"),
        tm_scope: Some("source.r"),
        language_id: Some(307),
        codemirror_mode: Some("r"),
        codemirror_mime_type: Some("text/x-rsrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RAML",
        language_type: "markup",
        color: Some("#77d9fb"),
        extensions: &[
            ".raml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.yaml"),
        language_id: Some(308),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RBS",
        language_type: "data",
        color: Some("#701516"),
        extensions: &[
            ".rbs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ruby"),
        tm_scope: Some("source.rbs"),
        language_id: Some(899227493),
        codemirror_mode: Some("ruby"),
        codemirror_mime_type: Some("text/x-ruby"),
        group: Some("Ruby"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RDoc",
        language_type: "prose",
        color: Some("#701516"),
        extensions: &[
            ".rdoc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("rdoc"),
        tm_scope: Some("text.rdoc"),
        language_id: Some(309),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "REALbasic",
        language_type: "programming",
        color: None,
        extensions: &[
            ".rbbas",
            ".rbfrm",
            ".rbmnu",
            ".rbres",
            ".rbtbar",
            ".rbuistate",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.vbnet"),
        language_id: Some(310),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "REXX",
        language_type: "programming",
        color: Some("#d90e09"),
        extensions: &[
            ".rexx",
            ".pprx",
            ".rex",
        ],
        aliases: &[
            "arexx",
        ],
        interpreters: &[
            "regina",
            "rexx",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.rexx"),
        language_id: Some(311),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RMarkdown",
        language_type: "prose",
        color: Some("#198ce7"),
        extensions: &[
            ".qmd",
            ".rmd",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("markdown"),
        tm_scope: Some("text.md"),
        language_id: Some(313),
        codemirror_mode: Some("gfm"),
        codemirror_mime_type: Some("text/x-gfm"),
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "RON",
        language_type: "data",
        color: Some("#a62c00"),
        extensions: &[
            ".ron",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("rust"),
        tm_scope: Some("source.ron"),
        language_id: Some(587855233),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ROS Interface",
        language_type: "data",
        color: Some("#22314e"),
        extensions: &[
            ".msg",
            ".action",
            ".srv",
        ],
        aliases: &[
            "rosmsg",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.rosmsg"),
        language_id: Some(809230569),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RPC",
        language_type: "programming",
        color: None,
        extensions: &[
            ".x",
        ],
        aliases: &[
            "rpcgen",
            "oncrpc",
            "xdr",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c"),
        language_id: Some(1031374237),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RPGLE",
        language_type: "programming",
        color: Some("#2BDE21"),
        extensions: &[
            ".rpgle",
            ".sqlrpgle",
        ],
        aliases: &[
            "ile rpg",
            "sqlrpgle",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.rpgle"),
        language_id: Some(609977990),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RPM Spec",
        language_type: "data",
        color: None,
        extensions: &[
            ".spec",
        ],
        aliases: &[
            "specfile",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.rpm-spec"),
        language_id: Some(314),
        codemirror_mode: Some("rpm"),
        codemirror_mime_type: Some("text/x-rpm-spec"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RUNOFF",
        language_type: "markup",
        color: Some("#665a4e"),
        extensions: &[
            ".rnh",
            ".rno",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.runoff"),
        language_id: Some(315),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Racket",
        language_type: "programming",
        color: Some("#3c5caa"),
        extensions: &[
            ".rkt",
            ".rktd",
            ".rktl",
            ".scrbl",
        ],
        aliases: &[
        ],
        interpreters: &[
            "racket",
        ],
        filenames: &[
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.racket"),
        language_id: Some(316),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ragel",
        language_type: "programming",
        color: Some("#9d5200"),
        extensions: &[
            ".rl",
        ],
        aliases: &[
            "ragel-rb",
            "ragel-ruby",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(317),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Raku",
        language_type: "programming",
        color: Some("#0000fb"),
        extensions: &[
            ".6pl",
            ".6pm",
            ".nqp",
            ".p6",
            ".p6l",
            ".p6m",
            ".pl",
            ".pl6",
            ".pm",
            ".pm6",
            ".raku",
            ".rakumod",
            ".t",
        ],
        aliases: &[
            "perl6",
            "perl-6",
        ],
        interpreters: &[
            "perl6",
            "raku",
            "rakudo",
        ],
        filenames: &[
        ],
        ace_mode: Some("raku"),
        tm_scope: Some("source.raku"),
        language_id: Some(283),
        codemirror_mode: Some("perl"),
        codemirror_mime_type: Some("text/x-perl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Rascal",
        language_type: "programming",
        color: Some("#fffaa0"),
        extensions: &[
            ".rsc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.rascal"),
        language_id: Some(173616037),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Raw token data",
        language_type: "data",
        color: None,
        extensions: &[
            ".raw",
        ],
        aliases: &[
            "raw",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(318),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ReScript",
        language_type: "programming",
        color: Some("#ed5051"),
        extensions: &[
            ".res",
            ".resi",
        ],
        aliases: &[
        ],
        interpreters: &[
            "ocaml",
        ],
        filenames: &[
        ],
        ace_mode: Some("rust"),
        tm_scope: Some("source.rescript"),
        language_id: Some(501875647),
        codemirror_mode: Some("rust"),
        codemirror_mime_type: Some("text/x-rustsrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Readline Config",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
            "inputrc",
            "readline",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".inputrc",
            "inputrc",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.inputrc"),
        language_id: Some(538732839),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("INI"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Reason",
        language_type: "programming",
        color: Some("#ff5847"),
        extensions: &[
            ".re",
            ".rei",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("rust"),
        tm_scope: Some("source.reason"),
        language_id: Some(869538413),
        codemirror_mode: Some("rust"),
        codemirror_mime_type: Some("text/x-rustsrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ReasonLIGO",
        language_type: "programming",
        color: Some("#ff5847"),
        extensions: &[
            ".religo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("rust"),
        tm_scope: Some("source.religo"),
        language_id: Some(319002153),
        codemirror_mode: Some("rust"),
        codemirror_mime_type: Some("text/x-rustsrc"),
        group: Some("LigoLANG"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Rebol",
        language_type: "programming",
        color: Some("#358a5b"),
        extensions: &[
            ".reb",
            ".r",
            ".r2",
            ".r3",
            ".rebol",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.rebol"),
        language_id: Some(319),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Record Jar",
        language_type: "data",
        color: Some("#0673ba"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "language-subtag-registry.txt",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.record-jar"),
        language_id: Some(865765202),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Red",
        language_type: "programming",
        color: Some("#f50000"),
        extensions: &[
            ".red",
            ".reds",
        ],
        aliases: &[
            "red/system",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("red"),
        tm_scope: Some("source.red"),
        language_id: Some(320),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Redcode",
        language_type: "programming",
        color: None,
        extensions: &[
            ".cw",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(321),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Redirect Rules",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
            "redirects",
        ],
        interpreters: &[
        ],
        filenames: &[
            "_redirects",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.redirects"),
        language_id: Some(1020148948),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Regular Expression",
        language_type: "data",
        color: Some("#009a00"),
        extensions: &[
            ".regexp",
            ".regex",
        ],
        aliases: &[
            "regexp",
            "regex",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.regexp"),
        language_id: Some(363378884),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ren'Py",
        language_type: "programming",
        color: Some("#ff7f7f"),
        extensions: &[
            ".rpy",
        ],
        aliases: &[
            "renpy",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("python"),
        tm_scope: Some("source.renpy"),
        language_id: Some(322),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RenderScript",
        language_type: "programming",
        color: None,
        extensions: &[
            ".rs",
            ".rsh",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(323),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Rez",
        language_type: "programming",
        color: Some("#FFDAB3"),
        extensions: &[
            ".r",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.rez"),
        language_id: Some(498022874),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Rich Text Format",
        language_type: "markup",
        color: None,
        extensions: &[
            ".rtf",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.rtf"),
        language_id: Some(51601661),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ring",
        language_type: "programming",
        color: Some("#2D54CB"),
        extensions: &[
            ".ring",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ring"),
        language_id: Some(431),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Riot",
        language_type: "markup",
        color: Some("#A71E49"),
        extensions: &[
            ".riot",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html"),
        tm_scope: Some("text.html.riot"),
        language_id: Some(878396783),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RobotFramework",
        language_type: "programming",
        color: Some("#00c0b5"),
        extensions: &[
            ".robot",
            ".resource",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("robot"),
        tm_scope: Some("text.robot"),
        language_id: Some(324),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Roc",
        language_type: "programming",
        color: Some("#7c38f5"),
        extensions: &[
            ".roc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.roc"),
        language_id: Some(440182480),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Rocq Prover",
        language_type: "programming",
        color: Some("#d0b68c"),
        extensions: &[
            ".v",
            ".coq",
        ],
        aliases: &[
            "coq",
            "rocq",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.coq"),
        language_id: Some(69),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Roff",
        language_type: "markup",
        color: Some("#ecdebe"),
        extensions: &[
            ".roff",
            ".1",
            ".1in",
            ".1m",
            ".1x",
            ".2",
            ".3",
            ".3in",
            ".3m",
            ".3p",
            ".3pm",
            ".3qt",
            ".3x",
            ".4",
            ".5",
            ".6",
            ".7",
            ".8",
            ".9",
            ".l",
            ".man",
            ".mdoc",
            ".me",
            ".ms",
            ".n",
            ".nr",
            ".rno",
            ".tmac",
        ],
        aliases: &[
            "groff",
            "man",
            "manpage",
            "man page",
            "man-page",
            "mdoc",
            "nroff",
            "troff",
        ],
        interpreters: &[
        ],
        filenames: &[
            "eqnrc",
            "mmn",
            "mmt",
            "troffrc",
            "troffrc-end",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.roff"),
        language_id: Some(141),
        codemirror_mode: Some("troff"),
        codemirror_mime_type: Some("text/troff"),
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Roff Manpage",
        language_type: "markup",
        color: Some("#ecdebe"),
        extensions: &[
            ".1",
            ".1in",
            ".1m",
            ".1x",
            ".2",
            ".3",
            ".3in",
            ".3m",
            ".3p",
            ".3pm",
            ".3qt",
            ".3x",
            ".4",
            ".5",
            ".6",
            ".7",
            ".8",
            ".9",
            ".man",
            ".mdoc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.roff"),
        language_id: Some(612669833),
        codemirror_mode: Some("troff"),
        codemirror_mime_type: Some("text/troff"),
        group: Some("Roff"),
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Rouge",
        language_type: "programming",
        color: Some("#cc0088"),
        extensions: &[
            ".rg",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("clojure"),
        tm_scope: Some("source.clojure"),
        language_id: Some(325),
        codemirror_mode: Some("clojure"),
        codemirror_mime_type: Some("text/x-clojure"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "RouterOS Script",
        language_type: "programming",
        color: Some("#DE3941"),
        extensions: &[
            ".rsc",
        ],
        aliases: &[
        ],
        interpreters: &[
            "RouterOS",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(592853203),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Ruby",
        language_type: "programming",
        color: Some("#701516"),
        extensions: &[
            ".rb",
            ".builder",
            ".eye",
            ".fcgi",
            ".gemspec",
            ".god",
            ".jbuilder",
            ".mspec",
            ".pluginspec",
            ".podspec",
            ".prawn",
            ".rabl",
            ".rake",
            ".rbi",
            ".rbuild",
            ".rbw",
            ".rbx",
            ".ru",
            ".ruby",
            ".spec",
            ".thor",
            ".watchr",
        ],
        aliases: &[
            "jruby",
            "macruby",
            "rake",
            "rb",
            "rbx",
        ],
        interpreters: &[
            "ruby",
            "macruby",
            "rake",
            "jruby",
            "rbx",
        ],
        filenames: &[
            ".irbrc",
            ".pryrc",
            ".simplecov",
            "Appraisals",
            "Berksfile",
            "Brewfile",
            "Buildfile",
            "Capfile",
            "Dangerfile",
            "Deliverfile",
            "Fastfile",
            "Gemfile",
            "Guardfile",
            "Jarfile",
            "Mavenfile",
            "Podfile",
            "Puppetfile",
            "Rakefile",
            "Snapfile",
            "Steepfile",
            "Thorfile",
            "Vagrantfile",
            "buildfile",
        ],
        ace_mode: Some("ruby"),
        tm_scope: Some("source.ruby"),
        language_id: Some(326),
        codemirror_mode: Some("ruby"),
        codemirror_mime_type: Some("text/x-ruby"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Rust",
        language_type: "programming",
        color: Some("#dea584"),
        extensions: &[
            ".rs",
            ".rs.in",
        ],
        aliases: &[
            "rs",
        ],
        interpreters: &[
            "rust-script",
        ],
        filenames: &[
        ],
        ace_mode: Some("rust"),
        tm_scope: Some("source.rust"),
        language_id: Some(327),
        codemirror_mode: Some("rust"),
        codemirror_mime_type: Some("text/x-rustsrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SAS",
        language_type: "programming",
        color: Some("#B34936"),
        extensions: &[
            ".sas",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.sas"),
        language_id: Some(328),
        codemirror_mode: Some("sas"),
        codemirror_mime_type: Some("text/x-sas"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SCSS",
        language_type: "markup",
        color: Some("#c6538c"),
        extensions: &[
            ".scss",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("scss"),
        tm_scope: Some("source.css.scss"),
        language_id: Some(329),
        codemirror_mode: Some("css"),
        codemirror_mime_type: Some("text/x-scss"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SELinux Policy",
        language_type: "data",
        color: None,
        extensions: &[
            ".te",
        ],
        aliases: &[
            "SELinux Kernel Policy Language",
            "sepolicy",
        ],
        interpreters: &[
        ],
        filenames: &[
            "file_contexts",
            "genfs_contexts",
            "initial_sids",
            "port_contexts",
            "security_classes",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.sepolicy"),
        language_id: Some(880010326),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SMT",
        language_type: "programming",
        color: None,
        extensions: &[
            ".smt2",
            ".smt",
            ".z3",
        ],
        aliases: &[
        ],
        interpreters: &[
            "boolector",
            "cvc4",
            "mathsat5",
            "opensmt",
            "smtinterpol",
            "smt-rat",
            "stp",
            "verit",
            "yices2",
            "z3",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.smt"),
        language_id: Some(330),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SPARQL",
        language_type: "data",
        color: Some("#0C4597"),
        extensions: &[
            ".sparql",
            ".rq",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("sparql"),
        tm_scope: Some("source.sparql"),
        language_id: Some(331),
        codemirror_mode: Some("sparql"),
        codemirror_mime_type: Some("application/sparql-query"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SQF",
        language_type: "programming",
        color: Some("#3F3F3F"),
        extensions: &[
            ".sqf",
            ".hqf",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.sqf"),
        language_id: Some(332),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SQL",
        language_type: "data",
        color: Some("#e38c00"),
        extensions: &[
            ".sql",
            ".cql",
            ".ddl",
            ".inc",
            ".mysql",
            ".prc",
            ".tab",
            ".udf",
            ".viw",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("sql"),
        tm_scope: Some("source.sql"),
        language_id: Some(333),
        codemirror_mode: Some("sql"),
        codemirror_mime_type: Some("text/x-sql"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SQLPL",
        language_type: "programming",
        color: Some("#e38c00"),
        extensions: &[
            ".sql",
            ".db2",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("sql"),
        tm_scope: Some("source.sql"),
        language_id: Some(334),
        codemirror_mode: Some("sql"),
        codemirror_mime_type: Some("text/x-sql"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SRecode Template",
        language_type: "markup",
        color: Some("#348a34"),
        extensions: &[
            ".srt",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.lisp"),
        language_id: Some(335),
        codemirror_mode: Some("commonlisp"),
        codemirror_mime_type: Some("text/x-common-lisp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SSH Config",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
            "sshconfig",
            "sshdconfig",
            "ssh_config",
            "sshd_config",
        ],
        interpreters: &[
        ],
        filenames: &[
            "ssh-config",
            "ssh_config",
            "sshconfig",
            "sshconfig.snip",
            "sshd-config",
            "sshd_config",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ssh-config"),
        language_id: Some(554920715),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("INI"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "STAR",
        language_type: "data",
        color: None,
        extensions: &[
            ".star",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.star"),
        language_id: Some(424510560),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "STL",
        language_type: "data",
        color: Some("#373b5e"),
        extensions: &[
            ".stl",
        ],
        aliases: &[
            "ascii stl",
            "stla",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.stl"),
        language_id: Some(455361735),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "STON",
        language_type: "data",
        color: None,
        extensions: &[
            ".ston",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.smalltalk"),
        language_id: Some(336),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Smalltalk"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SVG",
        language_type: "data",
        color: Some("#ff9900"),
        extensions: &[
            ".svg",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("svg"),
        tm_scope: Some("text.xml.svg"),
        language_id: Some(337),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SWIG",
        language_type: "programming",
        color: None,
        extensions: &[
            ".i",
            ".swg",
            ".swig",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c++"),
        language_id: Some(1066250075),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-c++src"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Sage",
        language_type: "programming",
        color: None,
        extensions: &[
            ".sage",
            ".sagews",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("python"),
        tm_scope: Some("source.python"),
        language_id: Some(338),
        codemirror_mode: Some("python"),
        codemirror_mime_type: Some("text/x-python"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Sail",
        language_type: "programming",
        color: Some("#259dd5"),
        extensions: &[
            ".sail",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.sail"),
        language_id: Some(1029438153),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SaltStack",
        language_type: "programming",
        color: Some("#646464"),
        extensions: &[
            ".sls",
        ],
        aliases: &[
            "saltstate",
            "salt",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.yaml.salt"),
        language_id: Some(339),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Sass",
        language_type: "markup",
        color: Some("#a53b70"),
        extensions: &[
            ".sass",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("sass"),
        tm_scope: Some("source.sass"),
        language_id: Some(340),
        codemirror_mode: Some("sass"),
        codemirror_mime_type: Some("text/x-sass"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Scala",
        language_type: "programming",
        color: Some("#c22d40"),
        extensions: &[
            ".scala",
            ".kojo",
            ".sbt",
            ".sc",
        ],
        aliases: &[
        ],
        interpreters: &[
            "scala",
        ],
        filenames: &[
        ],
        ace_mode: Some("scala"),
        tm_scope: Some("source.scala"),
        language_id: Some(341),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-scala"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Scaml",
        language_type: "markup",
        color: Some("#bd181a"),
        extensions: &[
            ".scaml",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.scaml"),
        language_id: Some(342),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Scenic",
        language_type: "programming",
        color: Some("#fdc700"),
        extensions: &[
            ".scenic",
        ],
        aliases: &[
        ],
        interpreters: &[
            "scenic",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.scenic"),
        language_id: Some(619814037),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Scheme",
        language_type: "programming",
        color: Some("#1e4aec"),
        extensions: &[
            ".scm",
            ".sch",
            ".sld",
            ".sls",
            ".sps",
            ".ss",
        ],
        aliases: &[
        ],
        interpreters: &[
            "scheme",
            "guile",
            "bigloo",
            "chicken",
            "csi",
            "gosh",
            "r6rs",
        ],
        filenames: &[
        ],
        ace_mode: Some("scheme"),
        tm_scope: Some("source.scheme"),
        language_id: Some(343),
        codemirror_mode: Some("scheme"),
        codemirror_mime_type: Some("text/x-scheme"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Scilab",
        language_type: "programming",
        color: Some("#ca0f21"),
        extensions: &[
            ".sci",
            ".sce",
            ".tst",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.scilab"),
        language_id: Some(344),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Self",
        language_type: "programming",
        color: Some("#0579aa"),
        extensions: &[
            ".self",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(345),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ShaderLab",
        language_type: "programming",
        color: Some("#222c37"),
        extensions: &[
            ".shader",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.shaderlab"),
        language_id: Some(664257356),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Shell",
        language_type: "programming",
        color: Some("#89e051"),
        extensions: &[
            ".sh",
            ".bash",
            ".bats",
            ".cgi",
            ".command",
            ".fcgi",
            ".ksh",
            ".sh.in",
            ".tmux",
            ".tool",
            ".trigger",
            ".zsh",
            ".zsh-theme",
        ],
        aliases: &[
            "sh",
            "shell-script",
            "bash",
            "zsh",
            "envrc",
        ],
        interpreters: &[
            "ash",
            "bash",
            "dash",
            "ksh",
            "mksh",
            "pdksh",
            "rc",
            "sh",
            "zsh",
        ],
        filenames: &[
            ".bash_aliases",
            ".bash_functions",
            ".bash_history",
            ".bash_logout",
            ".bash_profile",
            ".bashrc",
            ".cshrc",
            ".envrc",
            ".flaskenv",
            ".kshrc",
            ".login",
            ".profile",
            ".tmux.conf",
            ".xinitrc",
            ".xsession",
            ".zlogin",
            ".zlogout",
            ".zprofile",
            ".zshenv",
            ".zshrc",
            "9fs",
            "PKGBUILD",
            "bash_aliases",
            "bash_logout",
            "bash_profile",
            "bashrc",
            "cshrc",
            "gradlew",
            "kshrc",
            "login",
            "man",
            "mvnw",
            "profile",
            "tmux.conf",
            "xinitrc",
            "xsession",
            "zlogin",
            "zlogout",
            "zprofile",
            "zshenv",
            "zshrc",
        ],
        ace_mode: Some("sh"),
        tm_scope: Some("source.shell"),
        language_id: Some(346),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ShellCheck Config",
        language_type: "data",
        color: Some("#cecfcb"),
        extensions: &[
        ],
        aliases: &[
            "shellcheckrc",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".shellcheckrc",
        ],
        ace_mode: Some("ini"),
        tm_scope: Some("source.shellcheckrc"),
        language_id: Some(687511714),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ShellSession",
        language_type: "programming",
        color: None,
        extensions: &[
            ".sh-session",
        ],
        aliases: &[
            "bash session",
            "console",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("sh"),
        tm_scope: Some("text.shell-session"),
        language_id: Some(347),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Shen",
        language_type: "programming",
        color: Some("#120F14"),
        extensions: &[
            ".shen",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.shen"),
        language_id: Some(348),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Sieve",
        language_type: "programming",
        color: None,
        extensions: &[
            ".sieve",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.sieve"),
        language_id: Some(208976687),
        codemirror_mode: Some("sieve"),
        codemirror_mime_type: Some("application/sieve"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Simple File Verification",
        language_type: "data",
        color: Some("#C9BFED"),
        extensions: &[
            ".sfv",
        ],
        aliases: &[
            "sfv",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ini"),
        tm_scope: Some("source.sfv"),
        language_id: Some(735623761),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: Some("Checksums"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Singularity",
        language_type: "programming",
        color: Some("#64E6AD"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "Singularity",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.singularity"),
        language_id: Some(987024632),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Slang",
        language_type: "programming",
        color: Some("#1fbec9"),
        extensions: &[
            ".slang",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.slang"),
        language_id: Some(239357863),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Slash",
        language_type: "programming",
        color: Some("#007eff"),
        extensions: &[
            ".sl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.html.slash"),
        language_id: Some(349),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Slice",
        language_type: "programming",
        color: Some("#003fa2"),
        extensions: &[
            ".ice",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ice"),
        language_id: Some(894641667),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Slim",
        language_type: "markup",
        color: Some("#2b2b2b"),
        extensions: &[
            ".slim",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("slim"),
        tm_scope: Some("text.slim"),
        language_id: Some(350),
        codemirror_mode: Some("slim"),
        codemirror_mime_type: Some("text/x-slim"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Slint",
        language_type: "markup",
        color: Some("#2379F4"),
        extensions: &[
            ".slint",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.slint"),
        language_id: Some(119900149),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SmPL",
        language_type: "programming",
        color: Some("#c94949"),
        extensions: &[
            ".cocci",
        ],
        aliases: &[
            "coccinelle",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.smpl"),
        language_id: Some(164123055),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Smali",
        language_type: "programming",
        color: None,
        extensions: &[
            ".smali",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.smali"),
        language_id: Some(351),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Smalltalk",
        language_type: "programming",
        color: Some("#596706"),
        extensions: &[
            ".st",
            ".cs",
        ],
        aliases: &[
            "squeak",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.smalltalk"),
        language_id: Some(352),
        codemirror_mode: Some("smalltalk"),
        codemirror_mime_type: Some("text/x-stsrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Smarty",
        language_type: "programming",
        color: Some("#f0c040"),
        extensions: &[
            ".tpl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("smarty"),
        tm_scope: Some("text.html.smarty"),
        language_id: Some(353),
        codemirror_mode: Some("smarty"),
        codemirror_mime_type: Some("text/x-smarty"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Smithy",
        language_type: "programming",
        color: Some("#c44536"),
        extensions: &[
            ".smithy",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("smithy"),
        tm_scope: Some("source.smithy"),
        language_id: Some(1027892786),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Snakemake",
        language_type: "programming",
        color: Some("#419179"),
        extensions: &[
            ".smk",
            ".snakefile",
        ],
        aliases: &[
            "snakefile",
        ],
        interpreters: &[
        ],
        filenames: &[
            "Snakefile",
        ],
        ace_mode: Some("python"),
        tm_scope: Some("source.python"),
        language_id: Some(151241392),
        codemirror_mode: Some("python"),
        codemirror_mime_type: Some("text/x-python"),
        group: Some("Python"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Solidity",
        language_type: "programming",
        color: Some("#AA6746"),
        extensions: &[
            ".sol",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.solidity"),
        language_id: Some(237469032),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Soong",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "Android.bp",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.bp"),
        language_id: Some(222900098),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SourcePawn",
        language_type: "programming",
        color: Some("#f69e1d"),
        extensions: &[
            ".sp",
            ".inc",
        ],
        aliases: &[
            "sourcemod",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.sourcepawn"),
        language_id: Some(354),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Spline Font Database",
        language_type: "data",
        color: None,
        extensions: &[
            ".sfd",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("text.sfd"),
        language_id: Some(767169629),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Squirrel",
        language_type: "programming",
        color: Some("#800000"),
        extensions: &[
            ".nut",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.nut"),
        language_id: Some(355),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-squirrel"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Stan",
        language_type: "programming",
        color: Some("#b2011d"),
        extensions: &[
            ".stan",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.stan"),
        language_id: Some(356),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Standard ML",
        language_type: "programming",
        color: Some("#dc566d"),
        extensions: &[
            ".ml",
            ".fun",
            ".sig",
            ".sml",
        ],
        aliases: &[
            "sml",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ml"),
        language_id: Some(357),
        codemirror_mode: Some("mllike"),
        codemirror_mime_type: Some("text/x-sml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Starlark",
        language_type: "programming",
        color: Some("#76d275"),
        extensions: &[
            ".bzl",
            ".star",
        ],
        aliases: &[
            "bazel",
            "bzl",
        ],
        interpreters: &[
        ],
        filenames: &[
            "BUCK",
            "BUILD",
            "BUILD.bazel",
            "MODULE.bazel",
            "Tiltfile",
            "WORKSPACE",
            "WORKSPACE.bazel",
            "WORKSPACE.bzlmod",
        ],
        ace_mode: Some("python"),
        tm_scope: Some("source.python"),
        language_id: Some(960266174),
        codemirror_mode: Some("python"),
        codemirror_mime_type: Some("text/x-python"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Stata",
        language_type: "programming",
        color: Some("#1a5f91"),
        extensions: &[
            ".do",
            ".ado",
            ".doh",
            ".ihlp",
            ".mata",
            ".matah",
            ".sthlp",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.stata"),
        language_id: Some(358),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "StringTemplate",
        language_type: "markup",
        color: Some("#3fb34f"),
        extensions: &[
            ".st",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html"),
        tm_scope: Some("source.string-template"),
        language_id: Some(89855901),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Stylus",
        language_type: "markup",
        color: Some("#ff6347"),
        extensions: &[
            ".styl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("stylus"),
        tm_scope: Some("source.stylus"),
        language_id: Some(359),
        codemirror_mode: Some("stylus"),
        codemirror_mime_type: Some("text/x-styl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SubRip Text",
        language_type: "data",
        color: Some("#9e0101"),
        extensions: &[
            ".srt",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.srt"),
        language_id: Some(360),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SugarSS",
        language_type: "markup",
        color: Some("#2fcc9f"),
        extensions: &[
            ".sss",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.css.postcss.sugarss"),
        language_id: Some(826404698),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SuperCollider",
        language_type: "programming",
        color: Some("#46390b"),
        extensions: &[
            ".sc",
            ".scd",
        ],
        aliases: &[
        ],
        interpreters: &[
            "sclang",
            "scsynth",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.supercollider"),
        language_id: Some(361),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Survex data",
        language_type: "data",
        color: Some("#ffcc99"),
        extensions: &[
            ".svx",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(24470517),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Svelte",
        language_type: "markup",
        color: Some("#ff3e00"),
        extensions: &[
            ".svelte",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("html"),
        tm_scope: Some("source.svelte"),
        language_id: Some(928734530),
        codemirror_mode: Some("htmlmixed"),
        codemirror_mime_type: Some("text/html"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Sway",
        language_type: "programming",
        color: Some("#00F58C"),
        extensions: &[
            ".sw",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("rust"),
        tm_scope: Some("source.sway"),
        language_id: Some(271471144),
        codemirror_mode: Some("rust"),
        codemirror_mime_type: Some("text/x-rustsrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Sweave",
        language_type: "prose",
        color: Some("#198ce7"),
        extensions: &[
            ".rnw",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("tex"),
        tm_scope: Some("text.tex.latex.sweave"),
        language_id: Some(558779190),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Swift",
        language_type: "programming",
        color: Some("#F05138"),
        extensions: &[
            ".swift",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("swift"),
        tm_scope: Some("source.swift"),
        language_id: Some(362),
        codemirror_mode: Some("swift"),
        codemirror_mime_type: Some("text/x-swift"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "SystemVerilog",
        language_type: "programming",
        color: Some("#DAE1C2"),
        extensions: &[
            ".sv",
            ".svh",
            ".vh",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("verilog"),
        tm_scope: Some("source.systemverilog"),
        language_id: Some(363),
        codemirror_mode: Some("verilog"),
        codemirror_mime_type: Some("text/x-systemverilog"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TI Program",
        language_type: "programming",
        color: Some("#A0AA87"),
        extensions: &[
            ".8xp",
            ".8xp.txt",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.8xp"),
        language_id: Some(422),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TL-Verilog",
        language_type: "programming",
        color: Some("#C40023"),
        extensions: &[
            ".tlv",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("verilog"),
        tm_scope: Some("source.tlverilog"),
        language_id: Some(118656070),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TLA",
        language_type: "programming",
        color: Some("#4b0079"),
        extensions: &[
            ".tla",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.tla"),
        language_id: Some(364),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TOML",
        language_type: "data",
        color: Some("#9c4221"),
        extensions: &[
            ".toml",
            ".toml.example",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "Cargo.lock",
            "Cargo.toml.orig",
            "Gopkg.lock",
            "Pipfile",
            "pdm.lock",
            "poetry.lock",
            "uv.lock",
        ],
        ace_mode: Some("toml"),
        tm_scope: Some("source.toml"),
        language_id: Some(365),
        codemirror_mode: Some("toml"),
        codemirror_mime_type: Some("text/x-toml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TSPLIB data",
        language_type: "data",
        color: None,
        extensions: &[
            ".tsp",
        ],
        aliases: &[
            "travelling salesman problem",
            "traveling salesman problem",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(89289301),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TSQL",
        language_type: "programming",
        color: Some("#e38c00"),
        extensions: &[
            ".sql",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("sql"),
        tm_scope: Some("source.tsql"),
        language_id: Some(918334941),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TSV",
        language_type: "data",
        color: Some("#237346"),
        extensions: &[
            ".tsv",
            ".vcf",
        ],
        aliases: &[
            "tab-seperated values",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("tsv"),
        tm_scope: Some("source.generic-db"),
        language_id: Some(1035892117),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TSX",
        language_type: "programming",
        color: Some("#3178c6"),
        extensions: &[
            ".tsx",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("tsx"),
        tm_scope: Some("source.tsx"),
        language_id: Some(94901924),
        codemirror_mode: Some("jsx"),
        codemirror_mime_type: Some("text/typescript-jsx"),
        group: Some("TypeScript"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TXL",
        language_type: "programming",
        color: Some("#0178b8"),
        extensions: &[
            ".txl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.txl"),
        language_id: Some(366),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Tact",
        language_type: "programming",
        color: Some("#48b5ff"),
        extensions: &[
            ".tact",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.tact"),
        language_id: Some(606708469),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Talon",
        language_type: "programming",
        color: Some("#333333"),
        extensions: &[
            ".talon",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.talon"),
        language_id: Some(959889508),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Tcl",
        language_type: "programming",
        color: Some("#e4cc98"),
        extensions: &[
            ".tcl",
            ".adp",
            ".sdc",
            ".tcl.in",
            ".tm",
            ".xdc",
        ],
        aliases: &[
            "sdc",
            "xdc",
        ],
        interpreters: &[
            "tclsh",
            "wish",
        ],
        filenames: &[
            "owh",
            "starfield",
        ],
        ace_mode: Some("tcl"),
        tm_scope: Some("source.tcl"),
        language_id: Some(367),
        codemirror_mode: Some("tcl"),
        codemirror_mime_type: Some("text/x-tcl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Tcsh",
        language_type: "programming",
        color: None,
        extensions: &[
            ".tcsh",
            ".csh",
        ],
        aliases: &[
        ],
        interpreters: &[
            "tcsh",
            "csh",
        ],
        filenames: &[
        ],
        ace_mode: Some("sh"),
        tm_scope: Some("source.shell"),
        language_id: Some(368),
        codemirror_mode: Some("shell"),
        codemirror_mime_type: Some("text/x-sh"),
        group: Some("Shell"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TeX",
        language_type: "markup",
        color: Some("#3D6117"),
        extensions: &[
            ".tex",
            ".aux",
            ".bbx",
            ".cbx",
            ".cls",
            ".dtx",
            ".ins",
            ".lbx",
            ".ltx",
            ".mkii",
            ".mkiv",
            ".mkvi",
            ".sty",
            ".toc",
        ],
        aliases: &[
            "latex",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("tex"),
        tm_scope: Some("text.tex.latex"),
        language_id: Some(369),
        codemirror_mode: Some("stex"),
        codemirror_mime_type: Some("text/x-stex"),
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Tea",
        language_type: "markup",
        color: None,
        extensions: &[
            ".tea",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.tea"),
        language_id: Some(370),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Teal",
        language_type: "programming",
        color: Some("#00B1BC"),
        extensions: &[
            ".tl",
        ],
        aliases: &[
        ],
        interpreters: &[
            "tl",
        ],
        filenames: &[
        ],
        ace_mode: Some("lua"),
        tm_scope: Some("source.teal"),
        language_id: Some(719038619),
        codemirror_mode: Some("lua"),
        codemirror_mime_type: Some("text/x-lua"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Terra",
        language_type: "programming",
        color: Some("#00004c"),
        extensions: &[
            ".t",
        ],
        aliases: &[
        ],
        interpreters: &[
            "lua",
        ],
        filenames: &[
        ],
        ace_mode: Some("lua"),
        tm_scope: Some("source.terra"),
        language_id: Some(371),
        codemirror_mode: Some("lua"),
        codemirror_mime_type: Some("text/x-lua"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Terraform Template",
        language_type: "markup",
        color: Some("#7b42bb"),
        extensions: &[
            ".tftpl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ruby"),
        tm_scope: Some("source.hcl.terraform"),
        language_id: Some(856832701),
        codemirror_mode: Some("ruby"),
        codemirror_mime_type: Some("text/x-ruby"),
        group: Some("HCL"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Texinfo",
        language_type: "prose",
        color: None,
        extensions: &[
            ".texinfo",
            ".texi",
            ".txi",
        ],
        aliases: &[
        ],
        interpreters: &[
            "makeinfo",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.texinfo"),
        language_id: Some(988020015),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Text",
        language_type: "prose",
        color: None,
        extensions: &[
            ".txt",
            ".fr",
            ".nb",
            ".ncl",
            ".no",
        ],
        aliases: &[
            "fundamental",
            "plain text",
        ],
        interpreters: &[
        ],
        filenames: &[
            "CITATION",
            "CITATIONS",
            "COPYING",
            "COPYING.regex",
            "COPYRIGHT.regex",
            "FONTLOG",
            "INSTALL",
            "INSTALL.mysql",
            "LICENSE",
            "LICENSE.mysql",
            "NEWS",
            "README.me",
            "README.mysql",
            "README.nss",
            "click.me",
            "delete.me",
            "keep.me",
            "package.mask",
            "package.use.mask",
            "package.use.stable.mask",
            "read.me",
            "readme.1st",
            "test.me",
            "use.mask",
            "use.stable.mask",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(372),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "TextGrid",
        language_type: "data",
        color: Some("#c8506d"),
        extensions: &[
            ".TextGrid",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.textgrid"),
        language_id: Some(965696054),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TextMate Properties",
        language_type: "data",
        color: Some("#df66e4"),
        extensions: &[
        ],
        aliases: &[
            "tm-properties",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".tm_properties",
        ],
        ace_mode: Some("properties"),
        tm_scope: Some("source.tm-properties"),
        language_id: Some(981795023),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Textile",
        language_type: "prose",
        color: Some("#ffe7ac"),
        extensions: &[
            ".textile",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("textile"),
        tm_scope: Some("none"),
        language_id: Some(373),
        codemirror_mode: Some("textile"),
        codemirror_mime_type: Some("text/x-textile"),
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Thrift",
        language_type: "programming",
        color: Some("#D12127"),
        extensions: &[
            ".thrift",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.thrift"),
        language_id: Some(374),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Toit",
        language_type: "programming",
        color: Some("#c2c9fb"),
        extensions: &[
            ".toit",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.toit"),
        language_id: Some(356554395),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Tor Config",
        language_type: "data",
        color: Some("#59316b"),
        extensions: &[
        ],
        aliases: &[
            "torrc",
        ],
        interpreters: &[
        ],
        filenames: &[
            "torrc",
        ],
        ace_mode: Some("apache_conf"),
        tm_scope: Some("source.torrc"),
        language_id: Some(1016912802),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Tree-sitter Query",
        language_type: "programming",
        color: Some("#8ea64c"),
        extensions: &[
            ".scm",
        ],
        aliases: &[
            "tsq",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.scm"),
        language_id: Some(436081647),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Turing",
        language_type: "programming",
        color: Some("#cf142b"),
        extensions: &[
            ".t",
            ".tu",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.turing"),
        language_id: Some(375),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Turtle",
        language_type: "data",
        color: None,
        extensions: &[
            ".ttl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("turtle"),
        tm_scope: Some("source.turtle"),
        language_id: Some(376),
        codemirror_mode: Some("turtle"),
        codemirror_mime_type: Some("text/turtle"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Twig",
        language_type: "markup",
        color: Some("#c1d026"),
        extensions: &[
            ".twig",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("twig"),
        tm_scope: Some("text.html.twig"),
        language_id: Some(377),
        codemirror_mode: Some("twig"),
        codemirror_mime_type: Some("text/x-twig"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Type Language",
        language_type: "data",
        color: None,
        extensions: &[
            ".tl",
        ],
        aliases: &[
            "tl",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.tl"),
        language_id: Some(632765617),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TypeScript",
        language_type: "programming",
        color: Some("#3178c6"),
        extensions: &[
            ".ts",
            ".cts",
            ".mts",
        ],
        aliases: &[
            "ts",
        ],
        interpreters: &[
            "bun",
            "deno",
            "ts-node",
            "tsx",
        ],
        filenames: &[
        ],
        ace_mode: Some("typescript"),
        tm_scope: Some("source.ts"),
        language_id: Some(378),
        codemirror_mode: Some("javascript"),
        codemirror_mime_type: Some("application/typescript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "TypeSpec",
        language_type: "programming",
        color: Some("#4A3665"),
        extensions: &[
            ".tsp",
        ],
        aliases: &[
            "tsp",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.tsp"),
        language_id: Some(952272597),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Typst",
        language_type: "programming",
        color: Some("#239dad"),
        extensions: &[
            ".typ",
        ],
        aliases: &[
            "typ",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.typst"),
        language_id: Some(704730682),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Unified Parallel C",
        language_type: "programming",
        color: Some("#4e3617"),
        extensions: &[
            ".upc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c"),
        language_id: Some(379),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: Some("C"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Unity3D Asset",
        language_type: "data",
        color: Some("#222c37"),
        extensions: &[
            ".anim",
            ".asset",
            ".mask",
            ".mat",
            ".meta",
            ".prefab",
            ".unity",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.yaml"),
        language_id: Some(380),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Unix Assembly",
        language_type: "programming",
        color: None,
        extensions: &[
            ".s",
            ".ms",
        ],
        aliases: &[
            "gas",
            "gnu asm",
            "unix asm",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("assembly_x86"),
        tm_scope: Some("source.x86"),
        language_id: Some(120),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Assembly"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Uno",
        language_type: "programming",
        color: Some("#9933cc"),
        extensions: &[
            ".uno",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("csharp"),
        tm_scope: Some("source.cs"),
        language_id: Some(381),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csharp"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "UnrealScript",
        language_type: "programming",
        color: Some("#a54c4d"),
        extensions: &[
            ".uc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("java"),
        tm_scope: Some("source.java"),
        language_id: Some(382),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-java"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Untyped Plutus Core",
        language_type: "programming",
        color: Some("#36adbd"),
        extensions: &[
            ".uplc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.uplc"),
        language_id: Some(1061635506),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "UrWeb",
        language_type: "programming",
        color: Some("#ccccee"),
        extensions: &[
            ".ur",
            ".urs",
        ],
        aliases: &[
            "Ur/Web",
            "Ur",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ur"),
        language_id: Some(383),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "V",
        language_type: "programming",
        color: Some("#4f87c4"),
        extensions: &[
            ".v",
        ],
        aliases: &[
            "vlang",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("golang"),
        tm_scope: Some("source.v"),
        language_id: Some(603371597),
        codemirror_mode: Some("go"),
        codemirror_mime_type: Some("text/x-go"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "VBA",
        language_type: "programming",
        color: Some("#867db1"),
        extensions: &[
            ".bas",
            ".cls",
            ".frm",
            ".vba",
        ],
        aliases: &[
            "visual basic for applications",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.vba"),
        language_id: Some(399230729),
        codemirror_mode: Some("vb"),
        codemirror_mime_type: Some("text/x-vb"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "VBScript",
        language_type: "programming",
        color: Some("#15dcdc"),
        extensions: &[
            ".vbs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("vbscript"),
        tm_scope: Some("source.vbnet"),
        language_id: Some(408016005),
        codemirror_mode: Some("vbscript"),
        codemirror_mime_type: Some("text/vbscript"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "VCL",
        language_type: "programming",
        color: Some("#148AA8"),
        extensions: &[
            ".vcl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.vcl"),
        language_id: Some(384),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "VHDL",
        language_type: "programming",
        color: Some("#adb2cb"),
        extensions: &[
            ".vhdl",
            ".vhd",
            ".vhf",
            ".vhi",
            ".vho",
            ".vhs",
            ".vht",
            ".vhw",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("vhdl"),
        tm_scope: Some("source.vhdl"),
        language_id: Some(385),
        codemirror_mode: Some("vhdl"),
        codemirror_mime_type: Some("text/x-vhdl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Vala",
        language_type: "programming",
        color: Some("#a56de2"),
        extensions: &[
            ".vala",
            ".vapi",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("vala"),
        tm_scope: Some("source.vala"),
        language_id: Some(386),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Valve Data Format",
        language_type: "data",
        color: Some("#f26025"),
        extensions: &[
            ".vdf",
        ],
        aliases: &[
            "keyvalues",
            "vdf",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.keyvalues"),
        language_id: Some(544060961),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Velocity Template Language",
        language_type: "markup",
        color: Some("#507cff"),
        extensions: &[
            ".vtl",
        ],
        aliases: &[
            "vtl",
            "velocity",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("velocity"),
        tm_scope: Some("source.velocity"),
        language_id: Some(292377326),
        codemirror_mode: Some("velocity"),
        codemirror_mime_type: Some("text/velocity"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Vento",
        language_type: "markup",
        color: Some("#ff0080"),
        extensions: &[
            ".vto",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.vento"),
        language_id: Some(757053899),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Verilog",
        language_type: "programming",
        color: Some("#b2b7f8"),
        extensions: &[
            ".v",
            ".veo",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("verilog"),
        tm_scope: Some("source.verilog"),
        language_id: Some(387),
        codemirror_mode: Some("verilog"),
        codemirror_mime_type: Some("text/x-verilog"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Vim Help File",
        language_type: "prose",
        color: Some("#199f4b"),
        extensions: &[
            ".txt",
        ],
        aliases: &[
            "help",
            "vimhelp",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.vim-help"),
        language_id: Some(508563686),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Vim Script",
        language_type: "programming",
        color: Some("#199f4b"),
        extensions: &[
            ".vim",
            ".vba",
            ".vimrc",
            ".vmb",
        ],
        aliases: &[
            "vim",
            "viml",
            "nvim",
            "vimscript",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".exrc",
            ".gvimrc",
            ".nvimrc",
            ".vimrc",
            "_vimrc",
            "gvimrc",
            "nvimrc",
            "vimrc",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.viml"),
        language_id: Some(388),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Vim Snippet",
        language_type: "markup",
        color: Some("#199f4b"),
        extensions: &[
            ".snip",
            ".snippet",
            ".snippets",
        ],
        aliases: &[
            "SnipMate",
            "UltiSnip",
            "UltiSnips",
            "NeoSnippet",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.vim-snippet"),
        language_id: Some(81265970),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Visual Basic .NET",
        language_type: "programming",
        color: Some("#945db7"),
        extensions: &[
            ".vb",
            ".vbhtml",
        ],
        aliases: &[
            "visual basic",
            "vbnet",
            "vb .net",
            "vb.net",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.vbnet"),
        language_id: Some(389),
        codemirror_mode: Some("vb"),
        codemirror_mime_type: Some("text/x-vb"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Visual Basic 6.0",
        language_type: "programming",
        color: Some("#2c6353"),
        extensions: &[
            ".bas",
            ".cls",
            ".ctl",
            ".Dsr",
            ".frm",
        ],
        aliases: &[
            "vb6",
            "vb 6",
            "visual basic 6",
            "visual basic classic",
            "classic visual basic",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.vba"),
        language_id: Some(679594952),
        codemirror_mode: Some("vb"),
        codemirror_mime_type: Some("text/x-vb"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Volt",
        language_type: "programming",
        color: Some("#1F1F1F"),
        extensions: &[
            ".volt",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("d"),
        tm_scope: Some("source.d"),
        language_id: Some(390),
        codemirror_mode: Some("d"),
        codemirror_mime_type: Some("text/x-d"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Vue",
        language_type: "markup",
        color: Some("#41b883"),
        extensions: &[
            ".vue",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("vue"),
        tm_scope: Some("source.vue"),
        language_id: Some(391),
        codemirror_mode: Some("vue"),
        codemirror_mime_type: Some("text/x-vue"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Vyper",
        language_type: "programming",
        color: Some("#9F4CF2"),
        extensions: &[
            ".vy",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.vyper"),
        language_id: Some(1055641948),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "WDL",
        language_type: "programming",
        color: Some("#42f1f4"),
        extensions: &[
            ".wdl",
        ],
        aliases: &[
            "Workflow Description Language",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.wdl"),
        language_id: Some(374521672),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "WGSL",
        language_type: "programming",
        color: Some("#1a5e9a"),
        extensions: &[
            ".wgsl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.wgsl"),
        language_id: Some(836605993),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Wavefront Material",
        language_type: "data",
        color: None,
        extensions: &[
            ".mtl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.wavefront.mtl"),
        language_id: Some(392),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Wavefront Object",
        language_type: "data",
        color: None,
        extensions: &[
            ".obj",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.wavefront.obj"),
        language_id: Some(393),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Web Ontology Language",
        language_type: "data",
        color: Some("#5b70bd"),
        extensions: &[
            ".owl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml"),
        language_id: Some(394),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "WebAssembly",
        language_type: "programming",
        color: Some("#04133b"),
        extensions: &[
            ".wast",
            ".wat",
        ],
        aliases: &[
            "wast",
            "wasm",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("lisp"),
        tm_scope: Some("source.webassembly"),
        language_id: Some(956556503),
        codemirror_mode: Some("wast"),
        codemirror_mime_type: Some("text/webassembly"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "WebAssembly Interface Type",
        language_type: "data",
        color: Some("#6250e7"),
        extensions: &[
            ".wit",
        ],
        aliases: &[
            "wit",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.wit"),
        language_id: Some(134534086),
        codemirror_mode: Some("webidl"),
        codemirror_mime_type: Some("text/x-webidl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "WebIDL",
        language_type: "programming",
        color: None,
        extensions: &[
            ".webidl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.webidl"),
        language_id: Some(395),
        codemirror_mode: Some("webidl"),
        codemirror_mime_type: Some("text/x-webidl"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "WebVTT",
        language_type: "data",
        color: None,
        extensions: &[
            ".vtt",
        ],
        aliases: &[
            "vtt",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.vtt"),
        language_id: Some(658679714),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Wget Config",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
            "wgetrc",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".wgetrc",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.wgetrc"),
        language_id: Some(668457123),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("INI"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Whiley",
        language_type: "programming",
        color: Some("#d5c397"),
        extensions: &[
            ".whiley",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.whiley"),
        language_id: Some(888779559),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Wikitext",
        language_type: "prose",
        color: Some("#fc5757"),
        extensions: &[
            ".mediawiki",
            ".wiki",
            ".wikitext",
        ],
        aliases: &[
            "mediawiki",
            "wiki",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("mediawiki"),
        tm_scope: Some("text.html.mediawiki"),
        language_id: Some(228),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "Win32 Message File",
        language_type: "data",
        color: None,
        extensions: &[
            ".mc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ini"),
        tm_scope: Some("source.win32-messages"),
        language_id: Some(950967261),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Windows Registry Entries",
        language_type: "data",
        color: Some("#52d5ff"),
        extensions: &[
            ".reg",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("ini"),
        tm_scope: Some("source.reg"),
        language_id: Some(969674868),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Witcher Script",
        language_type: "programming",
        color: Some("#ff0000"),
        extensions: &[
            ".ws",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.witcherscript"),
        language_id: Some(686821385),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Wolfram Language",
        language_type: "programming",
        color: Some("#dd1100"),
        extensions: &[
            ".mathematica",
            ".cdf",
            ".m",
            ".ma",
            ".mt",
            ".nb",
            ".nbp",
            ".wl",
            ".wls",
            ".wlt",
        ],
        aliases: &[
            "mathematica",
            "mma",
            "wolfram",
            "wolfram lang",
            "wl",
        ],
        interpreters: &[
            "wolfram",
            "WolframKernel",
            "wolframscript",
            "math",
            "MathKernel",
            "MathematicaScript",
            "WolframNB",
            "Mathematica",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.mathematica"),
        language_id: Some(224),
        codemirror_mode: Some("mathematica"),
        codemirror_mime_type: Some("text/x-mathematica"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Wollok",
        language_type: "programming",
        color: Some("#a23738"),
        extensions: &[
            ".wlk",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("wollok"),
        tm_scope: Some("source.wollok"),
        language_id: Some(632745969),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "World of Warcraft Addon Data",
        language_type: "data",
        color: Some("#f7e43f"),
        extensions: &[
            ".toc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.toc"),
        language_id: Some(396),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Wren",
        language_type: "programming",
        color: Some("#383838"),
        extensions: &[
            ".wren",
        ],
        aliases: &[
            "wrenlang",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.wren"),
        language_id: Some(713580619),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "X BitMap",
        language_type: "data",
        color: None,
        extensions: &[
            ".xbm",
        ],
        aliases: &[
            "xbm",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c"),
        language_id: Some(782911107),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: Some("C"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "X Font Directory Index",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "encodings.dir",
            "fonts.alias",
            "fonts.dir",
            "fonts.scale",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.fontdir"),
        language_id: Some(208700028),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "X PixMap",
        language_type: "data",
        color: None,
        extensions: &[
            ".xpm",
            ".pm",
        ],
        aliases: &[
            "xpm",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c"),
        language_id: Some(781846279),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: Some("C"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "X10",
        language_type: "programming",
        color: Some("#4B6BEF"),
        extensions: &[
            ".x10",
        ],
        aliases: &[
            "xten",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.x10"),
        language_id: Some(397),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "XC",
        language_type: "programming",
        color: Some("#99DA07"),
        extensions: &[
            ".xc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.xc"),
        language_id: Some(398),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "XCompose",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            ".XCompose",
            "XCompose",
            "xcompose",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("config.xcompose"),
        language_id: Some(225167241),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "XML",
        language_type: "data",
        color: Some("#0060ac"),
        extensions: &[
            ".xml",
            ".adml",
            ".admx",
            ".ant",
            ".axaml",
            ".axml",
            ".builds",
            ".ccproj",
            ".ccxml",
            ".clixml",
            ".cproject",
            ".cscfg",
            ".csdef",
            ".csl",
            ".csproj",
            ".ct",
            ".depproj",
            ".dita",
            ".ditamap",
            ".ditaval",
            ".dll.config",
            ".dotsettings",
            ".filters",
            ".fsproj",
            ".fxml",
            ".glade",
            ".gml",
            ".gmx",
            ".gpx",
            ".grxml",
            ".gst",
            ".hzp",
            ".iml",
            ".ivy",
            ".jelly",
            ".jsproj",
            ".kml",
            ".launch",
            ".mdpolicy",
            ".mjml",
            ".mm",
            ".mod",
            ".mojo",
            ".mxml",
            ".natvis",
            ".ncl",
            ".ndproj",
            ".nproj",
            ".nuspec",
            ".odd",
            ".osm",
            ".pkgproj",
            ".pluginspec",
            ".proj",
            ".props",
            ".ps1xml",
            ".psc1",
            ".pt",
            ".qhelp",
            ".rdf",
            ".res",
            ".resx",
            ".rs",
            ".rss",
            ".sch",
            ".scxml",
            ".sfproj",
            ".shproj",
            ".slnx",
            ".srdf",
            ".storyboard",
            ".sublime-snippet",
            ".sw",
            ".targets",
            ".tml",
            ".ts",
            ".tsx",
            ".typ",
            ".ui",
            ".urdf",
            ".ux",
            ".vbproj",
            ".vcxproj",
            ".vsixmanifest",
            ".vssettings",
            ".vstemplate",
            ".vxml",
            ".wixproj",
            ".workflow",
            ".wsdl",
            ".wsf",
            ".wxi",
            ".wxl",
            ".wxs",
            ".x3d",
            ".xacro",
            ".xaml",
            ".xib",
            ".xlf",
            ".xliff",
            ".xmi",
            ".xml.dist",
            ".xmp",
            ".xproj",
            ".xsd",
            ".xspec",
            ".xul",
            ".zcml",
        ],
        aliases: &[
            "rss",
            "xsd",
            "wsdl",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".classpath",
            ".cproject",
            ".project",
            "App.config",
            "NuGet.config",
            "Settings.StyleCop",
            "Web.Debug.config",
            "Web.Release.config",
            "Web.config",
            "packages.config",
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml"),
        language_id: Some(399),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "XML Property List",
        language_type: "data",
        color: Some("#0060ac"),
        extensions: &[
            ".plist",
            ".stTheme",
            ".tmCommand",
            ".tmLanguage",
            ".tmPreferences",
            ".tmSnippet",
            ".tmTheme",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml.plist"),
        language_id: Some(75622871),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: Some("XML"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "XPages",
        language_type: "data",
        color: None,
        extensions: &[
            ".xsp-config",
            ".xsp.metadata",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml"),
        language_id: Some(400),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "XProc",
        language_type: "programming",
        color: None,
        extensions: &[
            ".xpl",
            ".xproc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml"),
        language_id: Some(401),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "XQuery",
        language_type: "programming",
        color: Some("#5232e7"),
        extensions: &[
            ".xquery",
            ".xq",
            ".xql",
            ".xqm",
            ".xqy",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xquery"),
        tm_scope: Some("source.xq"),
        language_id: Some(402),
        codemirror_mode: Some("xquery"),
        codemirror_mime_type: Some("application/xquery"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "XS",
        language_type: "programming",
        color: None,
        extensions: &[
            ".xs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("c_cpp"),
        tm_scope: Some("source.c"),
        language_id: Some(403),
        codemirror_mode: Some("clike"),
        codemirror_mime_type: Some("text/x-csrc"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "XSLT",
        language_type: "programming",
        color: Some("#EB8CEB"),
        extensions: &[
            ".xslt",
            ".xsl",
        ],
        aliases: &[
            "xsl",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("xml"),
        tm_scope: Some("text.xml.xsl"),
        language_id: Some(404),
        codemirror_mode: Some("xml"),
        codemirror_mime_type: Some("text/xml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Xmake",
        language_type: "programming",
        color: Some("#22a079"),
        extensions: &[
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            "xmake.lua",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.xmake"),
        language_id: Some(225223071),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Xojo",
        language_type: "programming",
        color: Some("#81bd41"),
        extensions: &[
            ".xojo_code",
            ".xojo_menu",
            ".xojo_report",
            ".xojo_script",
            ".xojo_toolbar",
            ".xojo_window",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.xojo"),
        language_id: Some(405),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Xonsh",
        language_type: "programming",
        color: Some("#285EEF"),
        extensions: &[
            ".xsh",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.python"),
        language_id: Some(614078284),
        codemirror_mode: Some("python"),
        codemirror_mime_type: Some("text/x-python"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Xtend",
        language_type: "programming",
        color: Some("#24255d"),
        extensions: &[
            ".xtend",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.xtend"),
        language_id: Some(406),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "YAML",
        language_type: "data",
        color: Some("#cb171e"),
        extensions: &[
            ".yml",
            ".mir",
            ".reek",
            ".rviz",
            ".sublime-syntax",
            ".syntax",
            ".yaml",
            ".yaml-tmlanguage",
            ".yaml.sed",
            ".yml.mysql",
        ],
        aliases: &[
            "yml",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".clang-format",
            ".clang-tidy",
            ".clangd",
            ".gemrc",
            "CITATION.cff",
            "glide.lock",
            "pixi.lock",
            "yarn.lock",
        ],
        ace_mode: Some("yaml"),
        tm_scope: Some("source.yaml"),
        language_id: Some(407),
        codemirror_mode: Some("yaml"),
        codemirror_mime_type: Some("text/x-yaml"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "YANG",
        language_type: "data",
        color: None,
        extensions: &[
            ".yang",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.yang"),
        language_id: Some(408),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "YARA",
        language_type: "programming",
        color: Some("#220000"),
        extensions: &[
            ".yar",
            ".yara",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.yara"),
        language_id: Some(805122868),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "YASnippet",
        language_type: "markup",
        color: Some("#32AB90"),
        extensions: &[
            ".yasnippet",
        ],
        aliases: &[
            "snippet",
            "yas",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.yasnippet"),
        language_id: Some(378760102),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Yacc",
        language_type: "programming",
        color: Some("#4B6C4B"),
        extensions: &[
            ".y",
            ".yacc",
            ".yy",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.yacc"),
        language_id: Some(409),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Yul",
        language_type: "programming",
        color: Some("#794932"),
        extensions: &[
            ".yul",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.yul"),
        language_id: Some(237469033),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ZAP",
        language_type: "programming",
        color: Some("#0d665e"),
        extensions: &[
            ".zap",
            ".xzap",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.zap"),
        language_id: Some(952972794),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ZIL",
        language_type: "programming",
        color: Some("#dc75e5"),
        extensions: &[
            ".zil",
            ".mud",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.zil"),
        language_id: Some(973483626),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Zeek",
        language_type: "programming",
        color: None,
        extensions: &[
            ".zeek",
            ".bro",
        ],
        aliases: &[
            "bro",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("zeek"),
        tm_scope: Some("source.zeek"),
        language_id: Some(40),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ZenScript",
        language_type: "programming",
        color: Some("#00BCD1"),
        extensions: &[
            ".zs",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.zenscript"),
        language_id: Some(494938890),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Zephir",
        language_type: "programming",
        color: Some("#118f9e"),
        extensions: &[
            ".zep",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("php"),
        tm_scope: Some("source.php.zephir"),
        language_id: Some(410),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Zig",
        language_type: "programming",
        color: Some("#ec915c"),
        extensions: &[
            ".zig",
            ".zig.zon",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("zig"),
        tm_scope: Some("source.zig"),
        language_id: Some(646424281),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Zimpl",
        language_type: "programming",
        color: Some("#d67711"),
        extensions: &[
            ".zimpl",
            ".zmpl",
            ".zpl",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("none"),
        language_id: Some(411),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "Zmodel",
        language_type: "data",
        color: Some("#ff7100"),
        extensions: &[
            ".zmodel",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.zmodel"),
        language_id: Some(803760908),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "cURL Config",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
            "curlrc",
        ],
        interpreters: &[
        ],
        filenames: &[
            ".curlrc",
            "_curlrc",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.curlrc"),
        language_id: Some(992375436),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("INI"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "crontab",
        language_type: "data",
        color: Some("#ead7ac"),
        extensions: &[
        ],
        aliases: &[
            "cron",
            "cron table",
        ],
        interpreters: &[
        ],
        filenames: &[
            "crontab",
        ],
        ace_mode: Some("tcl"),
        tm_scope: Some("text.crontab"),
        language_id: Some(705203557),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "desktop",
        language_type: "data",
        color: None,
        extensions: &[
            ".desktop",
            ".desktop.in",
            ".service",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.desktop"),
        language_id: Some(412),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "dircolors",
        language_type: "data",
        color: None,
        extensions: &[
            ".dircolors",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            ".dir_colors",
            ".dircolors",
            "DIR_COLORS",
            "_dir_colors",
            "_dircolors",
            "dir_colors",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.dircolors"),
        language_id: Some(691605112),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "eC",
        language_type: "programming",
        color: Some("#913960"),
        extensions: &[
            ".ec",
            ".eh",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.c.ec"),
        language_id: Some(413),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "edn",
        language_type: "data",
        color: None,
        extensions: &[
            ".edn",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("clojure"),
        tm_scope: Some("source.clojure"),
        language_id: Some(414),
        codemirror_mode: Some("clojure"),
        codemirror_mime_type: Some("application/edn"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "fish",
        language_type: "programming",
        color: Some("#4aae47"),
        extensions: &[
            ".fish",
        ],
        aliases: &[
        ],
        interpreters: &[
            "fish",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.fish"),
        language_id: Some(415),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("Shell"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "hoon",
        language_type: "programming",
        color: Some("#00b171"),
        extensions: &[
            ".hoon",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.hoon"),
        language_id: Some(560883276),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "iCalendar",
        language_type: "data",
        color: Some("#ec564c"),
        extensions: &[
            ".ics",
            ".ical",
        ],
        aliases: &[
            "iCal",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("properties"),
        tm_scope: Some("source.iCalendar"),
        language_id: Some(98384424),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "jq",
        language_type: "programming",
        color: Some("#c7254e"),
        extensions: &[
            ".jq",
        ],
        aliases: &[
        ],
        interpreters: &[
            "gojq",
            "jaq",
            "jq",
            "jqjq",
            "jqq",
            "query-json",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.jq"),
        language_id: Some(905371884),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "kvlang",
        language_type: "markup",
        color: Some("#1da6e0"),
        extensions: &[
            ".kv",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.python.kivy"),
        language_id: Some(970675279),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "mIRC Script",
        language_type: "programming",
        color: Some("#3d57c3"),
        extensions: &[
            ".mrc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.msl"),
        language_id: Some(517654727),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "mcfunction",
        language_type: "programming",
        color: Some("#E22837"),
        extensions: &[
            ".mcfunction",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.mcfunction"),
        language_id: Some(462488745),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "mdsvex",
        language_type: "markup",
        color: Some("#5f9ea0"),
        extensions: &[
            ".svx",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("markdown"),
        tm_scope: Some("none"),
        language_id: Some(566198445),
        codemirror_mode: Some("gfm"),
        codemirror_mime_type: Some("text/x-gfm"),
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "mupad",
        language_type: "programming",
        color: Some("#244963"),
        extensions: &[
            ".mu",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.mupad"),
        language_id: Some(416),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "nanorc",
        language_type: "data",
        color: Some("#2d004d"),
        extensions: &[
            ".nanorc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
            ".nanorc",
            "nanorc",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.nanorc"),
        language_id: Some(775996197),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: Some("INI"),
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "nesC",
        language_type: "programming",
        color: Some("#94B0C7"),
        extensions: &[
            ".nc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.nesc"),
        language_id: Some(417),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "ooc",
        language_type: "programming",
        color: Some("#b0b77e"),
        extensions: &[
            ".ooc",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.ooc"),
        language_id: Some(418),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "q",
        language_type: "programming",
        color: Some("#0040cd"),
        extensions: &[
            ".q",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.q"),
        language_id: Some(970539067),
        codemirror_mode: Some("q"),
        codemirror_mime_type: Some("text/x-q"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "reStructuredText",
        language_type: "prose",
        color: Some("#141414"),
        extensions: &[
            ".rst",
            ".rest",
            ".rest.txt",
            ".rst.txt",
        ],
        aliases: &[
            "rst",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("rst"),
        tm_scope: Some("text.restructuredtext"),
        language_id: Some(419),
        codemirror_mode: Some("rst"),
        codemirror_mime_type: Some("text/x-rst"),
        group: None,
        wrap: Some(true),
        fs_name: None,
    },
    LanguageMetadata {
        name: "robots.txt",
        language_type: "data",
        color: None,
        extensions: &[
        ],
        aliases: &[
            "robots",
            "robots txt",
        ],
        interpreters: &[
        ],
        filenames: &[
            "robots.txt",
        ],
        ace_mode: Some("text"),
        tm_scope: Some("text.robots-txt"),
        language_id: Some(674736065),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "sed",
        language_type: "programming",
        color: Some("#64b970"),
        extensions: &[
            ".sed",
        ],
        aliases: &[
        ],
        interpreters: &[
            "gsed",
            "minised",
            "sed",
            "ssed",
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.sed"),
        language_id: Some(847830017),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "templ",
        language_type: "markup",
        color: Some("#66D0DD"),
        extensions: &[
            ".templ",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.templ"),
        language_id: Some(795579337),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "vCard",
        language_type: "data",
        color: Some("#ee2647"),
        extensions: &[
            ".vcf",
        ],
        aliases: &[
            "virtual contact file",
            "electronic business card",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("properties"),
        tm_scope: Some("source.vcard"),
        language_id: Some(851476558),
        codemirror_mode: Some("properties"),
        codemirror_mime_type: Some("text/x-properties"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "wisp",
        language_type: "programming",
        color: Some("#7582D1"),
        extensions: &[
            ".wisp",
        ],
        aliases: &[
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("clojure"),
        tm_scope: Some("source.clojure"),
        language_id: Some(420),
        codemirror_mode: Some("clojure"),
        codemirror_mime_type: Some("text/x-clojure"),
        group: None,
        wrap: None,
        fs_name: None,
    },
    LanguageMetadata {
        name: "xBase",
        language_type: "programming",
        color: Some("#403a40"),
        extensions: &[
            ".prg",
            ".ch",
            ".prw",
        ],
        aliases: &[
            "advpl",
            "clipper",
            "foxpro",
        ],
        interpreters: &[
        ],
        filenames: &[
        ],
        ace_mode: Some("text"),
        tm_scope: Some("source.harbour"),
        language_id: Some(421),
        codemirror_mode: None,
        codemirror_mime_type: None,
        group: None,
        wrap: None,
        fs_name: None,
    },
];
