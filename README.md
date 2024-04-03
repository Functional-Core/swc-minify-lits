# swc-minify-lits

A plugin for [SWC](https://swc.rs/) for minifying HTML and CSS template literals via [`lightningcss`](https://lightningcss.dev/)
and [`minify-html`](https://github.com/wilsonzlin/minify-html).

Works with both tagged and untagged template literals.

## Configuration Options

| Name               | Description                                                                 | Default Value |
| ------------------ | --------------------------------------------------------------------------- | ------------- |
| `minifyCss`        | Should the plugin minify CSS literals?                                      | `true`        |
| `minifyHtml`       | Should the plugin minify HTML literals?                                     | `true`        |
| `minifyCssInHtml`  | Passed to `minify-html` to minify `<style>...` and `style=...` within HTML. | `false`       |
| `minifyJsInHtml`   | Passed to `minify-html` to minify `<script>...` within HTML.                | `false`       |
| `keepHtmlComments` | Keep comments in HTML?                                                      | `false`       |
