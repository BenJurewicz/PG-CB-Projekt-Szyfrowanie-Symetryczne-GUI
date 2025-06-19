module.exports = {
    content: [
        "./src/**/*.rs",
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require("daisyui"),
        function ({ addVariant }) {
            addVariant('portrait', '@media (orientation: portrait)');
            addVariant('landscape', '@media (orientation: landscape)');
        }
    ],
}