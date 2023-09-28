/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: "class",
    content: { 
      files: ["./site/*.html", "./site/src/**/*.rs", "./crates/**/*.rs", "./site/src/*.rs"],
    },
    theme: {
      extend: {},
    },
    plugins: [],
  }