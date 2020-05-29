use crate::*;

pub struct Executor;

impl Executor {
    pub fn execute<'a>(
        cw: &'a dyn Command<'a>,
        table: MainTable,
    ) -> AppResult<(CommandInto, Vec<EventObj>)> {
        match cw.command_type() {
            CommandTypes::Set => Ok(Executor::set(cw, table)?),
            CommandTypes::Get => Ok(Executor::get(cw, table)?),
            _ => Err(AppError::InvalidCaommdnForServer),
        }
    }

    pub fn get<'a>(
        cw: &'a dyn Command<'a>,
        table: MainTable,
    ) -> AppResult<(CommandInto, Vec<EventObj>)> {
        let mut args = cw.args();
        if let Some(path_arg) = args.get() {
            if let Some(path) = match path_arg.data()? {
                Value::Path(p) => Some(p),
                Value::String(s) => {
                    let mut vec = VecDeque::new();
                    vec.push_back(s);
                    Some(vec)
                }
                _ => None, //todo: return error
            } {
                return Ok((
                    CommGen::result(
                        table
                            .read()
                            .unwrap()
                            .get(path.clone())
                            .map(|val| val.clone())
                            .unwrap_or(Value::Null),
                    ),
                    Vec::with_capacity(0),
                ));
            }
        }
        Err(AppError::GetError)
    }

    pub fn set<'a>(
        cw: &'a dyn Command<'a>,
        table: MainTable,
    ) -> AppResult<(CommandInto, Vec<EventObj>)> {
        let mut args = cw.args();
        if let (Some(path_arg), Some(val_arg)) = (args.get(), args.get()) {
            if let Some(path) = match path_arg.data()? {
                Value::Path(p) => Some(p),
                Value::String(s) => {
                    let mut vec = VecDeque::new();
                    vec.push_back(s);
                    Some(vec)
                }
                _ => None, //todo: return error
            } {
                let val = &val_arg.data()?;
                table.write().unwrap().set(path.clone(), val);
                return Ok((
                    CommandInto::new_raw(
                        CommandTypes::Result,
                        vec![DataInto::new_raw(DataTypes::Bool, Value::Bool(true))],
                    ),
                    /* vec![EventObj::new(path, Event::Change(val.clone()))] */
                    Vec::with_capacity(0),
                ));
            } else {
                Err(AppError::SetError)
            }
        } else {
            Err(AppError::SetError)
        }
    }
}
