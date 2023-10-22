/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: [
    "*.html",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

