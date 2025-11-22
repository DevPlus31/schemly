<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Database\Eloquent\Factories\HasFactory;

class Link extends Model
{
    use HasFactory, SoftDeletes;

    protected $table = 'links';

    protected $fillable = [
        'userId',
        'categoryId',
        'title',
        'url',
        'isActive',
        'order',
        'user',
        'category'
    ];

    protected $casts = [
        'id' => 'integer',
        'userId' => 'integer',
        'categoryId' => 'integer',
        'isActive' => 'boolean',
        'order' => 'integer',
        'user' => 'integer',
        'category' => 'integer',
    ];

}
