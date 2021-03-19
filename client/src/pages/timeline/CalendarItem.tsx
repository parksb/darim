import React from 'react';
import { Link } from 'react-router-dom';
import styled from 'styled-components';
import dayjs from 'dayjs';

import { SummarizedPost } from '../../models';
import { Button, Section } from '../../components';

interface Props {
  posts?: SummarizedPost[];
  day: dayjs.Dayjs;
  cursorDate: dayjs.Dayjs;
}

const Container = styled(({ is_current_month, ...other }) => <Section {...other} />)`
  flex: 7;
  border-left: 1px solid #000000;
  border-right: 1px solid #000000;
  background-color: ${props => props.is_current_month ? '#ffffff' : '#f5f5f5'};
  min-width: 0;
  max-height: 100%;

  &:nth-child(even) {
    border-right: 0;
    border-left: 0;
  };

  &:hover button {
    display: block;
  }
`;

const ItemHead = styled(Section)`
  justify-content: space-between;
`;

const Date = styled.time`
  font-size: 12px;
  padding: 5px 0 0 10px;
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

const PostLink = styled(Link)`
  text-decoration: none;
  color: #000000;

  &:hover {
    background-color: #ffce05;
  }
`;

const NewPostButton = styled(Button)`
  display: none;
  border: 0;
  border-radius: 50%;
  background-color: transparent;
`;

const NewPostLink = styled(Link)`
  text-decoration: none;
  color: #000000;
`;

const OverflowSection = styled(Section)`
  overflow: auto;
`;

const CalendarItem: React.FC<Props> = ({ posts, day, cursorDate }) => {
  const displayed_date = day.date() === 1 ? day.format('MM / DD') : day.format('D');
  const is_today = day.format('YYYY-MM-DD') === dayjs().format('YYYY-MM-DD');

  return <Container is_current_month={day.month() === cursorDate.month()}>
    <ItemHead row>
      <Date dateTime={day.format('YYYY-MM-DD')}>
        {displayed_date}
        {is_today && '📌'}
      </Date>
      <NewPostLink to={`/post?date=${day.format('YYYY-MM-DD')}`}>
        <NewPostButton>+</NewPostButton>
      </NewPostLink>
    </ItemHead>
    <OverflowSection>
    {posts && posts.map((post) => {
      return (
        <PostLink key={post.id} to={`/post/${post.id}`}>
          <PostContainer>
            {post.title}
          </PostContainer>
        </PostLink>
      );
    })}
    </OverflowSection>
  </Container>
};

export default CalendarItem;
