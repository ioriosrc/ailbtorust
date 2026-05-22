```rust
use yew::{html, Component, Html, Properties};

#[derive(Debug)]
pub struct AlertProps {
    pub severity: &'static str,
    pub title: Option<String>,
    pub message: String,
    pub variant: &'static str,
}

impl Default for AlertProps {
    fn default() -> Self {
        Self {
            severity: "info",
            title: None,
            message: "This is a warning alert!".to_string(),
            variant: "outlined",
        }
    }
}

pub struct Alert {
    props: AlertProps,
}

impl Component for Alert {
    type Properties = AlertProps;
    type Message = ();

    fn create(props: Self::Properties) -> Self {
        Self { props }
    }

    fn update(&mut self, _message: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <AlertComponent severity={self.props.severity} title={self.props.title.clone()} message={self.props.message.clone()} variant={self.props.variant} />
        }
    }
}

#[derive(Debug)]
pub struct AlertComponent {
    props: AlertProps,
}

impl Component for AlertComponent {
    type Properties = AlertProps;
    type Message = ();

    fn create(props: Self::Properties) -> Self {
        Self { props }
    }

    fn update(&mut self, _message: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="alert alert-{props.severity}">
                {if let Some(title) = &self.props.title {
                    html! {<h4>{title}</h4>}
                }
                <p>{props.message}</p>
            </div>
        }
    }
}
```

### Explicação

1. **AlertProps**: Define as propriedades do componente `Alert`, incluindo o tipo de severidade, título opcional, mensagem e variante.

2. **Default for AlertProps**: Implementa a definição padrão para `AlertProps`.

3. **Alert**: É um componente genérico que recebe propriedades do tipo `AlertProps` e renderiza a(alerta) com base nas propriedades fornecidas.

4. **AlertComponent**: É o componente concreto que é responsável por renderizar o alerta específico. Ele aceita o tipo de severidade, título opcional, mensagem e variante como propriedades e renderiza um elemento `div` contendo as informações do alerta.

5. **yew**: O framework usado para criação da interface do usuário em Rust com a abordagem de componentes genéricos (POCs).