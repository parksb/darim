from diagrams import Cluster, Diagram, Edge
from diagrams.programming.framework import React
from diagrams.programming.language import Rust
from diagrams.programming.language import TypeScript
from diagrams.onprem.database import MariaDB
from diagrams.onprem.inmemory import Redis

with Diagram("Architecture", show=False):
    storages = [MariaDB("MariaDB"), Redis("Redis")]

    with Cluster("Client"):
        index = React("index.html")

        with Cluster("Pages"):
            pages_index = React("App.tsx")
            pages = pages_index - [React("timeline"), React("post")]
            index >> pages_index

        with Cluster("Components"):
            components_index = TypeScript("index.ts") 
            components = components_index - [React("TextField"), React("Section")]
            for i in range(len(pages)):
                pages[i] >> components_index

        with Cluster("Models"):
            models_index = TypeScript("index.ts")
            models = models_index - [TypeScript("post"), TypeScript("session")]

        with Cluster("API Fetchers"):
            api_fetchers = [TypeScript("fetcher"), TypeScript("user"), TypeScript("post")]

        for i in range(len(pages)):
            for j in range(len(api_fetchers)):
                pages[i] >> api_fetchers[j]

        for i in range(len(pages)):
            pages[i] >> models_index

    with Cluster("Server"):
        main = Rust("main.rs")

        with Cluster("Routes"):
            routes = [Rust("route"), Rust("user"), Rust("post")]
            main >> routes

        with Cluster("Services"):
            services = [Rust("service"), Rust("user"), Rust("post")]
            for i in range(min(len(routes), len(services))):
              routes[i] >> services[i] 

        with Cluster("Models"):
            models = [Rust("model"), Rust("user"), Rust("post")]
            for i in range(len(services)):
                for j in range(len(models)):
                    services[i] >> models[j] 

    for i in range(len(models)):
      for j in range(len(storages)):
        models[i] >> storages[j]

    index >> main
