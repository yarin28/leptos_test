/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
      files: ["*.html", "./src/**/*.rs"],
  },
  safelist:[
  'btn-error',
  'btn-info',
  'btn-success',
  'btn-warning',
  ],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"),require("daisyui")],
}

