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
  days: dayjs.Dayjs[]
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

  const [posts, setPosts] = useState<Post[]>([]);
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
    const post_list = await api.getPosts();
    setPosts(post_list);
  };

  useEffect(() => {
    load();
    calculateCalendar();
  }, []);

  return <Section>
    {calendar.map((week) => {
      return <WeekLine row>
        {week.days.map((day) => {
          return <CalendarItem day={day} />;
        })}
      </WeekLine>;
    })}
  </Section>
};

export default Calendar;
