use std::borrow::Cow;

use iced::{
    alignment, button, scrollable, slider, text_input, Button, Checkbox, Color,
    Column, Container, Element, Image, Length, Radio, Row, Sandbox, Scrollable,
    Settings, Slider, Space, Text, TextInput, Toggler, Alignment, pick_list, PickList, Application, executor, Command, Subscription, Font, container::StyleSheet,
};
use iced_native::Widget;
use iced_web::dodrio::builder::style;
use chrono::prelude::*;
mod modbus;



const MYFONT: Font = Font::External {
    name: "方正字体",
    bytes: include_bytes!("./FZJCGFKTK.TTF"), // 用 include_bytes 如果路径错误，会提示 
};


pub fn main() -> iced::Result {
    env_logger::init();

    Test::run(Settings::default())
}


struct Test{
    now:DateTime<Local>,
    theme:style::Theme,
    switch:Vec<Switch>,
    vacuum:button::State,
    hpInflation:button::State,
    hpBleed:button::State,
    emission:button::State,
    end:button::State,
    close:button::State,
    login:button::State,
    logout:button::State,
    run:button::State,
    stop:button::State,
}
#[derive(Debug, Clone)]
enum Message{
    Tick(DateTime<Local>),
}

impl Application for Test{
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        
        (
            
            Test{
                now: Local::now(),
                theme: style::Theme::default(),
                switch:Switch::all(),
                vacuum:button::State::new(),
                hpInflation:button::State::new(),
                hpBleed:button::State::new(),
                emission:button::State::new(),
                end:button::State::new(),
                close:button::State::new(),
                login:button::State::new(),
                logout:button::State::new(),
                run:button::State::new(),
                stop:button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("系统测试")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                let now = local_time;

                if now != self.now {
                    self.now = now;
                }
            },
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_secs(1)).map(|_| {
            Message::Tick(
                Local::now(),
            )
        })
    }
    
    fn view(&mut self) -> Element<Message> {
        let row_1 = Row::new()
            .push(Column::new().width(Length::FillPortion(2)).push(Text::new("实验室一级轻气炮监控系统").font(MYFONT).size(30)).align_items(Alignment::End))
            .push(Column::new().width(Length::FillPortion(1)).push(Text::new(self.now.format("%Y-%m-%d %H:%M:%S").to_string()).size(22)).align_items(Alignment::Center))
            ;
        let row_1 = Container::new(row_1)
            .style(self.theme)
            .width(Length::Fill)
            .height(Length::FillPortion(1));
        let row_2 = Row::new()
            .push(Column::new().width(Length::FillPortion(8))
                .push(Container:: new(Row::new()
                        .push(Container::new(Image::new(format!(
                            "{}/src/gas.png",
                            env!("CARGO_MANIFEST_DIR")
                            ))
                            .width(Length::Fill)
                            .height(Length::Fill))
                            .width(Length::Fill)
                            .height(Length::Fill) 
                            .center_x()))
                    .width(Length::Fill)
                    .height(Length::FillPortion(5))
                    .style(self.theme)
                    )
                .push(Space::new(Length::Fill,Length::Units(5)))
                .push(Container::new(Column::new()            
                        .push(Row::new().width(Length::Fill).height(Length::FillPortion(3))
                            .push(Column::new()
                                .push(Text::new(format!("{}:{} MPa","输入气压","10")).font(MYFONT).size(20))
                                .width(Length::FillPortion(1)))
                            .push(Column::new()
                                .push(Text::new("气室气压").font(MYFONT).size(20))
                                .width(Length::FillPortion(1)))
                            .push(Column::new()
                                .push(Text::new("靶室真空度").font(MYFONT).size(20))
                                .width(Length::FillPortion(1))))
                        .push(Row::new().width(Length::Fill).height(Length::FillPortion(4)).push(Text::new("开关状态").font(MYFONT).size(20))
                            .push(Row::with_children(
                                self.switch
                                    .iter_mut()
                                    .enumerate()
                                    .map(|(i,switch)|{
                                        Column::new().push(Text::new(switch.name).size(16)).push(Space::new(Length::Fill,Length::Units(20))).push(circle::Circle::new(9.0)).width(Length::FillPortion(1)).align_items(Alignment::Center).into()
                                    })
                                    .collect(),
                            ))))
                    .width(Length::Fill)
                    .height(Length::FillPortion(3))
                    .style(self.theme)
                )
            )
            .push(Space::new(Length::Units(10),Length::Fill))
            .push(Container::new(Column::new()
                    .push(Row::new().width(Length::Fill).height(Length::FillPortion(1))
                        .push(Column::new().height(Length::Fill).width(Length::FillPortion(1)).push(Button::new(&mut self.login,Text::new("登录").size(25).font(MYFONT))))
                        .push(Column::new().height(Length::Fill).width(Length::FillPortion(1)).push(Button::new(&mut self.logout,Text::new("注销").size(25).font(MYFONT)))))
                    .push(Row::new().width(Length::Fill).height(Length::FillPortion(1))
                        .push(Button::new(&mut self.run,Text::new("开始运行").size(35).font(MYFONT))))
                    .push(Row::new().width(Length::Fill).height(Length::FillPortion(1))
                        .push(Button::new(&mut self.stop,Text::new("急停").size(35).font(MYFONT))))
                    .push(Space::new(Length::Fill,Length::FillPortion(3))))
                .width(Length::FillPortion(2))
                .height(Length::Fill)
                .style(self.theme))
            .width(Length::Fill)
            .height(Length::FillPortion(5));
        

        let row_3 = Row::new()
            .push(Column::new().push(Space::new(Length::Fill,Length::Units(30))).push(Button::new(&mut self.vacuum,Text::new("抽真空").size(25).font(MYFONT))).width(Length::FillPortion(1)).height(Length::Fill).align_items(Alignment::Center))
            .push(Column::new().push(Space::new(Length::Fill,Length::Units(30))).push(Button::new(&mut self.hpInflation,Text::new("高压充气").size(25).font(MYFONT))).width(Length::FillPortion(1)).height(Length::Fill).align_items(Alignment::Center))
            .push(Column::new().push(Space::new(Length::Fill,Length::Units(30))).push(Button::new(&mut self.hpBleed,Text::new("高压放气").size(25).font(MYFONT))).width(Length::FillPortion(1)).height(Length::Fill).align_items(Alignment::Center))
            .push(Column::new().push(Space::new(Length::Fill,Length::Units(30))).push(Button::new(&mut self.emission,Text::new("发射阀").size(25).font(MYFONT))).width(Length::FillPortion(1)).height(Length::Fill).align_items(Alignment::Center))
            .push(Column::new().push(Space::new(Length::Fill,Length::Units(30))).push(Button::new(&mut self.end,Text::new("实验结束").size(25).font(MYFONT))).width(Length::FillPortion(1)).height(Length::Fill).align_items(Alignment::Center))
            .push(Column::new().push(Space::new(Length::Fill,Length::Units(30))).push(Button::new(&mut self.close,Text::new("关闭").size(25).font(MYFONT))).width(Length::FillPortion(1)).height(Length::Fill).align_items(Alignment::Center))
            .push(Column::new().push(Text::new("测试").font(MYFONT)).width(Length::FillPortion(3)).height(Length::Fill));

        let row_3 = Container::new(row_3)
            // .style(self.theme)
            .width(Length::Fill)
            .height(Length::FillPortion(2));

        let  content = Column::new()
            .push(row_1)
            .push(Space::new(Length::Fill,Length::Units(10)))
            .push(row_2)
            .push(Space::new(Length::Fill,Length::Units(5)))
            .push(row_3);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(30)
            .into()
    }
}
struct Switch{
    name:&'static str,
}
impl Switch{
    pub fn all()->Vec<Self>{
        vec![
            Switch{
                name:"VP1",
            },
            Switch{
                name:"VP2",
            },
            Switch{
                name:"VP3",
            },
            Switch{
                name:"SV1",
            },
            Switch{
                name:"SV2",
            },
            Switch{
                name:"SV3",
            },
            Switch{
                name:"SV4",
            },
            Switch{
                name:"SV5",
            },
            Switch{
                name:"SV6",
            },
            Switch{
                name:"SV7",
            },
            Switch{
                name:"SV8",
            },
            Switch{
                name:"SV9",
            },
        ]
    }
}


