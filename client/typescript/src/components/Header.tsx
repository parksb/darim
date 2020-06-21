import React from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';

import { Session } from "../models";

interface Props {
  session: Session | null;
}

const HeaderContainer = styled.header`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 40px;
`;

const StyledLink = styled(Link)`
  text-decoration: none;
  color: #000000;
`;

const Title = styled.h1`
  display: inline;
  font-size: 28px;
  font-weight: 700;
`;

const UserAvatar = styled(({ src, ...other }) => <div {...other} />)`
  width: 35px;
  height: 35px;
  background-color: #c0c0c0;
  border-radius: 50%;
  background-image: url(${props => props.src || ''});
  background-position: center;
  background-size: cover;
  background-repeat: no-repeat;
`;

const Header: React.FC<Props> = ({ session }) => {
  return <HeaderContainer>
    <StyledLink to='/'>
      <Title>🏕 Darim</Title>
    </StyledLink>
    {session && (
      <Link to='/settings'>
        <UserAvatar src={session.user_avatar_url} alt="Your profile"/>
      </Link>
    )}
  </HeaderContainer>
};

export default Header;
