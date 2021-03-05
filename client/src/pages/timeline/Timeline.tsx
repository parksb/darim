import React, { useState } from 'react';
import { useParams } from 'react-router-dom';

import Calendar from './Calendar';
import List from './List';
import TimelineHeader from './TimelineHeader';
import { Session } from '../../models';
import { Container } from '../../components'

interface Props {
  session: Session | null;
}

export enum ViewMode {
  CALENDAR,
  LIST,
}

const Timeline: React.FC<Props> = ({ session }) => {
  const getInitialViewMode = () => {
    const { viewMode } = useParams();
    switch (viewMode) {
      case 'calendar':
        return ViewMode.CALENDAR;
      case 'list':
        return ViewMode.LIST;
      default:
        return ViewMode.CALENDAR;
    }
  }

  const [viewMode, setViewMode] = useState(getInitialViewMode());

  const viewTimelineBody = (mode: ViewMode) => {
    switch (mode) {
      case ViewMode.CALENDAR:
        return <Calendar session={session} />;
      case ViewMode.LIST:
        return <List session={session} />;
    }
  };

  return <Container fullWidth fullHeight>
    <TimelineHeader viewModeState={[viewMode, setViewMode]} />
    {viewTimelineBody(viewMode)}
  </Container>
};

export default Timeline;
