use nvim_oxi as oxi;
use oxi::{
    api::{
        opts::CreateCommandOpts,
        types::{CommandArgs, CommandNArgs, WindowConfig, WindowRelativeTo, WindowTitle},
    },
    Dictionary,
};
use tabled::{
    settings::{Height, Width},
    Table,
};

use crate::config::CONFIG;

pub enum UI {
    ListDatabases,
    ShowTable(String),
}

impl UI {
    pub fn show(self) -> oxi::Result<()> {
        match self {
            UI::ShowTable(table) => UI::show_table(table),
            UI::ListDatabases => UI::list_databases_view(),
        }
    }

    pub fn list_databases_view() -> oxi::Result<()> {
        let win = oxi::api::get_current_win();
        let mut buf = oxi::api::create_buf(false, true)?;
        let databases = match CONFIG.get_browser().list_databases() {
            Ok(databases) => databases,
            Err(err) => {
                oxi::api::err_writeln(&format!("Was unable to list databases: {err}"));
                return Ok(());
            }
        };

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Cursor)
            .height(win.get_height().unwrap() - 13)
            .width(win.get_width().unwrap() - 13)
            .row(0)
            .col(0)
            .title(WindowTitle::SimpleString("Databases".into()))
            .border(oxi::api::types::WindowBorder::Double)
            .build();

        for (line, database) in databases.into_iter().enumerate() {
            buf.set_lines(line..=line, false, database.lines())?;
        }

        oxi::api::open_win(&buf, true, &config)?;

        Ok(())
    }

    pub fn show_table(table_name: String) -> oxi::Result<()> {
        let win = oxi::api::get_current_win();
        let mut buf = oxi::api::create_buf(false, true)?;
        let table = match CONFIG.get_browser().table_contents(&table_name) {
            Ok(databases) => databases,
            Err(err) => {
                oxi::api::err_writeln(&format!("Was unable to list databases: {err}"));
                return Ok(());
            }
        };

        let mut table: Table = table.into();
        let (win_width, win_height) = (win.get_width().unwrap(), win.get_height().unwrap());
        table
            .with(Width::truncate((win_width - 20) as usize));

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Cursor)
            .focusable(true)
            .height(win_height - 10)
            .width(win_width - 10)
            .row(0)
            .col(0)
            .title(WindowTitle::SimpleString(table_name.as_str().into()))
            .border(oxi::api::types::WindowBorder::Double)
            .style(oxi::api::types::WindowStyle::Minimal)
            .build();

        let table_str = table.to_string();
        buf.set_lines(0..=table_str.len(), false, table_str.lines())?;
        oxi::api::open_win(&buf, true, &config)?;

        Ok(())
    }
}

#[oxi::module]
pub fn ui() -> oxi::Result<oxi::Dictionary> {
    // Show table command
    let opts = CreateCommandOpts::builder()
        .desc("Show table contents in a.")
        .nargs(CommandNArgs::OneOrMore)
        .build();

    let show_table = |args: CommandArgs| {
        let table = args.args.unwrap_or("".to_owned());

        match UI::ShowTable(table).show() {
            Ok(()) => Ok(()),
            Err(err) => {
                oxi::api::err_writeln(&format!("{err}"));
                Ok(())
            }
        }
    };

    oxi::api::create_user_command("ShowTable", show_table, &opts)?;
    let list_databases_view = oxi::Function::from_fn(move |()| match UI::ListDatabases.show() {
        Ok(()) => oxi::Result::Ok(()),
        Err(err) => {
            oxi::api::err_writeln(&format!("{err}"));
            oxi::Result::Ok(())
        }
    });

    return Ok(Dictionary::from_iter([(
        "list_databases",
        list_databases_view,
    )]));
}
