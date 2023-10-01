use crate::model::conversation::Conversation;

use cfg_if::cfg_if;
use leptos::*;

#[server(Converse "/api")]
pub async fn converse(prompt: Conversation) -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::Data;
    use leptos_actix::extract;

    use llm::models::Llama;
    use llm::KnownModel;

    let model =
        extract(|data: Data<Llama>, _connection: ConnectionInfo| async move { data.into_inner() })
            .await
            .unwrap();

    let system_prompt = "A chat between a curious human and an artificial intelligence assistant. The assistant gives helpful, detailed, and polite answers to the human's questions.";
    let user_name = "### Human";
    let bot_name = "### Assistant";
    let mut history = format!(
        "{bot_name}:Hello! How may I help you today?\n\
        {user_name}:What is the capital of Taiwan?\n\
        {bot_name}:Taipei is the capital of Taiwan.\n"
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{user_name}:{msg}\n")
        } else {
            format!("{bot_name}:{msg}\n")
        };

        history.push_str(&curr_line);
    }

    let mut res = String::new();
    let mut buf = String::new();

    // use the model to generate text from a prompt
    let mut session = model.start_session(Default::default());

    session
        .infer(
            // model to use for text generation
            model.as_ref(),
            // randomness provider
            &mut rand::thread_rng(),
            // the prompt to use for text generation, as well as other
            // inference parameters
            &llm::InferenceRequest {
                prompt: format!("{system_prompt}\n{history}\n{bot_name}:")
                    .as_str()
                    .into(),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            // llm::OutputRequest
            &mut Default::default(),
            // output callback
            inference_callback(String::from(user_name), &mut buf, &mut res),
        )
        .unwrap_or_else(|e| panic!("{e}"));

    Ok(res)
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
    use std::convert::Infallible;

        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            out_str: &'a mut String,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
            move |resp| match resp {
                llm::InferenceResponse::InferredToken(token) => {
                    // We've generated a token, so we need to check if it's contained in the stop sequence.
                    let mut stop_sequence_buf = buf.clone();
                    stop_sequence_buf.push_str(token.as_str());

                    if stop_sequence.as_str().eq(stop_sequence_buf.as_str()) {
                        // We've generated the stop sequence, so we're done.
                        // Note that this will contain the extra tokens that were generated after the stop sequence,
                        // which may affect generation. This is non-ideal, but it's the best we can do without
                        // modifying the model.
                        buf.clear();
                        return Ok::<llm::InferenceFeedback, Infallible>(llm::InferenceFeedback::Halt);
                    } else if stop_sequence.as_str().starts_with(stop_sequence_buf.as_str()) {
                        // We've generated a prefix of the stop sequence, so we need to keep buffering.
                        buf.push_str(token.as_str());
                        return Ok(llm::InferenceFeedback::Continue);
                    }

                    // We've generated a token that isn't part of the stop sequence, so we can
                    // pass it to the callback.
                    if buf.is_empty() {
                        out_str.push_str(&token);
                    } else {
                        out_str.push_str(&stop_sequence_buf);
                    }

                    Ok(llm::InferenceFeedback::Continue)
                }
                llm::InferenceResponse::EotToken => Ok(llm::InferenceFeedback::Halt),
                _ => Ok(llm::InferenceFeedback::Continue),
            }
        }
    }
}
