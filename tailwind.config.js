const colors = require('tailwindcss/colors')

module.exports = {
  purge: [
    'index.html',
    './src/**/*.rs',
  ],
  darkMode: 'class', // or 'media' or 'class'
  theme: {
    extend: {
      colors: {
        gray: colors.warmGray,
        orange: colors.orange,
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
