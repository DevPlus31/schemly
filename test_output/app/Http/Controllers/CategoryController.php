<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Models\Category;
use App\Http\Resources\CategoryResource;

class CategoryController extends Controller
{
    public function index()
    {
        return CategoryResource::collection(Category::all());
    }

    public function show(Category $category)
    {
        return new CategoryResource($category); 
    }

    public function store(Request $request)
    {
        $validated = $request->validate([
            'userId' => 'required',
            'name' => 'required',
            'user' => 'required',
            'links' => 'required',
        ]);

        $category = Category::create($validated);
        return new CategoryResource($category); 
    }

    public function update(Request $request, Category $category)
    {
        $validated = $request->validate([
            'userId' => 'required',
            'name' => 'required',
            'user' => 'required',
            'links' => 'required',
        ]);

        $category->update($validated);
        return new CategoryResource($category); 
    }

    public function destroy(Category $category)
    {
        $category->delete();
        return response()->json(null, 204);
    }
}
