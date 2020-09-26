import React from 'react';
import styled from 'styled-components';
import { Link } from 'react-router-dom';

const FooterContainer = styled.footer`
  margin: auto;
  padding-top: 50px;
  padding-bottom: 40px;
`;

const StyledAnchor = styled.a`
  text-decoration: none;
  color: #586069;

  &:hover {
    text-decoration: underline;
  }
`;

const StyledLink = styled(Link)`
  text-decoration: none;
  color: #586069;

  &:hover {
    text-decoration: underline;
  }
`;

const List = styled.ul`
  display: flex;
  flex-direction: row;
  list-style: none;
`;

const ListItem = styled.li`
  margin-left: 5px;
  margin-right: 5px;
  color: #586069;
  font-size: 12px;
`;

const Footer: React.FC = () => {
  return <FooterContainer>
    <List>
      <ListItem>© 2020 Darim.</ListItem>
      <ListItem><StyledLink to='/static/terms'>Terms</StyledLink></ListItem>
      <ListItem><StyledLink to='/static/privacy'>Privacy</StyledLink></ListItem>
      <ListItem><StyledAnchor href='https://github.com/parksb/darim'>GitHub</StyledAnchor></ListItem>
    </List>
  </FooterContainer>
};

export default Footer;
