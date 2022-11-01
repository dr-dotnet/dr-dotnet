using System;
using System.Net.Http;
using DrDotnet;
using MatBlazor;
using Microsoft.Extensions.DependencyInjection;
using Photino.Blazor;

namespace Photino.Blazor.Sample
{
    class Program
    {
        [STAThread]
        static void Main(string[] args) {
            var appBuilder = PhotinoBlazorAppBuilder.CreateDefault(args);

            appBuilder.Services.AddLogging();

            //appBuilder.Services.AddRazorPages();
            //appBuilder.Services.AddServerSideBlazor();
            appBuilder.Services.AddMatBlazor();

            appBuilder.Services.AddSingleton<HttpClient>();
            appBuilder.Services.AddSingleton<ILogger, Logger>();
            appBuilder.Services.AddSingleton<ISessionDiscovery, SessionDiscovery>();
            appBuilder.Services.AddSingleton<IProcessDiscovery, ProcessDiscovery>();
            appBuilder.Services.AddSingleton<IProfilerDiscovery, ProfilersDiscovery>();

            // register root component and selector
            appBuilder.RootComponents.Add<DrDotnet.App>("app");

            var app = appBuilder.Build();

            // customize window
            app.MainWindow
                .SetIconFile("favicon.ico")
                .SetTitle("Photino Blazor Sample");

            AppDomain.CurrentDomain.UnhandledException += (sender, error) => {
                app.MainWindow.OpenAlertWindow("Fatal exception", error.ExceptionObject.ToString());
            };

            app.Run();

        }
    }
}
