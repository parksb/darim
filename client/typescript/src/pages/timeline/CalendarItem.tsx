import React from 'react';
import { Link } from "react-router-dom";
import styled from 'styled-components';
import dayjs from 'dayjs';

import { Post } from '../../models';
import { Section } from "../../components";

interface Props {
  posts?: Post[];
  day: dayjs.Dayjs;
}

const Container = styled(({ is_current_month, ...other }) => <Section {...other} />)`
  flex: 7;
  border-left: 1px solid #000000;
  border-right: 1px solid #000000;
  height: 150px;
  background-color: ${props => props.is_current_month ? '#ffffff' : '#f5f5f5'};
  max-width: 113px;
  min-width: 0;
  overflow: scroll;
  padding-bottom: 5px;

  &:nth-child(even) {
    border-right: 0;
    border-left: 0;
  };
`;

const Date = styled.time`
  font-size: 12px;
  padding: 10px 10px 5px;
`;

const PostContainer = styled.div`
  overflow: scroll;
  max-width: 100%;
  padding: 3px 10px 3px 10px;
  word-wrap: normal;
  white-space: nowrap;
  font-size: 14px;
  scrollbar-width: none;
  line-height: 21px;
`;

const StyledLink = styled(Link)`
  text-decoration: none;
  color: #000000;

  &:hover {
    background-color: #ffce05;
  }
`;

const CalendarItem: React.FC<Props> = ({ posts, day }) => {
  const displayed_date = day.date() === 1 ? day.format('MM / DD') : day.format('D');
  const is_today = day.format('YYYY-MM-DD') === dayjs().format('YYYY-MM-DD');

  return <Container is_current_month={day.month() === dayjs().month()}>
    <Section>
      <Date dateTime={day.format('YYYY-MM-DD')}>
        {displayed_date}
        {is_today && '📌'}
      </Date>
      {posts && posts.map((post) => {
        return (
          <StyledLink to={`/post/${post.id}`}>
            <PostContainer>
              {post.title}
            </PostContainer>
          </StyledLink>
        );
      })}
    </Section>
  </Container>
};

export default CalendarItem;
