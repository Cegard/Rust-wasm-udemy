import path from "path";
import copy from "copy-webpack-plugin";

module.exports = {
  entry: "./public/index.ts",
  output: {
    path: path.resolve(__dirname, "public/static"),
    filename: "index.js"
  },
  mode: "development",
  plugins: [
    new copy({
      patterns: [
        {from: "public/index.html", to: "./index.html"}
      ]
    })
  ]
};
