// import * as path from "path";
// import * as copy from "copy-webpack-plugin";
const path = require("path");
const copy = require("copy-webpack-plugin");

module.exports = {
  entry: "./public/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "public/static"),
    filename: "bootstrap.js"
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  plugins: [
    new copy({
      patterns: [
        {from: "public/index.html", to: "./index.html"}
      ]
    })
  ]
};
