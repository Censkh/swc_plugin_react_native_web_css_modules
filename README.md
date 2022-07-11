# swc_plugin_react_native_web_css_modules

A SWC plugin that allows you to use CSS modules with React Native Web

## How does it work?

All imports of `.css` or `.scss` files will be wrapped with a function call that turns all the exported classnames into https://github.com/necolas/styleq (what RNW uses internally) compatible objects
