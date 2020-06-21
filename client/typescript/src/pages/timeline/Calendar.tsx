import React, { useEffect, useState } from 'react';
import dayjs from 'dayjs';
import weekOfYear from 'dayjs/plugin/weekOfYear';
import styled from 'styled-components';

import * as api from './api';
import { Post } from '../../models';
import { Button, Section } from "../../components";
import CalendarItem from "./CalendarItem";

interface Week {
  week: number;
  days: dayjs.Dayjs[];
}

interface DateToPostsMap {
  [date: string]: Post[];
}

const WeekLine = styled(Section)`
  flex: 1;
  max-width: 100%;
  border-top: 1px solid #000000;
  border-bottom: 1px solid #000000;

   &:nth-child(n+2) {
    border-top: 0;
  };
`;

const WeekDayLine = styled(Section)`
  padding-bottom: 10px;
`;

const WeekDay = styled(Section)`
  flex: 7;
  text-align: center;
  font-size: 12px;
`;

const MonthControlContainer = styled(Section)`
  align-items: center;
  align-self: center;
  margin-bottom: 30px;
`;

const MonthControlButton = styled(Button)`
  border: none;
  height: 30px;
  width: 30px;
  padding: 0;
  margin: 0 15px 0 15px;
  font-weight: 700;
  border-radius: 50%;

  &:hover {
    background-color: #ffce05;
    color: #000000;
  }
`;

const MonthText = styled.h2`
  margin: 0;
`;

const Calendar: React.FC = () => {
  dayjs.extend(weekOfYear);

  const weekDays = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  const [postMap, setPostMap] = useState<DateToPostsMap>({});
  const [calendar, setCalendar] = useState<Week[]>([]);
  const [cursorDate, setCursorDate] = useState(dayjs());

  const calculateCalendar = () => {
    const weeks: Week[] = [];

    const startWeek = cursorDate.startOf('month').week();
    const endWeek = cursorDate.endOf('month').week();

    for (let week = startWeek; week < endWeek; week += 1) {
      weeks.push({
        week,
        days: Array(7).fill(0).map((n, i) => {
          return dayjs().week(week).startOf('week').clone().add(n + i, 'day')
        }),
      });
    }

    setCalendar(weeks);
  };

  const load = async () => {
    const postList = await api.getPosts();
    const dateToPostsMap: DateToPostsMap = {};

    postList.forEach((post) => {
      const postDate = dayjs(post.date).format('YYYY-MM-DD');
      if (!dateToPostsMap[postDate]) {
        dateToPostsMap[postDate] = [];
      }

      dateToPostsMap[postDate].push(post);
    });

    setPostMap(dateToPostsMap);
  };

  useEffect(() => {
    load();
    calculateCalendar();
  }, []);

  useEffect(() => {
    calculateCalendar();
  }, [cursorDate]);

  return <Section>
    <MonthControlContainer row>
      <MonthControlButton onClick={() => setCursorDate(cursorDate.subtract(1, 'month'))}>
        ＜
      </MonthControlButton>
      <MonthText>{cursorDate.format(cursorDate.year() === dayjs().year() ? 'MMMM' : 'YYYY MMMM')}</MonthText>
      <MonthControlButton onClick={() => setCursorDate(cursorDate.add(1, 'month'))}>
        ＞
      </MonthControlButton>
    </MonthControlContainer>
    <WeekDayLine row>
      {weekDays.map((weekDay) => {
        return <WeekDay>{weekDay}</WeekDay>
      })}
    </WeekDayLine>
    <Section>
      {calendar.map((week) => {
        return <WeekLine row>
          {week.days.map((day) => {
            const formattedDate = day.format('YYYY-MM-DD');
            const posts = postMap[formattedDate];
            return <CalendarItem key={formattedDate} day={day} cursorDate={cursorDate} posts={posts} />;
          })}
        </WeekLine>;
      })}
    </Section>
  </Section>
};

export default Calendar;
