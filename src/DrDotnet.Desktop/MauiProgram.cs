using MatBlazor;
using Microsoft.AspNetCore.Components.WebView.Maui;

namespace DrDotnet.Desktop
{
    public static class MauiProgram
    {
        public static MauiApp CreateMauiApp() {
            var builder = MauiApp.CreateBuilder();
            builder
                .UseMauiApp<App>()
                .ConfigureFonts(fonts => {
                    fonts.AddFont("OpenSans-Regular.ttf", "OpenSansRegular");
                });

            builder.Services.AddMauiBlazorWebView();
#if DEBUG
		    builder.Services.AddBlazorWebViewDeveloperTools();
#endif

            builder.Services.AddMatBlazor();

            //builder.Services.AddSingleton<IWebHostEnvironment, Env>();
            builder.Services.AddSingleton<HttpClient>();
            builder.Services.AddSingleton<ILogger, Logger>();
            builder.Services.AddSingleton<ISessionDiscovery, SessionDiscovery>();
            builder.Services.AddSingleton<IProcessDiscovery, ProcessDiscovery>();
            builder.Services.AddSingleton<IProfilerDiscovery, ProfilersDiscovery>();

            return builder.Build();
        }
    }
}