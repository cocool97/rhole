use std::vec;

use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ListProperties {
    pub header: Vec<&'static str>,
    pub input: Vec<Vec<String>>,
}

// Example can be found here: https://www.w3schools.com/css/css_table.asp
#[function_component]
pub fn InputList(props: &ListProperties) -> Html {
    html! {
        <table style="width: 100%">
            <tbody>
                <InputListHeader header={props.header.to_owned()} />
                <InputListContent values={props.input.to_owned()} />
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
pub struct ListHeaderProperties {
    pub header: Vec<&'static str>,
}

#[function_component]
fn InputListHeader(props: &ListHeaderProperties) -> Html {
    let mut th = vec![];
    for elem in &props.header {
        th.push(html!(
            <th>{elem}</th>
        ))
    }

    html! {
        <tr>
            {th}
        </tr>
    }
}

#[derive(Properties, PartialEq)]
pub struct ListContentProperties {
    pub values: Vec<Vec<String>>,
}

#[function_component]
fn InputListContent(props: &ListContentProperties) -> Html {
    let mut td = vec![];
    for line_values in &props.values {
        let mut val = vec![];
        for value in line_values {
            val.push(html!(<td>{value}</td>));
        }
        td.push(html!(
            <tr>{val}</tr>
        ));
    }

    html! {
        <>
            {td}
        </>
    }
}