mod style {
    use iced::{
        button, checkbox, container, progress_bar, radio, rule, scrollable,
        slider, text_input, toggler,Color,
    };

    
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Theme {
        Light,
        Theme_1,
    }

    impl Default for Theme {
        fn default() -> Theme {
            Theme::Theme_1
        }
    }

    impl<'a> From<Theme> for Box<dyn container::StyleSheet + 'a> {
        fn from(theme: Theme) -> Self {
            match theme {
                Theme::Light => Default::default(),
                Theme::Theme_1 => Container.into(),
            }
        }
    }

    
    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color::from_rgb8(204, 255, 204).into(),
                text_color: Color::BLACK.into(),
                ..container::Style::default()
            }
        }
    }

}

mod circle {
    // For now, to implement a custom native widget you will need to add
    // `iced_native` and `iced_wgpu` to your dependencies.
    //
    // Then, you simply need to define your widget type and implement the
    // `iced_native::Widget` trait with the `iced_wgpu::Renderer`.
    //
    // Of course, you can choose to make the implementation renderer-agnostic,
    // if you wish to, by creating your own `Renderer` trait, which could be
    // implemented by `iced_wgpu` and other renderers.
    use iced_native::layout::{self, Layout};
    use iced_native::renderer;
    use iced_native::{
        Color, Element, Hasher, Length, Point, Rectangle, Size, Widget,
    };

    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Self {
            Self { radius }
        }
    }

    impl<Message, Renderer> Widget<Message, Renderer> for Circle
    where
        Renderer: renderer::Renderer,
    {
        fn width(&self) -> Length {
            Length::Shrink
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(
            &self,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
        }

        fn hash_layout(&self, state: &mut Hasher) {
            use std::hash::Hash;

            self.radius.to_bits().hash(state);
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor_position: Point,
            _viewport: &Rectangle,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: self.radius,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                Color::BLACK,
            );
        }
    }

    impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for Circle
    where
        Renderer: renderer::Renderer,
    {
        fn into(self) -> Element<'a, Message, Renderer> {
            Element::new(self)
        }
    }
}
