/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  safelist: [
    {
      pattern: /.*/,
    },
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
