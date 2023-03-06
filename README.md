# Using OpenAI APIs from Rust applications

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](/LICENSE)
[![Twitter: elbruno](https://img.shields.io/twitter/follow/elbruno.svg?style=social)](https://twitter.com/kartben)
![GitHub: elbruno](https://img.shields.io/github/followers/elbruno?style=social)

🧠 OpenAI APIs can be used with the Rust programming language to provide developers with access to powerful artificial intelligence systems. 

🦀 The Rust programming language is an open source language that has been designed with safety and speed in mind. 

⚙️ To use OpenAI APIs with Rust, developers must first install the Rust compiler, Rust libraries, and the OpenAI SDK. Once these components are installed, developers can invoke OpenAI APIs using standard HTTP POST and GET actions.

## Resources
- [Take your first steps with Rust](https://learn.microsoft.com/training/paths/rust-first-steps/)
- [Setup Rust Development Environment](https://learn.microsoft.com/training/modules/rust-set-up-environment/)
- [OpenAI API Introduction](https://platform.openai.com/docs/introduction)
- [OpenAI API Text Completions](https://platform.openai.com/docs/guides/completion)
- [OpenAI API Image Generation](https://platform.openai.com/docs/guides/images)
- [OpenAI API Chat Completion](https://platform.openai.com/docs/guides/chat)

 
## Scenarios

### 1. Text Completions

You can view a sample console app using GPT-3 Completions code in the "[src/openai_apis_completions/](src/openai_apis_completions/)" directory in this repository.

### 2. Image Generation

You can view a sample console app using DALL-E image generation model code in the "[src/openai_apis_image_generations/](src/openai_apis_image_generations/)" directory in this repository.

### 3. ChatGPT API in a chat console app

You can view a sample console app that runs as a chat, using ChatGPT model {gpt-3.5-turbo}. The code for this sample is in the "[src/openai_apis_chatgpt/](src/openai_apis_chatgpt/)" directory in this repository.

_Disclaimer: This scenario does not implement a multi-turn conversations model. Right now, the scenario is a single-turn Q&A without any conversations._

## Testing scenarios 1 and 2

Cargo run is the easiest way to test the OpenAI Completions and the Image Generation sample apps. 

In scenarios 1 and 2, the console app receives 2 parametres: API Key and Prompt.

In example:

```bash
❯ cargo run <OpenAI API Key> "<Prompt>"
```

This is a sample output for the **text completion** scenario

```bash
cargo run <API Key> "generate a 2 paragraph description on how to use OpenAI APIs using Rust programming language"
    Finished dev [unoptimized + debuginfo] target(s) in 3.04s
     
Prompt: generate a 2 paragraph description on how to use OpenAI APIs using Rust programming language
🔥 Success!
💬 Response:

OpenAI APIs can be used with the Rust programming language to provide developers with access to powerful artificial intelligence systems. ...
```

This is a sample output for the **image generarion** scenario

```bash
cargo run <API Key> "a cat playing soccer on the moon"
    Finished dev [unoptimized + debuginfo] target(s) in 0.83s
     
Prompt: a cat playing soccer on mars
🔥 Success!
💬 Response: https://<location of the generated image>

```

And this is the generated image of a cat playing soccer on the moon:

![a cat playing soccer on the moon](img/catmoonsoccer.png "a cat playing soccer on the moon")

## Testing scenario 3

Cargo run is the easiest way to test the OpenAI ChatGPT sample app. 

In scenario 3, the console app receives 1 parametres: API Key.

In example:

```bash
❯ cargo run <OpenAI API Key>
```

This is a sample output for the **chatgpt console app** scenario

```bash
Welcome to ChatGPT API demo in Rust!
 
====================================
Please enter a question or [exit] to stop:
2 + 2
Your question: 2 + 2

🔥 Success!
💬 Response: 2 + 2 equals 4.

====================================
Please enter a question or [exit] to stop:
what is the rust programming language?
Your question: what is the rust programming language?

🔥 Success!
💬 Response: Rust is a system programming language that was initially developed by Mozilla in 2010. It is designed to be safe, concurrent, and fast while also being memory efficient. Rust was created to address the challenges of writing secure and scalable system software, by providing modern language features such as ownership, borrowing, and lifetimes, which help to prevent common programming errors such as null pointer dereferencing, data races, and buffer overflows. Rust is open source and has a growing community contributing to its development and adoption.

====================================
Please enter a question or [exit] to stop:
exit
Exiting...
```

## Additional Resources

In my personal blog "[ElBruno.com](https://elbruno.com)", I wrote about several scenarios on how to work and code with [Rust](https://elbruno.com/tag/rust/). 

## Author

👤 **Bruno Capuano**

* Website: https://elbruno.com
* Twitter: [@elbruno](https://twitter.com/elbruno)
* Github: [@elbruno](https://github.com/elbruno)
* LinkedIn: [@elbruno](https://linkedin.com/in/elbruno)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!

Feel free to check [issues page](https://github.com/elbruno/RustOpenAIAPIs/issues).

## Show your support

Give a ⭐️ if this project helped you!


## 📝 License

Copyright &copy; 2021 [Bruno Capuano](https://github.com/elbruno).

This project is [MIT](/LICENSE) licensed.

***
