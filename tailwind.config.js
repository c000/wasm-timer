const colors = require('tailwindcss/colors')

module.exports = {
  content: [
    'index.html',
    'index.js',
    './src/**/*.rs',
  ],
  darkMode: 'class', // or 'media' or 'class'
  theme: {
    extend: {
      colors: {
        gray: colors.stone,
        orange: colors.orange,
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
