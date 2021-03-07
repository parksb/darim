import React, {ButtonHTMLAttributes} from 'react';
import styled from 'styled-components';

interface Props extends ButtonHTMLAttributes<HTMLButtonElement>{
  disabled?: boolean;
}

const StyledButton = styled.button`
  font-size: 14px;
  border: 1px solid #000000;
  padding: 5px 10px;
  background-color: ${props => props.disabled ? '#e9e9e9' : '#ffffff'};
  color: ${props => props.disabled ? '#808080' : '#000000'};
  cursor: ${props => props.disabled ? 'arrow' : 'pointer'};

  &:hover {
    background-color: ${props => props.disabled ? '#e9e9e9' : '#ffce05'};
  }
`;

const Button: React.FC<Props> = ({ children, disabled, ...props }) => {
  return <StyledButton disabled={disabled} {...props}>{children}</StyledButton>
};

export default Button;
