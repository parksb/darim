import React from 'react';
import styled from 'styled-components';

interface Props {
  text: string
  valueState: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
}

const Label = styled.label`
  cursor: pointer;
`;

const Input = styled.input`
  margin: 0 3px 0 0;
  width: 15px;
  height: 15px;
`;

const Checkbox: React.FC<Props> = ({ text, valueState }) => {
  const [value, setValue] = valueState;

  const change = () => {
    setValue(!value);
  };

  return <Label>
    <Input type='checkbox' checked={value} onChange={change} />
    {text}
  </Label>
};

export default Checkbox;
