/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx,vue}"],
  theme: {
    extend: {
      colors: {
        primary: "#229A91",
        secondary: "#2EB08E",
      },
    },
  },
  plugins: [],
};
