<?php
namespace App\Models;

use Illuminate\Database\Eloquent\Concerns\HasUlids;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class CheckIn extends Model
{
    use HasFactory;
    use HasUlids;

    public $timestamps = ["created_at"];
    
    const UPDATED_AT = null;

    public function monitor(): BelongsTo
    {
        return $this->belongsTo(Monitor::class);
    }
}
