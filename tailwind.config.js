/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "*.html",
    "./src/**/*.rs",
    "./node_modules/flowbite/**/*.js",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('flowbite/plugin'),
  ],
}

