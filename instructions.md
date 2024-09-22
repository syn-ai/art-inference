This is an api proxy for ComfyUI art generation. It relays a request to the ComfyUI instance and returns the response.
Requirements:

1. API endpoint that accepts a POST request with a JSON body. There is an example of the prompt object [here](prompts/flux.json)
2. Should poll the static drive mounted on the api request endpoint of the ComfyUI Instance.
3. It should dynamically construct the prompt from templates of the components that make up the prompt. The ComfyUI art generator is made up a series of nodes that each have a set of inputs that are used to configure the models and generate the art. So we'll need to map the inputs to the nodes and generate the appropriate prompt object to make the call to Comfy.
4. Should accept prompts otherwise known as workflows and adds any nodes that are missing to its api. 
5. The nodes should be available via the api to get the input fields for a UI being developed in the future. 

We are going to work iteratively on this so construct a proposal for the project along with an overview and breakdown of the components and a high level architecture of the system. 
