/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{html, css,rs}",
    "./dist/**/*.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('tailwindcss-animated'),
  ],
}

