// AUTO-GENERATED FILE - DO NOT EDIT MANUALLY
// Generated from GitHub Linguist heuristics.yml
// Run: cargo run --bin sync-linguist --features sync-tool

//! Auto-generated language detection heuristics sourced from GitHub Linguist.
//! The data model mirrors Linguist's disambiguation rules so downstream consumers
//! can evaluate ambiguous extensions without depending on Linguist directly.

#[derive(Debug)]
pub struct HeuristicRuleCondition {
pub patterns: &'static [&'static str],
pub named_pattern: Option<&'static str>,
pub negative_patterns: &'static [&'static str],
}

#[derive(Debug)]
pub struct HeuristicRule {
pub language: &'static str,
pub patterns: &'static [&'static str],
pub named_pattern: Option<&'static str>,
pub negative_patterns: &'static [&'static str],
pub and: &'static [HeuristicRuleCondition],
}

#[derive(Debug)]
pub struct HeuristicEntry {
pub extensions: &'static [&'static str],
pub rules: &'static [HeuristicRule],
}

/// Named pattern definitions provided by GitHub Linguist.
pub const NAMED_HEURISTIC_PATTERNS: &[(&str, &[&str])] = &[
    ("cpp", &[
        "^\\s*#\\s*include <(cstdint|string|vector|map|list|array|bitset|queue|stack|forward_list|unordered_map|unordered_set|(i|o|io)stream)>",
        "^\\s*template\\s*<",
        "^[ \\t]*(try|constexpr)",
        "^[ \\t]*catch\\s*\\(",
        "^[ \\t]*(class|(using[ \\t]+)?namespace)\\s+\\w+",
        "^[ \\t]*(private|public|protected):$",
        "__has_cpp_attribute|__cplusplus >",
        "std::\\w+",
    ]),
    ("euphoria", &[
        "^\\s*namespace\\s",
        "^\\s*(?:public\\s+)?include\\s",
        "^\\s*(?:(?:public|export|global)\\s+)?(?:atom|constant|enum|function|integer|object|procedure|sequence|type)\\s",
    ]),
    ("fortran", &[
        "^(?i:[c*][^abd-z]|      (subroutine|program|end|data)\\s|\\s*!)",
    ]),
    ("freebasic", &[
        "(?i)^[ \\t]*#(?:define|endif|endmacro|ifn?def|include|lang|macro|pragma)(?:$|\\s)",
        "(?i)^[ \\t]*dim( shared)? [a-z_][a-z0-9_]* as [a-z_][a-z0-9_]* ptr",
    ]),
    ("gsc", &[
        "^\\s*#\\s*(?:using|insert|include|define|namespace)[ \\t]+\\w",
        "^\\s*((?:autoexec|private)\\s+){0,2}function\\s+((?:autoexec|private)\\s+){0,2}\\w+\\s*\\(",
        "\\b(?:level|self)[ \\t]+thread[ \\t]+(?:\\[\\[[ \\t]*(\\w+\\.)+[ \\t]*\\]\\]|\\w+)[ \\t]*\\([^\\r\\n\\)]*\\)[ \\t]*;",
        "^[ \\t]*#[ \\t]*(?:precache|using_animtree)[ \\t]*\\(",
    ]),
    ("json", &[
        "\\A\\s*[{\\[]",
    ]),
    ("key_equals_value", &[
        "^[^#!;][^=]*=",
    ]),
    ("m68k", &[
        "(?im)\\bmoveq(?:\\.l)?\\s+#(?:\\$-?[0-9a-f]{1,3}|%[0-1]{1,8}|-?[0-9]{1,3}),\\s*d[0-7]\\b",
        "(?im)^\\s*move(?:\\.[bwl])?\\s+(?:sr|usp),\\s*[^\\s]+",
        "(?im)^\\s*move\\.[bwl]\\s+.*\\b[ad]\\d",
        "(?im)^\\s*movem\\.[bwl]\\b",
        "(?im)^\\s*move[mp](?:\\.[wl])?\\b",
        "(?im)^\\s*btst\\b",
        "(?im)^\\s*dbra\\b",
    ]),
    ("man-heading", &[
        "^[.'][ \\t]*SH +(?:[^\"\\s]+|\"[^\"\\s]+)",
    ]),
    ("man-title", &[
        "^[.'][ \\t]*TH +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
    ]),
    ("mdoc-date", &[
        "^[.'][ \\t]*Dd +(?:[^\"\\s]+|\"[^\"]+\")",
    ]),
    ("mdoc-heading", &[
        "^[.'][ \\t]*Sh +(?:[^\"\\s]|\"[^\"]+\")",
    ]),
    ("mdoc-title", &[
        "^[.'][ \\t]*Dt +(?:[^\"\\s]+|\"[^\"]+\") +\"?(?:[1-9]|@[^\\s@]+@)",
    ]),
    ("objectivec", &[
        "^\\s*(@(interface|class|protocol|property|end|synchronised|selector|implementation)\\b|#import\\s+.+\\.h[\">])",
    ]),
    ("perl", &[
        "\\buse\\s+(?:strict\\b|v?5\\b)",
        "^\\s*use\\s+(?:constant|overload)\\b",
        "^\\s*(?:\\*|(?:our\\s*)?@)EXPORT\\s*=",
        "^\\s*package\\s+[^\\W\\d]\\w*(?:::\\w+)*\\s*(?:[;{]|\\sv?\\d)",
        "[\\s$][^\\W\\d]\\w*(?::\\w+)*->[a-zA-Z_\\[({]",
    ]),
    ("quickbasic", &[
        "^[ ]*(CONST|DIM|REDIM|DEFINT|PRINT|DECLARE (SUB|FUNCTION)|FUNCTION|SUB) ",
        "(#|$)lang:?\\s*\"?qb\"?",
        "(?i)'\\$INCLUDE:",
        "(?i)^[ ]*CLS[ ]*('|:|\\r|\\n)",
        "(?i)^[ ]*OPTION _EXPLICIT",
        "(?i)^[ ]*DIM SHARED ",
        "(?i)^[ ]*PRINT \"",
        "(?i) As _(Byte|Offset|MEM)",
        "(?i)^[ ]*_(DISPLAY|DEST|CONSOLE|SOURCE|FREEIMAGE|PALETTECOLOR|PRINTSTRING|LOADFONT|PUTIMAGE)",
        "(?i)^[ ]*_(TITLE|PLAYMOD) \"",
        "(?i)^[ ]*_(LIMIT|SCREEN|DELAY) \\.?\\d+",
        "(?i)\\b_(MOUSEBUTTON|NEWIMAGE|KEYDOWN|WIDTH|HEIGHT)\\(",
        "(?i)^[ ]*\\$(CONSOLE|CHECKING):",
        "(?i)^[ ]*\\$(FULLSCREEN|RESIZE|STATIC|DYNAMIC|NOPREFIX|SCREENSHOW|SCREENHIDE|EXEICON)\\b",
    ]),
    ("raku", &[
        "^\\s*(?:use\\s+v6\\b|\\bmodule\\b|\\b(?:my\\s+)?class\\b)",
    ]),
    ("vb-class", &[
        "^[ ]*VERSION [0-9]\\.[0-9] CLASS",
    ]),
    ("vb-form", &[
        "^[ ]*VERSION [0-9]\\.[0-9]{2}",
    ]),
    ("vb-module", &[
        "^[ ]*Attribute VB_Name = ",
    ]),
    ("vba", &[
        "\\b(?:VBA|[vV]ba)(?:\\b|[0-9A-Z_])",
        "^[ ]*(?:Public|Private)? Declare PtrSafe (?:Sub|Function)\\b",
        "^[ ]*#If Win64\\b",
        "^[ ]*(?:Dim|Const) [0-9a-zA-Z_]*[ ]*As Long(?:Ptr|Long)\\b",
        "^[ ]*Option (?:Private Module|Compare Database)\\b",
        "(?: |\\()(?:Access|Excel|Outlook|PowerPoint|Visio|Word|VBIDE)\\.\\w",
        "\\b(?:(?:Active)?VBProjects?|VBComponents?|Application\\.(?:VBE|ScreenUpdating))\\b",
        "\\b(?:ThisDrawing|AcadObject|Active(?:Explorer|Inspector|Window\\.Presentation|Presentation|Document)|Selection\\.(?:Document|Find|Paragraphs|Range))\\b",
        "\\b(?:(?:This|Active)?Workbooks?|Worksheets?|Active(?:Sheet|Chart|Cell)|WorksheetFunction)\\b",
        "\\b(?:Range\\(\".*|Cells\\([0-9a-zA-Z_]*, (?:[0-9a-zA-Z_]*|\"[a-zA-Z]{1,3}\"))\\)",
    ]),
];

