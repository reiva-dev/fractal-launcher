import { styled } from "solid-styled-components";

function PlaceHolder() {
  return (
    <PlaceHolderCss>
        <p>PlaceHolder!</p>
    </PlaceHolderCss>
  );
}

const PlaceHolderCss = styled.div`
  overflow-y: scroll;
  overflow-x: hidden;
  background-color: #232426;
  padding: 12px;
  display: flex;
  flex-grow: 3;
  flex-direction: column;
  justify-content: flex-start;
  
  &::-webkit-scrollbar {
    width: 8px;
  }

  &::-webkit-scrollbar-track {
    background: #232426;
  }

  &::-webkit-scrollbar-thumb {
    background: #888;
  }
`;

export default PlaceHolder;