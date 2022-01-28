# dprint-plugin-html

HTML formatting plugin for dprint, using the [`hast`] crate.

[`hast`]: https://github.com/malobre/hast

## Usage

[Install](https://dprint.dev/install) and [setup](https://dprint.dev/setup)
dprint, then:

1. Run
   ```shell
   dprint config add malobre/html
   ```
2. (Optional) Install plugins for the languages embeded in your html files (css, javascript, ..).
3. Ensure `.html` file extensions are matched in an `includes` pattern:
   ```jsonc
   {
     // -- snip --
     "includes": [
       "**/*.html"
     ]
   }
   ```
4. Add a `html` configuration property if desired:
   ```jsonc
   {
     // -- snip --
     "html": {
       // vue config goes here
     }
   }
   ```

## Configuration

| Key              | Default | Description                                |
| ---------------- | ------- | ------------------------------------------ |
| `lineWidth`      | dprint  | Desired maximum line width                 |
| `indentWidth`    | `2`     | Width of the indentation                   |
