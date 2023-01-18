/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ['*.html', './src/**/*.rs']
  },
  theme: {
    extend: {
      colors: {
        bg: '#36393f',
        igdb: '#9147ff'
      }
    }
  },
  plugins: []
}
