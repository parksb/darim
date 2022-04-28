import React from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';

import { getI18n } from '../utils/i18n';
import { Session } from '../models';

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

  &:hover {
    background-color: #ffce05;
  }
`;

const UserAvatar = styled.div<{ src: string }>`
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
  const i18n = getI18n({
    title: {
      ko: '🏕 Darim',
      en: '🏕 Darim',
    },
  });

  return <HeaderContainer>
    <StyledLink to='/'>
      <Title>{i18n.text('title')}</Title>
    </StyledLink>
    {session && (
      <Link to='/settings'>
        <UserAvatar src={session?.user?.avatar_url} />
      </Link>
    )}
  </HeaderContainer>
};

export default Header;
