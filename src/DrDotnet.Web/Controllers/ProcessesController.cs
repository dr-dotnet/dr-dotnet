using System;
using System.Collections.Generic;
using Microsoft.AspNetCore.Mvc;

namespace DrDotnet.Web.Controllers;

[ApiController]
[Route("api/[controller]")]
public class ProcessesController : ControllerBase
{
    private readonly IProcessDiscovery _processDiscovery;

    public ProcessesController(IProcessDiscovery processDiscovery)
    {
        ArgumentNullException.ThrowIfNull(processDiscovery);
        _processDiscovery = processDiscovery;
    }

    [HttpGet]
    public ActionResult<IReadOnlyList<SessionInfo>> GetProfilers()
    {
        return Ok(_processDiscovery.GetDotnetProcesses(static _ => { }));
    }
}