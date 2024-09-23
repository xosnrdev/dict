# Dict

This is a simple command-line interface (CLI) tool designed to fetch word definitions from the Free Dictionary API. It
allows users to quickly look up the meaning of words directly from the terminal.

## Features

- Fetches definitions for any word using the Free Dictionary API.
- Lightweight and easy to use.
- Only makes an API call for the first word provided as an argument.

## Prerequisites

- Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/xosnrdev/dict.git
cd dict
```

## Usage

To run the application and fetch the definition of a word, use the following command:

```bash
cargo run <word>
```

### Example

To get the definition of "pizza", run:

```bash
cargo run pizza
```

Only the first parameter after `cargo run` will be used for the API request. Additional parameters will be ignored.

## Notes

- The tool is limited to retrieving definitions for single words.
- Ensure you have an active internet connection as the tool requires making API calls.

## License

MIT License

Copyright (c) Success Kingsley

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.