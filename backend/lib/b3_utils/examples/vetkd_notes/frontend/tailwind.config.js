/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./public/index.html", "./src/**/*.svelte"],
  theme: {
    extend: {}
  },
  plugins: [require("daisyui"), require("@tailwindcss/line-clamp")]
}
