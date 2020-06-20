import React from 'react';
import styled from 'styled-components';
import dayjs from 'dayjs';

import { Post } from '../../models';
import { Section } from "../../components";

interface Props {
  post?: Post;
  day: dayjs.Dayjs;
}

const Container = styled(({ is_current_month, ...other }) => <Section {...other} />)`
  flex: 7;
  border-left: 1px solid black;
  border-right: 1px solid black;
  padding: 10px;
  height: 130px;
  background-color: ${props => props.is_current_month ? '#ffffff' : '#f5f5f5'};

  &:nth-child(even) {
    border-right: 0;
    border-left: 0;
  };
`;

const Date = styled.time`
  font-size: 12px;
`;

const CalendarItem: React.FC<Props> = ({ post, day }) => {
  const displayed_date = day.date() === 1 ? day.format('MM / DD') : day.format('D');
  const is_today = day.format('YYYY-MM-DD') === dayjs().format('YYYY-MM-DD');

  return <Container is_current_month={day.month() === dayjs().month()} row>
    <Date dateTime={day.format('YYYY-MM-DD')}>
      {displayed_date}
      {is_today && '📌'}
    </Date>
  </Container>
};

export default CalendarItem;
