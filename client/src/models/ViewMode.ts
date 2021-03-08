enum ViewMode {
  CALENDAR,
  LIST,
}

class ViewModeMethods {
  static convertNumberToString(mode: number | null): ViewMode {
    switch (mode) {
      case 0:
        return ViewMode.CALENDAR;
      case 1:
        return ViewMode.LIST;
      default:
        return ViewMode.CALENDAR;
    }
  }

  static convertViewModeToString(mode: ViewMode | null): string {
    switch (mode) {
      case ViewMode.CALENDAR:
        return 'calendar';
      case ViewMode.LIST:
        return 'list';
      default:
        return this.convertViewModeToString(mode);
    }
  }

  static convertStringToViewMode(mode: string | null): ViewMode {
    switch (mode) {
      case 'calendar':
        return ViewMode.CALENDAR;
      case 'list':
        return ViewMode.LIST;
      default:
        return ViewMode.CALENDAR;
    }
  }
}

export { ViewMode, ViewModeMethods };
