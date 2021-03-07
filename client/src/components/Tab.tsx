import styled from 'styled-components'

const Tab = styled.div`
  border: 1px solid #000000;
  border-top: 0;
  border-bottom: 0;
  font-size: 14px;
  background-color: #ffffff;
  color: #000000;
  padding: 5px 10px;
  cursor: pointer;
  text-align: center;
  max-height: 32px;

  &:hover {
    background-color: #ffce05;
  }
`;

export default Tab;
