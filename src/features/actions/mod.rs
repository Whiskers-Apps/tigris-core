use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultAction {
    pub action_type: ActionType,
    pub require_confirmation: bool,
    pub copy_text_action: Option<CopyTextAction>,
    pub copy_image_action: Option<CopyImageAction>,
    pub open_link_action: Option<OpenLinkAction>,
    pub open_app_action: Option<OpenAppAction>,
    pub open_form_action: Option<OpenFormAction>,
    pub run_extension_action: Option<RunExtensionAction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionType {
    CopyText,
    CopyImage,
    OpenLink,
    OpenApp,
    OpenForm,
    RunExtension,
    OpenSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyTextAction {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyImageAction {
    pub image_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenLinkAction {
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAppAction {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenFormAction {
    pub extension_id: String,
    pub form_id: String,
    pub args: Vec<String>,
    pub title: String,
    pub fields: Vec<Field>,
    pub button_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub field_type: FieldType,
    pub id: String,
    pub args: Vec<String>,
    pub title: String,
    pub description: String,
    pub text_field: Option<TextField>,
    pub text_area_field: Option<TextAreaField>,
    pub select_field: Option<SelectField>,
    pub switch_field: Option<SwitchField>,
    pub slider_field: Option<SliderField>,
    pub file_system_field: Option<FileSystemField>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FieldType {
    Text,
    TextArea,
    Select,
    Switch,
    Slider,
    FileSystem,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldValidation {
    pub only_numbers: bool,
    pub not_empty: bool,
    pub max_characters: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextField {
    pub value: String,
    pub placeholder: Option<String>,
    pub validation: Option<FieldValidation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextAreaField {
    pub value: String,
    pub placeholder: Option<String>,
    pub validation: Option<FieldValidation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectField {
    pub value: String,
    pub values: Vec<SelectFieldValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectFieldValue {
    pub id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwitchField {
    pub value: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SliderField {
    pub value: usize,
    pub min_value: usize,
    pub max_value: usize,
    pub step: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileSystemField {
    pub value: String,
    pub pick_directory: bool,
    pub filters: Option<Vec<String>>,
    pub validation: Option<FieldValidation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunExtensionAction {
    pub extension_id: String,
    pub extension_action: String,
    pub args: Vec<String>,
}

// =================================================================
// ==== Methods
// =================================================================

impl ResultAction {
    pub fn new_copy_text_action(action: &CopyTextAction) -> Self {
        Self {
            action_type: ActionType::CopyText,
            require_confirmation: false,
            copy_text_action: Some(action.to_owned()),
            copy_image_action: None,
            open_link_action: None,
            open_app_action: None,
            open_form_action: None,
            run_extension_action: None,
        }
    }

    pub fn new_copy_image_action(action: &CopyImageAction) -> Self {
        Self {
            action_type: ActionType::CopyImage,
            require_confirmation: false,
            copy_text_action: None,
            copy_image_action: Some(action.to_owned()),
            open_link_action: None,
            open_app_action: None,
            open_form_action: None,
            run_extension_action: None,
        }
    }

    pub fn new_open_link_action(action: &OpenLinkAction) -> Self {
        Self {
            action_type: ActionType::OpenLink,
            require_confirmation: false,
            copy_text_action: None,
            copy_image_action: None,
            open_link_action: Some(action.to_owned()),
            open_app_action: None,
            open_form_action: None,
            run_extension_action: None,
        }
    }

    pub fn new_open_app_action(action: &OpenAppAction) -> Self {
        Self {
            action_type: ActionType::OpenApp,
            require_confirmation: false,
            copy_text_action: None,
            copy_image_action: None,
            open_link_action: None,
            open_app_action: Some(action.to_owned()),
            open_form_action: None,
            run_extension_action: None,
        }
    }

    pub fn new_open_form_action(action: &OpenFormAction) -> Self {
        Self {
            action_type: ActionType::OpenForm,
            require_confirmation: false,
            copy_text_action: None,
            copy_image_action: None,
            open_link_action: None,
            open_app_action: None,
            open_form_action: Some(action.to_owned()),
            run_extension_action: None,
        }
    }

    pub fn new_run_extension_action(action: &RunExtensionAction) -> Self {
        Self {
            action_type: ActionType::RunExtension,
            require_confirmation: false,
            copy_text_action: None,
            copy_image_action: None,
            open_link_action: None,
            open_app_action: None,
            open_form_action: None,
            run_extension_action: Some(action.to_owned()),
        }
    }

    pub fn new_open_settings_action() -> Self {
        Self {
            action_type: ActionType::OpenSettings,
            require_confirmation: false,
            copy_text_action: None,
            copy_image_action: None,
            open_link_action: None,
            open_app_action: None,
            open_form_action: None,
            run_extension_action: None,
        }
    }

    pub fn set_require_confirmation(mut self, require_confirmation: bool) -> Self {
        self.require_confirmation = require_confirmation;
        self
    }
}

impl CopyTextAction {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }
}

impl CopyImageAction {
    pub fn new(image_path: &Path) -> Self {
        let image_path = image_path.to_owned();

        Self { image_path }
    }
}

impl OpenLinkAction {
    pub fn new(link: &str) -> Self {
        Self {
            link: link.to_owned(),
        }
    }
}

impl OpenAppAction {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
        }
    }
}

impl OpenFormAction {
    pub fn new(extension_id: &str, form_id: &str, title: &str, button_text: &str) -> Self {
        Self {
            extension_id: extension_id.to_owned(),
            form_id: form_id.to_owned(),
            args: vec![],
            title: title.to_owned(),
            fields: vec![],
            button_text: button_text.to_owned(),
        }
    }

    pub fn add_field(mut self, field: &Field) -> Self {
        let mut fields = self.fields;
        fields.push(field.to_owned());

        self.fields = fields;
        self
    }

    pub fn add_fields(mut self, fields: &Vec<Field>) -> Self {
        let mut fields = fields.to_owned();
        self.fields.append(&mut fields);
        self
    }

    pub fn add_arg(mut self, arg: &str) -> Self {
        let mut args = self.args;
        args.push(arg.to_owned());

        self.args = args;
        self
    }
}

impl Field {
    pub fn new_text_field(id: &str, title: &str, description: &str, field: &TextField) -> Self {
        Self {
            field_type: FieldType::Text,
            id: id.to_owned(),
            args: vec![],
            title: title.to_owned(),
            description: description.to_owned(),
            text_field: Some(field.to_owned()),
            text_area_field: None,
            select_field: None,
            switch_field: None,
            slider_field: None,
            file_system_field: None,
        }
    }

    pub fn new_text_area_field(
        id: &str,
        title: &str,
        description: &str,
        field: &TextAreaField,
    ) -> Self {
        Self {
            field_type: FieldType::TextArea,
            id: id.to_owned(),
            args: vec![],
            title: title.to_owned(),
            description: description.to_owned(),
            text_field: None,
            text_area_field: Some(field.to_owned()),
            select_field: None,
            switch_field: None,
            slider_field: None,
            file_system_field: None,
        }
    }

    pub fn new_select_field(id: &str, title: &str, description: &str, field: &SelectField) -> Self {
        Self {
            field_type: FieldType::Select,
            id: id.to_owned(),
            args: vec![],
            title: title.to_owned(),
            description: description.to_owned(),
            text_field: None,
            text_area_field: None,
            select_field: Some(field.to_owned()),
            switch_field: None,
            slider_field: None,
            file_system_field: None,
        }
    }

    pub fn new_switch_field(id: &str, title: &str, description: &str, field: &SwitchField) -> Self {
        Self {
            field_type: FieldType::Switch,
            id: id.to_owned(),
            args: vec![],
            title: title.to_owned(),
            description: description.to_owned(),
            text_field: None,
            text_area_field: None,
            select_field: None,
            switch_field: Some(field.to_owned()),
            slider_field: None,
            file_system_field: None,
        }
    }

    pub fn new_slider_field(id: &str, title: &str, description: &str, field: &SliderField) -> Self {
        Self {
            field_type: FieldType::Slider,
            id: id.to_owned(),
            args: vec![],
            title: title.to_owned(),
            description: description.to_owned(),
            text_field: None,
            text_area_field: None,
            select_field: None,
            switch_field: None,
            slider_field: Some(field.to_owned()),
            file_system_field: None,
        }
    }

    pub fn new_file_system_field(
        id: &str,
        title: &str,
        description: &str,
        field: &FileSystemField,
    ) -> Self {
        Self {
            field_type: FieldType::FileSystem,
            id: id.to_owned(),
            args: vec![],
            title: title.to_owned(),
            description: description.to_owned(),
            text_field: None,
            text_area_field: None,
            select_field: None,
            switch_field: None,
            slider_field: None,
            file_system_field: Some(field.to_owned()),
        }
    }

    pub fn add_arg(mut self, arg: &str) -> Self {
        let mut args = self.args;
        args.push(arg.to_owned());

        self.args = args;
        self
    }
}

impl FieldValidation {
    pub fn new() -> Self {
        Self {
            only_numbers: false,
            not_empty: false,
            max_characters: None,
        }
    }

    /// Makes the text field only accept numbers
    pub fn set_only_numbers(mut self, only_numbers: bool) -> Self {
        self.only_numbers = only_numbers;
        self
    }

    /// Makes the text/text-area/file-system field only valid if it's not empty
    pub fn set_not_empty(mut self, not_empty: bool) -> Self {
        self.not_empty = not_empty;
        self
    }

    /// Sets a max number of characters on text field
    pub fn set_max_characters(mut self, max_characters: usize) -> Self {
        self.max_characters = Some(max_characters);
        self
    }
}

impl TextField {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
            placeholder: None,
            validation: None,
        }
    }

    pub fn set_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_owned());
        self
    }

    pub fn set_validation(mut self, validation: &FieldValidation) -> Self {
        self.validation = Some(validation.to_owned());
        self
    }
}

impl TextAreaField {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
            placeholder: None,
            validation: None,
        }
    }

    pub fn set_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_owned());
        self
    }

    pub fn set_validation(mut self, validation: &FieldValidation) -> Self {
        self.validation = Some(validation.to_owned());
        self
    }
}

impl SelectField {
    pub fn new(value: &str, values: &Vec<SelectFieldValue>) -> Self {
        Self {
            value: value.to_owned(),
            values: values.to_owned(),
        }
    }
}

impl SelectFieldValue {
    pub fn new(id: &str, text: &str) -> Self {
        Self {
            id: id.to_owned(),
            text: text.to_owned(),
        }
    }
}

impl SwitchField {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl SliderField {
    pub fn new(value: usize, min_value: usize, max_value: usize, step: usize) -> Self {
        Self {
            value,
            min_value,
            max_value,
            step,
        }
    }
}

impl FileSystemField {
    pub fn new(value: &Path) -> Self {
        let value = value.to_owned();

        Self {
            value: value.into_os_string().into_string().unwrap(),
            pick_directory: false,
            filters: None,
            validation: None,
        }
    }

    pub fn set_pick_directory(mut self, pick_directory: bool) -> Self {
        self.pick_directory = pick_directory;
        self
    }

    pub fn add_filter(mut self, file_extension: &str) -> Self {
        let mut filters = self.filters.unwrap_or(vec![]);
        filters.push(file_extension.to_owned());

        self.filters = Some(filters);
        self
    }

    pub fn set_not_empty(mut self, not_empty: bool) -> Self {
        self.validation = if not_empty {
            Some(FieldValidation::new().set_not_empty(true))
        } else {
            None
        };

        self
    }
}

impl RunExtensionAction {
    pub fn new(extension_id: &str, extension_action: &str) -> Self {
        Self {
            extension_id: extension_id.to_owned(),
            extension_action: extension_action.to_owned(),
            args: vec![],
        }
    }

    pub fn add_arg(mut self, arg: &str) -> Self {
        let mut args = self.args;
        args.push(arg.to_owned());

        self.args = args;
        self
    }
}
