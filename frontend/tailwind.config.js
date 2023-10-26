/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.{js,jsx,ts,tsx}', './node_modules/flowbite/**/*.js'],
    theme: {
        extend: {
            dropShadow: {
                hard: '4px 4px 0px rgba(0,0,0,1)',
            },
            colors: {
                'white': '#FFFFFF', // AHA White
                'gray': '#636466',  // AHA Gray
                'black': '#000000', // AHA Black
                'deep-red': '#A51431',  // AHA Deep Red (Accent)
                // Define other colors you may need
                // ...
            },
            textColor: {
                'default': '#000000',    // Black as default text color
                'accent': '#A51431',      // AHA Deep Red as accent text color
                // Define other text colors you may need
                // ...
            },
            fontFamily: {
                'lub-dub': ['Lub Dub', 'sans-serif'],  // AHA's custom font
                'georgia': ['Georgia', 'serif'],       // Georgia Regular
                // Add other font styles from AHA
                // ...
            },
        },
    },
    plugins: [require('flowbite/plugin')],
};
