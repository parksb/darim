import React, { useEffect, useState } from 'react';
import dayjs from 'dayjs';
import weekOfYear from 'dayjs/plugin/weekOfYear';
import styled from 'styled-components';

import * as api from '../../api/post';
import { Post, Session } from '../../models';
import { Button, Container, Section } from "../../components";
import CalendarItem from "./CalendarItem";
import I18n from "../../utils/i18n";

interface Props {
  session: Session | null;
}

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

const Calendar: React.FC<Props> = ({ session }) => {
  dayjs.extend(weekOfYear);

  const [postMap, setPostMap] = useState<DateToPostsMap>({});
  const [calendar, setCalendar] = useState<Week[]>([]);
  const [cursorDate, setCursorDate] = useState(dayjs().date(1));

  const i18n = new I18n({
    sunday: {
      'ko-KR': '일',
      'en-US': 'Sun',
    },
    monday: {
      'ko-KR': '월',
      'en-US': 'Mon',
    },
    tuesday: {
      'ko-KR': '화',
      'en-US': 'Tue',
    },
    wednesday: {
      'ko-KR': '수',
      'en-US': 'Wed',
    },
    thursday: {
      'ko-KR': '목',
      'en-US': 'Thu',
    },
    friday: {
      'ko-KR': '금',
      'en-US': 'Fri',
    },
    saturday: {
      'ko-KR': '토',
      'en-US': 'Sat',
    },
  });

  const weekDays = [
    i18n.text('sunday'),
    i18n.text('monday'),
    i18n.text('tuesday'),
    i18n.text('wednesday'),
    i18n.text('thursday'),
    i18n.text('friday'),
    i18n.text('saturday'),
  ];

  const calculateCalendar = () => {
    const weeks: Week[] = [];

    const startWeek = cursorDate.startOf('month').week();
    let endWeek = cursorDate.endOf('month').week();
    if (cursorDate.month() === 11) {
      endWeek = cursorDate.endOf('month').subtract(7, 'day').week() + endWeek;
    }

    for (let week = startWeek; week <= endWeek; week += 1) {
      weeks.push({
        week,
        days: Array(7).fill(0).map((n, i) => cursorDate.week(week).startOf('week').add(n + i, 'day')),
      });
    }

    setCalendar(weeks);
  };

  const load = async () => {
    const postList = await api.fetchPosts(session?.user_public_key || '');
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

  return <Container fullWidth>
    <MonthControlContainer row>
      <MonthControlButton onClick={() => setCursorDate(cursorDate.subtract(1, 'month'))}>
        ＜
      </MonthControlButton>
      <h2>{cursorDate.format(cursorDate.year() === dayjs().year() ? 'MMMM' : 'YYYY MMMM')}</h2>
      <MonthControlButton onClick={() => setCursorDate(cursorDate.add(1, 'month'))}>
        ＞
      </MonthControlButton>
    </MonthControlContainer>
    <WeekDayLine row>
      {weekDays.map((weekDay) => {
        return <WeekDay key={weekDay}>{weekDay}</WeekDay>
      })}
    </WeekDayLine>
    <Section>
      {calendar.map((week) => {
        return <WeekLine key={week.week} row>
          {week.days.map((day) => {
            const formattedDate = day.format('YYYY-MM-DD');
            const posts = postMap[formattedDate];
            return <CalendarItem key={formattedDate} day={day} cursorDate={cursorDate} posts={posts} />;
          })}
        </WeekLine>;
      })}
    </Section>
  </Container>
};

export default Calendar;
