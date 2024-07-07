/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./app/**/*.rs", "./components/**/*.rs"]
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
  daisyui: {
    themes: ["light", "dark"],
  },
}
