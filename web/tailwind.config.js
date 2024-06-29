/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        background: '#452B4E',
        inputBackground: '#F5F5F5',
        primary: '#FF805D',
        black: '#3A3A3A',
        white: '#FFFFFF',
        whiteSecondary: '#FCFBFC',
        grey: '#898989',
        offGrey: '#EAEAEA'
      },
      borderRadius: {
        'lg': '1rem',
      }
    },
  },
  plugins: [],
}

