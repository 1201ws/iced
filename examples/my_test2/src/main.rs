use std::{borrow::Cow, fs::OpenOptions};

use iced::{
    alignment, button, scrollable, slider, text_input, Button, Checkbox, Color,
    Column, Container, Element, Image, Length, Radio, Row, Sandbox, Scrollable,
    Settings, Slider, Space, Text, TextInput, Toggler, Alignment, pick_list, PickList, Font,
};

use iced_web::{dodrio::builder::style};
use std::fs::File;
mod file;


pub fn main() -> iced::Result {
    env_logger::init();

    Test::run(Settings::default())
}
// #[derive(Default)]
struct Test {
    variants:Vec<Variant>,
    commit_button:button::State,
    scrollable: scrollable::State,
    latest_offset: f32,
    
    step:Step,
    confirmcommit_button:button::State,
    cancelcommit_button:button::State,
    // error:String,
    tip:String,
}

#[derive(Debug, Clone)]
enum Message{
    InputChanged(usize,String),
    Selected(usize,String),

    Scrolled(f32),
    CommitPressed,
    ConfirmCommitPressed,
    ConcelCommitPressed,

}


impl Sandbox for Test{
    type Message = Message;

    fn new() -> Self {
        Test {
            variants: Variant::all(),
            commit_button:button::State::default(),
            scrollable: scrollable::State::default(),
            latest_offset:0.0,
            step:Step::InformationCollection,
            confirmcommit_button:button::State::default(),
            cancelcommit_button:button::State::default(),
            //error:String::default(),
            tip:String::default(),
        }
    }

