/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./app/**/*.rs"]
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
  daisyui: {
    themes: ["light", "dark"],
  },
}
