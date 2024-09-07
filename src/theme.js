import { createTheme } from '@mui/material/styles';

// Create a theme instance with custom styles
const theme = createTheme({
  components: {
    MuiTextField: {
      styleOverrides: {
        root: {
          '& .MuiInputBase-input': {
            color: '#f6f6f6', // Default text color
            transition: 'color 0.3s ease', // Smooth transition for text color
          },
          '& .MuiInputLabel-root': {
            color: '#f6f6f6', // Default label color
            transition: 'color 0.3s ease', // Smooth transition for label color
          },
          '& .MuiOutlinedInput-notchedOutline': {
            borderColor: '#f6f6f6', // Default outline color
            transition: 'border-color 0.3s ease', // Smooth transition for border color
          },
          '&:hover .MuiOutlinedInput-notchedOutline': {
            borderColor: '#f6f6f6', // Outline color on hover
          },
        },
      },
    },
  },
});

export default theme;
