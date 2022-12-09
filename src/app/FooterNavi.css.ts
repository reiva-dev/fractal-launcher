import { style } from "@vanilla-extract/css"

export const footerNavi = {
  mainContainer: style({
    position: 'relative',
    alignItems: 'center',
    justifyContent: 'space-between',
    display: 'flex',
    flexDirection: 'row',
    flexGrow: 3,
    overflow: 'hidden',
    height: '20px',
    boxSizing: 'border-box',
    backgroundColor: '#232526',
    borderWidth: '1px',
    borderTopStyle: 'solid',
    borderColor: '#424242',
    fontSize: '12px'
  }),

  footerButton: style({
    display: 'inline-flex',
    height: '20px',
    padding: '0px 6px',
    alignItems: 'center',
    justifyContent: 'center',
    boxSizing: 'border-box',
    ':hover': {
      background: 'rgba(200, 200, 200, 0.6)',
    },
  }),

  account: style({
    height: '20px',
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center'
  }),

  statusBox: style({
    height: '20px',
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center'
  })
}