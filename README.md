# GPTO

This template should help get you started with GPTo.
GPTo is a text listener that will help you ask direct question to chat GPT.

## About

This tool was build as a side project in order to help everybody, on all plateform to have access to this kind of tool. 

I'm not a developper, but I have some knowledge. 
This is also my first application using Tauri and Rust so there's a lot of concept I didn't catch well and maybe also a lot of mistakes in my code, but as I'm learning I'm trying to improve it and make it better everyday.

<table align="center">
  <tr>
    <td><img src="https://user-images.githubusercontent.com/9827392/234107902-0c2d1178-7576-4e09-8bb8-54988bce6af9.gif" width="350" height="250"/>
</td>
    <td><img src="https://user-images.githubusercontent.com/9827392/234108945-5e403a34-67f8-43d5-8dd4-1d72d19137d5.gif" width="350" height="250"/>
</td>
  </tr>
  <tr>
    <td align="center" colspan="2"><img src="https://user-images.githubusercontent.com/9827392/234108509-9ca75d9f-a6d7-40a5-afb5-32bcfae6b181.gif"/>
</td>
  </tr>
</table>


## Status

For now the I'm listing all the bugs and improvement in the Issue of this repository (and there's a lot). 
If you are a expert in Rust and Tauri, and you want to help or contribute to the project feel free to do so. 
As I've mentioned I'm not a dev, nor a Rust/tauri expert and all help is welcome.

Check the issues if you want more information about what's working and what's still not 100%.

## Contribute

If you want to contribute feel free to update the code and submit merge requests. 
As I said it's a side project and I will update it whenever I have time.

# HOW TO USE IT

Install Tauri: https://tauri.app/
Then Git clone the repository, you can then start the project using ````yarn tauri dev````
You can also use ````yarn tauri build```` and install the application, but there's a issue I need to resolve. 
In the main folder the config.json file is missing so you need to create it by hand until the issue is resolved: https://github.com/thomaslc66/gpto/issues/1

## API KEY
You will need to login in to chat GPT developper console and create an API-KEY. 
You have 3 free months so you have pleny of time to use the tool

When connected click on your profile and click here:

![image](https://user-images.githubusercontent.com/9827392/234066321-33c5b6dc-88bc-4ff5-bfcb-6856fd685f30.png)

Then click on Create a new secret key button and give it a name:

![image](https://user-images.githubusercontent.com/9827392/234066468-0558a450-5737-4a2c-b2a9-3ad183fb2722.png)

On the next windows just copy the key in the settings of the app:

![image](https://user-images.githubusercontent.com/9827392/234066689-4f6f61b6-b21d-4f15-adfd-0d4d51fda151.png)

![image](https://user-images.githubusercontent.com/9827392/234066797-84ceed16-92ff-4b05-861c-30f1f51442c9.png)

## LICENSE

This was made as an Open-Source project under GNU GENERAL PUBLIC LICENSE.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
