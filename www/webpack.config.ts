import * as path from "path";
import copy from "copy-webpack-plugin";

module.exports = {
  entry: "./public/bootstrap.ts",
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
