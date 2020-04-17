use crate::*;

pub struct Executor;

impl Executor {
    pub fn execute<'a>(cw: &'a dyn Command<'a>, table: MainTable) -> AppResult<CommandInto> {
        match cw.command_type() {
            CommandTypes::Set => Ok(Executor::set(cw, table)?),
            _ => Err(AppError::InvalidCaommdnForServer),
        }
    }

    pub fn set<'a>(cw: &'a dyn Command<'a>, table: MainTable) -> AppResult<CommandInto> {
        let mut args = cw.args();
        if let (Some(path_arg), Some(val_arg)) = (args.get(), args.get()) {
            if let Some(path) = match path_arg.data()? {
                Value::Path(p) => Some(p),
                Value::String(s) => {
                    let mut vec = VecDeque::new();
                    vec.push_back(s);
                    Some(vec)
                }
                _ => None,
            } {
                table.lock().unwrap().set(path, &val_arg.data()?);
            }
        }
        Ok(CommandInto::new_raw(
            CommandTypes::Result,
            vec![DataInto::new_raw(DataTypes::Bool, Value::Bool(true))],
        ))
    }
}
