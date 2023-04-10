<?php

namespace App\Http\Controllers;

use App\Models\CheckIn;
use App\Models\Monitor;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;

class CheckInsController extends Controller
{
    /**
     * Display a listing of the resource.
     */
    public function index()
    {
        //
    }

    /**
     * Store a newly created resource in storage.
     */
    public function store(Request $request)
    {
        $request->validate([
            'monitor_slug' => 'required|string',
            'http_status' => 'required|numeric',
        ]);

        $monitor = Monitor::where('slug', $request->get('monitor_slug'))->firstOrFail();
        $checkIn = $monitor->checkIns()->forceCreate([
            'http_status' => $request->get('http_status'),
        ]);

        return response()->json($checkIn, 201);
    }

    /**
     * Display the specified resource.
     */
    public function show(string $id)
    {
        //
    }

    /**
     * Update the specified resource in storage.
     */
    public function update(Request $request, string $id)
    {
        //
    }

    /**
     * Remove the specified resource from storage.
     */
    public function destroy(string $id)
    {
        //
    }
}
