import { createTheme } from '@mui/material/styles';

/**
 * Theme for the input boxes to change to light purple instead of blue.
 */
const theme = createTheme({
  components: {
    MuiTextField: {
      styleOverrides: {
        root: {
          '& .MuiInputBase-input': {
            color: '#f6f6f6',
            transition: 'color 0.3s ease',
            '&:focus': {
              color: '#D8BFD8',
            },
          },
          '& .MuiInputLabel-root': {
            color: '#f6f6f6',
            transition: 'color 0.3s ease',
            '&.Mui-focused': {
              color: '#D8BFD8',
            },
          },
          '& .MuiOutlinedInput-notchedOutline': {
            borderColor: '#f6f6f6',
            transition: 'border-color 0.3s ease',
          },
          '&:hover .MuiOutlinedInput-notchedOutline': {
            borderColor: '#f6f6f6',
          },
          '& .MuiOutlinedInput-root.Mui-focused .MuiOutlinedInput-notchedOutline': {
            borderColor: '#D8BFD8',
          },
        },
      },
    },
  },
});

export default theme;
