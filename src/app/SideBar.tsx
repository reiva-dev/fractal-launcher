import { styled } from "solid-styled-components";

function SideBar() {
  return (
    <SideBarCss>

    </SideBarCss>
  )
}

const SideBarCss = styled.div`
  width: 45px;
  height: calc(100vh - 66px);
  overflow-y: scroll;
  overflow-x: hidden;
  background-color: #2b2e30;
  padding-top: 12px;
  padding-bottom: 12px;
  display: flex;
  flex-shrink: 0;
  flex-direction: column;
  justify-content: flex-start;

  &::-webkit-scrollbar {
    width: 1.5px;
  }

  &::-webkit-scrollbar-track {
    background: #232426;
  }

  &::-webkit-scrollbar-thumb {
    background: #888;
  }
`;

export default SideBar;