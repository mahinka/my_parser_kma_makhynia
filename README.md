# SVG File Parser

### Brief Describe

* Rust-based SVG parser designed to interpret Scalable Vector Graphics (SVG) files.
* Capable of extracting structural information such as SVG elements, attributes, and text content.
* Uses the Pest parser generator to parse SVG syntax.
* Outputs a structured representation of SVG content, including elements, attributes, and text.
* Suitable for Rust developers who work with SVG files, offering a basis for further manipulation or analysis of SVG data.
### Command-Line Interface

* Help
Display help information:

```
cargo run -- help
```

* Parse
Parse a SVG file and output file content.

```
cargo run -- parse -i input.svg -o src/output.txt
```

If you don't specify an output file, file content will be written to your command-line interface.

```
cargo run -- parse -i input.svg
```

### Grammar
```pest
svg_file = { svg_open ~ svg_content ~ svg_close }
svg_open = { "<svg" ~ attribute* ~ ">" ~ NEWLINE }
svg_content = { ((element | text_content ) ~ NEWLINE)* }
svg_close = { "</svg>" }

element = { circle | rect | line | ellipse }

circle = { "<circle" ~ attribute+ ~ "/>" }
rect = { "<rect" ~ attribute+ ~ "/>" }
line = { "<line" ~ attribute+ ~ "/>" }
ellipse = { "<ellipse" ~ attribute+ ~ "/>" }

attribute = { attribute_name ~ "=" ~ "\"" ~ attribute_value ~ "\"" }
attribute_name = { ASCII_ALPHA ~ ASCII_ALPHANUMERIC* }
attribute_value = { ASCII_ALPHANUMERIC* }
text_content = { (PUNCTUATION | ASCII_ALPHANUMERIC)+ ~ (PUNCTUATION | ASCII_ALPHANUMERIC)*}
WHITESPACE = _{ " " }
```
* svg_file: Represents a complete SVG file, consisting of an opening tag (svg_open), content (svg_content), and a closing tag (svg_close).
* svg_open: Defines the opening tag for an SVG element, allowing for attributes and ending with a newline.
* svg_content: Describes the content of the SVG file, which can be a sequence of SVG elements (element) or text content (text_content). Each line ends with a newline.
* svg_close: Specifies the closing tag for the SVG element.
* element: Represents different SVG elements, including circle, rect, line, and ellipse.
* circle, rect, line, ellipse: Define specific SVG elements with their respective attributes, enclosed in opening and closing tags.
* attribute: Represents a key-value pair within an SVG element, with a name (attribute_name) and a value (attribute_value).
* attribute_name: Specifies the name of an attribute, starting with an alphabet character and followed by alphanumeric characters.
* attribute_value: Represents the value of an attribute, consisting of alphanumeric characters.
* text_content: Describes text content within an SVG file, allowing a mix of punctuation and alphanumeric characters.
* WHITESPACE: Represents optional whitespace.

### Example
```
<svg>
    <circle cx="10" cy="10" r="5" />
    <rect x="20" y="20" width="10" height="10" />
    Some text content
</svg>
Result:
[
Svg(
        [],
    ),
    Circle(
        [
            (
                "cx",
                "10",
            ),
            (
                "cy",
                "10",
            ),
            (
                "r",
                "5",
            ),
        ],
    ),
    Rect(
        [
            (
                "x",
                "20",
            ),
            (
                "y",
                "20",
            ),
            (
                "width",
                "10",
            ),
            (
                "height",
                "10",
            ),
        ],
    ),
    Text(
        "Some text content",
    ),
]
```
