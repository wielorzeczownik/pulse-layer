use iced::widget::{button, text_input};
use iced::{Background, Border, Color, Shadow, Theme, Vector};
use iced_aw::style::Status as CpStatus;
use iced_aw::style::color_picker::Style as CpStyle;

// Palette
const SURFACE: Color = Color {
  r: 0.14,
  g: 0.14,
  b: 0.18,
  a: 1.0,
};
const BORDER: Color = Color {
  r: 1.00,
  g: 1.00,
  b: 1.00,
  a: 0.09,
};
const ACCENT: Color = Color {
  r: 0.40,
  g: 0.50,
  b: 0.90,
  a: 1.0,
};
const TEXT_DIM: Color = Color {
  r: 0.55,
  g: 0.55,
  b: 0.62,
  a: 1.0,
};

pub fn card(_theme: &Theme) -> iced::widget::container::Style {
  iced::widget::container::Style {
    background: Some(Background::Color(Color {
      r: SURFACE.r + 0.03,
      g: SURFACE.g + 0.03,
      b: SURFACE.b + 0.04,
      a: 1.0,
    })),
    border: Border {
      color: BORDER,
      width: 1.0,
      radius: 12.0.into(),
    },
    shadow: Shadow {
      color: Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.20,
      },
      offset: Vector::new(0.0, 2.0),
      blur_radius: 10.0,
    },
    ..Default::default()
  }
}

pub fn btn_primary(_theme: &Theme, status: button::Status) -> button::Style {
  let bg = match status {
    button::Status::Hovered => Color {
      r: 0.48,
      g: 0.58,
      b: 0.98,
      a: 1.0,
    },
    button::Status::Pressed => Color {
      r: 0.32,
      g: 0.40,
      b: 0.80,
      a: 1.0,
    },
    button::Status::Disabled => Color {
      r: ACCENT.r,
      g: ACCENT.g,
      b: ACCENT.b,
      a: 0.38,
    },
    button::Status::Active => ACCENT,
  };
  button::Style {
    background: Some(Background::Color(bg)),
    text_color: Color::WHITE,
    border: Border {
      radius: 8.0.into(),
      ..Default::default()
    },
    shadow: Shadow {
      color: Color {
        r: ACCENT.r,
        g: ACCENT.g,
        b: ACCENT.b,
        a: 0.28,
      },
      offset: Vector::new(0.0, 2.0),
      blur_radius: 8.0,
    },
    ..Default::default()
  }
}

pub fn btn_ghost(_theme: &Theme, status: button::Status) -> button::Style {
  let alpha = match status {
    button::Status::Hovered => 0.10,
    button::Status::Pressed => 0.18,
    _ => 0.04,
  };
  button::Style {
    background: Some(Background::Color(Color {
      r: 1.0,
      g: 1.0,
      b: 1.0,
      a: alpha,
    })),
    text_color: Color {
      r: 0.80,
      g: 0.80,
      b: 0.86,
      a: 1.0,
    },
    border: Border {
      color: BORDER,
      width: 1.0,
      radius: 8.0.into(),
    },
    ..Default::default()
  }
}

pub fn btn_icon(_theme: &Theme, status: button::Status) -> button::Style {
  let alpha = match status {
    button::Status::Hovered => 0.09,
    button::Status::Pressed => 0.16,
    _ => 0.0,
  };
  button::Style {
    background: Some(Background::Color(Color {
      r: 1.0,
      g: 1.0,
      b: 1.0,
      a: alpha,
    })),
    text_color: TEXT_DIM,
    border: Border {
      radius: 7.0.into(),
      ..Default::default()
    },
    ..Default::default()
  }
}

pub fn color_picker(_theme: &Theme, _status: CpStatus) -> CpStyle {
  CpStyle {
    background: Background::Color(Color {
      r: 0.07,
      g: 0.07,
      b: 0.11,
      a: 1.0,
    }),
    border_radius: 12.0,
    border_width: 1.0,
    border_color: Color {
      r: 0.22,
      g: 0.22,
      b: 0.32,
      a: 1.0,
    },
    bar_border_radius: 6.0,
    bar_border_width: 1.0,
    bar_border_color: Color {
      r: 0.22,
      g: 0.22,
      b: 0.32,
      a: 1.0,
    },
  }
}

pub fn text_input_field(theme: &Theme, status: text_input::Status) -> text_input::Style {
  let _ = theme;
  let border_color = match status {
    text_input::Status::Focused { .. } => ACCENT,
    text_input::Status::Hovered => Color {
      r: 1.0,
      g: 1.0,
      b: 1.0,
      a: 0.18,
    },
    _ => BORDER,
  };
  text_input::Style {
    background: Background::Color(Color {
      r: 0.10,
      g: 0.10,
      b: 0.14,
      a: 1.0,
    }),
    border: Border {
      color: border_color,
      width: 1.0,
      radius: 7.0.into(),
    },
    icon: TEXT_DIM,
    placeholder: Color {
      r: 0.38,
      g: 0.38,
      b: 0.44,
      a: 1.0,
    },
    value: Color {
      r: 0.88,
      g: 0.88,
      b: 0.94,
      a: 1.0,
    },
    selection: Color {
      r: ACCENT.r,
      g: ACCENT.g,
      b: ACCENT.b,
      a: 0.35,
    },
  }
}
