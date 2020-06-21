import React, { useEffect, useState } from 'react';
import dayjs from 'dayjs';
import weekOfYear from 'dayjs/plugin/weekOfYear';
import styled from 'styled-components';

import * as api from './api';
import { Post } from '../../models';
import { Section } from "../../components";
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
  border-top: 1px solid black;
  border-bottom: 1px solid black;

   &:nth-child(n+2) {
    border-top: 0;
  };
`;

const Calendar: React.FC = () => {
  dayjs.extend(weekOfYear);

  const [postMap, setPostMap] = useState<DateToPostsMap>({});
  const [calendar, setCalendar] = useState<Week[]>([]);

  const calculateCalendar = () => {
    const weeks: Week[] = [];

    const startWeek = dayjs().startOf('month').week();
    const endWeek = dayjs().endOf('month').week();

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

  return <Section>
    {calendar.map((week) => {
      return <WeekLine row>
        {week.days.map((day) => {
          const formattedDate = day.format('YYYY-MM-DD');
          const posts = postMap[formattedDate];
          return <CalendarItem key={formattedDate} day={day} posts={posts} />;
        })}
      </WeekLine>;
    })}
  </Section>
};

export default Calendar;
