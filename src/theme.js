import { createTheme } from '@mui/material/styles';

const theme = createTheme({
  components: {
    MuiTextField: {
      styleOverrides: {
        root: {
          '& .MuiInputBase-input': {
            color: '#f6f6f6', // Default text color (white-ish)
            transition: 'color 0.3s ease',
            '&:focus': {
              color: '#D8BFD8', // Light purple color on focus
            },
          },
          '& .MuiInputLabel-root': {
            color: '#f6f6f6', // Default label color
            transition: 'color 0.3s ease',
            '&.Mui-focused': {
              color: '#D8BFD8', // Light purple color when focused
            },
          },
          '& .MuiOutlinedInput-notchedOutline': {
            borderColor: '#f6f6f6', // Default outline border color
            transition: 'border-color 0.3s ease',
          },
          '&:hover .MuiOutlinedInput-notchedOutline': {
            borderColor: '#f6f6f6', // Outline color on hover
          },
          '& .MuiOutlinedInput-root.Mui-focused .MuiOutlinedInput-notchedOutline': {
            borderColor: '#D8BFD8', // Light purple border color when focused
          },
        },
      },
    },
  },
});

export default theme;
