<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Models\Link;
use App\Http\Resources\LinkResource;

class LinkController extends Controller
{
    public function index()
    {
        return LinkResource::collection(Link::all());
    }

    public function show(Link $link)
    {
        return new LinkResource($link); 
    }

    public function store(Request $request)
    {
        $validated = $request->validate([
            'userId' => 'required',
            'categoryId' => 'nullable',
            'title' => 'required',
            'url' => 'required',
            'isActive' => 'required',
            'order' => 'nullable',
            'user' => 'required',
            'category' => 'nullable',
        ]);

        $link = Link::create($validated);
        return new LinkResource($link); 
    }

    public function update(Request $request, Link $link)
    {
        $validated = $request->validate([
            'userId' => 'required',
            'categoryId' => 'nullable',
            'title' => 'required',
            'url' => 'required',
            'isActive' => 'required',
            'order' => 'nullable',
            'user' => 'required',
            'category' => 'nullable',
        ]);

        $link->update($validated);
        return new LinkResource($link); 
    }

    public function destroy(Link $link)
    {
        $link->delete();
        return response()->json(null, 204);
    }
}
