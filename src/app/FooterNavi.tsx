import Signin from "./footer/Signin";
import { footerNavi } from "./FooterNavi.css";

function FooterNavi() {
  return (
    <footer class={footerNavi.mainContainer}>
      <Signin />
    </footer>
  )
}

export default FooterNavi;