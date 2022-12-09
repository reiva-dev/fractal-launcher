import { style, styleVariants } from '@vanilla-extract/css';

export const pallette = {
  Normal: 'rgba(255, 255, 255, 0.5)',
  Alart: 'rgba(244, 67, 54, 0.5)'
} as const;

export type pallette = typeof pallette[keyof typeof pallette];

export const titleBar = {
  mainContainer: style({
    height: '30px',
    background: '#232526',
    userSelect: 'none',
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    top: 0,
    left: 0,
    right: 0,
    borderColor: '#424242',
    borderBottomStyle: 'solid',
    borderWidth: '1px',
    boxSizing: 'border-box'
  }),

  rights: style({ display: 'flex' }),
  icon: style({
    display: 'inline-flex',
    justifyContent: 'center',
    alignItems: 'center',
    width: '45px',
    height: '30px'
  }),
  title: style({
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    color: '#6d6d6d'
  }),

  toolBox: style({ display: 'flex' }),

  rootBox: style({ display: 'dlex' }),
  button: styleVariants(pallette, (hoveredColor) => [
    style({
      display: 'inline-flex',
      justifyContent: 'center',
      alignItems: 'center',
      width: '45px',
      height: '30px',
    }),

    {':hover': { background: hoveredColor } }
  ])
};
