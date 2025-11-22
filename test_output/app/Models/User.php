<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Factories\HasFactory;

class User extends Model
{
    use HasFactory, SoftDeletes;

    protected $table = 'users';

    protected $fillable = [
        'name',
        'email',
        'password',
        'bio',
        'avatarUrl',
        'links',
        'categories'
    ];

    protected $casts = [
        'id' => 'integer',
        'links' => 'integer',
        'categories' => 'integer',
    ];

}
