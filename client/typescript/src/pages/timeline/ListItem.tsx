import React from 'react';
import { Link } from "react-router-dom";
import styled from 'styled-components';
import dayjs from 'dayjs';

import { Post } from '../../models';
import { Container } from '../../components';

interface Props {
  post: Post;
}

const Title = styled.h3`
  font-size: 18px;
  font-weight: 700;
`;

const Date = styled.time`
  font-size: 12px;
  align-self: center;
`;

const HorizontalLine = styled.div`
  width: 30px;
  height: 1px;
  background-color: #000000;
  margin: 0 20px 0 20px;
  align-self: center;
`;

const StyledLink = styled(Link)`
  display: flex;
  text-decoration: none;
  color: #000000;

   &:hover {
    background-color: #ffce05;
  }
`;

const ListItem: React.FC<Props> = ({ post }) => {
  const { id, title, date } = post;
  const displayed_date = dayjs(date).format('YYYY / MM / DD');

  return <Container bottom={30} row>
    <StyledLink to={`/post/${id}`}>
      <Date dateTime={date}>{displayed_date}</Date>
      <HorizontalLine />
      <Title>{title}</Title>
    </StyledLink>
  </Container>
};

export default ListItem;
