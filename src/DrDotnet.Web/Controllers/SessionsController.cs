using System;
using System.Collections.Generic;
using System.IO;
using System.IO.Compression;
using System.Linq;
using DrDotnet.Utils;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;

namespace DrDotnet.Web.Controllers;

[ApiController]
[Route("api/[controller]")]
public class SessionsController : ControllerBase
{
    private readonly ISessionDiscovery _sessionDiscovery;
    private readonly IProfilerDiscovery _profilerDiscovery;
    private readonly IProcessDiscovery _processDiscovery;
    private readonly ILogger<SessionsController> _logger;

    public SessionsController(
        ISessionDiscovery sessionDiscovery,
        IProfilerDiscovery profilerDiscovery,
        IProcessDiscovery processDiscovery,
        ILogger<SessionsController> logger)
    {
        ArgumentNullException.ThrowIfNull(sessionDiscovery);
        ArgumentNullException.ThrowIfNull(profilerDiscovery);
        ArgumentNullException.ThrowIfNull(processDiscovery);
        ArgumentNullException.ThrowIfNull(logger);
        _sessionDiscovery = sessionDiscovery;
        _profilerDiscovery = profilerDiscovery;
        _processDiscovery = processDiscovery;
        _logger = logger;
    }

    [HttpGet]
    public ActionResult<IReadOnlyList<SessionInfo>> GetSessions()
    {
        return Ok(_sessionDiscovery.GetSessions());
    }
    
    [HttpGet("{sessionGuid}")]
    public ActionResult<IReadOnlyList<SessionInfo>> GetSession(Guid sessionGuid)
    {
        if (!_sessionDiscovery.TryGetSession(sessionGuid, out SessionInfo? sessionInfo))
        {
            return NotFound($"No session found with guid '{sessionGuid}'");
        }

        return Ok(sessionInfo);
    }
    
    [HttpGet("{sessionGuid}/download")]
    public ActionResult<IReadOnlyList<SessionInfo>> DownloadSessionResults(Guid sessionGuid)
    {
        if (!_sessionDiscovery.TryGetSession(sessionGuid, out SessionInfo? sessionInfo))
        {
            return NotFound($"No session found with guid '{sessionGuid}'");
        }
        
        var memoryStream = new MemoryStream();
        using (var archive = new ZipArchive(memoryStream, ZipArchiveMode.Create, leaveOpen: true))
        {
            foreach (var file in sessionInfo.EnumerateReports())
            {
                archive.CreateEntryFromFile(file.FullName, file.Name);
            }
        }

        memoryStream.Position = 0;

        return File(memoryStream, "application/zip", $"session-{sessionGuid}.zip");
    }
    
    [HttpPost]
    public ActionResult<SessionInfo> CreateSession([FromBody] CreateSessionDto createSessionDto)
    {
        ProfilerInfo? profiler = _profilerDiscovery.GetProfilers().FirstOrDefault(x => x.Guid == createSessionDto.ProfilerGuid);
        if (profiler == null)
        {
            return NotFound($"No profiler found with guid '{createSessionDto.ProfilerGuid}'");
        }
        if (!_processDiscovery.TryGetProcessInfoFromPid(createSessionDto.ProcessId, out ProcessInfo? processInfo))
        {
            return NotFound($"No process found with pid '{createSessionDto.ProcessId}'");
        }

        foreach (var parameter in createSessionDto.Parameters)
        {
            var matchingParameter = profiler.Parameters.FirstOrDefault(x => x.Key == parameter.Key);
            if (matchingParameter == null)
            {
                return NotFound($"No parameter found with key '{parameter.Key}'");
            }
            // Don't bother checking which value type it is, just set them all
            matchingParameter.ValueBoolean = parameter.ValueBoolean;
            matchingParameter.ValueInt32 = parameter.ValueInt32;
            matchingParameter.ValueFloat32 = parameter.ValueFloat32;
            matchingParameter.Value = parameter.Value;
        }

        return Ok(ProfilingExtensions.StartProfilingSession(profiler, processInfo, _logger));
    }

    public record CreateSessionDto
    {
        public Guid ProfilerGuid { get; init; }
        public int ProcessId { get; init; }
        public ProfilerParameter[] Parameters { get; init; } = Array.Empty<ProfilerParameter>();
    }
}