const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const WebpackNotifierPlugin = require('webpack-notifier');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const sass = require('node-sass');

const distPath = path.resolve(__dirname, "dist");
console.log(distPath);

module.exports = (env, argv) => {
  return {
    watch: argv.mode !== 'production',
    entry: {
      app: './web/app.js'
    },
    output: {
      path: distPath,
      filename: "[name].bundle.js",
      publicPath: '/static/',
      webassemblyModuleFilename: "app.wasm"
    },
    plugins: [
      new WebpackNotifierPlugin(),
      new CopyWebpackPlugin([
        { from: './web/static', to: distPath }
      ]),
      new HtmlWebpackPlugin({
        title: 'Message Feed',
        favicon: "./web/static/favicon.ico",
        inject: false,
        template: require('html-webpack-template'),
        mobile: true,
        headHtmlSnippet: require('fs').readFileSync('./web/snippets/head.html', 'utf8')
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, "crates", "webapp"),
        extraArgs: "--no-typescript"
      })
    ],
    module: {
      rules: [
        {
          test: /\.scss$/,
          use: [
            'style-loader',
            'css-loader',
            'sass-loader'
          ]
        }
      ]
    }
  };
};
