using DrDotnet.Logging;
using DrDotnet.Utils;
using MatBlazor;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Photino.Blazor;

namespace DrDotnet.Desktop;

class Program
{
    [STAThread]
    static void Main(string[] args)
    {
        var appBuilder = PhotinoBlazorAppBuilder.CreateDefault(args);

        appBuilder.Services.AddLogging();

        //appBuilder.Services.AddRazorPages();
        //appBuilder.Services.AddServerSideBlazor();
        appBuilder.Services.AddMatBlazor();

        appBuilder.Services.AddSingleton<HttpClient>();
        
        appBuilder.Services.AddLogging(lb => lb
            .AddSimpleConsole()
            .AddFileLogger(Path.Combine(PathUtils.DrDotnetBaseDirectory, "app.debug.log")));

        appBuilder.Services.AddSingleton<ISessionDiscovery, SessionsDiscovery>();
        appBuilder.Services.AddSingleton<IProcessDiscovery, ProcessDiscovery>();
        appBuilder.Services.AddSingleton<IProfilerDiscovery, ProfilersDiscovery>();

        // register root component and selector
        appBuilder.RootComponents.Add<App>("app");

        var app = appBuilder.Build();

        // customize window
        app.MainWindow
            .SetIconFile("favicon.ico")
            .SetTitle("Dr-Dotnet");

        AppDomain.CurrentDomain.UnhandledException += (sender, error) => {
            app.MainWindow.OpenAlertWindow("Fatal exception", error.ExceptionObject.ToString());
        };

        app.Run();
    }
}