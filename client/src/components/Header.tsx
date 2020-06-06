import React from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';

const HeaderContainer = styled.header`
  margin-bottom: 40px;
`;

const StyledLink = styled(Link)`
  text-decoration: none;
  color: #000000;
`;

const Title = styled.h1`
  display: inline;
`;

const Header: React.FC = () => {
  return <HeaderContainer>
    <StyledLink to='/'>
      <Title>🏕 Darim</Title>
    </StyledLink>
  </HeaderContainer>
};

export default Header;
