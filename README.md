# `colorname`
Get color data based on names in multiple languages. (Color name data collected from various sources, based on https://github.com/meodai/color-name-lists)

## Installation
You can install via `cargo install`:

```
cargo install colorname
```

## Usage

Example:

To search for colornames with "sky" in Wikipedia colors (https://github.com/meodai/wikipedia-color-names):

```
colorname sky
```

Search "sky" in all English language lists:

```
colorname sky --all-en
```

Search for "bleu" in French list, export to CSV:

```
colorname bleu --fr --output=csv
```

Search for "bleu" in French list, export to CSV, don't display it in terminal screen:

```
colorname bleu --fr --output=csv --quiet
```

Or:

```
colorname bleu --fr --output=csv -q
```

Include the metadata for colors if it exists:

```
colorname sky --with-info
```

<!-- start options -->
### Options

#### Lists

##### `--basic`

> "A set of basic colors, such as red, green, and blue."

Source: https://github.com/colorjs/color-namer/tree/master/lib/colors

##### `--chinese-traditional` (alias = `--zh`)

> "Traditional Colors of China: Color aesthetics in the Forbidden City (中国传统色：故宫里的色彩美学), including colors and their transliterations."

Source: https://github.com/ItMarki/files/blob/main/newcolorsandnames.csv

##### `--french` (alias = `--fr`)

> "A list of color names in French."

Source: https://github.com/meodai/noms-de-couleur

##### `--german` (alias = `--de`)

> "A list of color names in German."

Source: https://github.com/meodai/farbnamen

##### `--hindi` (alias = `--hi`)

> "A list of color names in Hindi."

Source: https://github.com/meodai/hindi-color-names

##### `--html`

> "HTML/CSS color names. These can be used as keywords in CSS, SVG, or HTML."

Source: https://github.com/colorjs/color-namer/tree/master/lib/colors

##### `--japanese-traditional` (alias = `--ja`)

> "Colors traditionally used in Japanese art, literature, textiles such as kimono, and other crafts."

Source: https://en.wikipedia.org/wiki/Traditional_colors_of_Japan

##### `--le-corbusier`

> "Architectural colors from Le Corbusier's color system."

Source: https://www.lescouleurs.ch/en/the-colours/63-colours

##### `--mlmc-chinese` (alias = `--mlmc-zh`)

> "Chinese color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-dutch` (alias = `--mlmc-nl`)

> "Dutch color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-english` (alias = `--mlmc-en`)

> "English color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-finnish` (alias = `--mlmc-fi`)

> "Finnish color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-french` (alias = `--mlmc-fr`)

> "French color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-german` (alias = `--mlmc-de`)

> "German color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-korean` (alias = `--mlmc-ko`)

> "Korean color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-persian` (alias = `--mlmc-fa`)

> "Persian color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-polish` (alias = `--mlmc-pl`)

> "Polish color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-portuguese` (alias = `--mlmc-pt`)

> "Portuguese color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-romanian` (alias = `--mlmc-ro`)

> "Romanian color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-russian` (alias = `--mlmc-ru`)

> "Russian color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-spanish` (alias = `--mlmc-es`)

> "Spanish color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--mlmc-swedish` (alias = `--mlmc-sv`)

> "Swedish color names from the UW Interactive Data Lab's Many Languages Many Colors project, a research effort exploring how different languages divide the color spectrum."

Source: https://idl.uw.edu/color-naming-in-different-languages/

##### `--nbs-iscc`

> "ISCC–NBS system of color designation based on 12 basic color terms and a small set of adjective modifiers."

Source: https://web.archive.org/web/20121103030619/http://tx4.us/nbs-iscc.htm

##### `--ntc`

> "NTC.js color matching library names, released in 2007. The names were collected from various sources such as Wikipedia, X11, and Crayola."

Source: https://github.com/colorjs/color-namer/tree/master/lib/colors

##### `--osxcrayons`

> "Color names used in the OS X color picker GUI."

Source: http://www.randomactsofsentience.com/2013/06/os-x-crayon-color-hex-table.html

##### `--ral`

> "RAL color matching names."

Source: https://jpederson.com/colornerd/

##### `--ridgway`

> "Color Nomenclature by Robert Ridgway (1850-1929), published in Washington, DC in 1912. It is part of the public domain in the USA. On August 31, 2020, it was added to the Gutenberg Project."

Source: https://github.com/davo/Color-Standards-and-Color-Nomenclature

##### `--risograph`

> "Popular colors for Risograph printing, with HEX, Pantone, and Z-Type codes."

Source: https://github.com/mattdesl/riso-colors

##### `--sanzo-wada-i`

> "Names from Wada Sanzō (和田 三造) Colors Dictionary, Volume I."

Source: https://sanzo-wada.dmbk.io/

##### `--spanish` (alias = `--es`)

> "A list of color names in Spanish."

Source: https://github.com/meodai/nombres-de-colores

##### `--thesaurus`

> "The Color Thesaurus by Ingrid Sundberg. As a writer, she collected color names to explore the emotion of a scene and create variety in her writing."

Source: https://ingridsnotes.wordpress.com/2014/02/04/the-color-thesaurus/

##### `--werner`

> "All colors from the book Werner's Nomenclature of Colours, collected and described in the late 18th century"

Source: https://www.c82.net/werner/

##### `--wikipedia`

> "A list of color names scraped from Wikipedia."

Source: https://github.com/meodai/wikipedia-color-names

##### `--windows`

> "Color names used in legacy Microsoft Windows systems."

Source: https://docs.microsoft.com/en-us/dotnet/api/system.windows.media.colors?view=windowsdesktop-6.0

##### `--x11`

> "Standard Xlib or X11 protocol color names."

Source: https://en.wikipedia.org/wiki/X11_color_names

##### `--xkcd`

> "The 954 most common RGB monitor colors, as defined by several hundred thousand participants in the XKCD color name survey."

Source: https://xkcd.com/color/rgb/


<!-- end options -->

#### Other options

##### `--all-en`
Search in all English lists. Currently:
- `--basic`
- `--html`
- `--mlmc-english` (alias = `--mlmc-en`)
- `--nbs-iscc`
- `--ntc`
- `--osxcrayons`
- `--ral`
- `--ridgway`
- `--risograph`
- `--sanzo-wada-i`
- `--thesaurus`
- `--werner`
- `--windows`
- `--wikipedia`
- `--xkcd`
- `--x11`

##### `--all-fr`
Search in all French lists. Currently:
- `--french` (alias = `--fr`)
- `--le-corbusier`
- `--mlmc-french` (alias = `--mlmc-fr`)

##### `--all-de`
Search in all German lists. Currently:
- `--german` (alias = `--de`)
- `--mlmc-german` (alias = `--mlmc-de`)

##### `--all-es`
Search in all Spanish lists. Currently:
- `--spanish` (alias = `--es`)
- `--mlmc-spanish` (alias = `--mlmc-es`)

##### `--all-zh`
Search in all Chinese lists. Currently:
- `--chinese-traditional` (alias = `--zh`)
- `--mlmc-chinese` (alias = `--mlmc-zh`)

##### `--with-info`
If metadata exists for colors in the list, include the metadata.

#### Output formats

##### `--html`
Export to HTML. Requires the `file_path` option to be set.

```
colorname sky --output=html --file-path=result.html
```

##### `--json`
Export to JSON. Requires the `file_path` option to be set.

```
colorname sky --output json --file-path /path/to/result.json
```

##### `--csv`
Export to CSV. Requires the `file_path` option to be set.

```
colorname sky --output csv --file-path result.csv
```

#### Other options

##### `--quiet` (`-q`)
Don't display output in terminal if the output format is given.

---

Note that if you use multiple lists, there might be duplicates for generic colors, i.e., pink.
