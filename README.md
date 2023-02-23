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

 
## Sample Code

### Text Completions

You can view a sample console app using GPT-3 Completions code in the "[src/openai_apis_completions/](src/openai_apis_completions/)" directory in this repository.

### Image Generation

You can view a sample console app using DALL-E image generation model code in the "[src/openai_apis_image_generations/](src/openai_apis_image_generations/)" directory in this repository.

## Testing the samples

Cargo run is the easiest way to test the OpenAI Completions sample app. 

The console app receives 2 parametres: API Key and Prompt.

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