    fn title(&self) -> String {
        String::from("系统测试")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ConfirmCommitPressed =>{
                let mut raw :Vec<String> = Vec::new();
                for variant in self.variants.iter(){
                    match variant.flag {
                        Flag::Choose => {
                            match &variant.choose.selected{
                                Some(selected) =>{
                                    raw.push(selected.clone());
                                },
                                _=>{
                                    self.tip = String::from(format!("{:?}未选择",variant.choose.placeholder));
                                    return
                                }
                            }
                        },
                        Flag::Input => {
                            if variant.input.value == String::default(){
                                self.tip = String::from(format!("{:?}未填写",variant.input.placeholder));
                                return 
                            }else{
                                raw.push(variant.input.value.clone());
                            }
                        },
                        _ =>{}

                    }
                }
                match file::storage(raw){
                    Err(err) =>{
                        self.tip = err.to_string();
                    },
                    _ =>{}
                }
                self.tip = String::from("提交成功");
            },
            Message::CommitPressed=>{
                self.step = Step::ConfirmCommit;
            },
            Message::ConcelCommitPressed =>{
                self.step = Step::InformationCollection;
                self.tip = String::default();
            }
            Message::InputChanged(i,value) => {
                if let Some(variant) = self.variants.get_mut(i) {
                    variant.input.value = value;
                }
            },
            Message::Selected(i,value) => {
                if let Some(variant) = self.variants.get_mut(i) {
                    variant.choose.selected = Some(value);
                }
            },
            Message::Scrolled(offset) =>{
                self.latest_offset = offset;
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = if self.step == Step::InformationCollection{
            let Test {
                variants,..
            } = self;
            let test_column = Column::with_children(
                self.variants
                    .iter_mut()
                    .enumerate()
                    .map(|(i,variant)|{
                        if variant.flag==Flag::Input{
                            let input = TextInput::new(
                                &mut variant.input.input,
                                variant.input.placeholder,
                                & variant.input.value,
                                move |value|{
                                    Message::InputChanged(i,value)
                                }
                            )
                            .size(20)
                            .padding(10)
                            .font(MYFONT); 
    
                            Row::new()
                            .push(input)
                            .into()    
                            
                        }else if variant.flag==Flag::Choose{
                            let mut new_selected_options = Vec::new();
                            for value in &variant.choose.selected_options{
                                new_selected_options.push(String::from(*value)); 
                            }
                            let pick = PickList::new(
                                &mut variant.choose.pick_list,
                                Cow::from(new_selected_options),
                                variant.choose.selected.clone(),
                                move |value|{
                                    Message::Selected(i,value)
                                }
                            )
                            .width(Length::Fill)
                            .placeholder(variant.choose.placeholder)
                            .font(MYFONT);
    
                            Row::new()
                            .push(pick)
                            .into()
                        }else{
                            Row::new()
                            .push(Text::new(variant.text.placeholder)
                                .font(MYFONT)
                                .size(variant.text.size)
                                .color(variant.text.color))
                            .into()
                        }
                        
                    })
                    .collect(),
            )
            .spacing(20);
            
    
            let mut controls = Column::new();
            controls = controls
               
                .push(Button::new(&mut self.commit_button, Text::new("提交").font(MYFONT).horizontal_alignment(alignment::Horizontal::Center).vertical_alignment(alignment::Vertical::Center))
                    .on_press(Message::CommitPressed))
                ;
            let scrollable = 
                Scrollable::new(&mut self.scrollable)
                    .align_items(Alignment::Center)
                    .padding(10)
                    .spacing(10)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_scroll(Message::Scrolled)
                    .scrollbar_width(10)
                    //.scrollbar_margin(10)
                    .scroller_width(10)
                    .push(test_column)
                    .push(Space::with_height(Length::Units(20)))
                    .push(controls);
    
            Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(20)
            .padding(20)
            .push(scrollable)
        }else if self.step == Step::ConfirmCommit{
            Column::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .push(Text::new("确定提交?").font(MYFONT).size(50))
            .push(
                Row::new()
                    .push(Button::new(
                            &mut self.confirmcommit_button,
                            Text::new("确定").font(MYFONT),
                        )
                        .padding([10, 15])
                        .on_press(Message::ConfirmCommitPressed),
                    )
                    .push(Space::with_width(Length::Units(40)))
                    .push(
                        Button::new(
                            &mut self.cancelcommit_button,
                            Text::new("取消").font(MYFONT),
                        )
                        .padding([10, 15])
                        .on_press(Message::ConcelCommitPressed),
                    )
                
            )
            .push(Text::new(self.tip.clone()).font(MYFONT))
        }else{
            Column::new()
            
        };
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
   

}
const MYFONT: Font = Font::External {
    name: "方正字体",
    bytes: include_bytes!("./FZJCGFKTK.TTF"), // 用 include_bytes 如果路径错误，会提示 
};

#[derive(PartialEq)]
enum Step{
    InformationCollection,
    ConfirmCommit,
    Error,
}
struct Variant {
    flag:Flag,
    choose:Choose,
    input:Input,
    text:SomeText,
    // placeholder:&'static str,
    // selected_options:Vec<&'static str>,

    // textconfig:TextConfig,

    // input:text_input::State,
    // pick_list:pick_list::State<String>,

    // value:String,
    // selected:Option<String>,
}

impl Variant {
    pub fn all() -> Vec<Self> {
        vec![       
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"实验前所需信息填写",
                    size:40,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"基础信息",
                    size:30,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Choose,
                choose:Choose{
                    pick_list:pick_list::State::default(),
                    selected:None,

                    placeholder:"实验人",
                    selected_options:vec!["张三","李四","王五"],
                },
                input:Input::default(),
                text:SomeText::default(),
                
                
            },
            Self {
                flag:Flag::Choose,
                choose:Choose{
                    pick_list:pick_list::State::default(),
                    selected:None,
                    placeholder:"审批人",
                    selected_options:vec!["张三","李四","王五"],
                },
                input:Input::default(),
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Choose,
                choose:Choose{
                    pick_list:pick_list::State::default(),
                    selected:None,
                    placeholder:"指导老师",
                    selected_options:vec!["张三","李四","王五"],
                },
                input:Input::default(),
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"归属项目/人员",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"实验材料",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Choose,
                choose:Choose{
                    pick_list:pick_list::State::default(),
                    selected:None,
                    placeholder:"实验类型",
                    selected_options:vec!["A","B","C"],
                },
                input:Input::default(),
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Choose,
                choose:Choose{
                    pick_list:pick_list::State::default(),
                    selected:None,
                    placeholder:"实验方案",
                    selected_options:vec!["1","2","3"],
                },
                input:Input::default(),
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"设备信息",
                    size:30,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Choose,
                choose:Choose{
                    pick_list:pick_list::State::default(),
                    selected:None,
                    placeholder:"气炮信息",
                    selected_options:vec!["一级炮","二级炮"],
                },
                input:Input::default(),
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Choose,
                choose:Choose{
                    pick_list:pick_list::State::default(),
                    selected:None,
                    placeholder:"探测设备",
                    selected_options:vec!["1","2","3"],
                },
                input:Input::default(),
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Choose,
                choose:Choose{
                    pick_list:pick_list::State::default(),
                    selected:None,
                    placeholder:"靶结构信息",
                    selected_options:vec!["1","2","3"],
                },
                input:Input::default(),
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"实验样品参数",
                    size:30,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"飞片材料",
                    size:25,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"飞片材料",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"飞片材料参数",
                    size:20,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"飞片材料直径",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"飞片材料厚度",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"飞片材料密度",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"飞片材料纵波声速",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"飞片材料横波声速",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"样品材料",
                    size:25,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"样品材料",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"样品材料参数",
                    size:20,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"样品材料直径",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"样品材料厚度",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"样品材料密度",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"样品材料纵波声速",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"样品材料横波声速",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"基板材料",
                    size:25,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"基板材料",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"基板材料参数",
                    size:20,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"基板材料直径",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"基板材料厚度",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"基板材料密度",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"基板材料纵波声速",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"基板材料横波声速",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"窗口材料",
                    size:25,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"窗口材料",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"窗口材料参数",
                    size:20,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"窗口材料直径",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"窗口材料厚度",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"窗口材料密度",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"窗口材料纵波声速",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"窗口材料横波声速",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"弹托参数",
                    size:25,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Choose,
                choose:Choose{
                    pick_list:pick_list::State::default(),
                    selected:None,
                    placeholder:"弹托类型",
                    selected_options:vec!["1","2","3"],
                },
                input:Input::default(),
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"弹托子弹重量",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Text,
                choose:Choose::default(),
                input:Input::default(),
                text:SomeText{
                    placeholder:"发射信息",
                    size:30,
                    color:Color::new(0.2, 0.8, 0.2, 1.0),
                },
            },
            Self {
                flag:Flag::Input,
                choose:Choose::default(),
                input:Input{
                    value:String::new(),
                    input:text_input::State::default(),
                    placeholder:"气压",
                },
                text:SomeText::default(),
            },
            Self {
                flag:Flag::Choose,
                choose:Choose{
                    pick_list:pick_list::State::default(),
                    selected:None,
                    placeholder:"气体",
                    selected_options:vec!["A","B","C"],
                },
                input:Input::default(),
                text:SomeText::default(),
            },
        ]
    }
}

#[derive(PartialEq)]
enum Flag{
    Choose,
    Input,
    Text,
}
#[derive(Default,Clone)]
struct  Choose{
    selected:Option<String>,
    placeholder:&'static str,
    selected_options:Vec<&'static str>,
    pick_list:pick_list::State<String>,
}
#[derive(Default,Clone)]
struct Input{
    value:String,
    placeholder:&'static str,
    input:text_input::State,
}
#[derive(Default,Clone)]
struct SomeText{
    placeholder:&'static str,
    size:u16,
    color:Color,
}
