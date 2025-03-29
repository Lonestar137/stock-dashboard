/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs", // Scan rust files for Tailwind classes
    "./index.html"
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}

