import React from 'react';
import styled from 'styled-components';

interface Props {
  text: string
  valueState: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
}

const Input = styled.input`
  margin: 0 3px 0 0;
  width: 18px;
  height: 18px;
`;

const Checkbox: React.FC<Props> = ({ text, valueState }) => {
  const [value, setValue] = valueState;

  const change = () => {
    setValue(!value);
  };

  return <label>
    <Input type='checkbox' checked={value} onChange={change} />
    {text}
  </label>
};

export default Checkbox;
