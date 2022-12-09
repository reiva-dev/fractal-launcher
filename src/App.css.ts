import { style } from "@vanilla-extract/css";

export const app = {
  main: style({
    height: '100vh',
    display: 'grid',
    gridTemplateRows: '30px 42px auto 20px',
    border: '#424242',
    borderColor: '#424242',
    borderWidth: '1px',
    borderStyle: 'solid',
    boxSizing: 'border-box',
  }),

  wrap: style({
    background: '#232426'
  })
}