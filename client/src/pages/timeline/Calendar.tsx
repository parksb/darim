import React, { useEffect, useState } from 'react';
import dayjs from 'dayjs';
import weekOfYear from 'dayjs/plugin/weekOfYear';
import styled from 'styled-components';
import { useParams, useNavigate } from 'react-router-dom';

import { getI18n } from '../../utils/i18n';
import * as api from '../../api/post';
import { SummarizedPost, Session } from '../../models';
import { Button, Container, Section } from '../../components';
import CalendarItem from './CalendarItem';

interface Props {
  session: Session;
}

interface Week {
  week: number;
  days: dayjs.Dayjs[];
}

interface DateToPostsMap {
  [date: string]: SummarizedPost[];
}

const WeekLine = styled(Section)`
  flex: 1;
  max-width: 100%;
  border-top: 1px solid #000000;
  border-bottom: 1px solid #000000;
  overflow: auto;

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

const MonthControlText = styled.h2`
  min-width: 130px;
  text-align: center;
`;

const WeekLineContainer = styled(Section)`
  flex: 1 1 0;
`;

const OverflowContainer = styled(Container)`
  overflow: auto;
`;

const Calendar: React.FC<Props> = ({ session }) => {
  dayjs.extend(weekOfYear);

  const getInitialCursorDate = () => {
    const { year, month } = useParams<{ year?: string, month?: string }>();
    const initialYear = year ? Number(year) : dayjs().year();
    const initialMonth = month ? Number(month) : (dayjs().month() + 1) - 1;
    return dayjs().year(initialYear).month(initialMonth).date(1);
  };

  const [postMap, setPostMap] = useState<DateToPostsMap>({});
  const [calendar, setCalendar] = useState<Week[]>([]);
  const [cursorDate, setCursorDate] = useState(getInitialCursorDate());

  const i18n = getI18n({
    sunday: {
      ko: '일',
      en: 'Sun',
    },
    monday: {
      ko: '월',
      en: 'Mon',
    },
    tuesday: {
      ko: '화',
      en: 'Tue',
    },
    wednesday: {
      ko: '수',
      en: 'Wed',
    },
    thursday: {
      ko: '목',
      en: 'Thu',
    },
    friday: {
      ko: '금',
      en: 'Fri',
    },
    saturday: {
      ko: '토',
      en: 'Sat',
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
    const postList = await api.fetchPosts(session.user.public_key || '', session.accessToken);
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

  const composeMonthControlURL = (date: dayjs.Dayjs) => `/calendar/${date.year()}/${date.month() + 1}`;

  const history = useNavigate();
  const PrevMonthControlButton = () => <MonthControlButton onClick={() => {
    const date = cursorDate.subtract(1, 'month');
    setCursorDate(date);
    history(composeMonthControlURL(date));
  }}>＜</MonthControlButton>;

  const NextMonthControlButton = () => <MonthControlButton onClick={() => {
    const date = cursorDate.add(1, 'month');
    setCursorDate(date);
    history(composeMonthControlURL(date));
  }}>＞</MonthControlButton>;

  useEffect(() => {
    load();
    calculateCalendar();
  }, []);

  useEffect(() => {
    calculateCalendar();
  }, [cursorDate]);

  return <OverflowContainer fullWidth fullHeight>
    <MonthControlContainer bottom={30} row>
      <PrevMonthControlButton />
      <MonthControlText>{cursorDate.format(cursorDate.year() === dayjs().year() ? 'MMMM' : 'YYYY MMMM')}</MonthControlText>
      <NextMonthControlButton />
    </MonthControlContainer>
    <WeekDayLine row>
      {weekDays.map((weekDay) => <WeekDay key={weekDay}>{weekDay}</WeekDay>)}
    </WeekDayLine>
    <WeekLineContainer fullHeight>
      {calendar.map((week) => <WeekLine key={week.week} row>
          {week.days.map((day) => {
            const formattedDate = day.format('YYYY-MM-DD');
            const posts = postMap[formattedDate];
            return <CalendarItem key={formattedDate} day={day} cursorDate={cursorDate} posts={posts} />;
          })}
        </WeekLine>)}
    </WeekLineContainer>
  </OverflowContainer>;
};

export default Calendar;