/// Disambiguation heuristics grouped by ambiguous extension.
pub const LANGUAGE_DETECTION_HEURISTICS: &[HeuristicEntry] = &[
    HeuristicEntry {
        extensions: &[
            ".1",
            ".2",
            ".3",
            ".4",
            ".5",
            ".6",
            ".7",
            ".8",
            ".9",
        ],
        rules: &[
            HeuristicRule {
                language: "Roff Manpage",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("mdoc-date"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("mdoc-title"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("mdoc-heading"),
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Roff Manpage",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("man-title"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("man-heading"),
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Roff",
                        patterns: &[
            "^\\.(?:[A-Za-z]{2}(?:\\s|$)|\\\\\")",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".1in",
            ".1m",
            ".1x",
            ".3in",
            ".3m",
            ".3p",
            ".3pm",
            ".3qt",
            ".3x",
            ".man",
            ".mdoc",
        ],
        rules: &[
            HeuristicRule {
                language: "Roff Manpage",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("mdoc-date"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("mdoc-title"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("mdoc-heading"),
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Roff Manpage",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("man-title"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("man-heading"),
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Roff",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".action",
        ],
        rules: &[
            HeuristicRule {
                language: "ROS Interface",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[
            "(?i)^[a-z]\\w*(?:\\/[a-z]\\w*)?(?:<=\\d+)?(?:\\[(?:<=\\d+)?\\])?\\s+\\w+\\b",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "^---\\s*$",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: None,
                                negative_patterns: &[
            "^\\s*[^\\s@#\\w-]",
        ],
                    },
                ],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".al",
        ],
        rules: &[
            HeuristicRule {
                language: "AL",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[
            "\\b(?i:(CODEUNIT|PAGE|PAGEEXTENSION|PAGECUSTOMIZATION|DOTNET|ENUM|ENUMEXTENSION|VALUE|QUERY|REPORT|TABLE|TABLEEXTENSION|XMLPORT|PROFILE|CONTROLADDIN|REPORTEXTENSION|INTERFACE|PERMISSIONSET|PERMISSIONSETEXTENSION|ENTITLEMENT))\\b",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Perl",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".app",
        ],
        rules: &[
            HeuristicRule {
                language: "Erlang",
                        patterns: &[
            "^\\{\\s*(?:application|'application')\\s*,\\s*(?:[a-z]+[\\w@]*|'[^']+')\\s*,\\s*\\[(?:.|[\\r\\n])*\\]\\s*\\}\\.[ \\t]*$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".as",
        ],
        rules: &[
            HeuristicRule {
                language: "ActionScript",
                        patterns: &[
            "^\\s*(?:package(?:\\s+[\\w.]+)?\\s+(?:\\{|$)|import\\s+[\\w.*]+\\s*;|(?:intrinsic\\s+)class\\s+[\\w<>.]+|\\s+class\\s+extends\\s+[\\w<>.]+|(?:(?:public|protected|private|static)\\s+)*(?:(?:var|const|local)\\s+\\w+\\s*:\\s*[\\w<>.]+(?:\\s*=.*)?\\s*;|function\\s+\\w+\\s*\\((?:\\s*\\w+\\s*:\\s*[\\w<>.]+\\s*(,\\s*\\w+\\s*:\\s*[\\w<>.]+\\s*)*)?\\)))",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".asc",
        ],
        rules: &[
            HeuristicRule {
                language: "Public Key",
                        patterns: &[
            "^(----[- ]BEGIN|ssh-(rsa|dss)) ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "AsciiDoc",
                        patterns: &[
            "^[=-]+\\s|\\{\\{[A-Za-z]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "AGS Script",
                        patterns: &[
            "^(\\/\\/.+|((import|export)\\s+)?(function|int|float|char)\\s+((room|repeatedly|on|game)_)?([A-Za-z]+[A-Za-z_0-9]+)\\s*[;\\(])",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".asm",
        ],
        rules: &[
            HeuristicRule {
                language: "Assembly",
                        patterns: &[
            "(?i)mov\\s+[^\\s]{2,},",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Motorola 68K Assembly",
                        patterns: &[],
                named_pattern: Some("m68k"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".asy",
        ],
        rules: &[
            HeuristicRule {
                language: "LTspice Symbol",
                        patterns: &[
            "^SymbolType[ \\t]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Asymptote",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".bas",
        ],
        rules: &[
            HeuristicRule {
                language: "B4X",
                        patterns: &[
            "\\A\\W{0,3}(?:.*(?:\\r?\\n|\\r)){0,9}B4(?:J|A|R|i)=true",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "FreeBASIC",
                        patterns: &[],
                named_pattern: Some("freebasic"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "FreeBASIC",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[
            "(?i)^[ \\t]*return ",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: None,
                                negative_patterns: &[
            "(?i)[ \\t]*gosub ",
        ],
                    },
                ],
            },
            HeuristicRule {
                language: "BASIC",
                        patterns: &[
            "\\A\\s*\\d",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "QuickBASIC",
                        patterns: &[],
                named_pattern: Some("quickbasic"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "VBA",
                        patterns: &[],
                named_pattern: Some("vba"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Visual Basic 6.0",
                        patterns: &[],
                named_pattern: Some("vb-module"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".bb",
        ],
        rules: &[
            HeuristicRule {
                language: "BlitzBasic",
                        patterns: &[
            "(<^\\s*; |End Function)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "BitBake",
                        patterns: &[
            "^(# |include|require|inherit)\\b",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Clojure",
                        patterns: &[
            "\\((def|defn|defmacro|let)\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".bf",
        ],
        rules: &[
            HeuristicRule {
                language: "Beef",
                        patterns: &[
            "(?-m)^\\s*using\\s+(System|Beefy)(\\.(.*))?;\\s*$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "HyPhy",
                        patterns: &[
            "(?-m)^\\s*#include\\s+\".*\";\\s*$",
            "\\sfprintf\\s*\\(",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Brainfuck",
                        patterns: &[
            "(>\\+>|>\\+<)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".bi",
        ],
        rules: &[
            HeuristicRule {
                language: "FreeBASIC",
                        patterns: &[],
                named_pattern: Some("freebasic"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "FreeBASIC",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[
            "(?i)^[ \\t]*return ",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: None,
                                negative_patterns: &[
            "(?i)[ \\t]*gosub ",
        ],
                    },
                ],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".bs",
        ],
        rules: &[
            HeuristicRule {
                language: "Bikeshed",
                        patterns: &[
            "^(?i:<pre\\s+class)\\s*=\\s*('|\\\"|\\b)metadata\\b('|\\\"|\\b)[^>\\r\\n]*>",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "BrighterScript",
                        patterns: &[
            "(?i:^\\s*(?:sub\\s*\\w+\\(.*?\\))|(?::\\s*sub\\(.*?\\))$)",
            "(?i:^\\s*(end\\ssub)$)",
            "(?i:^\\s*(?:function\\s*\\w+\\(.*?\\)\\s*as\\s*\\w*)|(?::\\s*function\\(.*?\\)\\s*as\\s*\\w*)$)",
            "(?i:^\\s*(end\\sfunction)$)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Bluespec BH",
                        patterns: &[
            "^package\\s+[A-Za-z_][A-Za-z0-9_']*(?:\\s*\\(|\\s+where)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".bst",
        ],
        rules: &[
            HeuristicRule {
                language: "BibTeX Style",
                        patterns: &[
            "ENTRY\\s*\\{",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "BuildStream",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".builds",
        ],
        rules: &[
            HeuristicRule {
                language: "XML",
                        patterns: &[
            "^(\\s*)(?i:<Project|<Import|<Property|<?xml|xmlns)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".cairo",
        ],
        rules: &[
            HeuristicRule {
                language: "Cairo Zero",
                        patterns: &[
            "(^(\\s*)%lang(\\s+)([A-Za-z0-9_]+))|(^(\\s*)%builtins(\\s+)([A-Za-z0-9_]+\\s*)*$)|(^(\\s*)from(\\s+)starkware\\.(cairo|starknet)\\.([A-Za-z0-9_.\\s]+?)import)|(,\\s*ap\\+\\+;$)|(;\\s*ap\\+\\+$)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Cairo",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".ch",
        ],
        rules: &[
            HeuristicRule {
                language: "xBase",
                        patterns: &[
            "^\\s*#\\s*(?i:if|ifdef|ifndef|define|command|xcommand|translate|xtranslate|include|pragma|undef)\\b",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".cl",
        ],
        rules: &[
            HeuristicRule {
                language: "Common Lisp",
                        patterns: &[
            "^\\s*\\((?i:defun|in-package|defpackage) ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Cool",
                        patterns: &[
            "^class",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "OpenCL",
                        patterns: &[
            "\\/\\* |\\/\\/ |^\\}",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".cls",
        ],
        rules: &[
            HeuristicRule {
                language: "Visual Basic 6.0",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("vb-class"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "^\\s*BEGIN(?:\\r?\\n|\\r)\\s*MultiUse\\s*=.*(?:\\r?\\n|\\r)\\s*Persistable\\s*=",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "VBA",
                        patterns: &[],
                named_pattern: Some("vb-class"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "TeX",
                        patterns: &[
            "^\\s*\\\\(?:NeedsTeXFormat|ProvidesClass)\\{",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "ObjectScript",
                        patterns: &[
            "^Class\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".cmp",
        ],
        rules: &[
            HeuristicRule {
                language: "Gerber Image",
                        patterns: &[
            "^[DGMT][0-9]{2}\\*(?:\\r?\\n|\\r)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".cs",
        ],
        rules: &[
            HeuristicRule {
                language: "Smalltalk",
                        patterns: &[
            "![\\w\\s]+methodsFor: ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "C#",
                        patterns: &[
            "^\\s*(using\\s+[A-Z][\\s\\w.]+;|namespace\\s*[\\w\\.]+\\s*(\\{|;)|\\/\\/)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".csc",
        ],
        rules: &[
            HeuristicRule {
                language: "GSC",
                        patterns: &[],
                named_pattern: Some("gsc"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".csl",
        ],
        rules: &[
            HeuristicRule {
                language: "XML",
                        patterns: &[
            "(?i:^\\s*(<\\?xml|xmlns))",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Kusto",
                        patterns: &[
            "(^\\|\\s*(where|extend|project|limit|summarize))|(^\\.\\w+)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".d",
        ],
        rules: &[
            HeuristicRule {
                language: "D",
                        patterns: &[
            "^module\\s+[\\w.]*\\s*;|import\\s+[\\w\\s,.:]*;|\\w+\\s+\\w+\\s*\\(.*\\)(?:\\(.*\\))?\\s*\\{[^}]*\\}|unittest\\s*(?:\\(.*\\))?\\s*\\{[^}]*\\}",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "DTrace",
                        patterns: &[
            "^(\\w+:\\w*:\\w*:\\w*|BEGIN|END|provider\\s+|(tick|profile)-\\w+\\s+\\{[^}]*\\}|#pragma\\s+D\\s+(option|attributes|depends_on)\\s|#pragma\\s+ident\\s)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Makefile",
                        patterns: &[
            "([\\/\\\\].*:\\s+.*\\s\\\\$|: \\\\$|^[ %]:|^[\\w\\s\\/\\\\.]+\\w+\\.\\w+\\s*:\\s+[\\w\\s\\/\\\\.]+\\w+\\.\\w+)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".dsp",
        ],
        rules: &[
            HeuristicRule {
                language: "Microsoft Developer Studio Project",
                        patterns: &[
            "# Microsoft Developer Studio Generated Build File",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Faust",
                        patterns: &[
            "\\bprocess\\s*[(=]|\\b(library|import)\\s*\\(\\s*\"|\\bdeclare\\s+(name|version|author|copyright|license)\\s+\"",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".e",
        ],
        rules: &[
            HeuristicRule {
                language: "E",
                        patterns: &[
            "^\\s*(def|var)\\s+(.+):=",
            "^\\s*(def|to)\\s+(\\w+)(\\(.+\\))?\\s+\\{",
            "^\\s*(when)\\s+(\\(.+\\))\\s+->\\s+\\{",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Eiffel",
                        patterns: &[
            "^\\s*\\w+\\s*(?:,\\s*\\w+)*[:]\\s*\\w+\\s",
            "^\\s*\\w+\\s*(?:\\(\\s*\\w+[:][^)]+\\))?(?:[:]\\s*\\w+)?(?:--.+\\s+)*\\s+(?:do|local)\\s",
            "^\\s*(?:across|deferred|elseif|ensure|feature|from|inherit|inspect|invariant|note|once|require|undefine|variant|when)\\s*$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Euphoria",
                        patterns: &[],
                named_pattern: Some("euphoria"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".ecl",
        ],
        rules: &[
            HeuristicRule {
                language: "ECLiPSe",
                        patterns: &[
            "^[^#]+:-",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "ECL",
                        patterns: &[
            ":=",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".es",
        ],
        rules: &[
            HeuristicRule {
                language: "Erlang",
                        patterns: &[
            "^\\s*(?:%%|main\\s*\\(.*?\\)\\s*->)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "JavaScript",
                        patterns: &[
            "\\/\\/|[\"']use strict[\"']|export\\s+default\\s|\\/\\*(?:.|[\\r\\n])*?\\*\\/",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".ex",
        ],
        rules: &[
            HeuristicRule {
                language: "Elixir",
                        patterns: &[
            "^\\s*@moduledoc\\s",
            "^\\s*(?:cond|import|quote|unless)\\s",
            "^\\s*def(?:exception|impl|macro|module|protocol)[(\\s]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Euphoria",
                        patterns: &[],
                named_pattern: Some("euphoria"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".f",
        ],
        rules: &[
            HeuristicRule {
                language: "Forth",
                        patterns: &[
            "^: ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Filebench WML",
                        patterns: &[
            "flowop",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Fortran",
                        patterns: &[],
                named_pattern: Some("fortran"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".for",
        ],
        rules: &[
            HeuristicRule {
                language: "Forth",
                        patterns: &[
            "^: ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Fortran",
                        patterns: &[],
                named_pattern: Some("fortran"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".fr",
        ],
        rules: &[
            HeuristicRule {
                language: "Forth",
                        patterns: &[
            "^(: |also |new-device|previous )",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Frege",
                        patterns: &[
            "^\\s*(import|module|package|data|type) ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Text",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".frm",
        ],
        rules: &[
            HeuristicRule {
                language: "VBA",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("vb-form"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "^\\s*Begin\\s+\\{[0-9A-Z\\-]*\\}\\s?",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Visual Basic 6.0",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("vb-form"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "^\\s*Begin\\s+VB\\.Form\\s+",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "INI",
                        patterns: &[
            "\\ATYPE=VIEW",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".fs",
        ],
        rules: &[
            HeuristicRule {
                language: "Forth",
                        patterns: &[
            "^(: |new-device)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "F#",
                        patterns: &[
            "^\\s*(#light|import|let|module|namespace|open|type)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "GLSL",
                        patterns: &[
            "^\\s*(#version|precision|uniform|varying|vec[234])",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Filterscript",
                        patterns: &[
            "#include|#pragma\\s+(rs|version)|__attribute__",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".ftl",
        ],
        rules: &[
            HeuristicRule {
                language: "FreeMarker",
                        patterns: &[
            "^(?:<|[a-zA-Z-][a-zA-Z0-9_-]+[ \\t]+\\w)|\\$\\{\\w+[^\\r\\n]*?\\}|^[ \\t]*(?:<#--.*?-->|<#(?:[a-z]+)\\s[^>]*>.*?</#(?:[a-z]+)>|\\[#--.*?--\\]|\\[#(?:[a-z]+)\\s[^\\]]*\\].*?\\[#(?:[a-z]+)\\])",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Fluent",
                        patterns: &[
            "^-?[a-zA-Z][a-zA-Z0-9_-]* *=|\\{\\$-?[a-zA-Z][-\\w]*(?:\\.[a-zA-Z][-\\w]*)?\\}",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".g",
        ],
        rules: &[
            HeuristicRule {
                language: "GAP",
                        patterns: &[
            "\\s*(Declare|BindGlobal|KeyDependentOperation|Install(Method|GlobalFunction)|SetPackageInfo)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "G-code",
                        patterns: &[
            "^[MG][0-9]+(?:\\r?\\n|\\r)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".gd",
        ],
        rules: &[
            HeuristicRule {
                language: "GAP",
                        patterns: &[
            "\\s*(Declare|BindGlobal|KeyDependentOperation)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "GDScript",
                        patterns: &[
            "\\s*(extends|var|const|enum|func|class|signal|tool|yield|assert|onready)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".gml",
        ],
        rules: &[
            HeuristicRule {
                language: "XML",
                        patterns: &[
            "(?i:^\\s*(<\\?xml|xmlns))",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Graph Modeling Language",
                        patterns: &[
            "(?i:^\\s*(graph|node)\\s+\\[$)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Gerber Image",
                        patterns: &[
            "^[DGMT][0-9]{2}\\*$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Game Maker Language",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".gs",
        ],
        rules: &[
            HeuristicRule {
                language: "GLSL",
                        patterns: &[
            "^#version\\s+[0-9]+\\b",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Gosu",
                        patterns: &[
            "^uses (java|gw)\\.",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Genie",
                        patterns: &[
            "^\\[indent=[0-9]+\\]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".gsc",
        ],
        rules: &[
            HeuristicRule {
                language: "GSC",
                        patterns: &[],
                named_pattern: Some("gsc"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".gsh",
        ],
        rules: &[
            HeuristicRule {
                language: "GSC",
                        patterns: &[],
                named_pattern: Some("gsc"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".gts",
        ],
        rules: &[
            HeuristicRule {
                language: "Gerber Image",
                        patterns: &[
            "^G0.",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Glimmer TS",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[
            "^G0.",
        ],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".h",
        ],
        rules: &[
            HeuristicRule {
                language: "Objective-C",
                        patterns: &[],
                named_pattern: Some("objectivec"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "C++",
                        patterns: &[],
                named_pattern: Some("cpp"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "C",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".hh",
        ],
        rules: &[
            HeuristicRule {
                language: "Hack",
                        patterns: &[
            "<\\?hh",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".html",
        ],
        rules: &[
            HeuristicRule {
                language: "Ecmarkup",
                        patterns: &[
            "<emu-(?:alg|annex|biblio|clause|eqn|example|figure|gann|gmod|gprose|grammar|intro|not-ref|note|nt|prodref|production|rhs|table|t|xref)(?:$|\\s|>)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "HTML",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".i",
        ],
        rules: &[
            HeuristicRule {
                language: "Motorola 68K Assembly",
                        patterns: &[],
                named_pattern: Some("m68k"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "SWIG",
                        patterns: &[
            "^[ \\t]*%[a-z_]+\\b|^%[{}]$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Assembly",
                        patterns: &[
            "(?i)mov\\s+[^\\s]+,",
            "^\\s+(i?)db\\s+[a-z\\d]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".ice",
        ],
        rules: &[
            HeuristicRule {
                language: "JSON",
                        patterns: &[],
                named_pattern: Some("json"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Slice",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".inc",
        ],
        rules: &[
            HeuristicRule {
                language: "Motorola 68K Assembly",
                        patterns: &[],
                named_pattern: Some("m68k"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "PHP",
                        patterns: &[
            "^<\\?(?:php)?",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "SourcePawn",
                        patterns: &[
            "^public\\s+(?:SharedPlugin(?:\\s+|:)__pl_\\w+\\s*=(?:\\s*\\{)?|(?:void\\s+)?__pl_\\w+_SetNTVOptional\\(\\)(?:\\s*\\{)?)",
            "^methodmap\\s+\\w+\\s+<\\s+\\w+",
            "^\\s*MarkNativeAsOptional\\s*\\(",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "NASL",
                        patterns: &[
            "^\\s*include\\s*\\(\\s*(?:\"|')[\\\\/\\w\\-\\.:\\s]+\\.(?:nasl|inc)\\s*(?:\"|')\\s*\\)\\s*;",
            "^\\s*(?:global|local)_var\\s+(?:\\w+(?:\\s*=\\s*[\\w\\-\"']+)?\\s*)(?:,\\s*\\w+(?:\\s*=\\s*[\\w\\-\"']+)?\\s*)*\\s*;",
            "^\\s*namespace\\s+\\w+\\s*\\{",
            "^\\s*object\\s+\\w+\\s*(?:extends\\s+\\w+(?:::\\w+)?)?\\s*\\{",
            "^\\s*(?:public\\s+|private\\s+|\\s*)function\\s+\\w+\\s*\\([\\w\\s,]*\\)\\s*\\{",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "POV-Ray SDL",
                        patterns: &[
            "^\\s*#(declare|local|macro|while)\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Pascal",
                        patterns: &[
            "(?i:^\\s*\\{\\$(?:mode|ifdef|undef|define)[ ]+[a-z0-9_]+\\})",
            "^\\s*end[.;]\\s*$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "BitBake",
                        patterns: &[
            "^inherit(\\s+[\\w.-]+)+\\s*$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Assembly",
                        patterns: &[
            "^(?i)[^\"m]*mov\\s+[^\\s]+,",
            "^\\s+(?i)db\\s+[a-z\\d]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".json",
        ],
        rules: &[
            HeuristicRule {
                language: "OASv2-json",
                        patterns: &[
            "\"swagger\":\\s?\"2.[0-9.]+\"",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "OASv3-json",
                        patterns: &[
            "\"openapi\":\\s?\"3.[0-9.]+\"",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "JSON",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".l",
        ],
        rules: &[
            HeuristicRule {
                language: "Common Lisp",
                        patterns: &[
            "\\(def(un|macro)\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Lex",
                        patterns: &[
            "^(%[%{}]xs|<.*>)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Roff",
                        patterns: &[
            "^\\.[A-Za-z]{2}(\\s|$)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "PicoLisp",
                        patterns: &[
            "^\\((de|class|rel|code|data|must)\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".lean",
        ],
        rules: &[
            HeuristicRule {
                language: "Lean",
                        patterns: &[
            "^import [a-z]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Lean 4",
                        patterns: &[
            "^import [A-Z]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".lp",
        ],
        rules: &[
            HeuristicRule {
                language: "Linear Programming",
                        patterns: &[
            "^(?i:minimize|minimum|min|maximize|maximum|max)(?i:\\s+multi-objectives)?$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Answer Set Programming",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".ls",
        ],
        rules: &[
            HeuristicRule {
                language: "LoomScript",
                        patterns: &[
            "^\\s*package\\s*[\\w\\.\\/\\*\\s]*\\s*\\{",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "LiveScript",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".lisp",
            ".lsp",
        ],
        rules: &[
            HeuristicRule {
                language: "Common Lisp",
                        patterns: &[
            "^\\s*\\((?i:defun|in-package|defpackage) ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "NewLisp",
                        patterns: &[
            "^\\s*\\(define ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".m",
        ],
        rules: &[
            HeuristicRule {
                language: "Objective-C",
                        patterns: &[],
                named_pattern: Some("objectivec"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Mercury",
                        patterns: &[
            ":- module",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "MUF",
                        patterns: &[
            "^: ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "M",
                        patterns: &[
            "^\\s*;",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Wolfram Language",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[
            "\\(\\*",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "\\*\\)$",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "MATLAB",
                        patterns: &[
            "^\\s*%",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Limbo",
                        patterns: &[
            "^\\w+\\s*:\\s*module\\s*\\{",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".m4",
        ],
        rules: &[
            HeuristicRule {
                language: "M4Sugar",
                        patterns: &[
            "AC_DEFUN|AC_PREREQ|AC_INIT",
            "^_?m4_",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "M4",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".mask",
        ],
        rules: &[
            HeuristicRule {
                language: "Unity3D Asset",
                        patterns: &[
            "tag:unity3d.com",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".mc",
        ],
        rules: &[
            HeuristicRule {
                language: "Win32 Message File",
                        patterns: &[
            "(?i)^[ \\t]*(\\/\\*\\s*)?MessageId=|^\\.$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "M4",
                        patterns: &[
            "^dnl|^divert\\((?:-?\\d+)?\\)|^\\w+\\(`[^\\r\\n]*?'[),]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Monkey C",
                        patterns: &[
            "\\b(?:using|module|function|class|var)\\s+\\w",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".md",
        ],
        rules: &[
            HeuristicRule {
                language: "Markdown",
                        patterns: &[
            "(^[-A-Za-z0-9=#!\\*\\[|>])|<\\/",
            "\\A\\z",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "GCC Machine Description",
                        patterns: &[
            "^(;;|\\(define_)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Markdown",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".ml",
        ],
        rules: &[
            HeuristicRule {
                language: "OCaml",
                        patterns: &[
            "(^\\s*module)|let rec |match\\s+(\\S+\\s)+with",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Standard ML",
                        patterns: &[
            "=> |case\\s+(\\S+\\s)+of",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".mod",
        ],
        rules: &[
            HeuristicRule {
                language: "XML",
                        patterns: &[
            "<!ENTITY ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "NMODL",
                        patterns: &[
            "\\b(NEURON|INITIAL|UNITS)\\b",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Modula-2",
                        patterns: &[
            "^\\s*(?i:MODULE|END) [\\w\\.]+;",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Linux Kernel Module",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "AMPL",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".mojo",
        ],
        rules: &[
            HeuristicRule {
                language: "Mojo",
                        patterns: &[
            "^\\s*(alias|def|from|fn|import|struct|trait)\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "XML",
                        patterns: &[
            "^\\s*<\\?xml",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".ms",
        ],
        rules: &[
            HeuristicRule {
                language: "Roff",
                        patterns: &[
            "^[.'][A-Za-z]{2}(\\s|$)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Unix Assembly",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: None,
                                negative_patterns: &[
            "/\\*",
        ],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "^\\s*\\.(?:include\\s|globa?l\\s|[A-Za-z][_A-Za-z0-9]*:)",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "MAXScript",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".msg",
        ],
        rules: &[
            HeuristicRule {
                language: "OMNeT++ MSG",
                        patterns: &[
            "^cplusplus\\(?[\\S]*\\)?[\\s]*\\{?\\{?|^namespace[\\s]+([^.\\s]*\\.)*[^.\\s]*;|^struct[\\s]+[\\S]+|^message[\\s]+[\\S]+(extends )?[\\S]*[\\s]*|^packet[\\s]+[\\S]+|^class[\\s]+[\\S]+(extends )?[\\S]*[\\s]*|^enum[\\s]+[\\S]+|^import ([^.\\s]*\\.)*[^.\\s]*;",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "ROS Interface",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[
            "(?i)^[a-z]\\w*(?:\\/[a-z]\\w*)?(?:<=\\d+)?(?:\\[(?:<=\\d+)?\\])?\\s+\\w+\\b",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: None,
                                negative_patterns: &[
            "^\\s*[^\\s@#\\w-]",
        ],
                    },
                ],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".n",
        ],
        rules: &[
            HeuristicRule {
                language: "Roff",
                        patterns: &[
            "^[.']",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Nemerle",
                        patterns: &[
            "^(module|namespace|using)\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".ncl",
        ],
        rules: &[
            HeuristicRule {
                language: "XML",
                        patterns: &[
            "^\\s*<\\?xml\\s+version",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Gerber Image",
                        patterns: &[
            "^[DGMT][0-9]{2}\\*(?:\\r?\\n|\\r)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Nickel",
                        patterns: &[
            "^let(?:\\srec)?(?:\\s[a-zA-Z_][a-zA-Z0-9_]*)?",
            "^import\\s\"[^\"]+\"\\s+as\\s",
            "std\\.[a-zA-Z_][a-zA-Z0-9_]*\\.",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Text",
                        patterns: &[
            "THE_TITLE",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "NCL",
                        patterns: &[
            "^load \"",
            "^begin$",
            "[0-9]\\.$",
            "^;",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".nl",
        ],
        rules: &[
            HeuristicRule {
                language: "NL",
                        patterns: &[
            "^(b|g)[0-9]+ ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "NewLisp",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".nr",
        ],
        rules: &[
            HeuristicRule {
                language: "Roff",
                        patterns: &[
            "^\\.",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Noir",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".nu",
        ],
        rules: &[
            HeuristicRule {
                language: "Nushell",
                        patterns: &[
            "^\\s*(import|export|module|def|let|let-env) ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Nu",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".odin",
        ],
        rules: &[
            HeuristicRule {
                language: "Object Data Instance Notation",
                        patterns: &[
            "(?:^|<)\\s*[A-Za-z0-9_]+\\s*=\\s*<",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Odin",
                        patterns: &[
            "package\\s+\\w+|\\b(?:im|ex)port\\s*\"[\\w:./]+\"|\\w+\\s*::\\s*(?:proc|struct)\\s*\\(|^\\s*//\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".p",
        ],
        rules: &[
            HeuristicRule {
                language: "Gnuplot",
                        patterns: &[
            "^s?plot\\b",
            "^set\\s+(term|terminal|out|output|[xy]tics|[xy]label|[xy]range|style)\\b",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "OpenEdge ABL",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".php",
        ],
        rules: &[
            HeuristicRule {
                language: "Hack",
                        patterns: &[
            "<\\?hh",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "PHP",
                        patterns: &[
            "<\\?[^h]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".pkl",
        ],
        rules: &[
            HeuristicRule {
                language: "Pkl",
                        patterns: &[
            "^\\s*(module|import|amends|extends|local|const|fixed|abstract|open|class|typealias|@\\w+)\\b",
            "^\\s*[a-zA-Z0-9_$]+\\s*(=|{|:)|^\\s*`[^`]+`\\s*(=|{|:)|for\\s*\\(|when\\s*\\(",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Pickle",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".pl",
        ],
        rules: &[
            HeuristicRule {
                language: "Prolog",
                        patterns: &[
            "^[^#]*:-",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Perl",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: None,
                                negative_patterns: &[
            "^\\s*use\\s+v6\\b",
        ],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("perl"),
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Raku",
                        patterns: &[],
                named_pattern: Some("raku"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".plist",
        ],
        rules: &[
            HeuristicRule {
                language: "XML Property List",
                        patterns: &[
            "^\\s*(?:<\\?xml\\s|<!DOCTYPE\\s+plist|<plist(?:\\s+version\\s*=\\s*[\"']\\d+(?:\\.\\d+)?[\"'])?\\s*>\\s*$)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "OpenStep Property List",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".plt",
        ],
        rules: &[
            HeuristicRule {
                language: "Prolog",
                        patterns: &[
            "^\\s*:-",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".pm",
        ],
        rules: &[
            HeuristicRule {
                language: "Perl",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: None,
                                negative_patterns: &[
            "^\\s*use\\s+v6\\b",
        ],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("perl"),
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Raku",
                        patterns: &[],
                named_pattern: Some("raku"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "X PixMap",
                        patterns: &[
            "^\\s*\\/\\* XPM \\*\\/",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".pod",
        ],
        rules: &[
            HeuristicRule {
                language: "Pod 6",
                        patterns: &[
            "^[\\s&&[^\\r\\n]]*=(comment|begin pod|begin para|item\\d+)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Pod",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".pp",
        ],
        rules: &[
            HeuristicRule {
                language: "Pascal",
                        patterns: &[
            "^\\s*end[.;]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Puppet",
                        patterns: &[
            "^\\s+\\w+\\s+=>\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".pro",
        ],
        rules: &[
            HeuristicRule {
                language: "Proguard",
                        patterns: &[
            "^-(include\\b.*\\.pro$|keep\\b|keepclassmembers\\b|keepattributes\\b)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Prolog",
                        patterns: &[
            "^[^\\[#]+:-",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "INI",
                        patterns: &[
            "last_client=",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "QMake",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[
            "HEADERS",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "SOURCES",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "IDL",
                        patterns: &[
            "^\\s*(?i:function|pro|compile_opt) \\w[ \\w,:]*$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".properties",
        ],
        rules: &[
            HeuristicRule {
                language: "INI",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("key_equals_value"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "^[;\\[]",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Java Properties",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("key_equals_value"),
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "^[#!]",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "INI",
                        patterns: &[],
                named_pattern: Some("key_equals_value"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Java Properties",
                        patterns: &[
            "^[^#!][^:]*:",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".q",
        ],
        rules: &[
            HeuristicRule {
                language: "q",
                        patterns: &[
            "((?i:[A-Z.][\\w.]*:\\{)|^\\\\(cd?|d|l|p|ts?) )",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "HiveQL",
                        patterns: &[
            "(?i:SELECT\\s+[\\w*,]+\\s+FROM|(CREATE|ALTER|DROP)\\s(DATABASE|SCHEMA|TABLE))",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".qs",
        ],
        rules: &[
            HeuristicRule {
                language: "Q#",
                        patterns: &[
            "^((\\/{2,3})?\\s*(namespace|operation)\\b)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Qt Script",
                        patterns: &[
            "(\\w+\\.prototype\\.\\w+|===|\\bvar\\b)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".r",
        ],
        rules: &[
            HeuristicRule {
                language: "Rebol",
                        patterns: &[
            "(?i:\\bRebol\\b)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Rez",
                        patterns: &[
            "(#include\\s+[\"<](Types\\.r|Carbon\\/Carbon\\.r)[\">])|((resource|data|type)\\s+'[A-Za-z0-9]{4}'\\s+((\\(.*\\)\\s+){0,1}){)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "R",
                        patterns: &[
            "<-|^\\s*#",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".re",
        ],
        rules: &[
            HeuristicRule {
                language: "Reason",
                        patterns: &[
            "^\\s*module\\s+type\\s",
            "^\\s*(?:include|open)\\s+\\w+\\s*;\\s*$",
            "^\\s*let\\s+(?:module\\s\\w+\\s*=\\s*\\{|\\w+:\\s+.*=.*;\\s*$)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "C++",
                        patterns: &[
            "^\\s*#(?:(?:if|ifdef|define|pragma)\\s+\\w|\\s*include\\s+<[^>]+>)",
            "^\\s*template\\s*<",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".res",
        ],
        rules: &[
            HeuristicRule {
                language: "ReScript",
                        patterns: &[
            "^\\s*(let|module|type)\\s+\\w*\\s+=\\s+",
            "^\\s*(?:include|open)\\s+\\w+\\s*$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".resource",
        ],
        rules: &[
            HeuristicRule {
                language: "RobotFramework",
                        patterns: &[
            "^\\*{3} (Settings|Variables|Keywords) \\*{3}$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".rno",
        ],
        rules: &[
            HeuristicRule {
                language: "RUNOFF",
                        patterns: &[
            "(?i:^\\.!|^\\f|\\f$|^\\.end lit(?:eral)?\\b|^\\.[a-zA-Z].*?;\\.[a-zA-Z](?:[; \\t])|\\^\\*[^\\s*][^*]*\\\\\\*(?:$|\\s)|^\\.c;[ \\t]*\\w+)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Roff",
                        patterns: &[
            "^\\.\\\\\" ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".rpy",
        ],
        rules: &[
            HeuristicRule {
                language: "Python",
                        patterns: &[
            "^(import|from|class|def)\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Ren'Py",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".rs",
        ],
        rules: &[
            HeuristicRule {
                language: "Rust",
                        patterns: &[
            "^(use |fn |mod |pub |macro_rules|impl|#!?\\[)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "RenderScript",
                        patterns: &[
            "#include|#pragma\\s+(rs|version)|__attribute__",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "XML",
                        patterns: &[
            "^\\s*<\\?xml",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".s",
        ],
        rules: &[
            HeuristicRule {
                language: "Unix Assembly",
                        patterns: &[
            "(?i:mov[lq]?)\\s+[%\\$]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Assembly",
                        patterns: &[
            "(?i:mov)\\s+[^\\s%]{2,},",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Motorola 68K Assembly",
                        patterns: &[],
                named_pattern: Some("m68k"),
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".sc",
        ],
        rules: &[
            HeuristicRule {
                language: "SuperCollider",
                        patterns: &[
            "(?i:\\^(this|super)\\.|^\\s*~\\w+\\s*=\\.)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Scala",
                        patterns: &[
            "(^\\s*import (scala|java)\\.|^\\s*class\\b)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".scd",
        ],
        rules: &[
            HeuristicRule {
                language: "SuperCollider",
                        patterns: &[
            "(?i:\\^(this|super)\\.|^\\s*(~\\w+\\s*=\\.|SynthDef\\b))",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Markdown",
                        patterns: &[
            "^#+\\s+(NAME|SYNOPSIS|DESCRIPTION)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".scm",
        ],
        rules: &[
            HeuristicRule {
                language: "Tree-sitter Query",
                        patterns: &[
            "\\(#[\\w-]+[!\\?]",
            "(?:[\\)\\]]\\s*[\\*\\+\\?](?:\\s|$))",
            "(?:^\\s*\\w+:\\s*[\\(\\[\\\"])",
            "\\(#(?:set!|(?:not-)?(?:any-of|match)\\?)",
            "@[\\w.-]+(?:\\)\\s|$)",
        ],
                named_pattern: None,
                        negative_patterns: &[
            "\\([+\\-:<>\\/=~\\)]",
        ],
                and: &[],
            },
            HeuristicRule {
                language: "Scheme",
                        patterns: &[
            "(?:'[\\(\\*#]|\\w->\\w|\\.\\.\\.[\\s\\)]|\\([+\\-:<>\\/=~\\)]|~>|[#`]\\(|#:\\w)",
            "^\\s*\\((?:define\\*?|import|library|lambda)",
        ],
                named_pattern: None,
                        negative_patterns: &[
            "\\(#[\\w-]+[!\\?]",
            "(?:[\\)\\]]\\s*[\\*\\+\\?](?:\\s|$))",
            "@[\\w.-]+(?:\\)\\s|$)",
        ],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".sol",
        ],
        rules: &[
            HeuristicRule {
                language: "Solidity",
                        patterns: &[
            "\\bpragma\\s+solidity\\b|\\b(?:abstract\\s+)?contract\\s+[a-zA-Z$_][a-zA-Z0-9$_]*(?:\\s+is\\s+(?:[a-zA-Z0-9$_][^\\{]*?)?)?\\s*\\{",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Gerber Image",
                        patterns: &[
            "^[DGMT][0-9]{2}\\*(?:\\r?\\n|\\r)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".sql",
        ],
        rules: &[
            HeuristicRule {
                language: "PLpgSQL",
                        patterns: &[
            "(?i:^\\\\i\\b|AS\\s+\\$\\$|LANGUAGE\\s+'?plpgsql'?|BEGIN(\\s+WORK)?\\s*;)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "SQLPL",
                        patterns: &[
            "(?i:ALTER\\s+MODULE|MODE\\s+DB2SQL|\\bSYS(CAT|PROC)\\.|ASSOCIATE\\s+RESULT\\s+SET|\\bEND!\\s*$)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "PLSQL",
                        patterns: &[
            "(?i:\\$\\$PLSQL_|XMLTYPE|systimestamp|\\.nextval|CONNECT\\s+BY|AUTHID\\s+(DEFINER|CURRENT_USER)|constructor\\W+function)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "TSQL",
                        patterns: &[
            "(?i:^\\s*GO\\b|BEGIN(\\s+TRY|\\s+CATCH)|OUTPUT\\s+INSERTED|DECLARE\\s+@|\\[dbo\\])",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "SQL",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".srt",
        ],
        rules: &[
            HeuristicRule {
                language: "SubRip Text",
                        patterns: &[
            "^(\\d{2}:\\d{2}:\\d{2},\\d{3})\\s*(-->)\\s*(\\d{2}:\\d{2}:\\d{2},\\d{3})$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".srv",
        ],
        rules: &[
            HeuristicRule {
                language: "ROS Interface",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[
            "(?i)^[a-z]\\w*(?:\\/[a-z]\\w*)?(?:<=\\d+)?(?:\\[(?:<=\\d+)?\\])?\\s+\\w+\\b",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "^---\\s*$",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: None,
                                negative_patterns: &[
            "^\\s*[^\\s@#\\w-]",
        ],
                    },
                ],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".st",
        ],
        rules: &[
            HeuristicRule {
                language: "StringTemplate",
                        patterns: &[
            "\\$\\w+[($]|.!\\s*.+?\\s*!.|<!\\s*.+?\\s*!>|\\[!\\s*.+?\\s*!\\]|\\{!\\s*.+?\\s*!\\}",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Smalltalk",
                        patterns: &[
            "\\A\\s*[\\[{(^\"'\\w#]|[a-zA-Z_]\\w*\\s*:=\\s*[a-zA-Z_]\\w*|class\\s*>>\\s*[a-zA-Z_]\\w*|^[a-zA-Z_]\\w*\\s+[a-zA-Z_]\\w*:|^Class\\s*\\{|if(?:True|False):\\s*\\[",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".star",
        ],
        rules: &[
            HeuristicRule {
                language: "STAR",
                        patterns: &[
            "^loop_\\s*$",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Starlark",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".stl",
        ],
        rules: &[
            HeuristicRule {
                language: "STL",
                        patterns: &[
            "\\A\\s*solid(?:$|\\s)[\\s\\S]*^endsolid(?:$|\\s)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".svx",
        ],
        rules: &[
            HeuristicRule {
                language: "Survex data",
                        patterns: &[
            "\\A(;|\\*[^*]+$)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "mdsvex",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".sw",
        ],
        rules: &[
            HeuristicRule {
                language: "Sway",
                        patterns: &[
            "^\\s*(?:(?:abi|dep|fn|impl|mod|pub|trait)\\s|#\\[)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "XML",
                        patterns: &[
            "^\\s*<\\?xml\\s+version",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".t",
        ],
        rules: &[
            HeuristicRule {
                language: "Perl",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: None,
                                negative_patterns: &[
            "^\\s*use\\s+v6\\b",
        ],
                    },
                    HeuristicRuleCondition {
                                patterns: &[],
                        named_pattern: Some("perl"),
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Raku",
                        patterns: &[
            "^\\s*(?:use\\s+v6\\b|\\bmodule\\b|\\bmy\\s+class\\b)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Turing",
                        patterns: &[
            "^\\s*%[ \\t]+|^\\s*var\\s+\\w+(\\s*:\\s*\\w+)?\\s*:=\\s*\\w+",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".tact",
        ],
        rules: &[
            HeuristicRule {
                language: "JSON",
                        patterns: &[
            "\\A\\s*\\{\\\"",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Tact",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".tag",
        ],
        rules: &[
            HeuristicRule {
                language: "Java Server Pages",
                        patterns: &[
            "<%[@!=\\s]?\\s*(taglib|tag|include|attribute|variable)\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".tl",
        ],
        rules: &[
            HeuristicRule {
                language: "Teal",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[
                    HeuristicRuleCondition {
                                patterns: &[
            "--.*",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                    HeuristicRuleCondition {
                                patterns: &[
            "\\b(local|function|end|record|interface|enum)\\b",
        ],
                        named_pattern: None,
                                negative_patterns: &[],
                    },
                ],
            },
            HeuristicRule {
                language: "Type Language",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".tlv",
        ],
        rules: &[
            HeuristicRule {
                language: "TL-Verilog",
                        patterns: &[
            "^\\\\.{0,10}TLV_version",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".toc",
        ],
        rules: &[
            HeuristicRule {
                language: "World of Warcraft Addon Data",
                        patterns: &[
            "^## |@no-lib-strip@",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "TeX",
                        patterns: &[
            "^\\\\(contentsline|defcounter|beamer|boolfalse)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".ts",
        ],
        rules: &[
            HeuristicRule {
                language: "XML",
                        patterns: &[
            "<TS\\b",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "TypeScript",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".tsp",
        ],
        rules: &[
            HeuristicRule {
                language: "TypeSpec",
                        patterns: &[
            "^(import|using|namespace|interface|op|model|scalar|alias|union|enum)\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "TSPLIB data",
                        patterns: &[
            "^(NAME|TYPE|DIMENSION|EDGE_WEIGHT_TYPE|EDGE_WEIGHT_FORMAT)\\s*:",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".tst",
        ],
        rules: &[
            HeuristicRule {
                language: "GAP",
                        patterns: &[
            "gap> ",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Scilab",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".tsx",
        ],
        rules: &[
            HeuristicRule {
                language: "XML",
                        patterns: &[
            "(?i:^\\s*<\\?xml\\s+version)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "TSX",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".txt",
        ],
        rules: &[
            HeuristicRule {
                language: "Vim Help File",
                        patterns: &[
            "(?:(?:^|[ \\t])(?:vi|Vi(?=m))(?:m[<=>]?[0-9]+|m)?|[ \\t]ex)(?=:(?=[ \\t]*set?[ \\t][^\\r\\n:]+:)|:(?![ \\t]*set?[ \\t]))(?:(?:[ \\t]*:[ \\t]*|[ \\t])\\w*(?:[ \\t]*=(?:[^\\\\\\s]|\\\\.)*)?)*[ \\t:](?:filetype|ft|syntax)[ \\t]*=(help)(?=$|\\s|:)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Hosts File",
                        patterns: &[
            "(?xi) ^\n\n# IPv4 address\n(?<ipv4>\n  (?!\\.)\n  (?:\\.?\n    (?: 25[0-5]  # 250-255\n    |   2[0-4]\\d # 200-249\n    |   1\\d\\d    # 100-199\n    |   [1-9]?\\d # 0-99\n    )\\b\n){4})\n\n# CIDR notation: /[0-32]\n(?<cidr>/(3[0-2]|[12]?\\d)\\b)?\n\n# Domains list\n(?<domains>\n  [ \\t]+\n  \\w[-\\w]* (?:\\.\\w[-\\w]*)*\n  (?<!-)\\b\n)*+\n\n(?:$|\\s)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Adblock Filter List",
                        patterns: &[
            "(?x)\\A\n\\[\n(?<version>\n  (?:\n    [Aa]d[Bb]lock\n    (?:[ \\t][Pp]lus)?\n    |\n    u[Bb]lock\n    (?:[ \\t][Oo]rigin)?\n    |\n    [Aa]d[Gg]uard\n  )\n  (?:[ \\t] \\d+(?:\\.\\d+)*+)?\n)\n(?:\n  [ \\t]?;[ \\t]?\n  \\g<version>\n)*+\n\\]",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Text",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".typ",
        ],
        rules: &[
            HeuristicRule {
                language: "Typst",
                        patterns: &[
            "^#(import|show|let|set)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "XML",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".url",
        ],
        rules: &[
            HeuristicRule {
                language: "INI",
                        patterns: &[
            "^\\[InternetShortcut\\](?:\\r?\\n|\\r)([^\\s\\[][^\\r\\n]*(?:\\r?\\n|\\r)){0,20}URL=",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".v",
        ],
        rules: &[
            HeuristicRule {
                language: "Rocq Prover",
                        patterns: &[
            "(?:^|\\s)(?:Proof|Qed)\\.(?:$|\\s)|(?:^|\\s)Require[ \\t]+(Import|Export)\\s",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Verilog",
                        patterns: &[
            "^[ \\t]*module\\s+[^\\s()]+\\s+\\#?\\(|^[ \\t]*`(?:define|ifdef|ifndef|include|timescale)|^[ \\t]*always[ \\t]+@|^[ \\t]*initial[ \\t]+(begin|@)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "V",
                        patterns: &[
            "\\$(?:if|else)[ \\t]|^[ \\t]*fn\\s+[^\\s()]+\\(.*?\\).*?\\{|^[ \\t]*for\\s*\\{",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".vba",
        ],
        rules: &[
            HeuristicRule {
                language: "Vim Script",
                        patterns: &[
            "^UseVimball",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "VBA",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".vcf",
        ],
        rules: &[
            HeuristicRule {
                language: "TSV",
                        patterns: &[
            "\\A##fileformat=VCF",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "vCard",
                        patterns: &[
            "\\ABEGIN:VCARD",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".w",
        ],
        rules: &[
            HeuristicRule {
                language: "OpenEdge ABL",
                        patterns: &[
            "&ANALYZE-SUSPEND _UIB-CODE-BLOCK _CUSTOM _DEFINITIONS",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "CWeb",
                        patterns: &[
            "^@(<|\\w+\\.)",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".x",
        ],
        rules: &[
            HeuristicRule {
                language: "DirectX 3D File",
                        patterns: &[
            "^xof 030(2|3)(?:txt|bin|tzip|bzip)\\b",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "RPC",
                        patterns: &[
            "\\b(program|version)\\s+\\w+\\s*\\{|\\bunion\\s+\\w+\\s+switch\\s*\\(",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Logos",
                        patterns: &[
            "^%(end|ctor|hook|group)\\b",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Linker Script",
                        patterns: &[
            "OUTPUT_ARCH\\(|OUTPUT_FORMAT\\(|SECTIONS",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".yaml",
            ".yml",
        ],
        rules: &[
            HeuristicRule {
                language: "MiniYAML",
                        patterns: &[
            "^\\t+.*?[^\\s:].*?:",
        ],
                named_pattern: None,
                        negative_patterns: &[
            "---",
        ],
                and: &[],
            },
            HeuristicRule {
                language: "OASv2-yaml",
                        patterns: &[
            "swagger:\\s?'?\"?2.[0-9.]+'?\"?",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "OASv3-yaml",
                        patterns: &[
            "openapi:\\s?'?\"?3.[0-9.]+'?\"?",
        ],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "YAML",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
    HeuristicEntry {
        extensions: &[
            ".yy",
        ],
        rules: &[
            HeuristicRule {
                language: "JSON",
                        patterns: &[],
                named_pattern: Some("json"),
                        negative_patterns: &[],
                and: &[],
            },
            HeuristicRule {
                language: "Yacc",
                        patterns: &[],
                named_pattern: None,
                        negative_patterns: &[],
                and: &[],
            },
        ],
    },
];
