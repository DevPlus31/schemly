<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Models\User;
use App\Http\Resources\UserResource;

class UserController extends Controller
{
    public function index()
    {
        return UserResource::collection(User::all());
    }

    public function show(User $user)
    {
        return new UserResource($user); 
    }

    public function store(Request $request)
    {
        $validated = $request->validate([
            'name' => 'required',
            'email' => 'required',
            'password' => 'required',
            'bio' => 'nullable',
            'avatarUrl' => 'nullable',
            'links' => 'required',
            'categories' => 'required',
        ]);

        $user = User::create($validated);
        return new UserResource($user); 
    }

    public function update(Request $request, User $user)
    {
        $validated = $request->validate([
            'name' => 'required',
            'email' => 'required',
            'password' => 'required',
            'bio' => 'nullable',
            'avatarUrl' => 'nullable',
            'links' => 'required',
            'categories' => 'required',
        ]);

        $user->update($validated);
        return new UserResource($user); 
    }

    public function destroy(User $user)
    {
        $user->delete();
        return response()->json(null, 204);
    }
}
