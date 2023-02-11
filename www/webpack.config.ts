import path from "path";

module.exports = {
    entry: "./index.ts",
    output: {
        path: path.resolve(__dirname, "public"),
        filename: "index.js"
    },
    mode: "development"
};
