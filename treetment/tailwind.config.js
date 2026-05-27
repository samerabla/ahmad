/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs", "./index.html", "./public/**/*.html"],
  theme: {
    extend: {
      colors: {
        forest: {
          DEFAULT: '#1a3d2b',
          50: '#e8f2ec',
          100: '#c5dece',
          200: '#9ec8ad',
          300: '#77b18b',
          400: '#50996a',
          500: '#1a3d2b',
          600: '#163425',
          700: '#122b1f',
          800: '#0e2218',
          900: '#0a1912',
        },
        sage: {
          DEFAULT: '#4a9c6d',
          light: '#6ab885',
          dark: '#3a7d57',
        },
        cream: '#f9f6f0',
        charcoal: '#1c1c1c',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        display: ['"Playfair Display"', 'Georgia', 'serif'],
      },
      animation: {
        'fade-up': 'fadeUp 0.6s ease-out forwards',
        'fade-in': 'fadeIn 0.5s ease-out forwards',
        'counter': 'counter 2s ease-out forwards',
      },
      keyframes: {
        fadeUp: {
          '0%': { opacity: '0', transform: 'translateY(30px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}
