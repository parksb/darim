import React, { useState } from 'react';
import styled from 'styled-components';

import { Section } from '../../components';
import Calendar from "./Calendar";
import List from "./List";
import TimelineHeader from "./TimelineHeader";

export enum ViewMode {
  CALENDAR,
  LIST,
}

const Container = styled(Section)`
  margin-bottom: 30px;
`;

const Timeline: React.FC = () => {
  const [viewMode, setViewMode] = useState(ViewMode.CALENDAR);

  const viewTimelineBody = (mode: ViewMode) => {
    switch (mode) {
      case ViewMode.CALENDAR:
        return <Calendar />;
      case ViewMode.LIST:
        return <List />;
    }
  };

  return <Container>
    <TimelineHeader viewModeState={[viewMode, setViewMode]} />
    {viewTimelineBody(viewMode)}
  </Container>
};

export default Timeline;
